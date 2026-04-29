use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use dd_design_spike::{
    command_string, out_path_arg, rss_kb, write_report, ExperimentReport, Metrics,
};
use serde::Serialize;
use serde_json::json;

type ContainerId = u64;
type Epoch = u64;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct ParentRow {
    parent_id: u64,
    container_id: ContainerId,
    row_ts: Epoch,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct LogicalParentRow {
    parent_id: u64,
    container_id: ContainerId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ContainerState {
    id: ContainerId,
    canonical_contents: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct SignedDiff {
    row: ParentRow,
    diff: isize,
    event_ts: Epoch,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct VisibleMatch {
    row: ParentRow,
    last_run: Epoch,
}

fn logical(row: ParentRow) -> LogicalParentRow {
    LogicalParentRow {
        parent_id: row.parent_id,
        container_id: row.container_id,
    }
}

fn refresh_dirty_parent_rows(
    rows: &[ParentRow],
    dirty_ids: &BTreeSet<ContainerId>,
    next_ts: Epoch,
) -> (Vec<ParentRow>, Vec<SignedDiff>) {
    let mut refreshed_rows = Vec::new();
    let mut diffs = Vec::new();

    for row in rows {
        if dirty_ids.contains(&row.container_id) {
            let refreshed = ParentRow {
                row_ts: next_ts,
                ..*row
            };
            diffs.push(SignedDiff {
                row: *row,
                diff: -1,
                event_ts: row.row_ts,
            });
            diffs.push(SignedDiff {
                row: refreshed,
                diff: 1,
                event_ts: next_ts,
            });
            refreshed_rows.push(refreshed);
        }
    }

    (refreshed_rows, diffs)
}

fn seminaive_matches(rows: &[ParentRow], last_run: Epoch) -> Vec<VisibleMatch> {
    rows.iter()
        .copied()
        .filter(|row| row.row_ts > last_run)
        .map(|row| VisibleMatch { row, last_run })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let old_ts = 10;
    let next_ts = 11;
    let last_run = old_ts;
    let before = ContainerState {
        id: 7,
        canonical_contents: "canonical:{a,b}",
    };
    let after = ContainerState {
        id: 7,
        canonical_contents: "canonical:{a,c}",
    };
    let parent_row = ParentRow {
        parent_id: 1,
        container_id: before.id,
        row_ts: old_ts,
    };
    let dirty_ids = BTreeSet::from([before.id]);

    assert_eq!(before.id, after.id, "container id must remain stable");
    assert_ne!(
        before.canonical_contents, after.canonical_contents,
        "container semantic content must change"
    );

    let (refreshed_rows, diffs) = refresh_dirty_parent_rows(&[parent_row], &dirty_ids, next_ts);
    assert_eq!(refreshed_rows.len(), 1);

    let refreshed_row = refreshed_rows[0];
    assert_eq!(
        logical(parent_row),
        logical(refreshed_row),
        "refresh must preserve the row's logical value"
    );
    assert_eq!(parent_row.row_ts, old_ts);
    assert_eq!(refreshed_row.row_ts, next_ts);
    assert!(
        refreshed_row.row_ts > parent_row.row_ts,
        "refresh must advance row_ts"
    );

    let expected_diffs = vec![
        SignedDiff {
            row: parent_row,
            diff: -1,
            event_ts: old_ts,
        },
        SignedDiff {
            row: refreshed_row,
            diff: 1,
            event_ts: next_ts,
        },
    ];
    assert_eq!(diffs, expected_diffs);

    let visible = seminaive_matches(&refreshed_rows, last_run);
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].row, refreshed_row);

    let retractions = diffs.iter().filter(|diff| diff.diff < 0).count();
    let insertions = diffs.iter().filter(|diff| diff.diff > 0).count();
    let pass = retractions == 1 && insertions == 1 && visible.len() == 1;

    let mut metrics: Metrics = BTreeMap::new();
    metrics.insert("dirty_ids".to_string(), json!(dirty_ids));
    metrics.insert("refreshed_rows".to_string(), json!(refreshed_rows));
    metrics.insert(
        "retractions".to_string(),
        json!(diffs
            .iter()
            .filter(|diff| diff.diff < 0)
            .collect::<Vec<_>>()),
    );
    metrics.insert(
        "insertions".to_string(),
        json!(diffs
            .iter()
            .filter(|diff| diff.diff > 0)
            .collect::<Vec<_>>()),
    );
    metrics.insert("old_ts".to_string(), json!(old_ts));
    metrics.insert("next_ts".to_string(), json!(next_ts));
    metrics.insert("matches_seen_after_refresh".to_string(), json!(visible));
    metrics.insert(
        "logical_value_unchanged".to_string(),
        json!(logical(parent_row) == logical(refreshed_row)),
    );
    metrics.insert(
        "row_ts_advanced".to_string(),
        json!(refreshed_row.row_ts > parent_row.row_ts),
    );
    metrics.insert("rss_kb".to_string(), json!(rss_kb()));

    let status = if pass { "pass" } else { "fail" };
    let report = ExperimentReport {
        experiment: "same-id-container-dirty-refresh",
        status,
        command: command_string(),
        configs: vec![json!({
            "container_before": before,
            "container_after": after,
            "parent_row_before": parent_row,
            "last_run": last_run,
            "freshness_filter": "row_ts > last_run"
        })],
        metrics,
        observations: vec![
            "Container id remains stable while canonical contents change.".to_string(),
            "Dirty refresh emits -parent@old_ts and +same-logical-parent@next_ts.".to_string(),
            "The next seminaive pass sees the refreshed parent through row_ts freshness.".to_string(),
        ],
        decision: "Pass: same-id container semantic changes require explicit parent retimestamping."
            .to_string(),
        limitations: vec![
            "This is an in-memory signed-diff model, not a Differential Dataflow trace."
                .to_string(),
            "The scenario covers one parent row and one dirty container id.".to_string(),
        ],
        next_action:
            "Use this as the minimal acceptance case for any DD design that owns container dirty refresh."
                .to_string(),
    };

    write_report(out_path_arg(), &report)?;
    println!(
        "status={status} dirty_ids={} refreshed_rows={} retractions={retractions} insertions={insertions} matches_seen_after_refresh={}",
        dirty_ids.len(),
        refreshed_rows.len(),
        visible.len()
    );

    Ok(())
}
