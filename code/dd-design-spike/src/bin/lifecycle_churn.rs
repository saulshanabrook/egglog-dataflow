use std::collections::BTreeSet;
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

const FRAGMENT_COUNTS: [usize; 3] = [10, 100, 1_000];
const WORKER_COUNTS: [usize; 2] = [1, 4];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
struct Row {
    fragment: u64,
    value: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
struct Output {
    fragment: u64,
    value: u64,
}

#[derive(Debug, Serialize)]
struct ConfigMetrics {
    workers: usize,
    fragments: usize,
    elapsed_ms: u128,
    build_ms: u128,
    step_count: u64,
    correctness_count: usize,
    expected_correctness_count: usize,
    probe_completions: usize,
    installed_after_drop: Vec<usize>,
    retained_installed_dataflows: usize,
    frontier_samples: usize,
    frontier_final: Vec<Epoch>,
    compaction_frontier: Epoch,
    rss_kb: Option<u64>,
}

#[derive(Debug)]
struct WorkerMetrics {
    build_ms: u128,
    step_count: u64,
    correctness_count: usize,
    probe_completions: usize,
    installed_after_drop: BTreeSet<usize>,
    frontier_samples: usize,
    frontier_final: Vec<Epoch>,
}

fn expected_output(fragment: u64) -> Output {
    Output {
        fragment,
        value: fragment * 10 + 7,
    }
}

fn run_config(workers: usize, fragments: usize) -> ConfigMetrics {
    let started = Instant::now();
    let joined = timely::execute(timely::Config::process(workers), move |worker| {
        let mut build_ms = 0;
        let mut step_count = 0;
        let mut correctness_count = 0;
        let mut probe_completions = 0;
        let mut installed_after_drop = BTreeSet::new();
        let mut frontier_samples = 0;
        let mut frontier_final = Vec::new();

        for fragment in 0..fragments {
            let dataflow_id = worker.next_dataflow_index();
            let observed = Arc::new(Mutex::new(Vec::<Output>::new()));
            let observed_for_dataflow = Arc::clone(&observed);
            let mut probe = ProbeHandle::<Epoch>::new();

            let build_started = Instant::now();
            let (mut input, mut trace) = worker.dataflow::<Epoch, _, _>(|scope| {
                let (input, rows) = scope.new_collection::<Row, isize>();
                let outputs = rows.map(|row| Output {
                    fragment: row.fragment,
                    value: row.value + 7,
                });

                outputs
                    .clone()
                    .inspect_batch(move |_time, data| {
                        let mut observed = observed_for_dataflow
                            .lock()
                            .expect("observed output log poisoned");
                        for (output, _time, diff) in data {
                            if *diff > 0 {
                                observed.push(*output);
                            }
                        }
                    })
                    .probe_with(&mut probe);

                let arranged = outputs.arrange_by_self();
                let trace = arranged.trace.clone();
                arranged.stream.probe_with(&mut probe);

                (input, trace)
            });
            build_ms += build_started.elapsed().as_millis();

            if worker.index() == 0 {
                let fragment = fragment as u64;
                input.update_at(
                    Row {
                        fragment,
                        value: fragment * 10,
                    },
                    0,
                    1,
                );
            }
            input.advance_to(1);
            input.flush();

            while probe.less_than(input.time()) {
                worker.step();
                step_count += 1;
            }
            probe_completions += 1;

            let frontier = probe.with_frontier(|frontier| frontier.to_vec());
            frontier_samples += 1;
            frontier_final = frontier;

            trace.set_logical_compaction(AntichainRef::new(&[1]));
            trace.set_physical_compaction(AntichainRef::new(&[1]));

            let expected = expected_output(fragment as u64);
            let mut observed = observed
                .lock()
                .expect("observed output log poisoned")
                .clone();
            observed.sort();
            if worker.index() == 0 {
                assert_eq!(observed, vec![expected]);
                correctness_count += 1;
            } else {
                assert!(observed.is_empty());
            }

            drop(input);
            drop(trace);
            worker.drop_dataflow(dataflow_id);
            let installed = worker.installed_dataflows();
            assert!(
                !installed.contains(&dataflow_id),
                "dataflow {dataflow_id} retained after drop"
            );
            installed_after_drop.extend(installed);
        }

        WorkerMetrics {
            build_ms,
            step_count,
            correctness_count,
            probe_completions,
            installed_after_drop,
            frontier_samples,
            frontier_final,
        }
    })
    .expect("timely execution failed")
    .join();

    let mut build_ms = 0;
    let mut step_count = 0;
    let mut correctness_count = 0;
    let mut probe_completions = 0;
    let mut installed_after_drop = BTreeSet::new();
    let mut frontier_samples = 0;
    let mut frontier_final = Vec::new();

    for worker in joined {
        let worker = worker.expect("worker failed");
        assert!(worker.installed_after_drop.is_empty());
        build_ms += worker.build_ms;
        step_count += worker.step_count;
        correctness_count += worker.correctness_count;
        probe_completions += worker.probe_completions;
        installed_after_drop.extend(worker.installed_after_drop);
        frontier_samples += worker.frontier_samples;
        frontier_final = worker.frontier_final;
    }

    assert_eq!(correctness_count, fragments);
    assert_eq!(probe_completions, workers * fragments);
    assert!(installed_after_drop.is_empty());

    ConfigMetrics {
        workers,
        fragments,
        elapsed_ms: started.elapsed().as_millis(),
        build_ms,
        step_count,
        correctness_count,
        expected_correctness_count: fragments,
        probe_completions,
        installed_after_drop: installed_after_drop.into_iter().collect(),
        retained_installed_dataflows: 0,
        frontier_samples,
        frontier_final,
        compaction_frontier: 1,
        rss_kb: rss_kb(),
    }
}

fn config_to_json(config: &ConfigMetrics) -> Value {
    json!({
        "workers": config.workers,
        "fragments": config.fragments,
        "elapsed_ms": config.elapsed_ms,
        "build_ms": config.build_ms,
        "step_count": config.step_count,
        "correctness_count": config.correctness_count,
        "expected_correctness_count": config.expected_correctness_count,
        "probe_completions": config.probe_completions,
        "installed_after_drop": config.installed_after_drop,
        "retained_installed_dataflows": config.retained_installed_dataflows,
        "frontier_samples": config.frontier_samples,
        "frontier_final": config.frontier_final,
        "compaction": {
            "logical_frontier": config.compaction_frontier,
            "physical_frontier": config.compaction_frontier
        },
        "rss_kb": config.rss_kb
    })
}

fn main() {
    let out_path = out_path_arg();
    let mut configs = Vec::new();

    for workers in WORKER_COUNTS {
        for fragments in FRAGMENT_COUNTS {
            let config = run_config(workers, fragments);
            println!(
                "workers={} fragments={} correct={}/{} probes={} retained={} elapsed_ms={} rss_kb={:?}",
                config.workers,
                config.fragments,
                config.correctness_count,
                config.expected_correctness_count,
                config.probe_completions,
                config.retained_installed_dataflows,
                config.elapsed_ms,
                config.rss_kb
            );
            configs.push(config);
        }
    }

    let passed = configs.iter().all(|config| {
        config.correctness_count == config.expected_correctness_count
            && config.probe_completions == config.workers * config.fragments
            && config.retained_installed_dataflows == 0
            && config.rss_kb.is_some()
            && config.frontier_samples == config.workers * config.fragments
    });
    assert!(passed);

    let mut metrics = Metrics::new();
    metrics.insert(
        "total_fragments".to_string(),
        json!(configs.iter().map(|config| config.fragments).sum::<usize>()),
    );
    metrics.insert(
        "total_correctness_count".to_string(),
        json!(configs
            .iter()
            .map(|config| config.correctness_count)
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
        "max_rss_kb".to_string(),
        json!(configs.iter().filter_map(|config| config.rss_kb).max()),
    );

    let report = ExperimentReport {
        experiment: "dd-lifecycle-churn",
        status: "pass",
        command: command_string(),
        configs: configs.iter().map(config_to_json).collect(),
        metrics,
        observations: vec![
            "Each generated fragment was installed as a separate Timely/DD dataflow and explicitly dropped after probe completion.".to_string(),
            "All checked dataflow identifiers were absent from installed_dataflows after drop.".to_string(),
            "Trace compaction frontiers and final probe frontiers were recorded for every worker/fragment configuration.".to_string(),
        ],
        decision: "Compiled generated fragments remain viable for the bounded lifecycle churn model.".to_string(),
        limitations: vec![
            "The model uses deterministic u64 rows and one output per fragment, not production egglog rules.".to_string(),
            "The run validates lifecycle churn and retained dataflow state, not end-to-end compilation latency.".to_string(),
        ],
        next_action: "Keep compiled fragments in the design path unless a larger production-shaped lifecycle run retains dropped dataflows or fails probe completion.".to_string(),
    };

    write_report(out_path, &report).expect("failed to write report");
}
