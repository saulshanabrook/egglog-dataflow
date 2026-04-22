# Option 3 Experiment Findings

This is the human-readable interpretation of the Option 3 experiment artifacts.
The reproducibility log remains in `option-3-experiments.md`, and the generated
lane index remains in `experiments/option-3/README.md`.

## Bottom Line

The experiments support the schedule-overlap subclaim and reject the permanent
middle-layer adapter framing. They do not reject Option 3 as a deliberate new
backend path.

The distinction is ownership. A middle-layer adapter would need to mirror native
egglog state across rebuild, containers, schedulers, and equality. A new backend
is allowed to own that logic, but it must prove that ownership in a vertical
slice instead of coordinating with native `core-relations` for the same
responsibility.

## What Passed

- Schedule overlap: `dd-barrier` and `dd-overlap` matched the oracle on the
  corrected scheduled reachability witness, while `broken-global` missed
  `reachable(1, 3)` as expected.
- Per-rule freshness: all DD semantic and scaling runs preserved the freshness
  condition, with zero early visibility violations.
- Scaling sanity: all 60 `chain`/`fanout` scaling runs were semantically
  correct and freshness-correct; maximum observed logical lag was three tasks.
- Native-oracle gates: targeted rebuild/action, equality/rebuild,
  scheduler/backoff, and container dirty-refresh regressions all passed on the
  native backend.
- Planner smoke checks: native `Gj`, `PureSize`, and `MinCover` representative
  tests passed, and `dataflow-join` examples compiled.

## What Remains Unproven

- Useful overlap through real rebuild/action/scheduler/container boundaries was
  not shown. The current evidence says those phases require explicit visibility,
  materialization, or refresh boundaries.
- Row-level rebuild and container counters are missing: canonical-id rewrites,
  retractions, reinsertions, refresh rows, and rebuild-index hits are not
  surfaced by the current binaries.
- Trace memory and compaction are not measured; the trace/progress lane only
  records frontier lag and candidate-flow proxies.
- WCOJ runtime comparison is blocked on graph inputs. The current WCOJ lane is
  compile-only plus blocked runtime probes.
- No experiment has yet shown a new backend owning rebuild, container refresh,
  scheduling, and per-rule timestamps without native shadow state.

## Interpretation

The adapter reading of Option 3 is downgraded: if native egglog remains
authoritative and a DD/FlowLog layer mirrors enough state to coordinate around
it, the result duplicates backend logic without proving a shared-substrate
payoff.

The replacement-backend reading remains open: if Option 3 owns the moved
runtime semantics directly, the large surface area becomes expected backend
scope rather than accidental adapter duplication.

## New Backend Decision Rule

Proceed only if the next slice has single ownership. The new backend must own
its selected relation state, per-rule freshness, visibility boundaries,
rebuild/canonicalization event, dirty-refresh-style invalidation, and scheduler
materialization boundary. Native egglog should be the oracle, not the execution
authority, for those moved responsibilities.

The gate fails if correctness depends on calling back into native
`core-relations` for a responsibility the new backend claims to own.
