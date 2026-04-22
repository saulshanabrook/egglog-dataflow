# Option 3: FlowLog/Datatoad/DD-Inspired New Backend

## Viability
- Medium and high-risk. Option 3 should now be treated as a new backend path,
  not a middle-layer adapter over native `core-relations`. The scheduling
  experiment gives this path one positive result: DD/Timely can preserve
  per-rule freshness and gated visibility on the corrected reachability witness
  while keeping later logical work physically in flight
  (`findings/option-3-experiment-findings.md`). The follow-up lanes do not rule
  out a new backend. They rule out a permanent adapter shape where native egglog
  remains authoritative while a DD/FlowLog layer mirrors enough state to
  coordinate around rebuild, containers, schedulers, and equality.
- The central requirement is single ownership. A responsibility either moves
  into the new backend and is tested against native egglog as an oracle, or it
  stays out of scope for the slice. The new backend must not permanently shadow
  native canonical ids, row freshness, rebuild invalidations, container dirty
  ids, scheduler matches, or rule timestamps.

## General Approach
- Build an authoritative egglog backend inspired by FlowLog's planner/control
  split, DD/Timely's maintained dataflow and frontier model, and
  datatoad/dataflow-join-style WCOJ kernels. The new backend owns its chosen
  runtime state instead of adapting to native table state after the fact.
- Keep the egglog language, parser, typechecker, command runner, existing test
  corpus, and native backend as compatibility surface and oracle. The new
  backend should be selected for experiments behind an explicit backend switch
  or separate crate; it should not mutate native `core-relations` state for the
  responsibilities it claims.
- Start with a vertical slice, not a broad port. The slice should own relation
  storage, per-rule timestamps, one rebuild/canonicalization event, one
  same-id dirty-refresh-style invalidation, and one scheduler/materialization
  boundary for a small workload. It should compare step-visible state against
  native egglog.
- Treat DD-overlapped scheduling as one backend optimization. The semantic
  contract remains exact logical scheduling: per-rule seminaive freshness,
  ruleset order, bounded `run`, staged `saturate`, scheduler admission, action
  visibility, and rebuild visibility must match native behavior unless a future
  experiment explicitly introduces a relaxed schedule mode.

## Backend Ownership Model
- **Owned by the new backend in the first serious slice:** relation/function
  table storage for the selected universe, row insertion/refresh timestamps,
  per-rule last-run timestamps, rule-body planning, match deduplication,
  visibility gates, scheduler materialization for selected rules, one
  rebuild/canonicalization event, and one same-id dirty-refresh-style
  invalidation or equivalent.
- **Owned by the new backend once the slice expands:** broader rebuild
  invalidation, broader container refresh signals, trace/frontier scheduling,
  binary/WCOJ join selection, and retained index/trace compaction.
- **Kept native initially:** frontend language semantics, parsing/typechecking,
  user-facing command orchestration, extraction/proof hooks, Python-facing API
  compatibility, and tests used as the oracle.
- **Not allowed as a final architecture:** a native table plus a mirrored DD
  table both claiming authority over canonical ids, freshness, dirty container
  state, scheduler match sets, or rebuild timing.

## Required Interfaces
- A backend-state API for the selected slice: typed rows, logical timestamps,
  rule ids, relation ids, canonical ids, and explicit row freshness semantics.
- A rule-planning API that can represent egglog atoms, repeated variables,
  filters, projections, semijoins, binary joins, WCOJ joins, recursive feedback,
  and action outputs.
- A schedule API that records rule last-run timestamps and separates logical
  schedule visibility from physical DD/Timely progress.
- A rebuild/invalidation API that represents representative rewrites,
  row refreshes, retractions/reinsertions, and same-id container dirty refresh
  without calling native `core-relations` for the moved responsibility.
- A scheduler API that supports full-match materialization, selected-match
  admission, backoff/skipping, and delayed action firing for the selected slice.
- A comparison API that records native-oracle and new-backend step-visible
  states at each logical boundary.

## Evidence From Current Experiments
- Passed: schedule overlap, per-rule freshness, and zero early visibility
  violations on the corrected reachability witness; all 60 scaling runs stayed
  semantically/freshness correct.
- Passed as native-oracle regressions: rebuild/action, equality/rebuild,
  custom scheduler/backoff, and same-id container dirty-refresh cases.
- Not proven: useful overlap through real native barriers, row-level
  rebuild/container counters, trace memory and compaction behavior, WCOJ runtime
  comparison, or single-owner replacement-backend semantics.
- Interpretation: the middle-layer adapter reading is downgraded because it
  would mirror too much native state. The new-backend reading remains viable
  because a backend is allowed to own that complexity, provided it can replace
  rather than duplicate it.

## Next Gate
- Build a replacement-backend vertical slice with one small relation/function
  table universe. It must own its state, preserve the scheduled reachability
  witness, handle one rebuild/canonicalization event, handle one same-id
  dirty-refresh-style invalidation or equivalent, preserve one scheduler
  materialization boundary, and compare step-visible state against native
  egglog.
- The gate fails if correctness depends on calling back into native
  `core-relations` for a responsibility the new backend claims to own.
- The gate also fails if the new backend cannot expose the counters the current
  experiments are missing: row rewrites, retractions/reinsertions, refresh rows,
  scheduler admissions/skips, frontier lag, trace memory, and compaction.

## Current Assessment
- Option 3 is no longer a small integration path. It is a replacement-backend
  research path. That makes its scope large but coherent: FlowLog/DD/datatoad
  ideas are evaluated as the design of a new backend rather than as a shim over
  native tables. The current evidence justifies the next vertical-slice
  experiment; it does not justify a broad rewrite yet.
