use std::collections::BTreeMap;

use dd_design_spike::{
    command_string, out_path_arg, rss_kb, write_report, ExperimentReport, Metrics,
};
use serde::Serialize;
use serde_json::json;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct CompleteMatch {
    id: u64,
    rule: &'static str,
    bindings: BTreeMap<&'static str, u64>,
    output: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct WorklistRow {
    round: u64,
    barrier: u64,
    match_id: u64,
    output: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ActionRow {
    round: u64,
    barrier: u64,
    match_id: u64,
    inserted: u64,
}

#[derive(Clone, Copy, Debug)]
struct RoundPlan {
    round: u64,
    admission_limit: usize,
}

#[derive(Debug)]
struct ScenarioResult {
    complete_matches: Vec<CompleteMatch>,
    selected_matches: Vec<CompleteMatch>,
    residual_matches: Vec<CompleteMatch>,
    residual_available_in_round_2: Vec<u64>,
    worklist_rows: Vec<WorklistRow>,
    action_rows: Vec<ActionRow>,
    pre_barrier_action_attempts: u64,
    barrier_count: u64,
    rounds: u64,
}

fn bindings(pairs: [(&'static str, u64); 2]) -> BTreeMap<&'static str, u64> {
    pairs.into_iter().collect()
}

fn dd_complete_matches() -> Vec<CompleteMatch> {
    vec![
        CompleteMatch {
            id: 1,
            rule: "edge-to-path",
            bindings: bindings([("src", 1), ("dst", 2)]),
            output: 12,
        },
        CompleteMatch {
            id: 2,
            rule: "edge-to-path",
            bindings: bindings([("src", 2), ("dst", 3)]),
            output: 23,
        },
        CompleteMatch {
            id: 3,
            rule: "edge-to-path",
            bindings: bindings([("src", 3), ("dst", 4)]),
            output: 34,
        },
        CompleteMatch {
            id: 4,
            rule: "join-path-edge",
            bindings: bindings([("path", 12), ("edge", 23)]),
            output: 123,
        },
        CompleteMatch {
            id: 5,
            rule: "join-path-edge",
            bindings: bindings([("path", 23), ("edge", 34)]),
            output: 234,
        },
    ]
}

fn scheduler_admit(
    mut candidates: Vec<CompleteMatch>,
    limit: usize,
) -> (Vec<CompleteMatch>, Vec<CompleteMatch>) {
    candidates.sort_by_key(|candidate| candidate.id);
    let residual = candidates.split_off(limit.min(candidates.len()));
    (candidates, residual)
}

fn fire_actions_after_barrier(
    barrier_open: bool,
    rows: &[WorklistRow],
    action_rows: &mut Vec<ActionRow>,
) -> usize {
    if !barrier_open {
        return 0;
    }

    let before = action_rows.len();
    action_rows.extend(rows.iter().map(|row| ActionRow {
        round: row.round,
        barrier: row.barrier,
        match_id: row.match_id,
        inserted: row.output,
    }));
    action_rows.len() - before
}

fn run_scenario() -> ScenarioResult {
    let complete_matches = dd_complete_matches();
    let plans = [
        RoundPlan {
            round: 1,
            admission_limit: 3,
        },
        RoundPlan {
            round: 2,
            admission_limit: 1,
        },
    ];

    let mut selected_matches: Vec<CompleteMatch> = Vec::new();
    let mut residual_matches: Vec<CompleteMatch> = Vec::new();
    let mut residual_available_in_round_2 = Vec::new();
    let mut worklist_rows = Vec::new();
    let mut action_rows = Vec::new();
    let mut pre_barrier_action_attempts = 0;
    let mut barrier_count = 0;

    for plan in plans {
        let candidates = if plan.round == 1 {
            complete_matches.clone()
        } else {
            assert!(
                !residual_matches.is_empty(),
                "skipped matches must remain available for the next round"
            );
            residual_available_in_round_2 = residual_matches
                .iter()
                .map(|candidate| candidate.id)
                .collect();
            residual_matches.clone()
        };

        let (selected, residual) = scheduler_admit(candidates, plan.admission_limit);
        let barrier = barrier_count + 1;
        let round_worklist: Vec<_> = selected
            .iter()
            .map(|candidate| WorklistRow {
                round: plan.round,
                barrier,
                match_id: candidate.id,
                output: candidate.output,
            })
            .collect();

        let fired_before_barrier =
            fire_actions_after_barrier(false, &round_worklist, &mut action_rows);
        assert_eq!(
            fired_before_barrier, 0,
            "action phase must not fire before the scheduler barrier"
        );
        pre_barrier_action_attempts += 1;

        worklist_rows.extend(round_worklist.iter().cloned());
        selected_matches.extend(selected);
        residual_matches = residual;

        barrier_count += 1;
        let fired_after_barrier =
            fire_actions_after_barrier(true, &round_worklist, &mut action_rows);
        assert_eq!(
            fired_after_barrier,
            round_worklist.len(),
            "every admitted worklist row must fire after the barrier"
        );
    }

    assert_eq!(
        selected_matches.len(),
        worklist_rows.len(),
        "only scheduler-selected matches may be written to the worklist"
    );
    assert_eq!(
        worklist_rows.len(),
        action_rows.len(),
        "each admitted worklist row must produce one delayed action row"
    );
    assert_eq!(
        residual_available_in_round_2,
        vec![4, 5],
        "round 2 must see skipped round 1 matches"
    );
    assert_eq!(
        residual_matches
            .iter()
            .map(|candidate| candidate.id)
            .collect::<Vec<_>>(),
        vec![5],
        "unadmitted matches must remain residual after the last round"
    );

    ScenarioResult {
        complete_matches,
        selected_matches,
        residual_matches,
        residual_available_in_round_2,
        worklist_rows,
        action_rows,
        pre_barrier_action_attempts,
        barrier_count,
        rounds: plans.len() as u64,
    }
}

fn metrics_for(result: &ScenarioResult) -> Metrics {
    let mut metrics = Metrics::new();
    metrics.insert(
        "complete_matches".to_string(),
        json!(result.complete_matches.len()),
    );
    metrics.insert(
        "selected_matches".to_string(),
        json!(result.selected_matches.len()),
    );
    metrics.insert(
        "residual_matches".to_string(),
        json!(result.residual_matches.len()),
    );
    metrics.insert(
        "worklist_rows".to_string(),
        json!(result.worklist_rows.len()),
    );
    metrics.insert("action_rows".to_string(), json!(result.action_rows.len()));
    metrics.insert("barrier_count".to_string(), json!(result.barrier_count));
    metrics.insert("rounds".to_string(), json!(result.rounds));
    metrics.insert(
        "pre_barrier_action_attempts".to_string(),
        json!(result.pre_barrier_action_attempts),
    );
    metrics.insert(
        "round_2_residual_input_match_ids".to_string(),
        json!(result.residual_available_in_round_2),
    );
    metrics.insert(
        "final_residual_match_ids".to_string(),
        json!(result
            .residual_matches
            .iter()
            .map(|candidate| candidate.id)
            .collect::<Vec<_>>()),
    );
    if let Some(rss) = rss_kb() {
        metrics.insert("rss_kb".to_string(), json!(rss));
    }
    metrics
}

fn main() {
    let out_path = out_path_arg();
    let result = run_scenario();
    let metrics = metrics_for(&result);

    let report = ExperimentReport {
        experiment: "scheduler-materialization",
        status: "pass",
        command: command_string(),
        configs: vec![json!({
            "model": "in-memory scheduler over complete DD match output",
            "round_admission_limits": [3, 1],
            "deterministic_order": "match_id",
        })],
        metrics,
        observations: vec![
            "DD-side complete match output can remain separate from scheduler admission."
                .to_string(),
            "Only selected matches are written back into the host worklist.".to_string(),
            "Action rows are produced only after an explicit scheduler barrier.".to_string(),
            "Skipped matches remain residual input for a later scheduler round.".to_string(),
        ],
        decision:
            "Scheduler materialization support should be designed before the first DD backend PR."
                .to_string(),
        limitations: vec![
            "This is an in-memory control-plane model, not a Differential Dataflow operator graph."
                .to_string(),
            "The scenario validates scheduler semantics but does not measure throughput.".to_string(),
        ],
        next_action:
            "Carry selected-match admission, worklist writeback, barrier-delayed actions, and residual retention into the backend interface plan."
                .to_string(),
    };

    write_report(out_path, &report).expect("failed to write scheduler materialization report");
    println!(
        "scheduler_materialization: status={} complete={} selected={} residual={} worklist={} actions={} barriers={} rounds={}",
        report.status,
        result.complete_matches.len(),
        result.selected_matches.len(),
        result.residual_matches.len(),
        result.worklist_rows.len(),
        result.action_rows.len(),
        result.barrier_count,
        result.rounds
    );
}
