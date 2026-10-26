//! The fixtures are copied as bytes, never interpreted as live observations.
use crate::{ensure, parse_json, pretty, render, sha256, write_new, Result};
use serde_json::Value;
use std::{fs, path::Path};

const MANIFEST: &[u8] = include_bytes!("../../../examples/synthetic-case/manifest.json");
const SOURCE: &[u8] = include_bytes!("../../../examples/synthetic-case/report-source.json");
const FILES: [(&str, &[u8]); 8] = [
    (
        "chain",
        include_bytes!("../../../examples/synthetic-case/evidence/chain.json"),
    ),
    (
        "header",
        include_bytes!("../../../examples/synthetic-case/evidence/header.json"),
    ),
    (
        "recheck",
        include_bytes!("../../../examples/synthetic-case/evidence/recheck.json"),
    ),
    (
        "runtime",
        include_bytes!("../../../examples/synthetic-case/evidence/runtime.json"),
    ),
    (
        "name",
        include_bytes!("../../../examples/synthetic-case/evidence/name.json"),
    ),
    (
        "symbol",
        include_bytes!("../../../examples/synthetic-case/evidence/symbol.json"),
    ),
    (
        "decimals",
        include_bytes!("../../../examples/synthetic-case/evidence/decimals.json"),
    ),
    (
        "total_supply",
        include_bytes!("../../../examples/synthetic-case/evidence/total_supply.json"),
    ),
];
pub fn create(root: &Path) -> Result<(Value, Value)> {
    if root.exists() {
        ensure!(
            root.is_dir()
                && fs::read_dir(root)
                    .map_err(|e| e.to_string())?
                    .next()
                    .is_none(),
            "Fixture destination must be a new or empty directory"
        );
    } else {
        fs::create_dir_all(root).map_err(|e| e.to_string())?;
    }
    fs::create_dir(root.join("evidence")).map_err(|e| e.to_string())?;
    let mut m = parse_json(MANIFEST)?;
    let source = parse_json(SOURCE)?;
    for (name, bytes) in FILES {
        write_new(&root.join("evidence").join(format!("{name}.json")), bytes)?;
    }
    let source_bytes = format!("{}\n", pretty(&source)?).into_bytes();
    m["report_source"]["sha256"] = sha256(&source_bytes).into();
    write_new(&root.join("report-source.json"), &source_bytes)?;
    write_new(
        &root.join("manifest.json"),
        format!("{}\n", pretty(&m)?).as_bytes(),
    )?;
    write_new(
        &root.join("report.md"),
        render::render(&m, &source)?.as_bytes(),
    )?;
    Ok((m, source))
}
