//! Minimal relation-only Differential Dataflow trial against native egglog.
//!
//! This crate is a coding preflight, not a replacement egglog backend. Each
//! scenario is represented twice: native egglog runs a real `.egg` fixture and
//! exports lower function-table rows through `EGraph::function_for_each`, while
//! the trial side evaluates a small hand-written relation/rule model in
//! Differential Dataflow. The comparison deliberately projects logical `i64`
//! input tuples and keeps lower-row output ids, raw values, sorts, and
//! `subsumed` flags as debug evidence.
//!
//! Implemented capacity is intentionally narrow: base `i64` relation facts,
//! relation atoms, repeated-variable equality filters, natural joins over shared
//! variables, non-recursive multi-way joins, recursive transitive closure via
//! `iterate`, explicit native staging snapshots, and final sorted logical-row
//! equality against the lower-row oracle.
//!
//! Deferred capacity is equally important: equality/merge/rebuild semantics,
//! containers, custom schedulers, host callbacks, extraction/proofs, direct
//! `ResolvedCoreRule` export, and performance measurement are not modeled here.
//! The point of this crate is to make the first DD mapping concrete enough that
//! later semantic and performance questions have a stable baseline.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use differential_dataflow::input::Input;
use differential_dataflow::lattice::Lattice;
use differential_dataflow::operators::Iterate;
use differential_dataflow::VecCollection;
use egglog::{EGraph, Value};
use serde::{Deserialize, Serialize};
use timely::progress::Timestamp;

pub type TrialResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub const PATH_REACHABILITY_EGG: &str = include_str!("../fixtures/path-reachability.egg");
pub const REPEATED_VARIABLE_EGG: &str = include_str!("../fixtures/repeated-variable.egg");
pub const THREE_WAY_JOIN_EGG: &str = include_str!("../fixtures/three-way-join.egg");

/// Logical relation tuple used by the DD model.
///
/// This is the DD-side row identity for the first gate: a relation name plus
/// the logical `i64` input tuple. It intentionally does not include egglog's
/// lower-row output/eclass id, because those ids are oracle debug evidence
/// rather than the relation payload being compared.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct RelationRow {
    pub relation: String,
    pub values: Vec<i64>,
}

/// Term in a relation atom.
///
/// Constants are supported by the matcher because they naturally fall out of
/// atom filtering, although the first acceptance scenarios only need variables.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum AtomTerm {
    Var(String),
    Const(i64),
}

/// A body or head atom in the small relation-rule language.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Atom {
    pub relation: String,
    pub terms: Vec<AtomTerm>,
}

/// One Datalog-shaped rule for the DD trial evaluator.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct RuleSpec {
    pub name: String,
    pub body: Vec<Atom>,
    pub head: Atom,
}

/// Paired native-oracle and DD-model description for one scenario.
///
/// `stages` are fed to native egglog for oracle snapshots. `facts` and `rules`
/// are the manually inspected DD model of the same relation-only program.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScenarioSpec {
    pub name: String,
    pub observed_functions: Vec<String>,
    // Native egglog stages are the oracle program. The DD side intentionally
    // uses the hand-written facts/rules below so the mapping stays inspectable.
    pub stages: Vec<ScenarioStage>,
    pub facts: Vec<RelationRow>,
    pub rules: Vec<RuleSpec>,
}

/// One native egglog execution step.
///
/// Staging lets the oracle expose intermediate lower-row snapshots while the DD
/// side currently computes the final relation closure in one dataflow.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScenarioStage {
    pub label: String,
    pub program: String,
}

/// Raw lower function-table row exported from native egglog.
///
/// These rows preserve the details needed to debug the oracle boundary: schema
/// sort names, raw lower values, decoded inputs/output where possible, and the
/// `subsumed` bit. Logical comparison is derived from these rows but does not
/// discard them from the report.
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

/// Decoded lower-row value.
///
/// The first gate only projects `i64` input columns into `LogicalRow`; other
/// values remain raw so non-`i64` evidence is still visible in reports.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DecodedValue {
    I64 { value: i64 },
    Raw { sort: String, value: String },
}

/// Comparable logical row projected from native lower rows or DD rows.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct LogicalRow {
    pub function: String,
    pub values: Vec<i64>,
}

