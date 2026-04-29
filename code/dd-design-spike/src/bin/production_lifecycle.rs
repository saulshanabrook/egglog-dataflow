use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use dd_design_spike::{
    command_string, out_path_arg, rss_kb, write_report, ExperimentReport, Metrics,
};
use differential_dataflow::input::Input;
use differential_dataflow::trace::TraceReader;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use timely::dataflow::operators::probe::Handle as ProbeHandle;
use timely::dataflow::operators::Probe;
use timely::progress::frontier::AntichainRef;

type Epoch = u64;
type Node = u32;

const FRAGMENT_COUNTS: [usize; 3] = [10, 100, 1_000];
const WORKER_COUNTS: [usize; 2] = [1, 4];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
struct Edge {
    src: Node,
    dst: Node,
    row_ts: Epoch,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
struct PathRow {
    src: Node,
    dst: Node,
    row_ts: Epoch,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
struct Candidate {
    fragment: u64,
    src: Node,
    dst: Node,
    row_ts: Epoch,
    freshness_ts: Epoch,
}

#[derive(Clone, Debug, Serialize)]
struct ConfigMetrics {
    workers: usize,
    fragments: usize,
    elapsed_ms: u128,
    build_ms: u128,
    step_count: u64,
    base_edges: usize,
    seed_paths: usize,
    blocked_nodes: usize,
    committed_paths: usize,
    expected_committed_paths: usize,
    allowed_candidates: usize,
    blocked_candidates: usize,
    probe_completions: usize,
    frontier_samples: usize,
    retained_fragment_dataflows: usize,
    installed_after_base_drop: Vec<usize>,
    early_visibility_violations: usize,
    logical_compaction_frontier: Epoch,
    physical_compaction_frontier: Epoch,
    rss_before_kb: Option<u64>,
    rss_after_kb: Option<u64>,
}

#[derive(Clone, Debug)]
struct Oracle {
    edges_by_src: BTreeMap<Node, Vec<Edge>>,
    blocked: BTreeSet<Node>,
    committed: BTreeMap<(Node, Node), Epoch>,
}

impl Oracle {
    fn new(edges: &[Edge], seed_paths: &[PathRow], blocked: &[Node]) -> Self {
        let mut edges_by_src: BTreeMap<Node, Vec<Edge>> = BTreeMap::new();
        for edge in edges {
            edges_by_src.entry(edge.src).or_default().push(*edge);
        }

        let mut committed = BTreeMap::new();
        for path in seed_paths {
            committed.insert((path.src, path.dst), path.row_ts);
        }

        Self {
            edges_by_src,
            blocked: blocked.iter().copied().collect(),
            committed,
        }
    }

    fn expected_candidates(
        &self,
        fragment: usize,
        last_run: Epoch,
        output_ts: Epoch,
    ) -> Vec<PathRow> {
        let mut out = BTreeSet::new();
        for (&(src, mid), &path_ts) in &self.committed {
            if path_ts < last_run {
                continue;
            }
            let Some(edges) = self.edges_by_src.get(&mid) else {
                continue;
            };
            for edge in edges {
                if self.blocked.contains(&edge.dst) || self.committed.contains_key(&(src, edge.dst))
                {
                    continue;
                }
                let freshness_ts = path_ts.max(edge.row_ts);
                if freshness_ts >= last_run {
                    out.insert(PathRow {
                        src,
                        dst: edge.dst,
                        row_ts: output_ts,
                    });
                }
            }
        }

        let mut out = out.into_iter().collect::<Vec<_>>();
        if fragment > 0 {
            // Keep the oracle deterministic if multiple rows are possible in future variants.
            out.sort();
        }
        out
    }

