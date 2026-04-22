# Option 3 Experiment Results

This records the first runnable Option 3 experiment suite in
`code/option3-overlap`. The suite tests whether a DD/Timely-backed rule matcher
can keep future logical rule tasks physically in flight while preserving exact
egglog-style per-rule freshness and visibility gates.

For interpretation of the results, see
`findings/option-3-experiment-findings.md`. For the generated ordered lane
index, see `findings/experiments/option-3/README.md`.

## Commands

```bash
cargo test --manifest-path code/option3-overlap/Cargo.toml
cargo run --release --manifest-path code/option3-overlap/Cargo.toml -- --suite semantic
cargo run --release --manifest-path code/option3-overlap/Cargo.toml -- --suite semantic --json-out findings/artifacts/option3-overlap-semantic.json
cargo run --release --manifest-path code/option3-overlap/Cargo.toml -- --suite scaling --json-out findings/artifacts/option3-overlap.json
python3 code/option3-overlap/scripts/validate_option3_lanes.py findings/artifacts/option-3
python3 code/option3-overlap/scripts/summarize_option3_lanes.py findings/artifacts/option-3 findings/experiments/option-3/README.md
```

## Semantic Gate

The minimal witness uses the corrected scheduled reachability rules:

```lisp
(relation edge1 (i64 i64))
(relation edge2 (i64 i64))
(relation reachable (i64 i64))

(ruleset tc1)
(rule ((edge1 x y)) ((reachable x y)) :ruleset tc1)
(rule ((reachable x y) (edge2 y z)) ((reachable x z)) :ruleset tc1)

(ruleset tc2)
(rule ((edge2 x y)) ((reachable x y)) :ruleset tc2)
(rule ((reachable x y) (edge1 y z)) ((reachable x z)) :ruleset tc2)
```

Base facts are `edge1(1, 2)` and `edge2(2, 3)`. Correct per-rule freshness
derives `reachable(1, 2)`, `reachable(2, 3)`, and `reachable(1, 3)`.

| Mode | Semantic | Freshness | Missing | Early visibility | Runtime us | Max lag | Barriers |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: |
| oracle | true | true | - | 0 | 1 | 0 | 0 |
| broken-global | false | false | `(1, 3)` | 0 | 0 | 0 | 0 |
| dd-barrier | true | true | - | 0 | 1798 | 0 | 3 |
| dd-overlap | true | true | - | 0 | 415 | 1 | 0 |

The broken global-seminaive model misses `reachable(1, 3)`, as intended. Both
DD modes match the oracle. The overlapped mode has a future task in flight
without exposing later results early.

## Scaling Summary

The scaling suite ran `chain` and `fanout` workloads at scales 8, 32, and 128
with one and four workers. All 60 scaling runs preserved semantic equivalence,
per-rule freshness, and zero early visibility violations. The maximum observed
logical task lag was three tasks.

| Workload | Scale | Workers | Barrier us | Best overlap | Native barrier us | Verdict |
| --- | ---: | ---: | ---: | --- | ---: | --- |
| chain | 8 | 1 | 3459 | w=2, 1358 us (2.55x) | 1424 | win |
| chain | 8 | 4 | 2002 | w=4, 1601 us (1.25x) | 1499 | win |
| chain | 32 | 1 | 5868 | w=4, 4939 us (1.19x) | 4484 | win |
| chain | 32 | 4 | 4958 | w=2, 4683 us (1.06x) | 4810 | win |
| chain | 128 | 1 | 311074 | w=1, 311281 us (1.00x) | 333536 | neutral/loss |
| chain | 128 | 4 | 577048 | w=2, 379099 us (1.52x) | 567355 | win |
| fanout | 8 | 1 | 479 | w=4, 362 us (1.32x) | 393 | win |
| fanout | 8 | 4 | 856 | w=1, 667 us (1.28x) | 1184 | win |
| fanout | 32 | 1 | 689 | w=4, 783 us (0.88x) | 823 | neutral/loss |
| fanout | 32 | 4 | 1050 | w=4, 968 us (1.08x) | 1088 | win |
| fanout | 128 | 1 | 407 | w=2, 619 us (0.66x) | 432 | neutral/loss |
| fanout | 128 | 4 | 1363 | w=4, 787 us (1.73x) | 727 | win |

## Scheduling Result

- DD-backed matching preserved per-rule freshness while allowing later logical
  tasks to be issued early and gated by progress in this harness.
- The performance evidence is workload-dependent. Overlap helped 9 of the 12
  summary rows, but three summary rows were neutral or losses and the native
  barrier variant sometimes matched or beat the best overlapped run.
- Synthetic native barriers collapse this harness back toward stop/start
  behavior. The option-level interpretation of that result is maintained in
  `findings/option-3-experiment-findings.md`.
- This log does not measure equality maintenance, rebuild invalidation,
  containers, WCOJ runtime, or provider boundaries.

## Follow-Up Lane Artifacts

The follow-up pass wrote ordered lane artifacts under
`findings/artifacts/option-3` and lane notes under
`findings/experiments/option-3`. Regenerate the index with:

```bash
python3 code/option3-overlap/scripts/validate_option3_lanes.py findings/artifacts/option-3
python3 code/option3-overlap/scripts/summarize_option3_lanes.py findings/artifacts/option-3 findings/experiments/option-3/README.md
```

The canonical interpretation is intentionally kept out of this reproducibility
log; see `findings/option-3-experiment-findings.md`.
