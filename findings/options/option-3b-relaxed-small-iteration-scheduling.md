# Option 3b: Relaxed Small-Iteration Scheduling

## Motivation

The FlowLog/datatoad-like middle layer should not assume egglog's current
physical execution schedule is the only useful contract. The local FlowLog
discussion reports a DD-oriented execution idea: work may run better when it is
spread across many smaller overlapping iterations instead of one large batch
per ruleset iteration (`messages/dec-17-2025-slack.md`). This is attractive
because DD/Timely can pipeline work across iterations and operators, while
large egglog batches may create high peak intermediates and coarse progress
boundaries.

The constraint is semantic, not just mechanical. The current egglog scheduler
defines seminaive freshness with per-rule last-run timestamps, and existing
programs can use `run`, `saturate`, `seq`, `repeat`, `run-with`, and ruleset
boundaries to control blowup, nontermination, and manual stratification
(`findings/source-notes/scaling-equality-saturation.md`,
`findings/source-notes/containers-frontends.md`). Option 3b therefore treats
small-iteration scheduling as an opt-in or scoped relaxed execution mode, not
as an exact implementation of today's scheduler.

## General Approach

- Keep exact scheduling as the default contract outside relaxed regions.
- Add a scoped relaxed region, ruleset annotation, or compiler-internal subset
  where the program accepts backend-chosen physical rule order.
- Let the backend lower a relaxed region into many smaller DD tasks: per-rule,
  per-stratum, per-delta-family, per-pattern-root, or per-rebuild-wave units.
- Allow DD/Timely timestamps and iterative scopes to pipeline those tasks when
  dependencies allow, rather than forcing one large "run all rules, rebuild,
  repeat" batch.
- Preserve declared region boundaries and final convergence checks for the
  relaxed region. The observable contract should be defined as a scoped result,
  not as exact agreement on every intermediate match set.
- Keep native equality/rebuild initially, but expose rebuild invalidation and
  same-id dirty refresh as first-class events so small iterations see semantic
  changes that do not look like ordinary tuple inserts/deletes.

## What Would Be Relaxed

- The exact order in which eligible rules in a relaxed region run.
- The exact per-rule physical iteration timing inside that region.
- The exact grouping of match discovery into current egglog ruleset batches.
- Potentially the timestamp policy used internally by DD, as long as the region
  contract is preserved.

This does not automatically relax equality correctness, rebuild consistency,
container semantics, action semantics, extraction behavior, or explicit
schedule boundaries outside the relaxed region.

## Why This Changes Option 3

- Option 3a asks a FlowLog/datatoad-like middle layer to preserve current
  logical schedules while improving the physical plan.
- Option 3b asks whether some workloads can accept a different scoped schedule
  contract that gives DD more room to choose physical order and timestamp
  granularity.
- This makes the DD fit more plausible because the backend no longer has to
  emulate every exact per-rule freshness window inside the relaxed region.
- It also makes the option more disruptive because it is a user-visible or
  compiler-visible semantic change, not only a backend substitution.

## Compatibility Constraints

- Programs that rely on bounded `run`, staged `saturate`, ruleset ordering,
  manual stratification, or blowup control may need exact mode.
- Current custom schedulers can materialize all matches, choose a subset, keep
  residual matches, and delay action firing (`repos/egglog/src/scheduler.rs`).
  That observable contract is hard to preserve under relaxed pipelining.
- Actions are not just derived facts: they can call primitives, set functions,
  delete, subsume, union, and trigger merge/rebuild behavior. Relaxed regions
  need a proof or restriction for which actions can be reordered.
- Rebuild timing is flexible in the e-graph sense, but DD needs explicit
  progress boundaries: it must know when a rebuild wave's invalidations have
  caught up before exposing matches that depend on them.
- The relaxed contract should probably start with monotone, schedule-insensitive
  rule subsets before touching deletion, subsumption, primitive side effects,
  analyses, extraction-sensitive costs, or custom scheduler APIs.

## Likely Benefits

- Better parallel throughput from overlapping small DD iterations instead of
  waiting for one large ruleset batch to finish.
- Lower peak intermediate state if rule matching is split by strata, deltas, or
  pattern roots and compacted between phases.
- A more natural mapping for nested egglog schedules, because FlowLog extended
  modes and DD iterative scopes already expose nested feedback structures.
- Simpler DD timestamp and progress policy than exact schedule preservation,
  if the relaxed region can use coarser semantic boundaries.
- Cleaner profiling: the middle layer could attribute cost to physical schedule
  units rather than only to whole ruleset runs.

## Likely Blockers

- The relaxed semantics must be specified clearly enough that users know which
  programs are eligible and what equivalence is promised.
- Custom schedulers may force full-match materialization and delayed firing,
  which works against small-iteration pipelining.
- It is unclear which egglog actions can be safely reordered across physical
  iterations. `union` and monotone set-like actions are easier than `delete`,
  `subsume`, primitive calls, merge functions, analyses, and
  extraction-sensitive cost updates.
- DD timestamp/progress overhead could erase the benefit if the chunks are too
  small. The right granularity is empirical.
- More physical parallelism may increase nondeterminism in when matches are
  observed, even if final e-graph saturation is equivalent for a supported
  subset.
- If too many real programs require exact mode, Option 3b becomes a niche
  optimization rather than a meaningful backend boundary.

## Evidence To Gather

- Classify representative egglog schedules into exact-required,
  relaxed-eligible, and unclear groups, including examples that use bounded
  `run`, staged `saturate`, manual stratification, and custom schedulers.
- Build a schedule-lowering sketch for one relaxed-eligible ruleset with 3-5
  interacting rules: current egglog bulk iteration, per-rule small iterations,
  and per-delta-family small iterations.
- Reproduce the scheduled reachability example from
  `repos/scaling-equality-saturation/egglog-new-backend.md` and use it to mark
  the boundary between exact mode and relaxed mode.
- Classify actions in those rules as freely reorderable, rebuild-dependent, or
  scheduler-sensitive.
- Prototype a DD/Timely toy loop with many small deltas and compare it with one
  large batch on operator utilization, retained traces, progress traffic, and
  peak memory.
- Run one custom scheduler/backoff example and measure how much full-match
  materialization blocks small-iteration pipelining.
- Check whether the same final saturated e-graph is reached when rebuild is run
  after every small iteration, after a group of small iterations, or only at
  coarse epoch boundaries.

## Current Assessment

Option 3b is the main way the scheduling discussion could make a DD-backed path
more viable: it gives DD more control over physical order instead of requiring
an exact emulation of egglog's current bulk ruleset execution. The cost is that
it is a scoped semantic relaxation. It should be evaluated as a possible
execution mode for schedule-insensitive subsets, not as a replacement for exact
egglog scheduling or custom scheduler behavior.
