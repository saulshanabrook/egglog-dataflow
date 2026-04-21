# Option 3 Refinement: Small-Iteration Scheduling

## Motivation

The FlowLog/datatoad-like middle layer should not assume egglog's current
physical execution schedule is fixed. In `messages/dec-17-2025-slack.md`,
Yihong reports a FlowLog takeaway from Hangdong: DD may perform better when work
is spread across many iterations with small per-iteration deltas, because
operators can work on different iterations at the same time and large batches
are a poor fit for incrementality. Eli also points out that lattice timestamps
could allow starting the next iteration before the previous one finishes.

This suggests a refinement of Option 3: treat scheduling as part of the
relational middle layer, not merely as a compatibility shim around the current
egglog scheduler.

## Refined Architecture

- Split scheduling into two layers:
  - **Logical schedule:** the user-facing ruleset, `run`, `saturate`, `seq`,
    `repeat`, and `run-with` semantics that must be preserved or explicitly
    scoped out.
  - **Physical schedule:** the backend execution plan that chooses how to break
    rule matching, rebuild propagation, and derived relation maintenance into
    DD epochs, iterative scopes, and feedback loops.
- Let the middle layer lower a large egglog ruleset into many smaller units of
  work: per-rule, per-stratum, per-delta-family, per-pattern-root, or
  per-rebuild-wave tasks.
- Use DD/Timely timestamps and iterative scopes to pipeline those small tasks
  when dependencies allow, rather than forcing one large "run all rules,
  rebuild, repeat" batch.
- Keep native equality/rebuild initially, but expose rebuild invalidation and
  same-id dirty refresh as first-class events so micro-iterations see semantic
  changes that do not look like ordinary tuple inserts/deletes.

## Why This Changes Option 3

- It makes Option 3 less like "compile egglog's current scheduler to DD" and
  more like "build a schedule-aware relational runtime for egglog rules."
- It gives DD a clearer role: not just maintained joins, but overlapping many
  small incremental updates across iterative scopes.
- It may reduce peak memory and improve parallel throughput if large egglog
  saturation batches currently create too much per-iteration work.
- It may weaken compatibility with custom schedulers unless the design defines
  which scheduler APIs remain exact and which become unsupported, advisory, or
  emulated.

## Compatibility Constraints

- Current ordinary `step_rules` applies every match found for a ruleset in one
  iteration (`repos/egglog/src/lib.rs`). A micro-iteration backend must still
  produce the same saturated result for schedule-insensitive, monotone rule
  sets.
- Current custom schedulers can materialize all matches, choose a subset, keep
  residual matches, and delay action firing (`repos/egglog/src/scheduler.rs`).
  That is an observable contract for `run-with` and backoff-style schedulers.
- Actions are not just derived facts: they can call primitives, set functions,
  delete, subsume, union, and trigger merge/rebuild behavior. A physical
  scheduler cannot freely reorder actions unless the affected subset is proven
  commutative/idempotent or the user-facing schedule opts into that behavior.
- Rebuild timing is flexible in the e-graph sense, but DD needs explicit
  progress boundaries: it must know when a rebuild wave's invalidations have
  caught up before exposing matches that depend on them.

## Likely Benefits

- Better parallel throughput from overlapping small DD iterations instead of
  waiting for one large batch to finish.
- Lower peak intermediate state if rule matching is split by strata, deltas, or
  pattern roots and compacted between phases.
- A more natural mapping for nested egglog schedules, because FlowLog extended
  modes and DD iterative scopes already expose nested feedback structures.
- Cleaner profiling: the middle layer could attribute cost to physical schedule
  units rather than only to whole ruleset runs.

## Likely Blockers

- The design may need a new schedule IR that separates logical semantics from
  physical execution. That is more ambitious than a rule-planning layer alone.
- Custom schedulers may force full-match materialization and delayed firing,
  which works against micro-iteration pipelining.
- It is unclear which egglog actions can be safely reordered across
  micro-iterations. `union` and monotone set-like actions are easier than
  `delete`, `subsume`, primitive calls, merge functions, analyses, and
  extraction-sensitive cost updates.
- DD timestamp/progress overhead could erase the benefit if the chunks are too
  small. The right granularity is empirical.
- More physical parallelism may increase nondeterminism in when matches are
  observed, even if final e-graph saturation is equivalent.

## Evidence To Gather

- Build a schedule-lowering sketch for one ruleset with 3-5 interacting rules:
  current egglog bulk iteration, per-rule micro-iterations, and
  per-delta-family micro-iterations.
- Classify actions in those rules as freely reorderable, rebuild-dependent, or
  scheduler-sensitive.
- Prototype a DD/Timely toy loop with many small deltas and compare it with one
  large batch on operator utilization, retained traces, progress traffic, and
  peak memory.
- Run one custom scheduler/backoff example and measure how much full-match
  materialization blocks micro-iteration pipelining.
- Check whether the same final saturated e-graph is reached when rebuild is run
  after every micro-iteration, after a group of micro-iterations, or only at
  coarse epoch boundaries.

## Current Assessment

This refinement makes Option 3 more interesting but also more ambitious. It
could be the strongest DD-specific reason to depart from egglog's current
physical scheduler: DD may benefit from many small overlapped iterations rather
than large saturation batches. It should be treated as an experimental execution
strategy for a supported subset first, not as a guaranteed replacement for
custom scheduler semantics.
