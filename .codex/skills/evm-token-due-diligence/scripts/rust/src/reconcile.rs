use crate::{address, array, ensure, string, text, Result};
use num_bigint::{BigInt, Sign};
use serde_json::{json, Value};
use std::collections::BTreeSet;

#[derive(Clone)]
struct Decimal {
    units: BigInt,
    scale: u32,
}
fn decimal(v: &Value, signed: bool) -> Result<Decimal> {
    let value = string(v)?;
    ensure!(
        !value.is_empty() && value.len() <= 2048,
        "Amounts must be bounded decimal strings"
    );
    let (negative, magnitude) = value
        .strip_prefix('-')
        .map(|s| (true, s))
        .unwrap_or((false, value));
    let parts: Vec<_> = magnitude.split('.').collect();
    ensure!(
        (1..=2).contains(&parts.len())
            && parts
                .iter()
                .all(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())),
        "Invalid decimal amount; no floats, exponents or nonfinite values"
    );
    let scale = parts.get(1).map_or(0, |s| s.len() as u32);
    let raw = parts.join("");
    let mut units = BigInt::parse_bytes(raw.as_bytes(), 10).ok_or("Invalid decimal amount")?;
    if negative {
        units = -units;
    }
    ensure!(
        signed || units.sign() != Sign::Minus,
        "Unsigned flow amount is negative"
    );
    Ok(Decimal { units, scale })
}
fn scaled(d: &Decimal, scale: u32) -> BigInt {
    &d.units * BigInt::from(10u8).pow(scale - d.scale)
}
fn display(value: &BigInt, scale: u32) -> String {
    let raw = value.magnitude().to_str_radix(10);
    let mut digits = if scale == 0 {
        raw
    } else {
        let raw = format!("{:0>width$}", raw, width = scale as usize + 1);
        let cut = raw.len() - scale as usize;
        format!("{}.{}", &raw[..cut], &raw[cut..])
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_owned()
    };
    if value.sign() == Sign::Minus {
        digits.insert(0, '-');
    }
    digits
}
pub fn reconcile(data: &Value) -> Result<Value> {
    let assets = array(&data["assets"])?;
    ensure!(!assets.is_empty(), "Missing assets");
    let mut identities = BTreeSet::new();
    let mut results = Vec::new();
    let mut all_within = true;
    for a in assets {
        let asset = string(&a["asset"])?;
        ensure!(
            asset == "native" || address(&a["asset"]),
            "Invalid asset identity"
        );
        let chain = crate::integer(&a["chain_id"])?;
        ensure!(chain > 0 && text(&a["unit"]), "Missing asset chain/unit");
        let key = (
            chain,
            asset.to_ascii_lowercase(),
            string(&a["unit"])?.to_owned(),
        );
        ensure!(identities.insert(key), "Duplicate asset identity or unit");
        let opening = decimal(&a["opening"], false)?;
        let closing = decimal(&a["closing"], false)?;
        let bound = decimal(&a["unexplained_bound"], false)?;
        let mut scale = opening.scale.max(closing.scale).max(bound.scale);
        let mut seen = BTreeSet::new();
        let mut flows = Vec::new();
        for row in array(&a["entries"])? {
            ensure!(
                text(&row["id"]) && seen.insert(string(&row["id"])?.to_owned()),
                "Duplicate or missing flow ID"
            );
            let kind = string(&row["kind"])?;
            ensure!(
                ["inflow", "outflow", "adjustment"].contains(&kind),
                "Invalid flow kind"
            );
            ensure!(
                text(&row["explanation"])
                    && !array(&row["evidence"])?.is_empty()
                    && array(&row["evidence"])?.iter().all(text),
                "Missing flow evidence/explanation"
            );
            let amount = decimal(&row["amount"], kind == "adjustment")?;
            scale = scale.max(amount.scale);
            flows.push((kind, amount));
        }
        let mut incoming = BigInt::from(0u8);
        let mut outgoing = BigInt::from(0u8);
        let mut adjustments = BigInt::from(0u8);
        for (kind, amount) in flows {
            let amount = scaled(&amount, scale);
            match kind {
                "inflow" => incoming += amount,
                "outflow" => outgoing += amount,
                _ => adjustments += amount,
            }
        }
        let opening = scaled(&opening, scale);
        let closing = scaled(&closing, scale);
        let bound = scaled(&bound, scale);
        let residual = &opening + &incoming + &adjustments - &outgoing - &closing;
        let within = residual.magnitude() <= bound.magnitude();
        all_within &= within;
        results.push(json!({"chain_id": chain, "asset": asset, "unit": a["unit"], "opening": display(&opening, scale),
            "inflows": display(&incoming, scale), "adjustments": display(&adjustments, scale), "outflows": display(&outgoing, scale),
            "closing": display(&closing, scale), "unexplained_delta": display(&residual, scale), "unexplained_bound": display(&bound, scale), "within_bound": within}));
    }
    Ok(json!({"assets": results, "all_within_bound": all_within,
        "limitation": "Arithmetic only; classifications, completeness, transformations, ownership, and fee origin require evidence review."}))
}
