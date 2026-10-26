use crate::{
    address, array, ensure, hash32, hex_bytes, identity, integer, parse_json, quantity,
    read_bounded, sha256, string, text, Result,
};
use chrono::{DateTime, Utc};
use num_bigint::BigUint;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

pub const SURFACES: [&str; 11] = [
    "token_controls",
    "canonical_lp_custody",
    "side_pool_removal",
    "sellability_depth",
    "current_concentration",
    "launch_integrity",
    "admin_treasury_reward_custody",
    "reward_accounting_liveness",
    "utility_redemption",
    "external_dependencies",
    "development_disclosure",
];
const STATUSES: [&str; 5] = [
    "finding",
    "bounded_negative",
    "unknown",
    "not_assessed",
    "not_applicable",
];
const COVERAGE: [&str; 5] = [
    "complete",
    "partial",
    "unavailable",
    "not_assessed",
    "not_applicable",
];
const CONFIDENCE: [&str; 4] = ["high", "medium", "low", "unknown"];
const METHODS: [&str; 11] = [
    "eth_chainId",
    "eth_getBlockByNumber",
    "eth_getBlockByHash",
    "eth_getCode",
    "eth_getStorageAt",
    "eth_call",
    "eth_getBalance",
    "eth_getLogs",
    "eth_getTransactionByHash",
    "eth_getTransactionReceipt",
    "eth_getProof",
];

pub struct Case {
    pub manifest: Value,
    pub source: Value,
}
struct Artifact {
    meta: Value,
    bytes: Vec<u8>,
    rpc: Option<Value>,
}
struct Evidence {
    artifacts: BTreeMap<String, Artifact>,
    pins: BTreeMap<String, Value>,
}

fn is(v: &Value, options: &[&str]) -> bool {
    v.as_str().is_some_and(|s| options.contains(&s))
}
fn required_text(v: &Value, keys: &[&str]) -> Result<()> {
    for key in keys {
        ensure!(text(&v[*key]), "Missing text field: {key}");
    }
    Ok(())
}
fn texts(v: &Value, nonempty: bool) -> Result<()> {
    let a = array(v)?;
    ensure!(
        (!nonempty || !a.is_empty()) && a.iter().all(text),
        "Missing or malformed text list"
    );
    Ok(())
}
fn severity(v: &Value) -> Option<usize> {
    ["none", "low", "medium", "high", "critical"]
        .iter()
        .position(|s| v.as_str() == Some(s))
}
fn checked_file(root: &Path, v: &Value) -> Result<Vec<u8>> {
    let relative = Path::new(string(&v["path"])?);
    ensure!(!relative.is_absolute(), "Artifact path must be relative");
    let path = root
        .join(relative)
        .canonicalize()
        .map_err(|e| format!("Missing evidence file: {e}"))?;
    ensure!(
        path.starts_with(root) && path != root,
        "Artifact escapes case directory"
    );
    let bytes = read_bounded(&path)?;
    ensure!(
        v["sha256"].as_str() == Some(&sha256(&bytes)),
        "Artifact hash mismatch: {}",
        relative.display()
    );
    Ok(bytes)
}
fn pin_matches(tag: &Value, pin: &Value) -> Result<bool> {
    Ok(if tag.is_object() {
        tag["blockHash"].as_str().map(str::to_ascii_lowercase)
            == pin["hash"].as_str().map(str::to_ascii_lowercase)
            && tag["requireCanonical"] == true
    } else {
        tag.as_str() == Some(&format!("0x{:x}", integer(&pin["number"])?))
    })
}
impl Evidence {
    fn artifact(&self, id: &Value) -> Result<&Artifact> {
        self.artifacts
            .get(string(id)?)
            .ok_or_else(|| format!("Unknown evidence reference: {id}"))
    }
    fn rpc(&self, id: &Value, method: Option<&str>, success: bool) -> Result<&Value> {
        let row = self
            .artifact(id)?
            .rpc
            .as_ref()
            .ok_or("Missing raw RPC evidence")?;
        ensure!(
            method.is_none_or(|m| row["request"]["method"] == m),
            "RPC decoding/method mismatch"
        );
        ensure!(
            !success
                || (row["response"].get("result").is_some()
                    && !row["response"]["result"].is_null()),
            "Successful non-null RPC evidence required"
        );
        Ok(row)
    }
    fn pin(&self, id: &Value) -> Result<&Value> {
        self.pins
            .get(string(id)?)
            .ok_or_else(|| "Missing pin".into())
    }
    fn references(&self, refs: &Value, needed: bool) -> Result<()> {
        let refs = array(refs)?;
        ensure!(!needed || !refs.is_empty(), "Missing evidence references");
        for id in refs {
            self.artifact(id)?;
        }
        Ok(())
    }
    fn same_pin_chain(&self, v: &Value) -> Result<()> {
        ensure!(
            self.pin(&v["pin_id"])?["chain_id"] == v["chain_id"],
            "Inconsistent scope chain or pin"
        );
        Ok(())
    }
    fn usable(&self, id: &Value) -> Result<bool> {
        let a = self.artifact(id)?;
        Ok(a.rpc.as_ref().is_none_or(|r| {
            r["response"].get("result").is_some() && !r["response"]["result"].is_null()
        }))
    }
}