/// Native egglog oracle snapshot after one staged execution step.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OracleSnapshot {
    pub stage: String,
    pub rows: BTreeMap<String, Vec<LowerRow>>,
    pub logical_rows: BTreeMap<String, Vec<LogicalRow>>,
}

/// Per-scenario result comparing native lower rows with DD logical rows.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScenarioReport {
    pub scenario: String,
    pub oracle_snapshots: Vec<OracleSnapshot>,
    pub dd_logical_rows: BTreeMap<String, Vec<LogicalRow>>,
    pub oracle_final_rows: BTreeMap<String, Vec<LogicalRow>>,
    pub matches_oracle: bool,
}

/// Aggregate acceptance report emitted by the CLI and tests.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrialReport {
    pub scenarios: Vec<ScenarioReport>,
    pub all_match_oracle: bool,
    pub observations: Vec<String>,
    pub limitations: Vec<String>,
}

type Binding = BTreeMap<String, i64>;
type RelationCollection<'scope, T> = VecCollection<'scope, T, RelationRow, isize>;

/// All scenarios currently included in the first semantic acceptance gate.
pub fn acceptance_scenarios() -> Vec<ScenarioSpec> {
    vec![
        path_scenario(),
        repeated_variable_scenario(),
        three_way_join_scenario(),
    ]
}

/// Recursive reachability scenario.
///
/// This is the smallest recursive DD shape in the gate: base `edge` facts,
/// direct `edge -> path`, and recursive `path(x,y), edge(y,z) -> path(x,z)`.
pub fn path_scenario() -> ScenarioSpec {
    scenario(
        "path-reachability",
        PATH_REACHABILITY_EGG,
        "(run 3)",
        3,
        &["edge", "path"],
        rows("edge", &[&[1, 2], &[2, 3], &[3, 4]]),
        vec![
            RuleSpec {
                name: "edge-to-path".to_string(),
                body: vec![atom("edge", &[var("x"), var("y")])],
                head: atom("path", &[var("x"), var("y")]),
            },
            RuleSpec {
                name: "path-step".to_string(),
                body: vec![
                    atom("path", &[var("x"), var("y")]),
                    atom("edge", &[var("y"), var("z")]),
                ],
                head: atom("path", &[var("x"), var("z")]),
            },
        ],
    )
}

/// Repeated-variable scenario.
///
/// The body atom `(pair x x)` checks that atom matching treats repeated
/// variables as equality constraints before rules are joined or projected.
pub fn repeated_variable_scenario() -> ScenarioSpec {
    scenario(
        "repeated-variable",
        REPEATED_VARIABLE_EGG,
        "(run 1)",
        1,
        &["pair", "same"],
        rows("pair", &[&[1, 1], &[1, 2], &[2, 2]]),
        vec![RuleSpec {
            name: "diagonal".to_string(),
            body: vec![atom("pair", &[var("x"), var("x")])],
            head: atom("same", &[var("x")]),
        }],
    )
}

/// Non-recursive three-way join scenario.
///
/// This checks that the rule lowering can chain shared-variable joins across
/// more than two body atoms without relying on reachability-specific code.
pub fn three_way_join_scenario() -> ScenarioSpec {
    let mut facts = rows("a", &[&[1, 2], &[9, 9]]);
    facts.extend(rows("b", &[&[2, 3], &[2, 4], &[9, 3]]));
    facts.extend(rows("c", &[&[3, 5], &[4, 6], &[8, 8]]));

    scenario(
        "three-way-join",
        THREE_WAY_JOIN_EGG,
        "(run 1)",
        1,
        &["a", "b", "c", "out"],
        facts,
        vec![RuleSpec {
            name: "abc-to-out".to_string(),
            body: vec![
                atom("a", &[var("x"), var("y")]),
                atom("b", &[var("y"), var("z")]),
                atom("c", &[var("z"), var("w")]),
            ],
            head: atom("out", &[var("x"), var("w")]),
        }],
    )
}

