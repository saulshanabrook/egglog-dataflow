use std::env;
use std::path::PathBuf;

use minimal_dd_trial::{run_acceptance_trial, write_report, TrialResult};

fn main() -> TrialResult<()> {
    let out = out_path_arg();
    let report = run_acceptance_trial()?;

    if let Some(path) = out {
        write_report(path, &report)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&report)?);
    }

    if report.all_match_oracle {
        Ok(())
    } else {
        Err("one or more DD scenario rows did not match lower-row egglog oracle".into())
    }
}

fn out_path_arg() -> Option<PathBuf> {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--out" {
            return args.next().map(PathBuf::from);
        }
        if let Some(path) = arg.strip_prefix("--out=") {
            return Some(PathBuf::from(path));
        }
    }
    None
}