pub fn decode_metadata(field: &str, result: &Value) -> Result<Value> {
    let bytes = hex_bytes(result)?;
    if ["decimals", "total_supply"].contains(&field) {
        ensure!(
            bytes.len() == 32,
            "Nonstandard integer metadata must remain unresolved"
        );
        if field == "decimals" {
            ensure!(
                bytes[..31].iter().all(|b| *b == 0),
                "Invalid uint8 decimals"
            );
            return Ok(bytes[31].into());
        }
        return Ok(BigUint::from_bytes_be(&bytes).to_str_radix(10).into());
    }
    ensure!(
        bytes.len() >= 64 && BigUint::from_bytes_be(&bytes[..32]) == BigUint::from(32u8),
        "Nonstandard string metadata must remain unresolved"
    );
    let size = BigUint::from_bytes_be(&bytes[32..64])
        .to_str_radix(10)
        .parse::<usize>()
        .map_err(|_| "Oversized ABI string")?;
    ensure!(size <= bytes.len() - 64, "Truncated ABI string");
    let value = std::str::from_utf8(&bytes[64..64 + size])
        .map_err(|_| "Non-UTF8 metadata must remain unresolved")?;
    ensure!(
        !value.trim().is_empty(),
        "Empty metadata must remain unresolved"
    );
    Ok(value.into())
}