/// Construct a scenario from a native fixture and an explicit DD model.
///
/// The fixture remains the source of truth for native egglog execution. The
/// DD facts/rules are intentionally written next to the fixture reference so a
/// reader can audit the mapping instead of trusting a parser or bridge.
fn scenario(
    name: &str,
    fixture: &str,
    final_run_command: &str,
    run_steps: usize,
    observed_functions: &[&str],
    facts: Vec<RelationRow>,
    rules: Vec<RuleSpec>,
) -> ScenarioSpec {
    let setup = fixture
        .trim_end()
        .strip_suffix(final_run_command)
        .unwrap_or_else(|| panic!("fixture must end with {final_run_command:?}"))
        .trim_end()
        .to_string();
    let mut stages = vec![ScenarioStage {
        label: "setup".to_string(),
        program: setup,
    }];

    for index in 1..=run_steps {
        stages.push(ScenarioStage {
            label: format!("run-{index}"),
            program: "(run 1)".to_string(),
        });
    }

    ScenarioSpec {
        name: name.to_string(),
        observed_functions: names(observed_functions),
        stages,
        facts,
        rules,
    }
}

/// Run every acceptance scenario and assemble the aggregate report.
///
/// This is the CLI entrypoint's main library call. A single failing scenario
/// keeps its full oracle/DD row evidence in the report and flips
/// `all_match_oracle` to false.
pub fn run_acceptance_trial() -> TrialResult<TrialReport> {
    let scenarios = acceptance_scenarios()
        .iter()
        .map(run_scenario_trial)
        .collect::<TrialResult<Vec<_>>>()?;
    let all_match_oracle = scenarios.iter().all(|scenario| scenario.matches_oracle);

    Ok(TrialReport {
        scenarios,
        all_match_oracle,
        observations: vec![
            "oracle rows were exported with EGraph::function_for_each, not print-function"
                .to_string(),
            "DD results are netted from signed diffs before logical-row comparison".to_string(),
            "relation identity projects i64 input columns and keeps raw lower-row output ids as debug evidence".to_string(),
        ],
        limitations: vec![
            "this acceptance gate is relation-only over i64 values".to_string(),
            "equality/rebuild, containers, custom schedulers, primitives, extraction, proofs, and direct ResolvedCoreRule export remain follow-up probes".to_string(),
            "performance measurement is intentionally deferred until semantic acceptance is stable".to_string(),
        ],
    })
}

/// Run one scenario through native egglog and the DD evaluator, then compare.
///
/// This is the main semantic gate: native egglog provides lower-row oracle
/// snapshots, DD computes relation rows, and both sides are projected to sorted
/// logical `i64` rows for exact equality.
pub fn run_scenario_trial(spec: &ScenarioSpec) -> TrialResult<ScenarioReport> {
    // Each scenario is run twice: native egglog exports lower table rows as the
    // oracle, while DD evaluates the small relation-only model.
    let oracle_snapshots = run_native_oracle(spec)?;
    let final_snapshot = oracle_snapshots
        .last()
        .ok_or_else(|| format!("scenario {} produced no final snapshot", spec.name))?;
    let oracle_final_rows = spec
        .observed_functions
        .iter()
        .map(|function| Ok((function.clone(), logical_rows(final_snapshot, function)?)))
        .collect::<TrialResult<BTreeMap<_, _>>>()?;
    let dd_rows = dd_evaluate_scenario(spec);
    let dd_logical_rows = spec
        .observed_functions
        .iter()
        .map(|function| {
            let rows = dd_rows
                .iter()
                .filter(|row| row.relation == *function)
                .map(|row| LogicalRow {
                    function: function.clone(),
                    values: row.values.clone(),
                })
                .collect::<Vec<_>>();
            (function.clone(), rows)
        })
        .collect();
    let matches_oracle = dd_logical_rows == oracle_final_rows;

    Ok(ScenarioReport {
        scenario: spec.name.clone(),
        oracle_snapshots,
        dd_logical_rows,
        oracle_final_rows,
        matches_oracle,
    })
}

/// Execute the staged native egglog oracle for a scenario.
///
/// All stages run on one `EGraph`, so later snapshots include effects from
/// earlier declarations, facts, and runs. This matches the explicit staged
/// boundary the MVP intends to compare against.
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

/// Run a complete `.egg` fixture and return one final lower-row snapshot.
///
/// Tests use this for the direct oracle preflight where no staged replay is
/// needed.
pub fn run_fixture_program(
    program: &str,
    observed_functions: &[String],
) -> TrialResult<OracleSnapshot> {
    let mut egraph = EGraph::default();
    egraph.parse_and_run_program(Some("fixture".to_string()), program)?;
    snapshot_functions(&egraph, "fixture-final", observed_functions)
}

