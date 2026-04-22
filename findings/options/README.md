# Backend Boundary Option Findings

This directory stores the tradeoff analysis for the backend boundary options
identified in `../synthesis.md`.

These tradeoffs are working hypotheses. Provider-style relation boundaries are
a cross-cutting design axis: ordinary rule relations might use DD/FlowLog while
equality, containers, and rebuild-sensitive relations stay behind specialized
providers. That axis needs a concrete comparison before it becomes a separate
option or sub-option.

Arbitrary scheduling with seminaive evaluation is another cross-cutting
constraint. The scaling-equality-saturation draft shows that egglog needs
per-rule last-run timestamps and timestamp-window scans; options that move rule
matching must either preserve that logical freshness model exactly or introduce
an explicitly relaxed scheduling mode.

## Complexity Ladder

| Path | Main Benefit | Long-Term Cost / Blocker | Note |
| --- | --- | --- | --- |
| Native improvement / borrow ideas | Preserves existing semantics while incrementally adopting WCOJ planning, provider interfaces, columnar storage, profiling, timestamp/index work, or cleaner rule IR boundaries. | Does not answer the shared-substrate motivation unless provider-style boundaries isolate reusable pieces from native-only behavior. | [Option 4](option-4-no-dd-backend-borrow-ideas.md) |
| Exact hybrid DD rule evaluation | Tests maintained relational matching and indexing while keeping equality/rebuild/container behavior and logical schedules native. | Needs a precise delta interface for canonical-id changes, explicit rebuild invalidations, same-id dirty refresh, per-rule seminaive timestamps, scheduler match selection, match deduplication, and action handoff. | [Option 1](option-1-native-equality-dd-rule-eval.md) |
| Option 3b: relaxed small-iteration DD scheduling | Lets DD spread work across many small overlapping physical iterations inside explicit relaxed regions. | Changes the schedule contract. It must be scoped away from programs that depend on bounded `run`, staged `saturate`, blowup control, manual stratification, or exact custom scheduler behavior. | [Option 3b](option-3b-relaxed-small-iteration-scheduling.md) |
| Option 3a: exact FlowLog/datatoad middle layer | Could become a coherent long-term relational planner with DD execution, WCOJ-style join kernels, and schedule-aware physical planning while preserving current logical schedule semantics. | Requires a substantial new planner, index universe, recursive-control model, egglog-specific adapter, per-rule freshness model, and rebuild/equality invalidation model. | [Option 3a](option-3-flowlog-datatoad-middle-layer.md) |
| Proof/term encoding to DD | Provides a concrete relational specification for equality, UF/view/rebuild tables, and proof-oriented experiments. | Current encoding is slow, incomplete for current egglog features, only a partial validation oracle, and incompatible with container/presort/scheduler/per-rule seminaive semantics without a native side channel. | [Option 2](option-2-proof-term-encoding-dd.md) |

## Exact vs Relaxed Scheduling

- Exact mode must preserve per-rule timestamp windows, custom scheduler
  behavior, ruleset order, `run`/`saturate` boundaries, and manual
  stratification. This is the compatibility-preserving contract for Option 1
  and Option 3a.
- Relaxed mode may allow backend-chosen physical order, overlapping DD
  iterations, and a coarser or DD-friendlier timestamp policy inside explicitly
  marked regions. This is the semantic bet behind Option 3b.
- Relaxed mode must be scoped because existing programs may rely on bounded
  `run`, staged `saturate`, rule ordering to control blowup or
  nontermination, manual stratification, and full-match materialization for
  custom schedulers.

## Tradeoff Summary

- Native improvement is the least disruptive baseline, but it gives up most of
  the social and maintenance benefit of sharing a substrate with
  DD/FlowLog/datatoad unless a provider-style boundary can separate reusable
  relation work from native-only equality/container/rebuild/scheduling
  behavior.
- Exact hybrid DD rule evaluation is the smallest DD migration surface, but it
  may force egglog and DD to maintain overlapping indexes and carefully
  synchronized deltas, including per-rule freshness windows.
- Option 3b makes the DD fit more plausible by relaxing the logical scheduling
  contract inside scoped regions, but it is a semantic change rather than a
  transparent backend substitution.
- Option 3a has broad architecture upside, but it is a large system design
  project rather than a small backend substitution, especially if it preserves
  exact scheduling while also owning planner/runtime structure.
- Proof/term encoding is a clear specification path, but it currently looks too
  expensive and feature-incomplete for production lowering.
