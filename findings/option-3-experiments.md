# Option 3 Experiment Results

This records the first runnable Option 3 experiment suite in
`code/option3-overlap`. The suite tests whether a DD/Timely-backed rule matcher
can keep future logical rule tasks physically in flight while preserving exact
egglog-style per-rule freshness and visibility gates.

## Commands

```bash
cargo test --manifest-path code/option3-overlap/Cargo.toml
cargo run --release --manifest-path code/option3-overlap/Cargo.toml -- --suite semantic
cargo run --release --manifest-path code/option3-overlap/Cargo.toml -- --suite semantic --json-out findings/artifacts/option3-overlap-semantic.json
cargo run --release --manifest-path code/option3-overlap/Cargo.toml -- --suite scaling --json-out findings/artifacts/option3-overlap.json
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
| dd-barrier | true | true | - | 0 | 1323 | 0 | 3 |
| dd-overlap | true | true | - | 0 | 428 | 1 | 0 |

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
| chain | 8 | 1 | 3887 | w=4, 668 us (5.82x) | 722 | win |
| chain | 8 | 4 | 1534 | w=2, 1167 us (1.31x) | 1668 | win |
| chain | 32 | 1 | 4941 | w=2, 4406 us (1.12x) | 3977 | win |
| chain | 32 | 4 | 4636 | w=1, 4409 us (1.05x) | 5108 | win |
| chain | 128 | 1 | 329958 | w=1, 307894 us (1.07x) | 307520 | win |
| chain | 128 | 4 | 340313 | w=1, 349306 us (0.97x) | 404159 | neutral/loss |
| fanout | 8 | 1 | 559 | w=4, 343 us (1.63x) | 354 | win |
| fanout | 8 | 4 | 683 | w=4, 424 us (1.61x) | 463 | win |
| fanout | 32 | 1 | 289 | w=1, 269 us (1.07x) | 328 | win |
| fanout | 32 | 4 | 475 | w=4, 399 us (1.19x) | 448 | win |
| fanout | 128 | 1 | 348 | w=4, 278 us (1.25x) | 299 | win |
| fanout | 128 | 4 | 542 | w=4, 465 us (1.17x) | 480 | win |

## Verdict

- The semantic part of the Option 3 hypothesis passed this small test:
  DD-backed matching preserved per-rule freshness while allowing later logical
  tasks to be issued early and gated by progress.
- The performance evidence is positive but not decisive. Overlap helped most
  runs, but the largest four-worker chain regressed slightly and the native
  barrier variant sometimes matched or beat the best overlapped run.
- Synthetic native barriers collapse the experiment back toward stop/start
  behavior. This keeps native actions, rebuild, and custom scheduler boundaries
  as first-class blockers for a broader Option 3 architecture.
- This does not test equality maintenance, rebuild invalidation, containers,
  WCOJ, or provider boundaries. Those remain follow-up gates before promoting
  Option 3 beyond a promising scheduling result.
