use evm_diligence::{
    fixture, parse_json, pretty, read_json, reconcile::reconcile, render, sha256,
    validate::validate,
};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{fs, path::PathBuf, process::Command};
use tempfile::TempDir;

const TOKEN: &str = "0x1234567890abcdef1234567890abcdef12345678";
const OTHER: &str = "0xabcdef1234567890abcdef1234567890abcdef12";
struct Fixture {
    dir: TempDir,
    m: Value,
    s: Value,
}
impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let (m, s) = fixture::create(dir.path()).unwrap();
        Self { dir, m, s }
    }
    fn path(&self, p: &str) -> PathBuf {
        self.dir.path().join(p)
    }
    fn sync(&mut self) {
        let bytes = pretty(&self.s).unwrap().into_bytes();
        fs::write(self.path("report-source.json"), &bytes).unwrap();
        self.m["report_source"]["sha256"] = sha256(&bytes).into();
        fs::write(self.path("manifest.json"), pretty(&self.m).unwrap()).unwrap();
    }
    fn rpc(&mut self, id: &str, edit: impl FnOnce(&mut Value)) {
        let i = self.m["artifacts"]
            .as_array()
            .unwrap()
            .iter()
            .position(|a| a["id"] == id)
            .unwrap();
        let path = self
            .dir
            .path()
            .join(self.m["artifacts"][i]["path"].as_str().unwrap());
        let mut row = read_json(&path).unwrap();
        edit(&mut row);
        let bytes = pretty(&row).unwrap().into_bytes();
        fs::write(path, &bytes).unwrap();
        self.m["artifacts"][i]["sha256"] = sha256(&bytes).into();
    }
    fn reject(&mut self, message: &str) {
        self.sync();
        let error = validate(&self.path("manifest.json"), None, true)
            .err()
            .expect("malicious/inconsistent fixture was accepted");
        assert!(
            error.contains(message),
            "expected {message:?}, got {error:?}"
        );
    }
    fn accept(&mut self) {
        self.sync();
        validate(&self.path("manifest.json"), None, true).unwrap();
    }
    fn add_rpc(
        &mut self,
        id: &str,
        method: &str,
        params: Value,
        result: Value,
        address: &str,
        pin: &str,
    ) {
        let row = json!({"synthetic": true, "endpoint": "[REDACTED]", "endpoint_label": "offline fixture",
            "captured_at_utc": "2023-11-14T22:13:21Z", "request": {"jsonrpc":"2.0","id":id,"method":method,"params":params},
            "response":{"jsonrpc":"2.0","id":id,"result":result}});
        let bytes = pretty(&row).unwrap().into_bytes();
        let path = format!("evidence/{id}.json");
        fs::write(self.path(&path), &bytes).unwrap();
        let meta = json!({"id":id,"path":path,"sha256":sha256(&bytes),"kind":"rpc","chain_id":1,"address":address,
            "pin_id":pin,"target_binding":self.m["target"]["requested"],"provenance":"Synthetic regression fixture"});
        self.m["artifacts"].as_array_mut().unwrap().push(meta);
    }
    fn negative(&mut self, index: usize, evidence: Value) {
        let f = &mut self.s["findings"][index];
        f["status"] = "bounded_negative".into();
        f["severity"] = "none".into();
        f["confidence"] = "high".into();
        f["evidence_type"] = "fact".into();
        f["coverage"] = "complete".into();
        f["evidence"] = evidence;
        let r = &mut self.s["ratings"][index];
        r["status"] = "bounded_negative".into();
        r["severity"] = "none".into();
        r["coverage"] = "complete".into();
    }
    fn receipt(&mut self, status: &str) -> Value {
        let tx = format!("0x{}", sha256(b"SYNTHETIC RECEIPT"));
        self.add_rpc("receipt", "eth_getTransactionReceipt", json!([tx]),
            json!({"transactionHash":tx,"blockHash":self.m["pins"][0]["hash"],"blockNumber":"0x1e240","status":status,"logs":[]}), TOKEN, "pin-1");
        tx.into()
    }
    fn fork(&mut self) {
        let fork = json!({"disposable":true,"synthetic_accounts_only":true,"loopback_only":true,"no_upstream_writes":true,
            "counterfactual":true,"upstream_pin":"pin-1","local_chain_id":31337,"process_identity":"Synthetic offline process",
            "overrides":"Synthetic gas only","disposal":"No real process exists","verification_artifacts":["fork"]});
        let mut config = fork.clone();
        config["local_endpoint"] = "http://127.0.0.1:8545".into();
        config["upstream_chain_id"] = 1.into();
        config["upstream_block_hash"] = self.m["pins"][0]["hash"].clone();
        let bytes = pretty(&config).unwrap().into_bytes();
        fs::write(self.path("evidence/fork.json"), &bytes).unwrap();
        let mut a = self.m["artifacts"][0].clone();
        a["id"] = "fork".into();
        a["kind"] = "fork".into();
        a["path"] = "evidence/fork.json".into();
        a["sha256"] = sha256(&bytes).into();
        self.m["artifacts"].as_array_mut().unwrap().push(a);
        self.m["safety"]["simulation_used"] = true.into();
        self.m["safety"]["forks"] = json!([fork]);
    }
    fn exit(&mut self) {
        let tx = self.receipt("0x1");
        let hash = format!("0x{}", sha256(b"SYNTHETIC BEFORE PIN"));
        self.add_rpc(
            "before-chain",
            "eth_chainId",
            json!([]),
            json!("0x1"),
            TOKEN,
            "pin-0",
        );
        self.add_rpc(
            "before-header",
            "eth_getBlockByNumber",
            json!(["0x1e23f", false]),
            json!({"number":"0x1e23f","hash":hash,"timestamp":"0x6553f0ff"}),
            TOKEN,
            "pin-0",
        );
        self.m["pins"].as_array_mut().unwrap().push(json!({"id":"pin-0","chain_id":1,"number":123455,"hash":hash,
            "timestamp_utc":"2023-11-14T22:13:19Z","time_basis":"historical","header_artifact":"before-header","chain_id_artifact":"before-chain"}));
        self.add_rpc(
            "asset-code",
            "eth_getCode",
            json!([OTHER, "0x1e240"]),
            json!("0x6001600055"),
            OTHER,
            "pin-1",
        );
        let mut scope = self.m["scope"][0].clone();
        scope["address"] = OTHER.into();
        scope["role"] = "Synthetic underlying asset".into();
        scope["runtime"]["artifact"] = "asset-code".into();
        scope["provenance"] = json!(["asset-code"]);
        self.m["scope"].as_array_mut().unwrap().push(scope);
        let calldata = format!("0x70a08231{}{}", "0".repeat(24), &TOKEN[2..]);
        self.add_rpc(
            "balance-before",
            "eth_call",
            json!([{"to":OTHER,"data":calldata},"0x1e23f"]),
            json!(word(1)),
            OTHER,
            "pin-0",
        );
        self.add_rpc(
            "balance-after",
            "eth_call",
            json!([{"to":OTHER,"data":calldata},"0x1e240"]),
            json!(word(21)),
            OTHER,
            "pin-1",
        );
        self.negative(3, json!([{"artifact_id":"receipt","decoding_basis":"Synthetic receipt"},
            {"artifact_id":"balance-before","decoding_basis":"balanceOf recipient, uint256"},{"artifact_id":"balance-after","decoding_basis":"balanceOf recipient, uint256"}]));
        let f = &mut self.s["findings"][3];
        f["transaction"] = tx;
        f["claim_kind"] = "executed_exit".into();
        f["execution"] = json!({"basis":"historical","receipt_artifact":"receipt","underlying_asset":OTHER,"recipient":TOKEN,
            "balance_before_artifact":"balance-before","balance_after_artifact":"balance-after","minimum_received_raw":"10",
            "route":"Synthetic route only","costs":"Synthetic gas excluded from token amount","attribution_basis":"Synthetic receipts and isolated balance observations"});
    }
}
fn word(value: u64) -> String {
    format!("0x{value:064x}")
}
fn abi_string(s: &str) -> String {
    let mut value = format!("0x{:064x}{:064x}", 32, s.len());
    for byte in s.bytes() {
        value.push_str(&format!("{byte:02x}"));
    }
    value.push_str(&"00".repeat((32 - s.len() % 32) % 32));
    value
}
macro_rules! reject_test {
    ($name:ident, $f:ident, $body:block, $message:literal) => {
        #[test]
        fn $name() {
            let mut $f = Fixture::new();
            $body;
            $f.reject($message);
        }
    };
}