/// Write a JSON acceptance report, creating the parent directory if needed.
pub fn write_report(path: impl AsRef<Path>, report: &TrialReport) -> TrialResult<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let body = serde_json::to_string_pretty(report)?;
    fs::write(path, format!("{body}\n"))?;
    Ok(())
}

/// Evaluate one scenario with a tiny relation-only DD fixpoint.
///
/// The dataflow imports all scenario facts at one outer timestamp, repeatedly
/// derives rule heads from the current closure, and returns the set of rows with
/// a positive net differential count after the fixpoint stabilizes. This models
/// final relation contents only; native per-stage scheduling and per-rule
/// freshness are intentionally outside this first evaluator.
pub fn dd_evaluate_scenario(spec: &ScenarioSpec) -> BTreeSet<RelationRow> {
    let facts = spec.facts.clone();
    let rules = spec.rules.clone();
    let diffs = Arc::new(Mutex::new(BTreeMap::<RelationRow, isize>::new()));
    let diffs_for_worker = Arc::clone(&diffs);

    timely::execute_directly(move |worker| {
        let (mut input, probe) = worker.dataflow::<u64, _, _>(move |scope| {
            // Timely still supplies the worker and progress tracking, but the
            // actual evaluator starts as a Differential `Collection` and stays
            // in collection operators from this point on.
            let (input, base_rows) = scope.new_collection_from(facts);
            let base_rows_for_iter = base_rows.clone();

            // DD fixpoint shape:
            //
            // - `base_rows` is the static fact collection in the outer scope.
            //   Entering it into the nested scope makes the facts available at
            //   every recursive iteration without re-sending them from Rust.
            // - `rows` is Differential's current approximation of the closure.
            //   Each rule derives candidate head rows from that approximation.
            // - `concat(...).distinct()` implements set semantics: duplicates
            //   can be produced by multiple rules or iterations, but the visible
            //   relation contains each logical tuple once.
            let closure = base_rows.iterate(move |inner_scope, rows| {
                let base_rows = base_rows_for_iter.clone().enter(inner_scope);
                let empty = rows.clone().filter(|_| false);
                let derived = rules.iter().cloned().fold(empty, |acc, rule| {
                    acc.concat(apply_rule(rows.clone(), rule))
                });
                base_rows.concat(derived).distinct()
            });

            // Differential collections emit signed updates, not a final Vec.
            // `consolidate` coalesces updates at the current timestamp, and
            // `inspect` lets this single-process test harness copy those diffs
            // back to Rust. We still net by row on the host side because future
            // rules with negation/residual behavior can produce meaningful
            // negative corrections before the final visible set is known.
            let diffs = Arc::clone(&diffs_for_worker);
            let (probe, _) = closure
                .consolidate()
                .inspect(move |(row, _time, diff)| {
                    let mut diffs = diffs.lock().expect("DD result map poisoned");
                    *diffs.entry(row.clone()).or_insert(0) += *diff;
                })
                .probe();

            (input, probe)
        });
        input.advance_to(1);
        input.flush();
        // The probe is the handoff point from Timely progress to the host test:
        // after it is no longer less than the input time, all dataflow work
        // caused by the inserted facts has completed.
        worker.step_while(|| probe.less_than(input.time()));
    });

    let diffs = diffs.lock().expect("DD result map poisoned");
    diffs
        .iter()
        .filter_map(|(row, diff)| (*diff > 0).then_some(row.clone()))
        .collect()
}

/// Fetch projected logical rows for one function from an oracle snapshot.
pub fn logical_rows(snapshot: &OracleSnapshot, function: &str) -> TrialResult<Vec<LogicalRow>> {
    snapshot
        .logical_rows
        .get(function)
        .cloned()
        .ok_or_else(|| format!("missing logical rows for function {function}").into())
}

