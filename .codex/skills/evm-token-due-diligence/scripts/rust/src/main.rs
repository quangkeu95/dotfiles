use evm_diligence::{fixture, pretty, read_json, reconcile, render, validate, write_new, Result};
use std::{env, path::PathBuf, process::ExitCode};

const HELP: &str = "Offline EVM diligence helpers (no network/signing/broadcast).\n\
  evm-diligence validate_report MANIFEST [--report REPORT] [--allow-synthetic]\n\
  evm-diligence render_report MANIFEST --output NEW_REPORT [--allow-synthetic]\n\
  evm-diligence reconcile LEDGER\n\
  evm-diligence selftest --emit-example NEW_OR_EMPTY_DIRECTORY\n\
Run regression tests with: cargo test --locked --offline\n\
Exit codes: 0 consistent/within bound, 1 rejected, 2 reconciliation exceeds bound.\n\
Validation is internal consistency only, not RPC honesty or protocol safety.";

fn run() -> Result<u8> {
    let mut args = env::args_os().skip(1);
    let command = args
        .next()
        .ok_or(HELP)?
        .into_string()
        .map_err(|_| "Invalid command encoding")?;
    if ["--help", "-h", "help"].contains(&command.as_str()) {
        println!("{HELP}");
        return Ok(0);
    }
    if command == "selftest" || command == "--emit-example" {
        if command == "selftest"
            && args.next().as_deref() != Some(std::ffi::OsStr::new("--emit-example"))
        {
            return Err(HELP.into());
        }
        let out = PathBuf::from(args.next().ok_or("Missing fixture destination")?);
        if args.next().is_some() {
            return Err("Unexpected arguments".into());
        }
        fixture::create(&out)?;
        println!("Created SYNTHETIC example: {}", out.display());
        return Ok(0);
    }
    let input = PathBuf::from(args.next().ok_or("Missing input path")?);
    let mut output = None;
    let mut report = None;
    let mut allow = false;
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--allow-synthetic") if !allow => allow = true,
            Some("--output") if output.is_none() => {
                output = Some(PathBuf::from(args.next().ok_or("Missing output path")?))
            }
            Some("--report") if report.is_none() => {
                report = Some(PathBuf::from(args.next().ok_or("Missing report path")?))
            }
            _ => return Err("Unexpected or duplicate argument".into()),
        }
    }
    match command.as_str() {
        "validate_report" => {
            if output.is_some() {
                return Err("--output is only for render_report".into());
            }
            let case = validate::validate(&input, report.as_deref(), allow)?;
            println!("CONSISTENT: {} case; {} findings, {} pins. Internal consistency only; not RPC honesty, discovery completeness, or protocol safety.", case.manifest["evidence_kind"], case.source["findings"].as_array().map_or(0, Vec::len), case.manifest["pins"].as_array().map_or(0, Vec::len));
            Ok(0)
        }
        "render_report" => {
            if report.is_some() {
                return Err("Use --report with validate_report".into());
            }
            let output = output.ok_or("render_report requires --output")?;
            let case = validate::validate(&input, None, allow)?;
            let content = render::render(&case.manifest, &case.source)?;
            write_new(&output, content.as_bytes())?;
            println!("Rendered from validated source: {}", output.display());
            Ok(0)
        }
        "reconcile" => {
            if report.is_some() || output.is_some() || allow {
                return Err("reconcile accepts only a ledger path".into());
            }
            let result = reconcile::reconcile(&read_json(&input)?)?;
            println!("{}", pretty(&result)?);
            Ok(if result["all_within_bound"] == true {
                0
            } else {
                2
            })
        }
        _ => Err(HELP.into()),
    }
}
fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("REJECTED: {error}");
            ExitCode::FAILURE
        }
    }
}