pub fn validate(path: &Path, report: Option<&Path>, allow_synthetic: bool) -> Result<Case> {
    let path = path.canonicalize().map_err(|e| e.to_string())?;
    let root = path.parent().ok_or("Manifest has no parent directory")?;
    let m = parse_json(&read_bounded(&path)?)?;
    ensure!(
        m["schema_version"].as_u64() == Some(1),
        "Unsupported manifest schema"
    );
    ensure!(
        is(&m["evidence_kind"], &["live", "synthetic"]),
        "Missing evidence kind"
    );
    ensure!(
        m["evidence_kind"] != "synthetic" || allow_synthetic,
        "Synthetic fixture rejected: testing requires --allow-synthetic"
    );
    ensure!(
        is(&m["mode"], &["focused", "broad", "formal"]),
        "Invalid operating mode"
    );
    let target = identity(&m["target"]["requested"])?;
    ensure!(
        target.1[2..].bytes().any(|b| b != b'0'),
        "Zero address cannot be token target"
    );
    for label in ["observed", "queried"] {
        ensure!(
            identity(&m["target"][label])? == target,
            "Requested/{label} target mismatch"
        );
    }
    required_text(&m, &["question", "materiality_rules"])?;
    texts(&m["limitations"], false)?;
    let mut e = Evidence {
        artifacts: BTreeMap::new(),
        pins: BTreeMap::new(),
    };
    ensure!(!array(&m["artifacts"])?.is_empty(), "Missing raw evidence");
    for a in array(&m["artifacts"])? {
        required_text(a, &["id", "provenance", "pin_id"])?;
        let id = string(&a["id"])?;
        identity(a)?;
        ensure!(!e.artifacts.contains_key(id), "Duplicate artifact ID");
        ensure!(
            identity(&a["target_binding"])? == target,
            "Artifact target binding mismatch"
        );
        ensure!(
            is(&a["kind"], &["rpc", "source", "document", "fork"]),
            "Unknown artifact kind"
        );
        let bytes = checked_file(root, a)?;
        let rpc = if a["kind"] == "rpc" {
            let r = parse_json(&bytes)?;
            ensure!(
                m["evidence_kind"] != "live"
                    || (r["synthetic"] != true && r["evidence_kind"] != "synthetic"),
                "Synthetic raw evidence cannot be relabeled live"
            );
            let req = &r["request"];
            let res = &r["response"];
            ensure!(
                req["jsonrpc"] == "2.0"
                    && res["jsonrpc"] == "2.0"
                    && !req["id"].is_null()
                    && req["id"] == res["id"],
                "Invalid RPC envelope or response ID mismatch"
            );
            ensure!(is(&req["method"], &METHODS), "Non-allowlisted RPC method");
            array(&req["params"])?;
            ensure!(
                res.get("result").is_some() != res.get("error").is_some(),
                "RPC must have result OR error"
            );
            ensure!(
                r["endpoint"] == "[REDACTED]" && text(&r["endpoint_label"]),
                "Redacted endpoint and provider label required"
            );
            let capture = string(&r["captured_at_utc"])?;
            ensure!(
                capture.ends_with('Z') && DateTime::parse_from_rfc3339(capture).is_ok(),
                "Invalid capture UTC timestamp"
            );
            Some(r)
        } else {
            None
        };
        e.artifacts.insert(
            id.to_owned(),
            Artifact {
                meta: a.clone(),
                bytes,
                rpc,
            },
        );
    }
    ensure!(!array(&m["pins"])?.is_empty(), "Missing block pins");
    for p in array(&m["pins"])? {
        required_text(p, &["id"])?;
        let id = string(&p["id"])?;
        ensure!(
            !e.pins.contains_key(id) && integer(&p["chain_id"])? > 0,
            "Duplicate/invalid pin"
        );
        let number = integer(&p["number"])?;
        ensure!(hash32(&p["hash"]), "Placeholder or malformed block hash");
        ensure!(
            is(&p["time_basis"], &["current", "historical"]),
            "Pin time basis missing"
        );
        let chain = e.rpc(&p["chain_id_artifact"], Some("eth_chainId"), true)?;
        ensure!(
            array(&chain["request"]["params"])?.is_empty()
                && quantity(&chain["response"]["result"])? == integer(&p["chain_id"])?,
            "RPC chain ID disagrees with pin"
        );
        let row = e.rpc(&p["header_artifact"], None, true)?;
        ensure!(
            is(
                &row["request"]["method"],
                &["eth_getBlockByNumber", "eth_getBlockByHash"]
            ),
            "Pin needs captured block header"
        );
        let header = &row["response"]["result"];
        ensure!(
            quantity(&header["number"])? == number && header["hash"] == p["hash"],
            "Fake/inconsistent pin disagrees with captured header"
        );
        let timestamp = i64::try_from(quantity(&header["timestamp"])?)
            .map_err(|_| "Block timestamp overflow")?;
        let utc = DateTime::<Utc>::from_timestamp(timestamp, 0)
            .ok_or("Invalid header timestamp")?
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        ensure!(
            p["timestamp_utc"] == utc,
            "Pin UTC timestamp disagrees with header"
        );
        let params = array(&row["request"]["params"])?;
        ensure!(
            params.len() == 2 && params[1] == false,
            "Malformed header query"
        );
        let expected = if row["request"]["method"] == "eth_getBlockByHash" {
            string(&p["hash"])?.to_owned()
        } else {
            format!("0x{number:x}")
        };
        ensure!(
            params[0] == expected
                || (p["time_basis"] == "current"
                    && is(&params[0], &["latest", "safe", "finalized"])),
            "Header query does not identify pin"
        );
        for id in [&p["header_artifact"], &p["chain_id_artifact"]] {
            let a = &e.artifact(id)?.meta;
            ensure!(
                a["chain_id"] == p["chain_id"] && a["pin_id"] == p["id"],
                "Pin evidence chain mismatch"
            );
        }
        if p["time_basis"] == "current" {
            let r = e.rpc(
                &p["header_recheck_artifact"],
                Some("eth_getBlockByNumber"),
                true,
            )?;
            ensure!(
                r["request"]["params"] == serde_json::json!([format!("0x{number:x}"), false]),
                "Header recheck must use explicit number"
            );
            ensure!(
                r["response"]["result"] == *header,
                "Header recheck changed: possible reorg"
            );
            let a = &e.artifact(&p["header_recheck_artifact"])?.meta;
            ensure!(
                a["chain_id"] == p["chain_id"] && a["pin_id"] == p["id"],
                "Header recheck chain mismatch"
            );
        }
        e.pins.insert(id.to_owned(), p.clone());
    }
    let current = e.pin(&m["current_pin"])?;
    ensure!(
        integer(&current["chain_id"])? == target.0 && current["time_basis"] == "current",
        "Missing target current pin"
    );
    for a in e.artifacts.values() {
        e.same_pin_chain(&a.meta)?;
        if let Some(r) = &a.rpc {
            validate_query(&e, a, r)?;
        }
    }
    let mut scope = BTreeMap::new();
    for s in array(&m["scope"])? {
        let key = identity(s)?;
        ensure!(
            !scope.contains_key(&key) && text(&s["role"]) && s["material"].is_boolean(),
            "Invalid/duplicate scope entry"
        );
        e.same_pin_chain(s)?;
        e.references(&s["provenance"], true)?;
        let runtime = &s["runtime"];
        ensure!(
            is(&runtime["status"], &["code", "no_code", "unresolved"]),
            "Missing runtime status"
        );
        if runtime["status"] == "unresolved" {
            required_text(runtime, &["reason"])?;
            e.references(&runtime["evidence"], false)?;
        } else {
            let a = e.artifact(&runtime["artifact"])?;
            ensure!(
                identity(&a.meta)? == key && a.meta["pin_id"] == s["pin_id"],
                "Runtime query identity mismatch"
            );
            let row = e.rpc(&runtime["artifact"], Some("eth_getCode"), true)?;
            let bytes = hex_bytes(&row["response"]["result"])?;
            ensure!(
                (runtime["status"] == "no_code") == bytes.is_empty(),
                "Runtime presence conflicts with bytes"
            );
            ensure!(runtime["sha256"] == sha256(&bytes), "Runtime hash mismatch");
        }
        let proxy = &s["proxy"];
        ensure!(
            is(
                &proxy["status"],
                &["resolved", "not_detected", "unresolved"]
            ),
            "Missing proxy status"
        );
        required_text(proxy, &["basis"])?;
        e.references(&proxy["evidence"], proxy["status"] != "unresolved")?;
        array(&proxy["related_scope"])?;
        scope.insert(key, s);
    }
    let target_scope = scope.get(&target).ok_or("Target missing from scope")?;
    ensure!(
        target_scope["material"] == true && target_scope["pin_id"] == m["current_pin"],
        "Target missing from material current scope"
    );
    for a in e.artifacts.values() {
        ensure!(
            scope.contains_key(&identity(&a.meta)?),
            "Artifact subject not inventoried in scope"
        );
    }
    for s in scope.values() {
        for related in array(&s["proxy"]["related_scope"])? {
            ensure!(
                scope.contains_key(&identity(related)?),
                "Uninventoried implementation/beacon/authority"
            );
        }
    }
    let code_id = &target_scope["runtime"]["artifact"];
    ensure!(
        *code_id == m["target"]["queried"]["code_artifact"],
        "Queried target runtime binding mismatch"
    );
    e.rpc(code_id, Some("eth_getCode"), false)?;
    let code_meta = &e.artifact(code_id)?.meta;
    ensure!(
        identity(code_meta)? == target && code_meta["pin_id"] == m["current_pin"],
        "Queried runtime anchor has wrong subject/pin"
    );
    let metadata = m["metadata"].as_object().ok_or("Missing metadata")?;
    let selectors = [
        ("name", "0x06fdde03"),
        ("symbol", "0x95d89b41"),
        ("decimals", "0x313ce567"),
        ("total_supply", "0x18160ddd"),
    ];
    ensure!(
        metadata.len() == 4
            && selectors
                .iter()
                .all(|(field, _)| metadata.contains_key(*field)),
        "Metadata fields must be explicit"
    );
    for (field, selector) in selectors {
        let v = &metadata[field];
        ensure!(
            is(&v["status"], &["resolved", "unresolved"]),
            "Invalid metadata status"
        );
        e.references(&v["evidence"], v["status"] == "resolved")?;
        if v["status"] == "unresolved" {
            ensure!(
                v["value"].is_null() && text(&v["reason"]),
                "Unresolved metadata needs null value and reason"
            );
            continue;
        }
        let refs = array(&v["evidence"])?;
        ensure!(
            refs.len() == 1,
            "Resolved metadata needs one raw target call"
        );
        let a = &e.artifact(&refs[0])?.meta;
        ensure!(
            identity(a)? == target && a["pin_id"] == m["current_pin"],
            "Metadata from wrong target/time"
        );
        let row = e.rpc(&refs[0], Some("eth_call"), true)?;
        ensure!(
            row["request"]["params"][0]["data"] == selector,
            "Metadata selector conflict"
        );
        ensure!(
            decode_metadata(field, &row["response"]["result"])? == v["value"],
            "Metadata differs from target RPC"
        );
    }
    validate_safety(&m, &e)?;
    let source = parse_json(&checked_file(root, &m["report_source"])?)?;
    ensure!(
        identity(&source["target"])? == target,
        "Report-source identity differs from manifest"
    );
    for field in [
        "metadata",
        "mode",
        "question",
        "evidence_kind",
        "current_pin",
    ] {
        ensure!(
            source[field] == m[field],
            "Report {field} differs from manifest"
        );
    }
    required_text(&source, &["verdict"])?;
    ensure!(
        is(
            &source["decision"],
            &["conditional_go", "no_go", "inconclusive", "focused_answer"]
        ),
        "Missing conditional decision"
    );
    for k in [
        "conditions",
        "reasons",
        "contrary_evidence",
        "unknowns",
        "evidence_to_change",
        "recommendations",
    ] {
        texts(&source[k], true)?;
    }
    validate_findings(&m, &source, &e, &scope)?;
    let discoveries = array(&m["discovery"])?;
    ensure!(
        !discoveries.is_empty() || m["mode"] == "focused",
        "Broad report needs discovery coverage"
    );
    for d in discoveries {
        ensure!(
            scope.contains_key(&identity(d)?),
            "Discovery identity outside scope"
        );
        e.same_pin_chain(d)?;
        required_text(
            d,
            &[
                "claim",
                "search_universe",
                "pagination",
                "inclusion_rules",
                "exclusions",
                "materiality",
            ],
        )?;
        ensure!(
            integer(&d["from_block"])? <= integer(&d["to_block"])?
                && integer(&d["to_block"])? <= integer(&e.pin(&d["pin_id"])?["number"])?,
            "Discovery block range invalid"
        );
        ensure!(is(&d["coverage"], &COVERAGE), "Discovery coverage missing");
        e.references(
            &d["evidence"],
            !is(&d["coverage"], &["unavailable", "not_assessed"]),
        )?;
    }
    if let Some(report) = report {
        ensure!(
            read_bounded(report)? == crate::render::render(&m, &source)?.as_bytes(),
            "Rendered report differs from validated source/target; regenerate it"
        );
    }
    Ok(Case {
        manifest: m,
        source,
    })
}