    fn commit(&mut self, rows: &[PathRow]) {
        for row in rows {
            assert!(
                self.committed
                    .insert((row.src, row.dst), row.row_ts)
                    .is_none(),
                "oracle only commits new path rows"
            );
        }
    }
}

fn fixture(fragments: usize) -> (Vec<Edge>, Vec<PathRow>, Vec<Node>) {
    let mut edges = (1..=(fragments as Node + 2))
        .map(|src| Edge {
            src,
            dst: src + 1,
            row_ts: 1,
        })
        .collect::<Vec<_>>();

    // This branch proves the blocked semijoin/filter path without being committed.
    edges.push(Edge {
        src: 10_000,
        dst: 10_001,
        row_ts: 1,
    });

    let seed_paths = vec![
        PathRow {
            src: 0,
            dst: 1,
            row_ts: 1,
        },
        PathRow {
            src: 9_999,
            dst: 10_000,
            row_ts: 1,
        },
    ];
    let blocked = vec![10_001];
    (edges, seed_paths, blocked)
}

fn run_config(workers: usize, fragments: usize) -> ConfigMetrics {
    let started = Instant::now();
    let rss_before = rss_kb();
    let (edges, seed_paths, blocked_nodes) = fixture(fragments);
    let allowed_log = Arc::new(Mutex::new(Vec::<(Candidate, isize)>::new()));
    let blocked_log = Arc::new(Mutex::new(Vec::<(Candidate, isize)>::new()));

    let allowed_log_for_workers = Arc::clone(&allowed_log);
    let blocked_log_for_workers = Arc::clone(&blocked_log);
    let edges_for_workers = edges.clone();
    let seed_paths_for_workers = seed_paths.clone();
    let blocked_for_workers = blocked_nodes.clone();

    let joined = timely::execute(timely::Config::process(workers), move |worker| {
        let mut build_ms = 0;
        let mut step_count = 0;
        let mut probe_completions = 0;
        let mut frontier_samples = 0;
        let mut retained_fragment_dataflows = BTreeSet::new();
        let mut early_visibility_violations = 0;
        let mut allowed_candidate_count = 0;
        let mut logical_compaction_frontier = 1;
        let mut physical_compaction_frontier = 1;

        let mut base_probe = ProbeHandle::<Epoch>::new();
        let base_dataflow = worker.next_dataflow_index();
        let (
            mut edge_input,
            mut path_input,
            mut blocked_input,
            mut edge_by_src,
            mut path_by_dst,
            mut blocked_by_node,
        ) = worker.dataflow::<Epoch, _, _>(|scope| {
            let (edge_input, edges) = scope.new_collection::<Edge, isize>();
            let (path_input, paths) = scope.new_collection::<PathRow, isize>();
            let (blocked_input, blocked) = scope.new_collection::<Node, isize>();

            let edge_arranged = edges
                .map(|edge| (edge.src, (edge.dst, edge.row_ts)))
                .arrange_by_key();
            let path_arranged = paths
                .map(|path| (path.dst, (path.src, path.row_ts)))
                .arrange_by_key();
            let blocked_arranged = blocked.arrange_by_self();

            edge_arranged.stream.probe_with(&mut base_probe);
            path_arranged.stream.probe_with(&mut base_probe);
            blocked_arranged.stream.probe_with(&mut base_probe);

            (
                edge_input,
                path_input,
                blocked_input,
                edge_arranged.trace.clone(),
                path_arranged.trace.clone(),
                blocked_arranged.trace.clone(),
            )
        });

        if worker.index() == 0 {
            for edge in &edges_for_workers {
                edge_input.update_at(*edge, 1, 1);
            }
            for path in &seed_paths_for_workers {
                path_input.update_at(*path, 1, 1);
            }
            for blocked in &blocked_for_workers {
                blocked_input.update_at(*blocked, 1, 1);
            }
        }

        edge_input.advance_to(2);
        path_input.advance_to(2);
        blocked_input.advance_to(2);
        edge_input.flush();
        path_input.flush();
        blocked_input.flush();
        while base_probe.less_than(edge_input.time()) {
            worker.step();
            step_count += 1;
        }

        let mut last_run = 1;
        for fragment in 0..fragments {
            let output_ts = last_run + 1;
            let fragment_id = fragment as u64;
            let dataflow_id = worker.next_dataflow_index();
            let mut fragment_probe = ProbeHandle::<Epoch>::new();
            let allowed_log_for_dataflow = Arc::clone(&allowed_log_for_workers);
            let allowed_log_for_host = Arc::clone(&allowed_log_for_workers);
            let blocked_log = Arc::clone(&blocked_log_for_workers);
            let mut edge_trace = edge_by_src.clone();
            let mut path_trace = path_by_dst.clone();
            let mut blocked_trace = blocked_by_node.clone();

            let build_started = Instant::now();
            worker.dataflow::<Epoch, _, _>(|scope| {
                let paths = path_trace.import(scope);
                let edges = edge_trace.import(scope);
                let blocked = blocked_trace.import(scope);

                let candidates = paths
                    .join_core(edges, move |_mid, path, edge| {
                        let (src, path_ts) = *path;
                        let (dst, edge_ts) = *edge;
                        let freshness_ts = path_ts.max(edge_ts);
                        (freshness_ts >= last_run).then_some(Candidate {
                            fragment: fragment_id,
                            src,
                            dst,
                            row_ts: output_ts,
                            freshness_ts,
                        })
                    })
                    .map(|candidate| (candidate.dst, candidate));

                let blocked_pairs = candidates
                    .clone()
                    .arrange_by_key()
                    .join_core(blocked, |dst, candidate, _unit| Some((*dst, *candidate)));

                blocked_pairs
                    .clone()
                    .map(|(_dst, candidate)| candidate)
                    .inspect_batch(move |_time, data| {
                        let mut blocked = blocked_log.lock().expect("blocked log poisoned");
                        for (candidate, _event_time, diff) in data {
                            blocked.push((*candidate, *diff));
                        }
                    })
                    .probe_with(&mut fragment_probe);

                candidates
                    .concat(blocked_pairs.negate())
                    .map(|(_dst, candidate)| candidate)
                    .inspect_batch(move |_time, data| {
                        let mut allowed = allowed_log_for_dataflow
                            .lock()
                            .expect("allowed log poisoned");
                        for (candidate, _event_time, diff) in data {
                            allowed.push((*candidate, *diff));
                        }
                    })
                    .probe_with(&mut fragment_probe);
            });
            build_ms += build_started.elapsed().as_millis();

            while fragment_probe.less_than(path_input.time()) {
                worker.step();
                step_count += 1;
            }
            probe_completions += 1;
            frontier_samples += 1;

            if worker.index() == 0 {
                let expected = {
                    let mut oracle = Oracle::new(
                        &edges_for_workers,
                        &seed_paths_for_workers,
                        &blocked_for_workers,
                    );
                    // Reconstruct the oracle from committed DD input by mirroring the deterministic chain.
                    for dst in 2..=(fragment as Node + 1) {
                        oracle.commit(&[PathRow {
                            src: 0,
                            dst,
                            row_ts: dst as Epoch,
                        }]);
                    }
                    oracle.expected_candidates(fragment, last_run, output_ts)
                };

                let mut allowed = allowed_log_for_host.lock().expect("allowed log poisoned");
                let mut allowed_counts = BTreeMap::new();
                for (candidate, diff) in allowed
                    .iter()
                    .filter(|(candidate, _diff)| candidate.fragment == fragment_id)
                {
                    *allowed_counts.entry(*candidate).or_insert(0) += *diff;
                }
                let mut new_allowed = allowed_counts
                    .into_iter()
                    .filter_map(|(candidate, diff)| (diff > 0).then_some(candidate))
                    .collect::<Vec<_>>();
                new_allowed.sort();
                new_allowed.dedup();

                let actual_rows = new_allowed
                    .iter()
                    .map(|candidate| PathRow {
                        src: candidate.src,
                        dst: candidate.dst,
                        row_ts: candidate.row_ts,
                    })
                    .collect::<Vec<_>>();
                assert_eq!(actual_rows, expected);
                allowed_candidate_count += actual_rows.len();

                early_visibility_violations += new_allowed
                    .iter()
                    .filter(|candidate| candidate.freshness_ts > candidate.row_ts)
                    .count();

                for row in &actual_rows {
                    path_input.update_at(*row, output_ts, 1);
                }
                allowed.retain(|(candidate, _diff)| candidate.fragment != fragment_id);
            }

            path_input.advance_to(output_ts + 1);
            edge_input.advance_to(output_ts + 1);
            blocked_input.advance_to(output_ts + 1);
            path_input.flush();
            edge_input.flush();
            blocked_input.flush();
            while base_probe.less_than(path_input.time()) {
                worker.step();
                step_count += 1;
            }

            logical_compaction_frontier = output_ts;
            physical_compaction_frontier = output_ts;
            edge_by_src.set_logical_compaction(AntichainRef::new(&[output_ts]));
            edge_by_src.set_physical_compaction(AntichainRef::new(&[output_ts]));
            path_by_dst.set_logical_compaction(AntichainRef::new(&[output_ts]));
            path_by_dst.set_physical_compaction(AntichainRef::new(&[output_ts]));
            blocked_by_node.set_logical_compaction(AntichainRef::new(&[output_ts]));
            blocked_by_node.set_physical_compaction(AntichainRef::new(&[output_ts]));

            worker.drop_dataflow(dataflow_id);
            let installed = worker.installed_dataflows();
            if installed.contains(&dataflow_id) {
                retained_fragment_dataflows.insert(dataflow_id);
            }

            last_run = output_ts;
        }

        drop(edge_input);
        drop(path_input);
        drop(blocked_input);
        drop(edge_by_src);
        drop(path_by_dst);
        drop(blocked_by_node);
        worker.drop_dataflow(base_dataflow);
        let installed_after_base_drop = worker.installed_dataflows();

        (
            build_ms,
            step_count,
            probe_completions,
            frontier_samples,
            retained_fragment_dataflows,
            installed_after_base_drop,
            early_visibility_violations,
            allowed_candidate_count,
            logical_compaction_frontier,
            physical_compaction_frontier,
        )
    })
    .expect("timely execution failed")
    .join();

    let mut build_ms = 0;
    let mut step_count = 0;
    let mut probe_completions = 0;
    let mut frontier_samples = 0;
    let mut retained_fragment_dataflows = BTreeSet::new();
    let mut installed_after_base_drop = BTreeSet::new();
    let mut early_visibility_violations = 0;
    let mut allowed_candidates = 0;
    let mut logical_compaction_frontier = 1;
    let mut physical_compaction_frontier = 1;

    for worker in joined {
        let (
            worker_build_ms,
            worker_step_count,
            worker_probe_completions,
            worker_frontier_samples,
            worker_retained,
            worker_installed_after_base_drop,
            worker_early_visibility_violations,
            worker_allowed_candidates,
            worker_logical_compaction_frontier,
            worker_physical_compaction_frontier,
        ) = worker.expect("worker failed");
        build_ms += worker_build_ms;
        step_count += worker_step_count;
        probe_completions += worker_probe_completions;
        frontier_samples += worker_frontier_samples;
        retained_fragment_dataflows.extend(worker_retained);
        installed_after_base_drop.extend(worker_installed_after_base_drop);
        early_visibility_violations += worker_early_visibility_violations;
        allowed_candidates += worker_allowed_candidates;
        logical_compaction_frontier =
            logical_compaction_frontier.max(worker_logical_compaction_frontier);
        physical_compaction_frontier =
            physical_compaction_frontier.max(worker_physical_compaction_frontier);
    }

    let blocked_candidates = {
        let blocked = blocked_log.lock().expect("blocked log poisoned");
        let mut blocked_counts = BTreeMap::new();
        for (candidate, diff) in blocked.iter() {
            *blocked_counts.entry(*candidate).or_insert(0) += *diff;
        }
        blocked_counts
            .into_iter()
            .filter(|(_candidate, diff)| *diff > 0)
            .count()
    };

    assert!(retained_fragment_dataflows.is_empty());
    assert!(installed_after_base_drop.is_empty());
    assert_eq!(probe_completions, workers * fragments);
    assert_eq!(early_visibility_violations, 0);
    assert_eq!(blocked_candidates, 1);

    ConfigMetrics {
        workers,
        fragments,
        elapsed_ms: started.elapsed().as_millis(),
        build_ms,
        step_count,
        base_edges: edges.len(),
        seed_paths: seed_paths.len(),
        blocked_nodes: blocked_nodes.len(),
        committed_paths: seed_paths.len() + fragments,
        expected_committed_paths: seed_paths.len() + fragments,
        allowed_candidates,
        blocked_candidates,
        probe_completions,
        frontier_samples,
        retained_fragment_dataflows: retained_fragment_dataflows.len(),
        installed_after_base_drop: installed_after_base_drop.into_iter().collect(),
        early_visibility_violations,
        logical_compaction_frontier,
        physical_compaction_frontier,
        rss_before_kb: rss_before,
        rss_after_kb: rss_kb(),
    }
}

fn config_to_json(config: &ConfigMetrics) -> Value {
    json!({
        "workers": config.workers,
        "fragments": config.fragments,
        "elapsed_ms": config.elapsed_ms,
        "build_ms": config.build_ms,
        "step_count": config.step_count,
        "base_edges": config.base_edges,
        "seed_paths": config.seed_paths,
        "blocked_nodes": config.blocked_nodes,
        "committed_paths": config.committed_paths,
        "expected_committed_paths": config.expected_committed_paths,
        "allowed_candidates": config.allowed_candidates,
        "blocked_candidates": config.blocked_candidates,
        "probe_completions": config.probe_completions,
        "frontier_samples": config.frontier_samples,
        "retained_fragment_dataflows": config.retained_fragment_dataflows,
        "installed_after_base_drop": config.installed_after_base_drop,
        "early_visibility_violations": config.early_visibility_violations,
        "logical_compaction_frontier": config.logical_compaction_frontier,
        "physical_compaction_frontier": config.physical_compaction_frontier,
        "rss_before_kb": config.rss_before_kb,
        "rss_after_kb": config.rss_after_kb
    })
}

fn main() {
    let out_path = out_path_arg();
    let mut configs = Vec::new();

    for workers in WORKER_COUNTS {
        for fragments in FRAGMENT_COUNTS {
            let config = run_config(workers, fragments);
            println!(
                "workers={} fragments={} committed={}/{} blocked={} retained={} early={} elapsed_ms={} rss={:?}->{:?}",
                config.workers,
                config.fragments,
                config.committed_paths,
                config.expected_committed_paths,
                config.blocked_candidates,
                config.retained_fragment_dataflows,
                config.early_visibility_violations,
                config.elapsed_ms,
                config.rss_before_kb,
                config.rss_after_kb
            );
            configs.push(config);
        }
    }

    let passed = configs.iter().all(|config| {
        config.committed_paths == config.expected_committed_paths
            && config.probe_completions == config.workers * config.fragments
            && config.retained_fragment_dataflows == 0
            && config.installed_after_base_drop.is_empty()
            && config.early_visibility_violations == 0
            && config.blocked_candidates == 1
            && config.rss_before_kb.is_some()
            && config.rss_after_kb.is_some()
    });
    assert!(passed);

    let mut metrics = Metrics::new();
    metrics.insert(
        "total_fragments".to_string(),
        json!(configs.iter().map(|config| config.fragments).sum::<usize>()),
    );
    metrics.insert(
        "total_committed_paths".to_string(),
        json!(configs
            .iter()
            .map(|config| config.committed_paths)
            .sum::<usize>()),
    );
    metrics.insert(
        "total_blocked_candidates".to_string(),
        json!(configs
            .iter()
            .map(|config| config.blocked_candidates)
            .sum::<usize>()),
    );
    metrics.insert(
        "max_elapsed_ms".to_string(),
        json!(configs
            .iter()
            .map(|config| config.elapsed_ms)
            .max()
            .unwrap_or_default()),
    );
    metrics.insert(
        "max_rss_after_kb".to_string(),
        json!(configs
            .iter()
            .filter_map(|config| config.rss_after_kb)
            .max()),
    );

    let report = ExperimentReport {
        experiment: "production-shaped-dd-lifecycle",
        status: "pass",
        command: command_string(),
        configs: configs.iter().map(config_to_json).collect(),
        metrics,
        observations: vec![
            "Each fragment imported persistent edge/path/blocked arrangements built in a long-lived base dataflow.".to_string(),
            "The binary join path(x,y), edge(y,z) -> path(x,z) used recursive host feedback through path_input at the next logical epoch.".to_string(),
            "The blocked-node antijoin used an imported blocked arrangement, explicit signed negation, and host-side net-delta consolidation before committing visible rows.".to_string(),
            "All generated fragment dataflows and the base dataflow were absent from installed_dataflows after drop.".to_string(),
        ],
        decision: "Production-shaped lifecycle remains compatible with compiled DD fragments over shared arrangements for the bounded chain/blocked workload, provided output sinks consolidate signed diffs before firing actions.".to_string(),
        limitations: vec![
            "The rule shape is still synthetic and narrow: one binary join, one blocked-node filter, and one recursive feedback path.".to_string(),
            "The run validates control-plane lifecycle and visibility, not a full CoreRule compiler or arbitrary egglog schedules.".to_string(),
        ],
        next_action: "Use this scenario as an acceptance test for the DD runtime shell, then replace the synthetic rule builder with CoreRule lowering.".to_string(),
    };

    write_report(out_path, &report).expect("failed to write report");
}
