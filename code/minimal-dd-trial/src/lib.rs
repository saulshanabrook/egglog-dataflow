use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use differential_dataflow::input::Input;
use differential_dataflow::operators::Iterate;
use egglog::{EGraph, Value};
use serde::{Deserialize, Serialize};
use timely::dataflow::operators::probe::Handle;

pub type TrialResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub const PATH_REACHABILITY_EGG: &str = include_str!("../fixtures/path-reachability.egg");

const PATH_SETUP: &str = r#"
(relation edge (i64 i64))
(relation path (i64 i64))

(rule ((edge x y))
      ((path x y)))

(rule ((path x y) (edge y z))
      ((path x z)))

(edge 1 2)
(edge 2 3)
(edge 3 4)
"#;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScenarioSpec {
    pub name: String,
    pub observed_functions: Vec<String>,
    pub stages: Vec<ScenarioStage>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScenarioStage {
    pub label: String,
    pub program: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct LowerRow {
    pub function: String,
    pub input_sorts: Vec<String>,
    pub output_sort: String,
    pub raw_values: Vec<String>,
    pub input_values: Vec<DecodedValue>,
    pub output_value: DecodedValue,
    pub subsumed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DecodedValue {
    I64 { value: i64 },
    Raw { sort: String, value: String },
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct LogicalRow {
    pub function: String,
    pub values: Vec<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OracleSnapshot {
    pub stage: String,
    pub rows: BTreeMap<String, Vec<LowerRow>>,
    pub logical_rows: BTreeMap<String, Vec<LogicalRow>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Pair {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrialReport {
    pub scenario: String,
    pub oracle_snapshots: Vec<OracleSnapshot>,
    pub dd_logical_rows: Vec<LogicalRow>,
    pub oracle_final_rows: Vec<LogicalRow>,
    pub matches_oracle: bool,
    pub observations: Vec<String>,
    pub limitations: Vec<String>,
}

pub fn path_scenario() -> ScenarioSpec {
    ScenarioSpec {
        name: "path-reachability".to_string(),
        observed_functions: vec!["edge".to_string(), "path".to_string()],
        stages: vec![
            ScenarioStage {
                label: "setup".to_string(),
                program: PATH_SETUP.to_string(),
            },
            ScenarioStage {
                label: "run-1".to_string(),
                program: "(run 1)".to_string(),
            },
            ScenarioStage {
                label: "run-2".to_string(),
                program: "(run 1)".to_string(),
            },
            ScenarioStage {
                label: "run-3".to_string(),
                program: "(run 1)".to_string(),
            },
        ],
    }
}

pub fn run_native_oracle(spec: &ScenarioSpec) -> TrialResult<Vec<OracleSnapshot>> {
    let mut egraph = EGraph::default();
    let mut snapshots = Vec::new();

    for stage in &spec.stages {
        egraph.parse_and_run_program(Some(stage.label.clone()), &stage.program)?;
        snapshots.push(snapshot_functions(
            &egraph,
            &stage.label,
            &spec.observed_functions,
        )?);
    }

    Ok(snapshots)
}

pub fn run_fixture_program(
    program: &str,
    observed_functions: &[String],
) -> TrialResult<OracleSnapshot> {
    let mut egraph = EGraph::default();
    egraph.parse_and_run_program(Some("fixture".to_string()), program)?;
    snapshot_functions(&egraph, "fixture-final", observed_functions)
}

pub fn run_path_trial() -> TrialResult<TrialReport> {
    let spec = path_scenario();
    let snapshots = run_native_oracle(&spec)?;
    let setup = snapshots
        .first()
        .ok_or_else(|| "path scenario produced no setup snapshot".to_string())?;
    let final_snapshot = snapshots
        .last()
        .ok_or_else(|| "path scenario produced no final snapshot".to_string())?;

    let edge_pairs = logical_pairs(setup, "edge")?;
    let dd_pairs = dd_reachability(&edge_pairs.iter().copied().collect::<Vec<_>>());
    let dd_logical_rows = pairs_to_rows("path", &dd_pairs);
    let oracle_final_rows = logical_rows(final_snapshot, "path")?;
    let matches_oracle = dd_logical_rows == oracle_final_rows;

    Ok(TrialReport {
        scenario: spec.name,
        oracle_snapshots: snapshots,
        dd_logical_rows,
        oracle_final_rows,
        matches_oracle,
        observations: vec![
            "oracle rows were exported with EGraph::function_for_each, not print-function".to_string(),
            "logical relation identity projects i64 input columns and keeps raw output ids as debug evidence".to_string(),
        ],
        limitations: vec![
            "this preflight covers relation-only reachability over i64 values".to_string(),
            "scheduler admission, equality/rebuild, containers, primitives, extraction, and proofs remain follow-up probes".to_string(),
        ],
    })
}

pub fn write_report(path: impl AsRef<Path>, report: &TrialReport) -> TrialResult<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let body = serde_json::to_string_pretty(report)?;
    fs::write(path, format!("{body}\n"))?;
    Ok(())
}

pub fn dd_reachability(edges: &[Pair]) -> BTreeSet<Pair> {
    let edge_data = edges.to_vec();
    let diffs = Arc::new(Mutex::new(BTreeMap::<Pair, isize>::new()));
    let diffs_for_worker = Arc::clone(&diffs);

    timely::execute_directly(move |worker| {
        let mut probe = Handle::new();
        let diffs_for_dataflow = Arc::clone(&diffs_for_worker);
        let (mut input, ()) = worker.dataflow::<u64, _, _>(|scope| {
            let (input, edge_collection) = scope.new_collection::<Pair, isize>();

            let reachable = edge_collection.clone().iterate(|inner_scope, path| {
                let edges = edge_collection.enter(inner_scope);
                let edges_by_src = edges.clone().map(|Pair { x, y }| (x, y));
                let recursive = path
                    .map(|Pair { x, y }| (y, x))
                    .join_map(edges_by_src, |_mid, x, z| Pair { x: *x, y: *z });

                edges.concat(recursive).distinct()
            });

            reachable
                .consolidate()
                .inspect(move |(pair, _time, diff)| {
                    let mut diffs = diffs_for_dataflow.lock().expect("DD result map poisoned");
                    *diffs.entry(*pair).or_insert(0) += *diff;
                })
                .probe_with(&mut probe);

            (input, ())
        });

        for edge in edge_data {
            input.insert(edge);
        }
        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));
    });

    let diffs = diffs.lock().expect("DD result map poisoned");
    diffs
        .iter()
        .filter_map(|(pair, diff)| (*diff > 0).then_some(*pair))
        .collect()
}

pub fn logical_rows(snapshot: &OracleSnapshot, function: &str) -> TrialResult<Vec<LogicalRow>> {
    snapshot
        .logical_rows
        .get(function)
        .cloned()
        .ok_or_else(|| format!("missing logical rows for function {function}").into())
}

pub fn logical_pairs(snapshot: &OracleSnapshot, function: &str) -> TrialResult<BTreeSet<Pair>> {
    let mut pairs = BTreeSet::new();
    for row in logical_rows(snapshot, function)? {
        if row.values.len() != 2 {
            return Err(format!(
                "expected binary relation row for {function}, found {:?}",
                row.values
            )
            .into());
        }
        pairs.insert(Pair {
            x: row.values[0],
            y: row.values[1],
        });
    }
    Ok(pairs)
}

fn snapshot_functions(
    egraph: &EGraph,
    stage: &str,
    functions: &[String],
) -> TrialResult<OracleSnapshot> {
    let mut rows = BTreeMap::new();
    let mut logical = BTreeMap::new();

    for function in functions {
        let function_rows = lower_rows(egraph, function)?;
        let mut logical_rows = project_i64_logical_rows(&function_rows);
        logical_rows.sort();
        rows.insert(function.clone(), function_rows);
        logical.insert(function.clone(), logical_rows);
    }

    Ok(OracleSnapshot {
        stage: stage.to_string(),
        rows,
        logical_rows: logical,
    })
}

fn lower_rows(egraph: &EGraph, function: &str) -> TrialResult<Vec<LowerRow>> {
    let function_info = egraph
        .get_function(function)
        .ok_or_else(|| format!("missing function {function}"))?;
    let schema = function_info.schema();
    let input_sorts = schema
        .input
        .iter()
        .map(|sort| sort.name().to_string())
        .collect::<Vec<_>>();
    let output_sort = schema.output.name().to_string();
    let mut rows = Vec::new();

    egraph.function_for_each(function, |row| {
        let input_values = row
            .vals
            .iter()
            .take(input_sorts.len())
            .zip(input_sorts.iter())
            .map(|(value, sort)| decode_value(egraph, sort, *value))
            .collect::<Vec<_>>();
        let output_value = row
            .vals
            .get(input_sorts.len())
            .map(|value| decode_value(egraph, &output_sort, *value))
            .unwrap_or_else(|| DecodedValue::Raw {
                sort: output_sort.clone(),
                value: "<missing-output>".to_string(),
            });

        rows.push(LowerRow {
            function: function.to_string(),
            input_sorts: input_sorts.clone(),
            output_sort: output_sort.clone(),
            raw_values: row.vals.iter().map(raw_value).collect(),
            input_values,
            output_value,
            subsumed: row.subsumed,
        });
    })?;

    rows.sort();
    Ok(rows)
}

fn decode_value(egraph: &EGraph, sort: &str, value: Value) -> DecodedValue {
    if sort == "i64" {
        DecodedValue::I64 {
            value: egraph.value_to_base::<i64>(value),
        }
    } else {
        DecodedValue::Raw {
            sort: sort.to_string(),
            value: raw_value(&value),
        }
    }
}

fn raw_value(value: &Value) -> String {
    format!("{value:?}")
}

fn project_i64_logical_rows(rows: &[LowerRow]) -> Vec<LogicalRow> {
    rows.iter()
        .filter(|row| !row.subsumed)
        .filter_map(|row| {
            let mut values = Vec::with_capacity(row.input_values.len());
            for value in &row.input_values {
                match value {
                    DecodedValue::I64 { value } => values.push(*value),
                    DecodedValue::Raw { .. } => return None,
                }
            }
            Some(LogicalRow {
                function: row.function.clone(),
                values,
            })
        })
        .collect()
}

fn pairs_to_rows(function: &str, pairs: &BTreeSet<Pair>) -> Vec<LogicalRow> {
    pairs
        .iter()
        .map(|pair| LogicalRow {
            function: function.to_string(),
            values: vec![pair.x, pair.y],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected_path_pairs() -> BTreeSet<Pair> {
        [
            Pair { x: 1, y: 2 },
            Pair { x: 2, y: 3 },
            Pair { x: 3, y: 4 },
            Pair { x: 1, y: 3 },
            Pair { x: 2, y: 4 },
            Pair { x: 1, y: 4 },
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn oracle_lower_rows_path() -> TrialResult<()> {
        let observed = vec!["edge".to_string(), "path".to_string()];
        let snapshot = run_fixture_program(PATH_REACHABILITY_EGG, &observed)?;

        assert_eq!(logical_pairs(&snapshot, "edge")?.len(), 3);
        assert_eq!(logical_pairs(&snapshot, "path")?, expected_path_pairs());
        let path_rows = snapshot.rows.get("path").expect("path rows");
        assert!(path_rows
            .iter()
            .all(|row| row.input_sorts == ["i64", "i64"]));
        assert!(path_rows
            .iter()
            .all(|row| row.raw_values.len() == row.input_sorts.len() + 1));

        Ok(())
    }

    #[test]
    fn staged_oracle_snapshots() -> TrialResult<()> {
        let snapshots = run_native_oracle(&path_scenario())?;
        let counts = snapshots
            .iter()
            .map(|snapshot| logical_pairs(snapshot, "path").map(|rows| rows.len()))
            .collect::<TrialResult<Vec<_>>>()?;

        assert_eq!(counts.first().copied(), Some(0));
        assert_eq!(counts.last().copied(), Some(expected_path_pairs().len()));
        assert!(counts.windows(2).all(|window| window[0] <= window[1]));

        Ok(())
    }

    #[test]
    fn dd_reachability_matches_oracle() -> TrialResult<()> {
        let report = run_path_trial()?;

        assert!(report.matches_oracle);
        assert_eq!(
            report
                .dd_logical_rows
                .iter()
                .map(|row| Pair {
                    x: row.values[0],
                    y: row.values[1],
                })
                .collect::<BTreeSet<_>>(),
            expected_path_pairs()
        );

        Ok(())
    }
}