#[test]
fn fixture_and_render_are_consistent() {
    let f = Fixture::new();
    validate(&f.path("manifest.json"), Some(&f.path("report.md")), true).unwrap();
}
#[test]
fn synthetic_requires_opt_in() {
    let f = Fixture::new();
    assert!(validate(&f.path("manifest.json"), None, false)
        .err()
        .unwrap()
        .contains("Synthetic fixture rejected"));
}
reject_test!(
    same_symbol_other_chain,
    f,
    {
        f.m["target"]["queried"]["chain_id"] = 10.into();
    },
    "Requested/queried"
);
reject_test!(
    wrong_report_target,
    f,
    {
        f.s["target"]["address"] = OTHER.into();
    },
    "Report-source identity"
);
reject_test!(
    fake_pin,
    f,
    {
        f.m["pins"][0]["hash"] = format!("0x{}", sha256(b"FAKE")).into();
    },
    "Fake/inconsistent pin"
);
reject_test!(
    placeholder_pin,
    f,
    {
        f.m["pins"][0]["hash"] = format!("0x{}", "0".repeat(64)).into();
    },
    "Placeholder"
);
reject_test!(
    wrong_timestamp,
    f,
    {
        f.m["pins"][0]["timestamp_utc"] = "2023-11-14T22:13:19Z".into();
    },
    "UTC timestamp"
);
reject_test!(
    reorged_header,
    f,
    {
        f.rpc("recheck", |r| {
            r["response"]["result"]["hash"] = format!("0x{}", sha256(b"REORG")).into()
        });
    },
    "recheck changed"
);
reject_test!(
    wrong_rpc_chain,
    f,
    {
        f.rpc("chain", |r| r["response"]["result"] = "0xa".into());
    },
    "RPC chain ID"
);
reject_test!(
    wrong_queried_address,
    f,
    {
        f.rpc("runtime", |r| r["request"]["params"][0] = OTHER.into());
    },
    "Queried address differs"
);
reject_test!(
    unpinned_call,
    f,
    {
        f.rpc("name", |r| r["request"]["params"][1] = "latest".into());
    },
    "Unpinned"
);
reject_test!(
    wrong_metadata,
    f,
    {
        f.m["metadata"]["symbol"]["value"] = "OTHER".into();
        f.s["metadata"] = f.m["metadata"].clone();
    },
    "Metadata differs"
);
reject_test!(
    unknown_as_pass,
    f,
    {
        f.s["ratings"][0]["status"] = "bounded_negative".into();
        f.s["ratings"][0]["severity"] = "none".into();
        f.s["ratings"][0]["coverage"] = "complete".into();
    },
    "Unknown or risky"
);
reject_test!(
    unknown_evidence_as_negative,
    f,
    {
        f.s["findings"][0]["status"] = "bounded_negative".into();
        f.s["findings"][0]["coverage"] = "complete".into();
        f.s["findings"][0]["evidence"] =
            json!([{"artifact_id":"runtime","decoding_basis":"fixture"}]);
    },
    "Unknown/error/skipped"
);
reject_test!(
    missing_claim_evidence,
    f,
    {
        f.s["findings"][0]["status"] = "finding".into();
    },
    "Missing evidence"
);
reject_test!(
    missing_file,
    f,
    {
        fs::remove_file(f.path("evidence/runtime.json")).unwrap();
    },
    "Missing evidence file"
);
reject_test!(
    hash_tampering,
    f,
    {
        fs::write(f.path("evidence/runtime.json"), "{}").unwrap();
    },
    "hash mismatch"
);
reject_test!(
    inconsistent_scope_chain,
    f,
    {
        f.m["scope"][0]["chain_id"] = 10.into();
    },
    "Inconsistent scope chain"
);
reject_test!(
    malformed_address,
    f,
    {
        f.m["target"]["requested"]["address"] = "0x1234".into();
    },
    "Malformed chain/address"
);
reject_test!(
    missing_no_broadcast,
    f,
    {
        f.m["safety"]
            .as_object_mut()
            .unwrap()
            .remove("no_broadcast");
    },
    "Missing safety declaration"
);
reject_test!(
    simulation_without_fork,
    f,
    {
        f.m["safety"]["simulation_used"] = true.into();
    },
    "Simulation/fork"
);
reject_test!(
    signing_method,
    f,
    {
        f.rpc("runtime", |r| {
            r["request"]["method"] = "eth_sendRawTransaction".into()
        });
    },
    "Non-allowlisted"
);
reject_test!(
    partial_negative,
    f,
    {
        f.negative(
            0,
            json!([{"artifact_id":"runtime","decoding_basis":"fixture"}]),
        );
        f.s["findings"][0]["coverage"] = "partial".into();
    },
    "Unknown/partial"
);
reject_test!(
    null_result_cannot_pass,
    f,
    {
        f.rpc("symbol", |r| r["response"]["result"] = Value::Null);
    },
    "Successful non-null"
);
#[test]
fn unresolved_nonstandard_metadata() {
    let mut f = Fixture::new();
    f.rpc("name", |r| r["response"]["result"] = "0xdeadbeef".into());
    f.m["metadata"]["name"] = json!({"status":"unresolved","value":null,"reason":"Nonstandard ABI response","evidence":["name"]});
    f.s["metadata"] = f.m["metadata"].clone();
    f.accept();
}
#[test]
fn rpc_failure_remains_coverage() {
    let mut f = Fixture::new();
    f.rpc("symbol", |r| {
        r["response"].as_object_mut().unwrap().remove("result");
        r["response"]["error"] = json!({"code":-32000,"message":"Archive unavailable"});
    });
    f.m["metadata"]["symbol"] = json!({"status":"unresolved","value":null,"reason":"Archive unavailable","evidence":["symbol"]});
    f.s["metadata"] = f.m["metadata"].clone();
    f.accept();
    f.s["findings"][0]["status"] = "finding".into();
    f.s["findings"][0]["evidence"] =
        json!([{"artifact_id":"symbol","decoding_basis":"RPC error only"}]);
    f.reject("Unknown/error/skipped");
}
#[test]
fn complete_fork_restrictions() {
    let mut f = Fixture::new();
    f.fork();
    f.accept();
}
#[test]
fn public_endpoint_cannot_be_local_fork() {
    let mut f = Fixture::new();
    f.fork();
    let mut config = read_json(&f.path("evidence/fork.json")).unwrap();
    config["local_endpoint"] = "https://public.example".into();
    let bytes = pretty(&config).unwrap().into_bytes();
    fs::write(f.path("evidence/fork.json"), &bytes).unwrap();
    f.m["artifacts"].as_array_mut().unwrap().last_mut().unwrap()["sha256"] = sha256(&bytes).into();
    f.reject("literal loopback");
}
#[test]
fn critical_finding_cannot_be_averaged_down_or_orphaned() {
    let mut f = Fixture::new();
    f.negative(
        0,
        json!([{"artifact_id":"runtime","decoding_basis":"Synthetic inference"}]),
    );
    f.s["findings"][0]["status"] = "finding".into();
    f.s["findings"][0]["severity"] = "critical".into();
    f.s["ratings"][0]["status"] = "finding".into();
    f.s["ratings"][0]["severity"] = "low".into();
    f.reject("averaged down");
    f.s["ratings"][0]["status"] = "unknown".into();
    f.s["ratings"][0]["severity"] = "unknown".into();
    f.s["ratings"][0]["finding_ids"] = json!(["F02"]);
    f.reject("Orphan finding");
}
#[test]
fn reverted_receipt_cannot_support_success_claim() {
    let mut f = Fixture::new();
    let tx = f.receipt("0x0");
    f.negative(
        3,
        json!([{"artifact_id":"receipt","decoding_basis":"Synthetic status 0x0"}]),
    );
    f.s["findings"][3]["transaction"] = tx;
    f.reject("Reverted receipt");
}
#[test]
fn successful_receipt_without_underlying_is_not_exit_proof() {
    let mut f = Fixture::new();
    let tx = f.receipt("0x1");
    f.negative(
        3,
        json!([{"artifact_id":"receipt","decoding_basis":"Synthetic status 0x1"}]),
    );
    f.s["findings"][3]["transaction"] = tx;
    f.s["findings"][3]["claim_kind"] = "executed_exit".into();
    f.reject("Missing text field");
}
#[test]
fn pinned_exit_balance_delta_is_accepted() {
    let mut f = Fixture::new();
    f.exit();
    f.accept();
}
#[test]
fn no_underlying_increase_is_rejected() {
    let mut f = Fixture::new();
    f.exit();
    f.rpc("balance-after", |r| {
        r["response"]["result"] = word(1).into()
    });
    f.reject("No intended underlying");
}
#[test]
fn wrong_underlying_recipient_is_rejected() {
    let mut f = Fixture::new();
    f.exit();
    f.s["findings"][3]["execution"]["recipient"] = OTHER.into();
    f.reject("recipient mismatch");
}
#[test]
fn relabeled_synthetic_evidence_rejected_without_opt_in() {
    let mut f = Fixture::new();
    f.m["evidence_kind"] = "live".into();
    f.s["evidence_kind"] = "live".into();
    f.sync();
    assert!(validate(&f.path("manifest.json"), None, false)
        .err()
        .unwrap()
        .contains("Synthetic raw evidence"));
}
#[test]
fn metadata_markup_stays_inert_after_validation() {
    let mut f = Fixture::new();
    let payload = "Synthetic\n\n## Forged\n![tracking](https://example.invalid/pixel)<img src=x>";
    f.rpc("name", |r| {
        r["response"]["result"] = abi_string(payload).into()
    });
    f.m["metadata"]["name"]["value"] = payload.into();
    f.s["metadata"] = f.m["metadata"].clone();
    f.accept();
    let report = render::render(&f.m, &f.s).unwrap();
    assert!(!report.contains("\n## Forged"));
    assert!(!report.contains("![tracking]"));
    assert!(!report.contains("<img"));
}
#[test]
fn wrong_rendered_target_is_rejected() {
    let f = Fixture::new();
    let content = fs::read_to_string(f.path("report.md"))
        .unwrap()
        .replace(TOKEN, OTHER);
    fs::write(f.path("report.md"), content).unwrap();
    assert!(
        validate(&f.path("manifest.json"), Some(&f.path("report.md")), true)
            .err()
            .unwrap()
            .contains("Rendered report differs")
    );
}
#[test]
fn duplicate_json_keys_rejected() {
    assert!(parse_json(br#"{"target":{"chain_id":1,"chain_id":10}}"#).is_err());
}
#[test]
fn nonfinite_json_rejected() {
    assert!(parse_json(br#"{"amount":NaN}"#).is_err());
}
#[test]
fn traversal_rejected() {
    let mut f = Fixture::new();
    f.m["artifacts"][0]["path"] = "../outside.json".into();
    f.reject("Missing evidence file");
}
#[cfg(unix)]
#[test]
fn symlink_escape_rejected() {
    let mut f = Fixture::new();
    let outside = tempfile::NamedTempFile::new().unwrap();
    std::os::unix::fs::symlink(outside.path(), f.path("escape.json")).unwrap();
    f.m["artifacts"][0]["path"] = "escape.json".into();
    f.reject("escapes case");
}
#[test]
fn fixture_refuses_overwrite() {
    let f = Fixture::new();
    assert!(fixture::create(f.dir.path()).is_err());
}

fn ledger() -> Value {
    json!({"assets":[{"chain_id":1,"asset":TOKEN,"unit":"raw token units","opening":"40","closing":"30","unexplained_bound":"0",
        "entries":[{"id":"fee","kind":"inflow","amount":"10","evidence":["synthetic receipt"],"explanation":"Synthetic fees"},
                   {"id":"payout","kind":"outflow","amount":"20","evidence":["synthetic receipt"],"explanation":"Partially prefunded"}]}]})
}
#[test]
fn prefunding_explains_large_distributions() {
    assert_eq!(reconcile(&ledger()).unwrap()["all_within_bound"], true);
}
#[test]
fn residual_not_hidden() {
    let mut l = ledger();
    l["assets"][0]["closing"] = "29".into();
    let r = reconcile(&l).unwrap();
    assert_eq!(r["all_within_bound"], false);
    assert_eq!(r["assets"][0]["unexplained_delta"], "1");
}
#[test]
fn duplicate_flows_rejected() {
    let mut l = ledger();
    let flow = l["assets"][0]["entries"][0].clone();
    l["assets"][0]["entries"].as_array_mut().unwrap().push(flow);
    assert!(reconcile(&l).is_err());
}
#[test]
fn uint256_precision() {
    let mut l = ledger();
    let n = BigUint::from(1u8) << 255;
    l["assets"][0]["opening"] = format!("{n}").into();
    l["assets"][0]["closing"] = format!("{}", n - BigUint::from(10u8)).into();
    assert_eq!(reconcile(&l).unwrap()["all_within_bound"], true);
}
#[test]
fn fractional_precision() {
    let mut l = ledger();
    l["assets"][0]["opening"] = "40.000000000000000001".into();
    l["assets"][0]["closing"] = "30.000000000000000001".into();
    assert_eq!(reconcile(&l).unwrap()["all_within_bound"], true);
}
#[test]
fn signed_adjustments() {
    let mut l = ledger();
    l["assets"][0]["closing"] = "25".into();
    l["assets"][0]["entries"].as_array_mut().unwrap().push(json!({"id":"burn","kind":"adjustment","amount":"-5","evidence":["receipt"],"explanation":"Synthetic burn"}));
    assert_eq!(reconcile(&l).unwrap()["all_within_bound"], true);
}
#[test]
fn float_nonfinite_exponent_rejected() {
    for v in [json!(1.5), json!("NaN"), json!("Infinity"), json!("1e1000")] {
        let mut l = ledger();
        l["assets"][0]["opening"] = v;
        assert!(reconcile(&l).is_err());
    }
}
#[test]
fn malformed_asset_rejected() {
    let mut l = ledger();
    l["assets"][0]["asset"] = "symbol-only".into();
    assert!(reconcile(&l).is_err());
}
#[test]
fn cli_roundtrip_and_exclusive_output() {
    let f = Fixture::new();
    let out = f.path("new-report.md");
    let bin = env!("CARGO_BIN_EXE_evm-diligence");
    let run = || {
        Command::new(bin)
            .arg("render_report")
            .arg(f.path("manifest.json"))
            .arg("--output")
            .arg(&out)
            .arg("--allow-synthetic")
            .output()
            .unwrap()
    };
    assert!(run().status.success());
    validate(&f.path("manifest.json"), Some(&out), true).unwrap();
    assert!(!run().status.success());
}
#[test]
fn cli_validation_error_exit() {
    let f = Fixture::new();
    let result = Command::new(env!("CARGO_BIN_EXE_evm-diligence"))
        .arg("validate_report")
        .arg(f.path("manifest.json"))
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(1));
}
#[test]
fn cli_reconcile_exit_codes() {
    let f = Fixture::new();
    let path = f.path("ledger.json");
    let mut l = ledger();
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_evm-diligence"))
            .arg("reconcile")
            .arg(&path)
            .output()
            .unwrap()
    };
    fs::write(&path, pretty(&l).unwrap()).unwrap();
    assert_eq!(run().status.code(), Some(0));
    l["assets"][0]["closing"] = "29".into();
    fs::write(&path, pretty(&l).unwrap()).unwrap();
    assert_eq!(run().status.code(), Some(2));
}
