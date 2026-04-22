use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::env;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use differential_dataflow::input::InputSession;
use serde::{Deserialize, Serialize};
use timely::dataflow::operators::probe::Handle as ProbeHandle;

type Epoch = u64;
type Node = i64;

const EDGE_TS: Epoch = 1;
const TC1: u8 = 1;
const TC2: u8 = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Oracle,
    BrokenGlobal,
    DdBarrier,
    DdOverlap,
}

impl Mode {
    fn as_str(self) -> &'static str {
        match self {
            Mode::Oracle => "oracle",
            Mode::BrokenGlobal => "broken-global",
            Mode::DdBarrier => "dd-barrier",
            Mode::DdOverlap => "dd-overlap",
        }
    }

    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "oracle" => Ok(Mode::Oracle),
            "broken-global" => Ok(Mode::BrokenGlobal),
            "dd-barrier" => Ok(Mode::DdBarrier),
            "dd-overlap" => Ok(Mode::DdOverlap),
            _ => Err(format!("unknown mode: {s}")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Suite {
    Semantic,
    Scaling,
    All,
}

impl Suite {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "semantic" => Ok(Suite::Semantic),
            "scaling" => Ok(Suite::Scaling),
            "all" => Ok(Suite::All),
            _ => Err(format!("unknown suite: {s}")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorkloadKind {
    Mini,
    Chain,
    Fanout,
}

impl WorkloadKind {
    fn as_str(self) -> &'static str {
        match self {
            WorkloadKind::Mini => "mini",
            WorkloadKind::Chain => "chain",
            WorkloadKind::Fanout => "fanout",
        }
    }

    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "mini" => Ok(WorkloadKind::Mini),
            "chain" => Ok(WorkloadKind::Chain),
            "fanout" => Ok(WorkloadKind::Fanout),
            _ => Err(format!("unknown workload: {s}")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
struct Pair {
    x: Node,
    y: Node,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
struct TimedPair {
    x: Node,
    y: Node,
    row_ts: Epoch,
}

impl TimedPair {
    fn pair(self) -> Pair {
        Pair {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
struct Task {
    task_id: u64,
    ruleset: u8,
    last_run: Epoch,
    output_ts: Epoch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
struct Candidate {
    task_id: u64,
    ruleset: u8,
    x: Node,
    y: Node,
    output_ts: Epoch,
    source: u8,
}

impl Candidate {
    fn pair(self) -> Pair {
        Pair {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
struct JoinSeed {
    ruleset: u8,
    x: Node,
    y: Node,
    newest_ts: Epoch,
    source: u8,
}

#[derive(Clone, Debug)]
struct CandidateEvent {
    timely_time: Epoch,
    candidate: Candidate,
    diff: isize,
}

#[derive(Clone, Debug)]
struct Workload {
    kind: WorkloadKind,
    scale: usize,
    edge1: Vec<TimedPair>,
    edge2: Vec<TimedPair>,
    schedule: Vec<u8>,
}

impl Workload {
    fn new(kind: WorkloadKind, scale: usize) -> Self {
        match kind {
            WorkloadKind::Mini => Self {
                kind,
                scale: 1,
                edge1: vec![timed_pair(1, 2)],
                edge2: vec![timed_pair(2, 3)],
                schedule: vec![TC1, TC2, TC1],
            },
            WorkloadKind::Chain => {
                let edge_count = scale.max(2);
                let mut edge1 = Vec::new();
                let mut edge2 = Vec::new();
                for i in 1..=edge_count {
                    let pair = timed_pair(i as Node, i as Node + 1);
                    if i % 2 == 1 {
                        edge1.push(pair);
                    } else {
                        edge2.push(pair);
                    }
                }

                let mut schedule = Vec::new();
                for _ in 0..=(edge_count + 1) {
                    schedule.push(TC1);
                    schedule.push(TC2);
                }

                Self {
                    kind,
                    scale: edge_count,
                    edge1,
                    edge2,
                    schedule,
                }
            }
            WorkloadKind::Fanout => {
                let fanout = scale.max(1);
                let sink = fanout as Node + 2;
                let mut edge1 = Vec::new();
                let mut edge2 = Vec::new();
                for i in 0..fanout {
                    let mid = i as Node + 2;
                    edge1.push(timed_pair(1, mid));
                    edge2.push(timed_pair(mid, sink));
                }

                Self {
                    kind,
                    scale: fanout,
                    edge1,
                    edge2,
                    schedule: vec![TC1, TC2, TC1],
                }
            }
        }
    }

    fn tasks(&self) -> Vec<Task> {
        let mut last_run = BTreeMap::from([(TC1, 0), (TC2, 0)]);
        let mut tasks = Vec::with_capacity(self.schedule.len());
        let mut current_ts = EDGE_TS;

        for (index, &ruleset) in self.schedule.iter().enumerate() {
            let output_ts = current_ts + 1;
            let last = *last_run.get(&ruleset).unwrap_or(&0);
            tasks.push(Task {
                task_id: index as u64,
                ruleset,
                last_run: last,
                output_ts,
            });
            last_run.insert(ruleset, output_ts);
            current_ts = output_ts;
        }

        tasks
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FrontierSample {
    committed_task: u64,
    issued_through_task: u64,
    lag_tasks: u64,
    visible_frontier: Vec<Epoch>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ExperimentResult {
    workload: String,
    scale: usize,
    mode: String,
    workers: usize,
    overlap_window: usize,
    native_barriers: bool,
    semantic_equivalence: bool,
    per_rule_freshness: bool,
    expected_reachable_count: usize,
    actual_reachable_count: usize,
    missing_reachable_count: usize,
    extra_reachable_count: usize,
    missing_reachable_sample: Vec<Pair>,
    extra_reachable_sample: Vec<Pair>,
    early_visibility_violations: usize,
    runtime_micros: u128,
    candidate_count: usize,
    committed_row_count: usize,
    duplicate_stale_candidate_count: usize,
    frontier_lag_samples: Vec<FrontierSample>,
    barrier_count: usize,
    barrier_reasons: Vec<String>,
}

#[derive(Clone, Debug)]
struct RunMetrics {
    actual: BTreeSet<Pair>,
    candidate_count: usize,
    duplicate_stale_candidate_count: usize,
    early_visibility_violations: usize,
    frontier_lag_samples: Vec<FrontierSample>,
    barrier_count: usize,
    barrier_reasons: Vec<String>,
    last_run: BTreeMap<u8, Epoch>,
}

impl RunMetrics {
    fn empty() -> Self {
        Self {
            actual: BTreeSet::new(),
            candidate_count: 0,
            duplicate_stale_candidate_count: 0,
            early_visibility_violations: 0,
            frontier_lag_samples: Vec::new(),
            barrier_count: 0,
            barrier_reasons: Vec::new(),
            last_run: BTreeMap::from([(TC1, 0), (TC2, 0)]),
        }
    }
}

#[derive(Clone, Debug)]
struct Config {
    suite: Option<Suite>,
    mode: Option<Mode>,
    workload: WorkloadKind,
    scale: usize,
    workers: usize,
    overlap_window: usize,
    native_barriers: bool,
    json_out: Option<String>,
}

fn main() {
    if let Err(err) = run_cli() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run_cli() -> Result<(), String> {
    let config = Config::parse(env::args().skip(1))?;
    let results = if let Some(suite) = config.suite {
        run_suite(suite)?
    } else {
        let mode = config
            .mode
            .ok_or_else(|| "either --suite or --mode is required".to_string())?;
        vec![run_experiment(
            mode,
            Workload::new(config.workload, config.scale),
            config.workers,
            config.overlap_window,
            config.native_barriers,
        )?]
    };

    if let Some(path) = config.json_out {
        write_json(&path, &results)?;
    } else {
        for result in &results {
            println!(
                "{}",
                serde_json::to_string(result).map_err(|err| err.to_string())?
            );
        }
    }

    Ok(())
}

impl Config {
    fn parse<I>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = String>,
    {
        let mut config = Self {
            suite: None,
            mode: None,
            workload: WorkloadKind::Mini,
            scale: 1,
            workers: 1,
            overlap_window: 2,
            native_barriers: false,
            json_out: None,
        };

        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--suite" => {
                    let value = next_value(&mut iter, "--suite")?;
                    config.suite = Some(Suite::parse(&value)?);
                }
                "--mode" => {
                    let value = next_value(&mut iter, "--mode")?;
                    config.mode = Some(Mode::parse(&value)?);
                }
                "--workload" => {
                    let value = next_value(&mut iter, "--workload")?;
                    config.workload = WorkloadKind::parse(&value)?;
                }
                "--scale" => {
                    config.scale = parse_usize(&next_value(&mut iter, "--scale")?, "--scale")?;
                }
                "--workers" => {
                    config.workers =
                        parse_usize(&next_value(&mut iter, "--workers")?, "--workers")?;
                    if config.workers == 0 {
                        return Err("--workers must be greater than zero".to_string());
                    }
                }
                "--overlap-window" => {
                    config.overlap_window = parse_usize(
                        &next_value(&mut iter, "--overlap-window")?,
                        "--overlap-window",
                    )?;
                    if config.overlap_window == 0 {
                        return Err("--overlap-window must be greater than zero".to_string());
                    }
                }
                "--native-barriers" => {
                    config.native_barriers = true;
                }
                "--json-out" => {
                    config.json_out = Some(next_value(&mut iter, "--json-out")?);
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                other => return Err(format!("unknown argument: {other}")),
            }
        }

        if config.suite.is_some() && config.mode.is_some() {
            return Err("use either --suite or --mode, not both".to_string());
        }

        Ok(config)
    }
}

fn next_value(iter: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    iter.next()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn parse_usize(value: &str, flag: &str) -> Result<usize, String> {
    value
        .parse::<usize>()
        .map_err(|err| format!("invalid {flag} value {value:?}: {err}"))
}

fn print_help() {
    println!(
        "option3-overlap --suite semantic|scaling|all [--json-out PATH]\n\
         option3-overlap --mode oracle|broken-global|dd-barrier|dd-overlap \\\n+         \t[--workload mini|chain|fanout] [--scale N] [--workers N] \\\n+         \t[--overlap-window N] [--native-barriers] [--json-out PATH]"
    );
}

fn run_suite(suite: Suite) -> Result<Vec<ExperimentResult>, String> {
    let mut results = Vec::new();

    if matches!(suite, Suite::Semantic | Suite::All) {
        let workload = Workload::new(WorkloadKind::Mini, 1);
        results.push(run_experiment(Mode::Oracle, workload.clone(), 1, 1, false)?);
        results.push(run_experiment(
            Mode::BrokenGlobal,
            workload.clone(),
            1,
            1,
            false,
        )?);
        results.push(run_experiment(
            Mode::DdBarrier,
            workload.clone(),
            1,
            1,
            false,
        )?);
        results.push(run_experiment(Mode::DdOverlap, workload, 1, 2, false)?);
    }

    if matches!(suite, Suite::Scaling | Suite::All) {
        let scales = [8, 32, 128];
        let worker_counts = [1, 4];
        let windows = [1, 2, 4];

        for kind in [WorkloadKind::Chain, WorkloadKind::Fanout] {
            for scale in scales {
                let workload = Workload::new(kind, scale);
                for workers in worker_counts {
                    results.push(run_experiment(
                        Mode::DdBarrier,
                        workload.clone(),
                        workers,
                        1,
                        false,
                    )?);
                    for window in windows {
                        results.push(run_experiment(
                            Mode::DdOverlap,
                            workload.clone(),
                            workers,
                            window,
                            false,
                        )?);
                    }
                    results.push(run_experiment(
                        Mode::DdOverlap,
                        workload.clone(),
                        workers,
                        4,
                        true,
                    )?);
                }
            }
        }
    }

    Ok(results)
}

fn run_experiment(
    mode: Mode,
    workload: Workload,
    workers: usize,
    overlap_window: usize,
    native_barriers: bool,
) -> Result<ExperimentResult, String> {
    let expected = oracle_metrics(&workload).actual;
    let started = Instant::now();
    let metrics = match mode {
        Mode::Oracle => oracle_metrics(&workload),
        Mode::BrokenGlobal => broken_global_metrics(&workload),
        Mode::DdBarrier => dd_metrics(&workload, workers, 1, false)?,
        Mode::DdOverlap => dd_metrics(&workload, workers, overlap_window, native_barriers)?,
    };
    let runtime_micros = started.elapsed().as_micros();

    Ok(build_result(
        mode,
        &workload,
        workers,
        overlap_window,
        native_barriers,
        expected,
        metrics,
        runtime_micros,
    ))
}

fn build_result(
    mode: Mode,
    workload: &Workload,
    workers: usize,
    overlap_window: usize,
    native_barriers: bool,
    expected: BTreeSet<Pair>,
    metrics: RunMetrics,
    runtime_micros: u128,
) -> ExperimentResult {
    let missing = expected
        .difference(&metrics.actual)
        .copied()
        .collect::<Vec<_>>();
    let extra = metrics
        .actual
        .difference(&expected)
        .copied()
        .collect::<Vec<_>>();
    let semantic_equivalence = missing.is_empty() && extra.is_empty();
    let per_rule_freshness = metrics.actual.contains(&Pair { x: 1, y: 3 });

    let expected_reachable_count = expected.len();
    let actual_reachable_count = metrics.actual.len();
    let missing_reachable_count = missing.len();
    let extra_reachable_count = extra.len();
    let committed_row_count = metrics.actual.len();

    ExperimentResult {
        workload: workload.kind.as_str().to_string(),
        scale: workload.scale,
        mode: mode.as_str().to_string(),
        workers,
        overlap_window,
        native_barriers,
        semantic_equivalence,
        per_rule_freshness,
        expected_reachable_count,
        actual_reachable_count,
        missing_reachable_count,
        extra_reachable_count,
        missing_reachable_sample: missing.into_iter().take(8).collect(),
        extra_reachable_sample: extra.into_iter().take(8).collect(),
        early_visibility_violations: metrics.early_visibility_violations,
        runtime_micros,
        candidate_count: metrics.candidate_count,
        committed_row_count,
        duplicate_stale_candidate_count: metrics.duplicate_stale_candidate_count,
        frontier_lag_samples: metrics.frontier_lag_samples,
        barrier_count: metrics.barrier_count,
        barrier_reasons: metrics.barrier_reasons,
    }
}

fn oracle_metrics(workload: &Workload) -> RunMetrics {
    let mut metrics = RunMetrics::empty();
    let mut timed_reachable = BTreeMap::<Pair, Epoch>::new();

    for task in workload.tasks() {
        let candidates = native_candidates(workload, &timed_reachable, task);
        let mut seen_this_task = BTreeSet::new();
        for candidate in candidates {
            metrics.candidate_count += 1;
            let pair = candidate.pair();
            if timed_reachable.contains_key(&pair) || !seen_this_task.insert(pair) {
                metrics.duplicate_stale_candidate_count += 1;
                continue;
            }
            timed_reachable.insert(pair, task.output_ts);
            metrics.actual.insert(pair);
        }
        metrics.last_run.insert(task.ruleset, task.output_ts);
    }

    metrics
}

fn broken_global_metrics(workload: &Workload) -> RunMetrics {
    let mut metrics = RunMetrics::empty();
    let mut reachable = BTreeSet::<Pair>::new();
    let mut recent = BTreeSet::<Pair>::new();

    for task in workload.tasks() {
        let mut candidates = Vec::new();
        match task.ruleset {
            TC1 => {
                candidates.extend(workload.edge1.iter().map(|edge| edge.pair()));
                for reach in &recent {
                    for edge in &workload.edge2 {
                        if reach.y == edge.x {
                            candidates.push(Pair {
                                x: reach.x,
                                y: edge.y,
                            });
                        }
                    }
                }
            }
            TC2 => {
                candidates.extend(workload.edge2.iter().map(|edge| edge.pair()));
                for reach in &recent {
                    for edge in &workload.edge1 {
                        if reach.y == edge.x {
                            candidates.push(Pair {
                                x: reach.x,
                                y: edge.y,
                            });
                        }
                    }
                }
            }
            _ => unreachable!("unknown ruleset"),
        }

        recent.clear();
        let mut seen_this_task = BTreeSet::new();
        for pair in candidates {
            metrics.candidate_count += 1;
            if reachable.contains(&pair) || !seen_this_task.insert(pair) {
                metrics.duplicate_stale_candidate_count += 1;
                continue;
            }
            reachable.insert(pair);
            recent.insert(pair);
            metrics.actual.insert(pair);
        }
        metrics.last_run.insert(task.ruleset, task.output_ts);
    }

    metrics
}

fn native_candidates(
    workload: &Workload,
    timed_reachable: &BTreeMap<Pair, Epoch>,
    task: Task,
) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    match task.ruleset {
        TC1 => {
            candidates.extend(base_candidates(&workload.edge1, task, 1));
            for (&reach, &reach_ts) in timed_reachable {
                for edge in &workload.edge2 {
                    if reach.y == edge.x && reach_ts.max(edge.row_ts) >= task.last_run {
                        candidates.push(Candidate {
                            task_id: task.task_id,
                            ruleset: task.ruleset,
                            x: reach.x,
                            y: edge.y,
                            output_ts: task.output_ts,
                            source: 2,
                        });
                    }
                }
            }
        }
        TC2 => {
            candidates.extend(base_candidates(&workload.edge2, task, 3));
            for (&reach, &reach_ts) in timed_reachable {
                for edge in &workload.edge1 {
                    if reach.y == edge.x && reach_ts.max(edge.row_ts) >= task.last_run {
                        candidates.push(Candidate {
                            task_id: task.task_id,
                            ruleset: task.ruleset,
                            x: reach.x,
                            y: edge.y,
                            output_ts: task.output_ts,
                            source: 4,
                        });
                    }
                }
            }
        }
        _ => unreachable!("unknown ruleset"),
    }
    candidates
}

fn base_candidates(edges: &[TimedPair], task: Task, source: u8) -> Vec<Candidate> {
    edges
        .iter()
        .filter(|edge| edge.row_ts >= task.last_run)
        .map(|edge| Candidate {
            task_id: task.task_id,
            ruleset: task.ruleset,
            x: edge.x,
            y: edge.y,
            output_ts: task.output_ts,
            source,
        })
        .collect()
}

fn dd_metrics(
    workload: &Workload,
    workers: usize,
    overlap_window: usize,
    native_barriers: bool,
) -> Result<RunMetrics, String> {
    let tasks = workload.tasks();
    let event_log = Arc::new(Mutex::new(Vec::<CandidateEvent>::new()));
    let workload = Arc::new(workload.clone());
    let event_log_for_workers = Arc::clone(&event_log);
    let effective_window = if native_barriers {
        1
    } else {
        overlap_window.max(1)
    };

    let guards = timely::execute(timely::Config::process(workers), move |worker| {
        let index = worker.index();
        let workload = Arc::clone(&workload);
        let event_log = Arc::clone(&event_log_for_workers);
        let mut edge1_input = InputSession::<Epoch, TimedPair, isize>::new();
        let mut edge2_input = InputSession::<Epoch, TimedPair, isize>::new();
        let mut reachable_input = InputSession::<Epoch, TimedPair, isize>::new();
        let mut task_input = InputSession::<Epoch, Task, isize>::new();
        let probe = ProbeHandle::new();
        let inspect_event_log = Arc::clone(&event_log);

        worker.dataflow(|scope| {
            let edge1 = edge1_input.to_collection(scope);
            let edge2 = edge2_input.to_collection(scope);
            let reachable = reachable_input.to_collection(scope);
            let tasks = task_input.to_collection(scope);

            let task_by_ruleset = tasks.map(|task| (task.ruleset, task));

            let edge1_base = task_by_ruleset
                .clone()
                .join_map(
                    edge1.clone().map(|edge| (TC1, edge)),
                    |_ruleset, task, edge| candidate_from_edge(*task, *edge, 1),
                )
                .filter(|candidate| candidate.source != 0);

            let edge2_base = task_by_ruleset
                .clone()
                .join_map(
                    edge2.clone().map(|edge| (TC2, edge)),
                    |_ruleset, task, edge| candidate_from_edge(*task, *edge, 3),
                )
                .filter(|candidate| candidate.source != 0);

            let tc1_join_seed = reachable
                .clone()
                .map(|reach| (reach.y, (reach.x, reach.row_ts)))
                .join_map(
                    edge2.map(|edge| (edge.x, (edge.y, edge.row_ts))),
                    |_mid, reach, edge| JoinSeed {
                        ruleset: TC1,
                        x: reach.0,
                        y: edge.0,
                        newest_ts: reach.1.max(edge.1),
                        source: 2,
                    },
                );

            let tc2_join_seed = reachable
                .map(|reach| (reach.y, (reach.x, reach.row_ts)))
                .join_map(
                    edge1.map(|edge| (edge.x, (edge.y, edge.row_ts))),
                    |_mid, reach, edge| JoinSeed {
                        ruleset: TC2,
                        x: reach.0,
                        y: edge.0,
                        newest_ts: reach.1.max(edge.1),
                        source: 4,
                    },
                );

            let recursive_candidates = task_by_ruleset
                .join_map(
                    tc1_join_seed
                        .concat(tc2_join_seed)
                        .map(|seed| (seed.ruleset, seed)),
                    |_ruleset, task, seed| candidate_from_seed(*task, *seed),
                )
                .filter(|candidate| candidate.source != 0);

            edge1_base
                .concat(edge2_base)
                .concat(recursive_candidates)
                .inspect_batch(move |time, data| {
                    let mut events = inspect_event_log.lock().expect("event log poisoned");
                    for (candidate, event_time, diff) in data {
                        events.push(CandidateEvent {
                            timely_time: (*event_time).max(*time),
                            candidate: *candidate,
                            diff: *diff,
                        });
                    }
                })
                .probe_with(&probe);
        });

        if index == 0 {
            for edge in &workload.edge1 {
                edge1_input.update_at(*edge, EDGE_TS, 1);
            }
            for edge in &workload.edge2 {
                edge2_input.update_at(*edge, EDGE_TS, 1);
            }
        }

        let max_ts = tasks
            .last()
            .map(|task| task.output_ts + 2)
            .unwrap_or(EDGE_TS + 2);
        edge1_input.advance_to(max_ts);
        edge2_input.advance_to(max_ts);
        edge1_input.flush();
        edge2_input.flush();
        reachable_input.advance_to(EDGE_TS + 1);
        reachable_input.flush();
        task_input.advance_to(EDGE_TS + 1);
        task_input.flush();

        let mut metrics = RunMetrics::empty();
        if native_barriers {
            metrics.barrier_count = tasks.len().saturating_sub(1);
            metrics
                .barrier_reasons
                .push("synthetic native barrier forced overlap window to 1".to_string());
        } else if effective_window == 1 {
            metrics.barrier_count = tasks.len();
            metrics
                .barrier_reasons
                .push("stop/start logical schedule boundary".to_string());
        }

        let mut issued = 0usize;
        let mut processed_events = HashSet::new();
        let mut committed = BTreeSet::<Pair>::new();

        for (task_index, task) in tasks.iter().copied().enumerate() {
            let issue_limit = (task_index + effective_window).min(tasks.len());
            while issued < issue_limit {
                let future = tasks[issued];
                if index == 0 {
                    task_input.update_at(future, future.output_ts, 1);
                }
                issued += 1;
            }
            let issued_through_time = tasks[issued - 1].output_ts + 1;
            task_input.advance_to(issued_through_time);
            task_input.flush();

            reachable_input.advance_to(task.output_ts + 1);
            reachable_input.flush();
            while probe.less_equal(&task.output_ts) {
                worker.step();
            }

            if index == 0 {
                let mut new_rows = Vec::new();
                let mut seen_this_task = BTreeSet::new();
                let events = event_log.lock().expect("event log poisoned");
                for (event_index, event) in events.iter().enumerate() {
                    if processed_events.contains(&event_index)
                        || event.diff <= 0
                        || event.candidate.task_id != task.task_id
                    {
                        continue;
                    }
                    processed_events.insert(event_index);
                    metrics.candidate_count += event.diff as usize;
                    if event.timely_time > task.output_ts {
                        metrics.early_visibility_violations += 1;
                    }

                    let pair = event.candidate.pair();
                    if committed.contains(&pair) || !seen_this_task.insert(pair) {
                        metrics.duplicate_stale_candidate_count += 1;
                    } else {
                        new_rows.push(pair);
                    }
                }
                drop(events);

                for pair in new_rows {
                    committed.insert(pair);
                    metrics.actual.insert(pair);
                    reachable_input.update_at(
                        TimedPair {
                            x: pair.x,
                            y: pair.y,
                            row_ts: task.output_ts,
                        },
                        task.output_ts + 1,
                        1,
                    );
                }
                metrics.last_run.insert(task.ruleset, task.output_ts);
            }

            let frontier = probe.with_frontier(|frontier| frontier.to_vec());
            metrics.frontier_lag_samples.push(FrontierSample {
                committed_task: task.task_id,
                issued_through_task: tasks[issued - 1].task_id,
                lag_tasks: tasks[issued - 1].task_id.saturating_sub(task.task_id),
                visible_frontier: frontier,
            });
        }

        reachable_input.advance_to(max_ts);
        task_input.advance_to(max_ts);
        reachable_input.flush();
        task_input.flush();
        while probe.less_than(&max_ts) {
            worker.step();
        }

        if index == 0 {
            Some(metrics)
        } else {
            None
        }
    })
    .map_err(|err| format!("timely execution failed: {err}"))?;

    let worker_results = guards.join();
    for worker_result in worker_results {
        let maybe_metrics = worker_result.map_err(|err| format!("worker panic: {err}"))?;
        if let Some(metrics) = maybe_metrics {
            return Ok(metrics);
        }
    }

    Err("worker 0 did not return metrics".to_string())
}

fn candidate_from_edge(task: Task, edge: TimedPair, source: u8) -> Candidate {
    if edge.row_ts >= task.last_run {
        Candidate {
            task_id: task.task_id,
            ruleset: task.ruleset,
            x: edge.x,
            y: edge.y,
            output_ts: task.output_ts,
            source,
        }
    } else {
        empty_candidate(task)
    }
}

fn candidate_from_seed(task: Task, seed: JoinSeed) -> Candidate {
    if seed.newest_ts >= task.last_run {
        Candidate {
            task_id: task.task_id,
            ruleset: task.ruleset,
            x: seed.x,
            y: seed.y,
            output_ts: task.output_ts,
            source: seed.source,
        }
    } else {
        empty_candidate(task)
    }
}

fn empty_candidate(task: Task) -> Candidate {
    Candidate {
        task_id: task.task_id,
        ruleset: task.ruleset,
        x: 0,
        y: 0,
        output_ts: task.output_ts,
        source: 0,
    }
}

fn timed_pair(x: Node, y: Node) -> TimedPair {
    TimedPair {
        x,
        y,
        row_ts: EDGE_TS,
    }
}

fn write_json(path: &str, results: &[ExperimentResult]) -> Result<(), String> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
        }
    }
    let json = serde_json::to_string_pretty(results).map_err(|err| err.to_string())?;
    fs::write(path, format!("{json}\n")).map_err(|err| format!("failed to write {path}: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_mini_reaches_across_rulesets() {
        let workload = Workload::new(WorkloadKind::Mini, 1);
        let metrics = oracle_metrics(&workload);
        let expected = BTreeSet::from([
            Pair { x: 1, y: 2 },
            Pair { x: 2, y: 3 },
            Pair { x: 1, y: 3 },
        ]);
        assert_eq!(metrics.actual, expected);
        assert_eq!(metrics.last_run.get(&TC1), Some(&4));
        assert_eq!(metrics.last_run.get(&TC2), Some(&3));
    }

    #[test]
    fn broken_global_misses_cross_ruleset_freshness() {
        let workload = Workload::new(WorkloadKind::Mini, 1);
        let metrics = broken_global_metrics(&workload);
        assert!(metrics.actual.contains(&Pair { x: 1, y: 2 }));
        assert!(metrics.actual.contains(&Pair { x: 2, y: 3 }));
        assert!(!metrics.actual.contains(&Pair { x: 1, y: 3 }));
    }

    #[test]
    fn dd_modes_match_oracle_on_mini() {
        let workload = Workload::new(WorkloadKind::Mini, 1);
        let expected = oracle_metrics(&workload).actual;
        let barrier = dd_metrics(&workload, 1, 1, false).expect("barrier DD run");
        let overlap = dd_metrics(&workload, 1, 2, false).expect("overlap DD run");
        assert_eq!(barrier.actual, expected);
        assert_eq!(overlap.actual, expected);
        assert_eq!(overlap.early_visibility_violations, 0);
    }

    #[test]
    fn dd_overlap_matches_oracle_on_small_chain() {
        let workload = Workload::new(WorkloadKind::Chain, 4);
        let expected = oracle_metrics(&workload).actual;
        let overlap = dd_metrics(&workload, 1, 2, false).expect("overlap DD run");
        assert_eq!(overlap.actual, expected);
    }

    #[test]
    fn result_json_contains_required_fields() {
        let workload = Workload::new(WorkloadKind::Mini, 1);
        let result = run_experiment(Mode::Oracle, workload, 1, 1, false).expect("oracle run");
        let value = serde_json::to_value(result).expect("json value");
        for field in [
            "workload",
            "scale",
            "mode",
            "workers",
            "overlap_window",
            "semantic_equivalence",
            "per_rule_freshness",
            "expected_reachable_count",
            "actual_reachable_count",
            "missing_reachable_count",
            "extra_reachable_count",
            "missing_reachable_sample",
            "extra_reachable_sample",
            "early_visibility_violations",
            "runtime_micros",
            "candidate_count",
            "committed_row_count",
            "duplicate_stale_candidate_count",
            "frontier_lag_samples",
            "barrier_count",
            "barrier_reasons",
        ] {
            assert!(value.get(field).is_some(), "missing JSON field {field}");
        }
    }
}
