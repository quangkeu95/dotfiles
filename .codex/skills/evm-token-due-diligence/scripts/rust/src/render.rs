use crate::{array, ensure, pretty, string, Result};
use serde_json::{json, Value};

/// Treat all case text as data, never Markdown/HTML instructions. Character
/// references are emitted before Markdown parsing, not by trusting viewer filters.
pub fn escape(value: &str) -> String {
    let mut output = String::new();
    for c in value.chars() {
        if c.is_ascii_punctuation()
            || c.is_control()
            || matches!(c, '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
        {
            output.push_str(&format!("&#x{:x};", c as u32));
        } else {
            output.push(c);
        }
    }
    output
}
fn displayed(v: &Value) -> String {
    escape(
        v.as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| v.to_string())
            .as_str(),
    )
}
fn json_block(v: &Value) -> Result<String> {
    // Keep even code-fence and raw HTML payloads inert, while retaining JSON meaning.
    Ok(format!(
        "```json\n{}\n```",
        pretty(v)?
            .replace('`', "\\u0060")
            .replace('<', "\\u003c")
            .replace('>', "\\u003e")
    ))
}
pub fn render(m: &Value, source: &Value) -> Result<String> {
    let pin = array(&m["pins"])?
        .iter()
        .find(|p| p["id"] == source["current_pin"])
        .ok_or("Missing report pin")?;
    let target = &source["target"];
    let mut out = vec![
        "# EVM token due diligence".into(),
        String::new(),
        displayed(&source["verdict"]),
        String::new(),
        format!(
            "Evidence kind: **{}**. Decision: {}.",
            escape(&string(&source["evidence_kind"])?.to_uppercase()),
            displayed(&source["decision"])
        ),
        String::new(),
        format!(
            "Target: chain {}, address {}.",
            displayed(&target["chain_id"]),
            displayed(&target["address"])
        ),
        format!(
            "Current pin: block {}, hash {}, UTC {}.",
            displayed(&pin["number"]),
            displayed(&pin["hash"]),
            displayed(&pin["timestamp_utc"])
        ),
        String::new(),
        format!("Question: {}", displayed(&source["question"])),
        String::new(),
        "## Metadata".into(),
        String::new(),
    ];
    for field in ["name", "symbol", "decimals", "total_supply"] {
        let v = &source["metadata"][field];
        let label = if v["status"] == "resolved" {
            displayed(&v["value"])
        } else {
            format!("Unresolved: {}", displayed(&v["reason"]))
        };
        out.push(format!("- {}: {label}", escape(field)));
    }
    for (field, heading) in [
        ("conditions", "Conditions"),
        ("reasons", "Reasons"),
        ("contrary_evidence", "Contrary evidence"),
        ("unknowns", "Unknowns"),
        ("evidence_to_change", "Evidence to change"),
        ("recommendations", "Recommendations"),
    ] {
        out.extend([String::new(), format!("## {heading}"), String::new()]);
        for v in array(&source[field])? {
            out.push(format!("- {}", displayed(v)));
        }
    }
    out.extend([String::new(), "## Separate surface assessments".into(), String::new(),
        "| Surface | Status | Severity | Likelihood | Confidence | Coverage | Time basis | Findings |".into(),
        "| --- | --- | --- | --- | --- | --- | --- | --- |".into()]);
    for r in array(&source["ratings"])? {
        let mut cells: Vec<_> = [
            "surface",
            "status",
            "severity",
            "likelihood",
            "confidence",
            "coverage",
            "time_basis",
        ]
        .iter()
        .map(|k| displayed(&r[*k]))
        .collect();
        cells.push(
            array(&r["finding_ids"])?
                .iter()
                .map(displayed)
                .collect::<Vec<_>>()
                .join(", "),
        );
        out.push(format!("| {} |", cells.join(" | ")));
    }
    out.extend([
        String::new(),
        "## Finding-to-evidence ledger".into(),
        String::new(),
    ]);
    for f in array(&source["findings"])? {
        out.extend([
            format!("### {}", displayed(&f["id"])),
            String::new(),
            displayed(&f["proposition"]),
            String::new(),
            json_block(f)?,
            String::new(),
        ]);
    }
    out.extend(["## Discovery coverage".into(), String::new(), json_block(&m["discovery"])?, String::new(),
        "## Pins, scope, and artifact inventory".into(), String::new(),
        json_block(&json!({"pins": m["pins"], "scope": m["scope"], "artifacts": m["artifacts"], "materiality_rules": m["materiality_rules"], "limitations": m["limitations"], "safety": m["safety"]}))?, String::new(),
        "Validation checks internal consistency only. It does not establish RPC honesty, discovery completeness, or protocol safety.".into(), String::new()]);
    let result = out.join("\n");
    ensure!(
        result.len() as u64 <= crate::MAX_FILE_BYTES,
        "Rendered report exceeds 32 MiB limit"
    );
    Ok(result)
}
