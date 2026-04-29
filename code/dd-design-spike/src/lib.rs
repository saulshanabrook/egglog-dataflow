use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use serde::Serialize;
use serde_json::Value;

pub type Metrics = BTreeMap<String, Value>;

#[derive(Debug, Serialize)]
pub struct ExperimentReport {
    pub experiment: &'static str,
    pub status: &'static str,
    pub command: String,
    pub configs: Vec<Value>,
    pub metrics: Metrics,
    pub observations: Vec<String>,
    pub decision: String,
    pub limitations: Vec<String>,
    pub next_action: String,
}

pub fn out_path_arg() -> Option<PathBuf> {
    let mut args = std::env::args().skip(1);
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

pub fn command_string() -> String {
    std::env::args().collect::<Vec<_>>().join(" ")
}

pub fn write_report(path: Option<PathBuf>, report: &ExperimentReport) -> io::Result<()> {
    if let Some(path) = path {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let body = serde_json::to_string_pretty(report)?;
        fs::write(path, format!("{body}\n"))?;
    }
    Ok(())
}

pub fn rss_kb() -> Option<u64> {
    let pid = std::process::id().to_string();
    let output = Command::new("ps")
        .args(["-o", "rss=", "-p", &pid])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout).ok()?.trim().parse().ok()
}
