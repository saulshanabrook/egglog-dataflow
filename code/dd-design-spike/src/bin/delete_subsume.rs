use std::collections::{BTreeMap, BTreeSet};

use dd_design_spike::{
    command_string, out_path_arg, rss_kb, write_report, ExperimentReport, Metrics,
};
use serde::Serialize;
use serde_json::json;

type Key = &'static str;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct Row {
    key: Key,
    value: &'static str,
    provenance: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct Diff {
    view: &'static str,
    row: Row,
    diff: isize,
}

#[derive(Default)]
struct SignedDiffModel {
    live: BTreeMap<Key, Row>,
    active: BTreeMap<Key, Row>,
    all_rows: BTreeMap<Key, Row>,
    provenance: BTreeMap<Key, BTreeSet<&'static str>>,
    diffs: Vec<Diff>,
    delete_count: u64,
    delete_noops: u64,
    subsume_count: u64,
}

impl SignedDiffModel {
    fn insert(&mut self, row: Row) {
        assert!(
            !self.live.contains_key(row.key),
            "scenario inserts each key at most once"
        );

        self.live.insert(row.key, row.clone());
        self.active.insert(row.key, row.clone());
        self.all_rows.insert(row.key, row.clone());
        self.provenance
            .entry(row.key)
            .or_default()
            .insert(row.provenance);

        self.emit("live", row.clone(), 1);
        self.emit("active", row.clone(), 1);
        self.emit("all_rows", row, 1);
    }

    fn hard_delete(&mut self, key: Key) {
        let Some(row) = self.live.remove(key) else {
            self.delete_noops += 1;
            return;
        };

        self.delete_count += 1;
        self.emit("live", row.clone(), -1);

        if self.active.remove(key).is_some() {
            self.emit("active", row, -1);
        }
    }

    fn subsume(&mut self, key: Key) {
        let Some(row) = self.active.remove(key) else {
            return;
        };

        assert!(
            self.live.contains_key(key),
            "subsumed rows remain live until hard delete"
        );
        assert!(
            self.all_rows.contains_key(key),
            "subsumed rows remain visible in all-row history"
        );

        self.subsume_count += 1;
        self.emit("active", row, -1);
    }

    fn emit(&mut self, view: &'static str, row: Row, diff: isize) {
        self.diffs.push(Diff { view, row, diff });
    }
}

fn row(key: Key, value: &'static str, provenance: &'static str) -> Row {
    Row {
        key,
        value,
        provenance,
    }
}

fn main() {
    let out = out_path_arg();
    let mut model = SignedDiffModel::default();

    model.insert(row("delete-idempotent", "alive-then-deleted", "assert-1"));
    model.hard_delete("delete-idempotent");
    model.hard_delete("delete-idempotent");

    model.insert(row("subsumed", "kept-for-proof", "assert-2"));
    model.subsume("subsumed");

    model.insert(row("survivor", "still-active", "assert-3"));

    assert!(model.live.get("delete-idempotent").is_none());
    assert!(model.active.get("delete-idempotent").is_none());
    assert!(model.all_rows.contains_key("delete-idempotent"));
    assert_eq!(
        model.provenance.get("delete-idempotent").unwrap(),
        &BTreeSet::from(["assert-1"])
    );

    assert!(model.live.contains_key("subsumed"));
    assert!(model.active.get("subsumed").is_none());
    assert!(model.all_rows.contains_key("subsumed"));
    assert_eq!(
        model.provenance.get("subsumed").unwrap(),
        &BTreeSet::from(["assert-2"])
    );

    assert!(model.live.contains_key("survivor"));
    assert!(model.active.contains_key("survivor"));
    assert!(model.all_rows.contains_key("survivor"));

    assert_eq!(model.delete_count, 1);
    assert_eq!(model.delete_noops, 1);
    assert_eq!(model.subsume_count, 1);
    assert_eq!(model.active.len(), 1);
    assert_eq!(model.all_rows.len(), 3);
    assert_eq!(model.provenance.len(), 3);

    assert!(model.diffs.iter().any(|diff| {
        diff.view == "live" && diff.row.key == "delete-idempotent" && diff.diff == -1
    }));
    assert_eq!(
        model
            .diffs
            .iter()
            .filter(|diff| diff.row.key == "delete-idempotent" && diff.diff == -1)
            .count(),
        2,
        "hard delete emits one live and one active negative diff for an active row"
    );
    assert!(model
        .diffs
        .iter()
        .any(|diff| diff.view == "active" && diff.row.key == "subsumed" && diff.diff == -1));
    assert!(!model
        .diffs
        .iter()
        .any(|diff| diff.view == "live" && diff.row.key == "subsumed" && diff.diff == -1));
    assert!(!model
        .diffs
        .iter()
        .any(|diff| diff.view == "all_rows" && diff.diff == -1));

    let mut metrics = Metrics::new();
    metrics.insert("delete_count".to_string(), json!(model.delete_count));
    metrics.insert("delete_noops".to_string(), json!(model.delete_noops));
    metrics.insert("subsume_count".to_string(), json!(model.subsume_count));
    metrics.insert("live_rows".to_string(), json!(model.live.len()));
    metrics.insert("active_rows".to_string(), json!(model.active.len()));
    metrics.insert("all_rows".to_string(), json!(model.all_rows.len()));
    metrics.insert("provenance_rows".to_string(), json!(model.provenance.len()));
    metrics.insert("signed_diffs".to_string(), json!(model.diffs));
    metrics.insert("rss_kb".to_string(), json!(rss_kb()));

    let report = ExperimentReport {
        experiment: "delete_subsume_signed_diff",
        status: "pass",
        command: command_string(),
        configs: vec![json!({
            "model": "in_memory_signed_diff",
            "workers": 1,
            "deterministic": true
        })],
        metrics,
        observations: vec![
            "Hard delete removes the current live row by key and emits a negative live diff."
                .to_string(),
            "A repeated hard delete for the same key is a no-op and does not emit another diff."
                .to_string(),
            "Subsume emits a negative active diff only; live, all-row, and provenance visibility remain."
                .to_string(),
        ],
        decision: "Pass: delete and subsume can be represented as signed diffs while retaining all-row/provenance visibility.".to_string(),
        limitations: vec![
            "This spike uses an in-memory model rather than a Differential Dataflow arrangement."
                .to_string(),
            "The scenario covers one deterministic row per operation class, not concurrent updates."
                .to_string(),
        ],
        next_action:
            "Map the signed-diff state transitions onto the candidate DD relation layout.".to_string(),
    };

    write_report(out, &report).expect("failed to write report");

    println!(
        "delete_subsume_signed_diff status={} delete_count={} delete_noops={} subsume_count={} active_rows={} all_rows={} provenance_rows={}",
        report.status,
        model.delete_count,
        model.delete_noops,
        model.subsume_count,
        model.active.len(),
        model.all_rows.len(),
        model.provenance.len()
    );
}