/// Lower and evaluate one relation rule inside a DD scope.
///
/// Each body atom becomes a collection of bindings from variable names to `i64`
/// values. Body atoms are then joined on variables they share with previous
/// atoms. If there are no shared variables, the join key is empty and the result
/// is the expected Cartesian product for independent atoms.
fn apply_rule<'scope, T>(
    rows: RelationCollection<'scope, T>,
    rule: RuleSpec,
) -> RelationCollection<'scope, T>
where
    T: Timestamp + Lattice + Ord + 'static,
{
    let Some((first, rest)) = rule.body.split_first() else {
        return rows.filter(|_| false);
    };

    // Start from the first atom's matching rows, then join in each additional
    // atom. This mirrors a simple left-deep relational plan; it is deliberately
    // not a WCOJ or optimized physical planner yet.
    let mut bindings = atom_bindings(rows.clone(), first.clone());
    let mut known_vars = atom_vars(first);

    for atom in rest.iter().cloned() {
        let atom_vars = atom_vars(&atom);
        let shared = known_vars
            .intersection(&atom_vars)
            .cloned()
            .collect::<Vec<_>>();
        let shared_for_left = shared.clone();
        let shared_for_right = shared.clone();
        let atom_side = atom_bindings(rows.clone(), atom);

        bindings = bindings
            .map(move |binding| (binding_key(&binding, &shared_for_left), binding))
            .join_map(
                atom_side.map(move |binding| (binding_key(&binding, &shared_for_right), binding)),
                |_key, left, right| {
                    let mut merged = left.clone();
                    for (name, value) in right {
                        if let Some(existing) = merged.insert(name.clone(), *value) {
                            assert_eq!(
                                existing, *value,
                                "bindings joined on shared variables should merge"
                            );
                        }
                    }
                    merged
                },
            );

        known_vars.extend(atom_vars);
    }

    let head = rule.head;
    // Project successful body bindings into the head relation. A missing head
    // variable drops the binding, which keeps malformed scenario specs from
    // silently inventing values.
    bindings.flat_map(move |binding| {
        let mut values = Vec::with_capacity(head.terms.len());
        for term in &head.terms {
            match term {
                AtomTerm::Var(name) => values.push(*binding.get(name)?),
                AtomTerm::Const(value) => values.push(*value),
            }
        }

        Some(RelationRow {
            relation: head.relation.clone(),
            values,
        })
    })
}

/// Convert relation rows matching one atom into variable bindings.
///
/// This is a DD `flat_map`: non-matching rows produce no binding, matching rows
/// produce exactly one binding.
fn atom_bindings<'scope, T>(
    rows: RelationCollection<'scope, T>,
    atom: Atom,
) -> VecCollection<'scope, T, Binding, isize>
where
    T: Timestamp + Lattice + Ord + 'static,
{
    rows.flat_map(move |row| match_atom(row, &atom))
}

/// Match one logical relation row against one atom outside DD.
///
/// The returned binding is the row-level payload that DD later joins. Repeated
/// variables are checked here so later joins can treat a binding as internally
/// consistent.
fn match_atom(row: RelationRow, atom: &Atom) -> Option<Binding> {
    if row.relation != atom.relation || row.values.len() != atom.terms.len() {
        return None;
    }

    let mut values = BTreeMap::new();
    for (term, value) in atom.terms.iter().zip(row.values.iter().copied()) {
        match term {
            AtomTerm::Var(name) => {
                // Repeated variables are equality filters, e.g. `(pair x x)`.
                if let Some(existing) = values.insert(name.clone(), value) {
                    if existing != value {
                        return None;
                    }
                }
            }
            AtomTerm::Const(expected) => {
                if *expected != value {
                    return None;
                }
            }
        }
    }

    Some(values)
}

/// Build the DD join key for a binding from a deterministic variable list.
fn binding_key(binding: &Binding, names: &[String]) -> Vec<i64> {
    names
        .iter()
        .map(|name| *binding.get(name).expect("missing join key variable"))
        .collect()
}

/// Collect distinct variable names referenced by one atom.
fn atom_vars(atom: &Atom) -> BTreeSet<String> {
    atom.terms
        .iter()
        .filter_map(|term| match term {
            AtomTerm::Var(name) => Some(name.clone()),
            AtomTerm::Const(_) => None,
        })
        .collect()
}