fn validate_query(e: &Evidence, a: &Artifact, r: &Value) -> Result<()> {
    let method = string(&r["request"]["method"])?;
    let params = array(&r["request"]["params"])?;
    let pin = e.pin(&a.meta["pin_id"])?;
    match method {
        "eth_getCode" | "eth_getStorageAt" | "eth_getBalance" | "eth_call" | "eth_getProof" => {
            let count = if ["eth_getStorageAt", "eth_getProof"].contains(&method) {
                3
            } else {
                2
            };
            ensure!(
                params.len() == count,
                "Unexpected state query parameters/overrides"
            );
            let subject = if method == "eth_call" {
                &params[0]["to"]
            } else {
                &params[0]
            };
            ensure!(
                address(subject)
                    && string(subject)?.eq_ignore_ascii_case(string(&a.meta["address"])?),
                "Queried address differs from artifact subject"
            );
            ensure!(
                pin_matches(&params[count - 1], pin)?,
                "Unpinned or wrong-block state query"
            );
        }
        "eth_getLogs" => {
            ensure!(
                params.len() == 1 && params[0].is_object(),
                "Malformed log query"
            );
            let f = &params[0];
            let subjects = if let Some(list) = f["address"].as_array() {
                list.clone()
            } else {
                vec![f["address"].clone()]
            };
            ensure!(
                !subjects.is_empty()
                    && subjects.iter().all(address)
                    && subjects
                        .iter()
                        .any(|v| v.as_str().is_some_and(|s| s
                            .eq_ignore_ascii_case(a.meta["address"].as_str().unwrap_or_default()))),
                "Log address universe missing subject"
            );
            if f.get("blockHash").is_some() {
                ensure!(f["blockHash"] == pin["hash"], "Log block hash mismatch");
            } else {
                ensure!(
                    quantity(&f["fromBlock"])? <= quantity(&f["toBlock"])?
                        && quantity(&f["toBlock"])? <= integer(&pin["number"])?,
                    "Invalid/unpinned log range"
                );
            }
        }
        "eth_getTransactionByHash" | "eth_getTransactionReceipt" => {
            ensure!(
                params.len() == 1 && hash32(&params[0]),
                "Transaction query needs exact hash"
            );
            let result = &r["response"]["result"];
            if !result.is_null() {
                ensure!(
                    result["blockHash"] == pin["hash"]
                        && quantity(&result["blockNumber"])? == integer(&pin["number"])?,
                    "Transaction needs matching historical pin"
                );
                let key = if method == "eth_getTransactionReceipt" {
                    "transactionHash"
                } else {
                    "hash"
                };
                ensure!(result[key] == params[0], "Transaction hash conflict");
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_safety(m: &Value, e: &Evidence) -> Result<()> {
    let safety = &m["safety"];
    for k in [
        "no_real_private_keys",
        "no_real_signing",
        "no_broadcast",
        "external_content_untrusted",
    ] {
        ensure!(safety[k] == true, "Missing safety declaration: {k}");
    }
    ensure!(
        safety["simulation_used"].is_boolean(),
        "Simulation use must be explicit"
    );
    let forks = array(&safety["forks"])?;
    ensure!(
        safety["simulation_used"] == !forks.is_empty(),
        "Simulation/fork declarations conflict"
    );
    for f in forks {
        for k in [
            "disposable",
            "synthetic_accounts_only",
            "loopback_only",
            "no_upstream_writes",
            "counterfactual",
        ] {
            ensure!(f[k] == true, "Fork restriction missing: {k}");
        }
        let upstream = e.pin(&f["upstream_pin"])?;
        ensure!(
            integer(&f["local_chain_id"])? > 0,
            "Missing local fork chain"
        );
        required_text(f, &["process_identity", "overrides", "disposal"])?;
        e.references(&f["verification_artifacts"], true)?;
        let mut verified = false;
        for id in array(&f["verification_artifacts"])? {
            let a = e.artifact(id)?;
            if a.meta["kind"] != "fork" {
                continue;
            }
            let config = parse_json(&a.bytes)?;
            ensure!(
                local_endpoint(string(&config["local_endpoint"])?),
                "Fork endpoint must be literal loopback without credentials"
            );
            for k in [
                "disposable",
                "synthetic_accounts_only",
                "loopback_only",
                "no_upstream_writes",
                "counterfactual",
                "process_identity",
                "local_chain_id",
                "upstream_pin",
                "overrides",
                "disposal",
            ] {
                ensure!(config[k] == f[k], "Fork config/declaration mismatch: {k}");
            }
            ensure!(
                config["upstream_chain_id"] == upstream["chain_id"]
                    && config["upstream_block_hash"] == upstream["hash"],
                "Fork upstream identity mismatch"
            );
            verified = true;
        }
        ensure!(
            verified,
            "Fork lacks captured local-process/config evidence"
        );
    }
    Ok(())
}
fn local_endpoint(value: &str) -> bool {
    let value = value.strip_suffix('/').unwrap_or(value);
    ["http://127.0.0.1", "http://[::1]"].iter().any(|base| {
        value == *base
            || value.strip_prefix(&format!("{base}:")).is_some_and(|port| {
                !port.is_empty()
                    && port.bytes().all(|b| b.is_ascii_digit())
                    && port.parse::<u16>().is_ok_and(|n| n > 0)
            })
    })
}

fn validate_findings(
    m: &Value,
    source: &Value,
    e: &Evidence,
    scope: &BTreeMap<(u64, String), &Value>,
) -> Result<()> {
    let mut findings = BTreeMap::new();
    for f in array(&source["findings"])? {
        required_text(f, &["id", "proposition", "time_basis", "stale_when"])?;
        let id = string(&f["id"])?;
        ensure!(
            !findings.contains_key(id) && scope.contains_key(&identity(f)?),
            "Duplicate finding ID or identity outside scope"
        );
        e.same_pin_chain(f)?;
        ensure!(
            is(&f["status"], &STATUSES) && is(&f["coverage"], &COVERAGE),
            "Invalid finding status/coverage; unknown is not a pass"
        );
        ensure!(
            is(
                &f["evidence_type"],
                &["fact", "strongly_supported", "inference", "unknown"]
            ) && is(&f["confidence"], &CONFIDENCE),
            "Missing epistemic status"
        );
        ensure!(
            severity(&f["severity"]).is_some()
                || is(&f["severity"], &["unknown", "not_applicable"]),
            "Finding severity missing"
        );
        texts(&f["alternatives"], true)?;
        let refs = array(&f["evidence"])?;
        let claims = is(&f["status"], &["finding", "bounded_negative"]);
        ensure!(!claims || !refs.is_empty(), "Missing evidence references");
        let mut usable = false;
        for r in refs {
            required_text(r, &["decoding_basis"])?;
            let can_use = e.usable(&r["artifact_id"])?;
            usable |= can_use;
            if f["status"] == "bounded_negative" {
                ensure!(can_use, "Failed/null RPC cannot support a pass");
            }
        }
        if claims {
            ensure!(
                usable
                    && f["evidence_type"] != "unknown"
                    && is(&f["coverage"], &["complete", "partial"]),
                "Unknown/error/skipped evidence presented as finding/pass"
            );
            ensure!(
                f["confidence"] != "unknown" && severity(&f["severity"]).is_some(),
                "Claim confidence/severity unresolved"
            );
        }
        match string(&f["status"])? {
            "finding" => ensure!(
                f["severity"] != "none",
                "Risk finding cannot have no severity"
            ),
            "bounded_negative" => ensure!(
                f["severity"] == "none" && f["coverage"] == "complete",
                "Unknown/partial check presented as pass"
            ),
            status => {
                required_text(f, &["reason"])?;
                ensure!(
                    f["severity"]
                        == if status == "not_applicable" {
                            "not_applicable"
                        } else {
                            "unknown"
                        },
                    "Unknown check assigned favorable severity"
                );
            }
        }
        let claim_kind = f
            .get("claim_kind")
            .map(string)
            .transpose()?
            .unwrap_or("observation");
        ensure!(
            ["observation", "executed_exit"].contains(&claim_kind),
            "Invalid claim_kind"
        );
        let mut receipt_ids = Vec::new();
        if !f["transaction"].is_null() {
            ensure!(hash32(&f["transaction"]), "Malformed finding transaction");
            for r in refs {
                let a = e.artifact(&r["artifact_id"])?;
                if let Some(row) = &a.rpc {
                    if row["request"]["method"] == "eth_getTransactionReceipt"
                        && row["response"]["result"]["transactionHash"] == f["transaction"]
                    {
                        ensure!(
                            a.meta["chain_id"] == f["chain_id"] && a.meta["pin_id"] == f["pin_id"],
                            "Finding receipt chain/pin mismatch"
                        );
                        receipt_ids.push(r["artifact_id"].clone());
                        if f["status"] == "bounded_negative" || claim_kind == "executed_exit" {
                            ensure!(
                                quantity(&row["response"]["result"]["status"])? == 1,
                                "Reverted receipt cannot support successful exit/no-risk claim"
                            );
                        }
                    }
                }
            }
            ensure!(
                !receipt_ids.is_empty(),
                "Transaction claim lacks matching receipt"
            );
        }
        if claim_kind == "executed_exit" {
            ensure!(
                claims && !receipt_ids.is_empty(),
                "Executed exit requires transaction and receipt"
            );
            validate_exit(m, f, e, &receipt_ids)?;
        } else {
            ensure!(
                f.get("execution").is_none(),
                "Execution proof requires claim_kind executed_exit"
            );
        }
        findings.insert(id.to_owned(), f);
    }
    ensure!(!findings.is_empty(), "Missing finding-to-evidence ledger");
    let mut surfaces = BTreeSet::new();
    let mut referenced = BTreeSet::new();
    for r in array(&source["ratings"])? {
        let surface = string(&r["surface"])?;
        ensure!(
            SURFACES.contains(&surface) && surfaces.insert(surface.to_owned()),
            "Unknown/duplicate surface rating"
        );
        ensure!(
            is(&r["status"], &STATUSES) && is(&r["coverage"], &COVERAGE),
            "Invalid rating status/coverage"
        );
        ensure!(
            severity(&r["severity"]).is_some()
                || is(&r["severity"], &["unknown", "not_applicable"]),
            "Missing severity"
        );
        ensure!(
            is(
                &r["likelihood"],
                &[
                    "observed",
                    "likely",
                    "plausible",
                    "unlikely",
                    "unknown",
                    "not_applicable"
                ]
            ) && is(&r["confidence"], &CONFIDENCE),
            "Rating likelihood/confidence missing"
        );
        required_text(r, &["time_basis"])?;
        let ids = array(&r["finding_ids"])?;
        ensure!(!ids.is_empty(), "Rating lacks ledger references");
        for id in ids {
            let id = string(id)?;
            let f = findings.get(id).ok_or("Rating refers to unknown finding")?;
            referenced.insert(id.to_owned());
            if let Some(s) = f.get("surface") {
                ensure!(*s == surface, "Finding linked to wrong surface");
            }
            if r["status"] == "bounded_negative" || r["severity"] == "none" {
                ensure!(
                    r["status"] == "bounded_negative"
                        && r["coverage"] == "complete"
                        && f["status"] == "bounded_negative",
                    "Unknown or risky check presented as a pass"
                );
            }
            if f["status"] == "finding" {
                ensure!(
                    r["status"] == "finding",
                    "Finding was averaged away by rating"
                );
                ensure!(
                    severity(&r["severity"])
                        .is_some_and(|s| s >= severity(&f["severity"]).unwrap_or(usize::MAX)),
                    "Material finding severity was averaged down"
                );
            }
        }
        if is(&r["status"], &["unknown", "not_assessed"]) {
            ensure!(
                r["severity"] == "unknown",
                "Unknown assessment assigned favorable severity"
            );
        }
    }
    ensure!(
        findings.keys().all(|id| referenced.contains(id)),
        "Orphan finding omitted from every surface rating"
    );
    if m["mode"] != "focused" {
        ensure!(
            surfaces.len() == SURFACES.len(),
            "Broad/formal report must rate all eleven surfaces"
        );
    }
    Ok(())
}

fn balance_value(
    e: &Evidence,
    id: &Value,
    asset: &str,
    recipient: &str,
    chain: &Value,
) -> Result<BigUint> {
    let a = &e.artifact(id)?.meta;
    ensure!(a["chain_id"] == *chain, "Underlying balance chain mismatch");
    if asset == "native" {
        let r = e.rpc(id, Some("eth_getBalance"), true)?;
        ensure!(
            string(&a["address"])?.eq_ignore_ascii_case(recipient),
            "Underlying recipient mismatch"
        );
        let raw = string(&r["response"]["result"])?;
        let hex = raw.strip_prefix("0x").ok_or("Invalid native balance")?;
        ensure!(
            !hex.is_empty() && hex.len() <= 64 && hex.bytes().all(|b| b.is_ascii_hexdigit()),
            "Invalid native balance"
        );
        BigUint::parse_bytes(hex.as_bytes(), 16).ok_or_else(|| "Invalid native balance".into())
    } else {
        let r = e.rpc(id, Some("eth_call"), true)?;
        ensure!(
            string(&a["address"])?.eq_ignore_ascii_case(asset),
            "Underlying asset mismatch"
        );
        let calldata = format!(
            "0x70a08231{}{}",
            "0".repeat(24),
            recipient[2..].to_ascii_lowercase()
        );
        ensure!(
            string(&r["request"]["params"][0]["data"])?.eq_ignore_ascii_case(&calldata),
            "Underlying balanceOf recipient mismatch"
        );
        let bytes = hex_bytes(&r["response"]["result"])?;
        ensure!(bytes.len() == 32, "Underlying balance must be uint256");
        Ok(BigUint::from_bytes_be(&bytes))
    }
}
fn validate_exit(m: &Value, f: &Value, e: &Evidence, receipts: &[Value]) -> Result<()> {
    let proof = &f["execution"];
    required_text(proof, &["route", "costs", "attribution_basis"])?;
    ensure!(
        is(&proof["basis"], &["historical", "fork"]),
        "Execution basis required"
    );
    if proof["basis"] == "fork" {
        ensure!(
            m["safety"]["simulation_used"] == true && proof["counterfactual"] == true,
            "Fork exit must be declared counterfactual"
        );
    }
    ensure!(
        receipts.contains(&proof["receipt_artifact"]),
        "Exit receipt not bound to finding"
    );
    let receipt = e.rpc(
        &proof["receipt_artifact"],
        Some("eth_getTransactionReceipt"),
        true,
    )?;
    ensure!(
        quantity(&receipt["response"]["result"]["status"])? == 1,
        "Exit receipt was not successful"
    );
    let asset = string(&proof["underlying_asset"])?;
    let recipient = string(&proof["recipient"])?;
    ensure!(
        (asset == "native" || address(&proof["underlying_asset"])) && address(&proof["recipient"]),
        "Exit asset/recipient invalid"
    );
    let before_id = &proof["balance_before_artifact"];
    let after_id = &proof["balance_after_artifact"];
    let ids: Vec<_> = array(&f["evidence"])?
        .iter()
        .map(|r| &r["artifact_id"])
        .collect();
    ensure!(
        ids.contains(&before_id) && ids.contains(&after_id),
        "Exit balances missing from finding evidence"
    );
    let before = balance_value(e, before_id, asset, recipient, &f["chain_id"])?;
    let after = balance_value(e, after_id, asset, recipient, &f["chain_id"])?;
    let before_pin = e.pin(&e.artifact(before_id)?.meta["pin_id"])?;
    let after_pin = e.pin(&e.artifact(after_id)?.meta["pin_id"])?;
    let receipt_number = quantity(&receipt["response"]["result"]["blockNumber"])?;
    ensure!(
        integer(&before_pin["number"])? < receipt_number
            && integer(&after_pin["number"])? >= receipt_number,
        "Exit balance pins do not bracket execution"
    );
    let minimum_raw = string(&proof["minimum_received_raw"])?;
    ensure!(
        !minimum_raw.is_empty()
            && minimum_raw.len() <= 78
            && minimum_raw.bytes().all(|b| b.is_ascii_digit()),
        "Exit minimum must be raw unsigned integer"
    );
    let minimum = BigUint::parse_bytes(minimum_raw.as_bytes(), 10).ok_or("Invalid exit minimum")?;
    ensure!(
        minimum > BigUint::from(0u8) && after > before && after - before >= minimum,
        "No intended underlying balance increase meeting minimum"
    );
    Ok(())
}
