use std::collections::{BTreeMap, BTreeSet, VecDeque};

use dd_design_spike::{
    command_string, out_path_arg, rss_kb, write_report, ExperimentReport, Metrics,
};
use serde::Serialize;
use serde_json::json;

type Id = u64;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct ConstructorRow {
    id: Id,
    symbol: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
struct DepRow {
    arg: Id,
    result: Id,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
struct RowDelta {
    row: DepRow,
    diff: isize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
struct DisplacedId {
    old: Id,
    new: Id,
}

#[derive(Debug, Serialize)]
struct ScenarioSummary {
    constructor_rows: Vec<ConstructorRow>,
    displaced_events: Vec<DisplacedId>,
    row_deltas: Vec<RowDelta>,
    final_live_rows: Vec<DepRow>,
    rewrite_count: u64,
    retraction_count: u64,
    insertion_count: u64,
    merge_count: u64,
    fixed_point_iterations: u64,
}

#[derive(Debug)]
struct RebuildModel {
    constructor_rows: BTreeSet<ConstructorRow>,
    canonical: BTreeMap<Id, Id>,
    live_rows: BTreeSet<DepRow>,
    reverse_index: BTreeMap<Id, BTreeSet<DepRow>>,
    pending: VecDeque<DisplacedId>,
    displaced_events: Vec<DisplacedId>,
    row_deltas: Vec<RowDelta>,
    rewrites: u64,
    retractions: u64,
    insertions: u64,
    merges: u64,
    fixed_point_iterations: u64,
}

impl RebuildModel {
    fn new(
        constructor_rows: impl IntoIterator<Item = ConstructorRow>,
        rows: impl IntoIterator<Item = DepRow>,
    ) -> Self {
        let mut model = Self {
            constructor_rows: BTreeSet::new(),
            canonical: BTreeMap::new(),
            live_rows: BTreeSet::new(),
            reverse_index: BTreeMap::new(),
            pending: VecDeque::new(),
            displaced_events: Vec::new(),
            row_deltas: Vec::new(),
            rewrites: 0,
            retractions: 0,
            insertions: 0,
            merges: 0,
            fixed_point_iterations: 0,
        };
        for row in constructor_rows {
            model.note_id(row.id);
            model.constructor_rows.insert(row);
        }
        for row in rows {
            model.note_id(row.arg);
            model.note_id(row.result);
            model.insert_row(row);
        }
        model
    }

    fn note_id(&mut self, id: Id) {
        self.canonical.entry(id).or_insert(id);
    }

    fn canonical(&self, id: Id) -> Id {
        let mut current = id;
        loop {
            let Some(&next) = self.canonical.get(&current) else {
                return current;
            };
            if next == current {
                return current;
            }
            current = next;
        }
    }

    fn union(&mut self, old: Id, new: Id) {
        self.note_id(old);
        self.note_id(new);
        let old_root = self.canonical(old);
        let new_root = self.canonical(new);
        if old_root == new_root {
            return;
        }
        self.canonical.insert(old_root, new_root);
        self.emit_displaced(DisplacedId {
            old: old_root,
            new: new_root,
        });
    }

    fn emit_displaced(&mut self, event: DisplacedId) {
        self.displaced_events.push(event);
        self.pending.push_back(event);
    }

    fn insert_row(&mut self, row: DepRow) -> bool {
        if !self.live_rows.insert(row) {
            return false;
        }
        self.reverse_index.entry(row.arg).or_default().insert(row);
        true
    }

    fn remove_row(&mut self, row: DepRow) -> bool {
        if !self.live_rows.remove(&row) {
            return false;
        }
        if let Some(rows) = self.reverse_index.get_mut(&row.arg) {
            rows.remove(&row);
            if rows.is_empty() {
                self.reverse_index.remove(&row.arg);
            }
        }
        true
    }

    fn emit_delta(&mut self, row: DepRow, diff: isize) {
        self.row_deltas.push(RowDelta { row, diff });
        if diff < 0 {
            self.retractions += 1;
        } else if diff > 0 {
            self.insertions += 1;
        }
    }

    fn step_rebuild(&mut self) -> bool {
        let Some(event) = self.pending.pop_front() else {
            return false;
        };
        self.fixed_point_iterations += 1;

        let rows_to_rewrite = self
            .reverse_index
            .get(&event.old)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<_>>();

        for old_row in rows_to_rewrite {
            let new_row = DepRow {
                arg: self.canonical(old_row.arg),
                result: self.canonical(old_row.result),
            };
            if new_row == old_row {
                continue;
            }

            assert!(
                self.remove_row(old_row),
                "reverse index returned a dead row"
            );
            self.emit_delta(old_row, -1);
            self.rewrites += 1;

            if let Some(collision) = self
                .live_rows
                .iter()
                .find(|row| row.arg == new_row.arg && row.result != new_row.result)
                .copied()
            {
                self.merges += 1;
                let survivor = collision.result.min(new_row.result);
                let displaced = collision.result.max(new_row.result);
                self.emit_delta(new_row, 1);
                self.union(displaced, survivor);
            } else if self.insert_row(new_row) {
                self.emit_delta(new_row, 1);
            }
        }

        true
    }

    fn run_to_fixed_point(&mut self) {
        while self.step_rebuild() {}
    }

    fn final_live_rows(&self) -> Vec<DepRow> {
        self.live_rows.iter().copied().collect()
    }

    fn summary(&self) -> ScenarioSummary {
        ScenarioSummary {
            constructor_rows: self.constructor_rows.iter().copied().collect(),
            displaced_events: self.displaced_events.clone(),
            row_deltas: self.row_deltas.clone(),
            final_live_rows: self.final_live_rows(),
            rewrite_count: self.rewrites,
            retraction_count: self.retractions,
            insertion_count: self.insertions,
            merge_count: self.merges,
            fixed_point_iterations: self.fixed_point_iterations,
        }
    }
}

fn run_scenario() -> ScenarioSummary {
    let mut model = RebuildModel::new(
        [
            ConstructorRow { id: 1, symbol: "a" },
            ConstructorRow { id: 2, symbol: "b" },
            ConstructorRow {
                id: 10,
                symbol: "x",
            },
            ConstructorRow {
                id: 20,
                symbol: "y",
            },
        ],
        [DepRow { arg: 1, result: 10 }, DepRow { arg: 2, result: 20 }],
    );

    model.union(2, 1);
    model.run_to_fixed_point();

    let summary = model.summary();
    assert_eq!(
        summary.constructor_rows,
        vec![
            ConstructorRow { id: 1, symbol: "a" },
            ConstructorRow { id: 2, symbol: "b" },
            ConstructorRow {
                id: 10,
                symbol: "x"
            },
            ConstructorRow {
                id: 20,
                symbol: "y"
            },
        ]
    );
    assert_eq!(
        summary.displaced_events,
        vec![
            DisplacedId { old: 2, new: 1 },
            DisplacedId { old: 20, new: 10 },
        ]
    );
    assert_eq!(
        summary.row_deltas,
        vec![
            RowDelta {
                row: DepRow { arg: 2, result: 20 },
                diff: -1,
            },
            RowDelta {
                row: DepRow { arg: 1, result: 20 },
                diff: 1,
            },
        ]
    );
    assert_eq!(summary.final_live_rows, vec![DepRow { arg: 1, result: 10 }]);
    assert_eq!(summary.rewrite_count, 1);
    assert_eq!(summary.retraction_count, 1);
    assert_eq!(summary.insertion_count, 1);
    assert_eq!(summary.merge_count, 1);
    assert_eq!(summary.fixed_point_iterations, 2);
    assert!(model.pending.is_empty());
    assert_eq!(model.canonical(2), 1);
    assert_eq!(model.canonical(20), 10);

    summary
}

fn main() -> std::io::Result<()> {
    let out_path = out_path_arg();
    let summary = run_scenario();

    let mut metrics = Metrics::new();
    metrics.insert(
        "constructor_rows".to_string(),
        json!(summary.constructor_rows),
    );
    metrics.insert("rewrites".to_string(), json!(summary.rewrite_count));
    metrics.insert("retractions".to_string(), json!(summary.retraction_count));
    metrics.insert("insertions".to_string(), json!(summary.insertion_count));
    metrics.insert("merges".to_string(), json!(summary.merge_count));
    metrics.insert(
        "fixed_point_iterations".to_string(),
        json!(summary.fixed_point_iterations),
    );
    metrics.insert(
        "final_live_rows".to_string(),
        json!(summary.final_live_rows),
    );
    metrics.insert("row_deltas".to_string(), json!(summary.row_deltas));
    metrics.insert(
        "displaced_events".to_string(),
        json!(summary.displaced_events),
    );
    if let Some(rss) = rss_kb() {
        metrics.insert("rss_kb".to_string(), json!(rss));
    }

    let report = ExperimentReport {
        experiment: "dd-full-refactor/rebuild-delta",
        status: "pass",
        command: command_string(),
        configs: vec![json!({
            "scenario": "one constructor/dependent table, union b into a, rewrite f(b)->y, merge y into x on f(a) collision",
            "deterministic": true,
            "uses_differential_dataflow": false
        })],
        metrics,
        observations: vec![
            "A displaced-id event is enough to find affected dependent rows through a reverse index.".to_string(),
            "The rewrite protocol emits an explicit -old_row/+new_row signed delta pair before resolving the collision.".to_string(),
            "A key collision on the rewritten dependent row runs merge and queues the resulting displaced-id event for the same fixed-point loop.".to_string(),
        ],
        decision: "Canonical-map rebuild can be modeled as a delta protocol: displaced ids drive reverse-index row rewrites, and rewritten dependent-row collisions become further id merges.".to_string(),
        limitations: vec![
            "This is an in-memory deterministic model, not a Differential Dataflow implementation.".to_string(),
            "The scenario covers one dependent-table collision shape and does not exercise multi-column keys or multiplicities greater than one.".to_string(),
        ],
        next_action: "Map the same displaced-id and signed-row-delta protocol onto the planned maintained-view/rebuild design.".to_string(),
    };

    write_report(out_path, &report)?;
    println!(
        "rebuild_delta: rewrites={} retractions={} insertions={} merges={} fixed_point_iterations={} final_live_rows={:?}",
        summary.rewrite_count,
        summary.retraction_count,
        summary.insertion_count,
        summary.merge_count,
        summary.fixed_point_iterations,
        summary.final_live_rows
    );

    Ok(())
}