/// Export lower rows and projected logical rows for all observed functions.
fn snapshot_functions(
    egraph: &EGraph,
    stage: &str,
    functions: &[String],
) -> TrialResult<OracleSnapshot> {
    let mut rows = BTreeMap::new();
    let mut logical = BTreeMap::new();

    for function in functions {
        // This is the oracle boundary: read the public lower function table
        // rows, not rendered `print-function` / TermDag output. The schema tells
        // us how many leading `vals` entries are logical inputs; the next value
        // is the lower output/eclass id.
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
        let mut function_rows = Vec::new();

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

            function_rows.push(LowerRow {
                function: function.to_string(),
                input_sorts: input_sorts.clone(),
                output_sort: output_sort.clone(),
                raw_values: row.vals.iter().map(raw_value).collect(),
                input_values,
                output_value,
                subsumed: row.subsumed,
            });
        })?;

        function_rows.sort();
        let mut logical_rows = function_rows
            .iter()
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
            .collect::<Vec<_>>();
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

/// Decode a lower egglog value using the sort name when this trial understands it.
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

/// Stable debug rendering for lower values this trial does not decode.
fn raw_value(value: &Value) -> String {
    format!("{value:?}")
}

fn rows(relation: &str, values: &[&[i64]]) -> Vec<RelationRow> {
    values
        .iter()
        .map(|values| RelationRow {
            relation: relation.to_string(),
            values: values.to_vec(),
        })
        .collect()
}

fn atom(relation: &str, terms: &[AtomTerm]) -> Atom {
    Atom {
        relation: relation.to_string(),
        terms: terms.to_vec(),
    }
}

fn var(name: &str) -> AtomTerm {
    AtomTerm::Var(name.to_string())
}

fn names(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected_rows(function: &str, values: &[&[i64]]) -> Vec<LogicalRow> {
        values
            .iter()
            .map(|values| LogicalRow {
                function: function.to_string(),
                values: values.to_vec(),
            })
            .collect()
    }

    fn scenario_report(name: &str) -> TrialResult<ScenarioReport> {
        acceptance_scenarios()
            .iter()
            .find(|scenario| scenario.name == name)
            .ok_or_else(|| format!("missing scenario {name}").into())
            .and_then(run_scenario_trial)
    }

    #[test]
    fn oracle_lower_rows_path() -> TrialResult<()> {
        let snapshot = run_fixture_program(PATH_REACHABILITY_EGG, &names(&["edge", "path"]))?;

        assert_eq!(
            logical_rows(&snapshot, "path")?,
            expected_rows(
                "path",
                &[&[1, 2], &[1, 3], &[1, 4], &[2, 3], &[2, 4], &[3, 4]]
            )
        );
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
            .map(|snapshot| logical_rows(snapshot, "path").map(|rows| rows.len()))
            .collect::<TrialResult<Vec<_>>>()?;

        assert_eq!(counts.first().copied(), Some(0));
        assert_eq!(counts.last().copied(), Some(6));
        assert!(counts.windows(2).all(|window| window[0] <= window[1]));

        Ok(())
    }

    #[test]
    fn generic_dd_path_reachability_matches_oracle() -> TrialResult<()> {
        let report = scenario_report("path-reachability")?;

        assert!(report.matches_oracle);
        assert_eq!(
            report
                .dd_logical_rows
                .get("path")
                .cloned()
                .unwrap_or_default(),
            expected_rows(
                "path",
                &[&[1, 2], &[1, 3], &[1, 4], &[2, 3], &[2, 4], &[3, 4]]
            )
        );

        Ok(())
    }

    #[test]
    fn generic_dd_repeated_variable_matches_oracle() -> TrialResult<()> {
        let report = scenario_report("repeated-variable")?;

        assert!(report.matches_oracle);
        assert_eq!(
            report
                .dd_logical_rows
                .get("same")
                .cloned()
                .unwrap_or_default(),
            expected_rows("same", &[&[1], &[2]])
        );

        Ok(())
    }

    #[test]
    fn generic_dd_three_way_join_matches_oracle() -> TrialResult<()> {
        let report = scenario_report("three-way-join")?;

        assert!(report.matches_oracle);
        assert_eq!(
            report
                .dd_logical_rows
                .get("out")
                .cloned()
                .unwrap_or_default(),
            expected_rows("out", &[&[1, 5], &[1, 6], &[9, 5]])
        );

        Ok(())
    }

    #[test]
    fn all_acceptance_scenarios_match_oracle() -> TrialResult<()> {
        let report = run_acceptance_trial()?;

        assert!(report.all_match_oracle);
        assert_eq!(report.scenarios.len(), 3);
        assert!(report
            .scenarios
            .iter()
            .all(|scenario| scenario.matches_oracle));

        Ok(())
    }
}
