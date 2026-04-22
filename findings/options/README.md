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
matching must preserve that logical freshness model unless they explicitly add
a new relaxed schedule mode. The current Option 3 hypothesis is stronger than
relaxation: DD may overlap physical work across logical egglog iterations while
preserving exact schedule semantics.

## Complexity Ladder

| Path | Main Benefit | Long-Term Cost / Blocker | Note |
| --- | --- | --- | --- |
| Native improvement / borrow ideas | Preserves existing semantics while incrementally adopting WCOJ planning, provider interfaces, columnar storage, profiling, timestamp/index work, or cleaner rule IR boundaries. | Does not answer the shared-substrate motivation unless provider-style boundaries isolate reusable pieces from native-only behavior. | [Option 4](option-4-no-dd-backend-borrow-ideas.md) |
| Exact hybrid DD rule evaluation | Tests maintained relational matching and indexing while keeping equality/rebuild/container behavior and logical schedules native. | Needs a precise delta interface for canonical-id changes, explicit rebuild invalidations, same-id dirty refresh, per-rule seminaive timestamps, scheduler match selection, match deduplication, and action handoff. | [Option 1](option-1-native-equality-dd-rule-eval.md) |
| FlowLog/datatoad middle layer with DD-overlapped scheduling | Could become a coherent long-term relational planner with DD execution, WCOJ-style join kernels, exact logical scheduling, and DD-overlapped physical execution across logical iterations. | Requires a substantial new planner, index universe, recursive-control model, egglog-specific adapter, per-rule freshness model, timestamp/frontier design, and rebuild/equality invalidation model. Native actions and custom schedulers may still force barriers. | [Option 3](option-3-flowlog-datatoad-middle-layer.md) |
| Proof/term encoding to DD | Provides a concrete relational specification for equality, UF/view/rebuild tables, and proof-oriented experiments. | Current encoding is slow, incomplete for current egglog features, only a partial validation oracle, and incompatible with container/presort/scheduler/per-rule seminaive semantics without a native side channel. | [Option 2](option-2-proof-term-encoding-dd.md) |

## Logical vs Physical Scheduling

- Exact logical scheduling must preserve per-rule timestamp windows, custom
  scheduler behavior, ruleset order, `run`/`saturate` boundaries, and manual
  stratification. This is the compatibility contract for Options 1 and 3.
- DD-overlapped physical scheduling may still preserve that contract. Timely/DD
  can track multidimensional timestamps and frontiers, so a middle layer may be
  able to start physical work for logical iteration `N+1` before all of
  iteration `N` has completed, then make later results observable only when the
  relevant frontiers prove earlier work is complete.
- Explicitly relaxed scheduling is a fallback variant, not the main Option 3
  hypothesis. It would need a scoped user/compiler contract because existing
  programs may rely on bounded `run`, staged `saturate`, rule ordering to
  control blowup or nontermination, manual stratification, and full-match
  materialization for custom schedulers.

## Tradeoff Summary

- Native improvement is the least disruptive baseline, but it gives up most of
  the social and maintenance benefit of sharing a substrate with
  DD/FlowLog/datatoad unless a provider-style boundary can separate reusable
  relation work from native-only equality/container/rebuild/scheduling
  behavior.
- Exact hybrid DD rule evaluation is the smallest DD migration surface, but it
  may force egglog and DD to maintain overlapping indexes and carefully
  synchronized deltas, including per-rule freshness windows.
- Option 3 has broad architecture upside, especially if DD-overlapped physical
  scheduling improves throughput without changing egglog's logical schedule
  semantics. It is still a large system design project rather than a small
  backend substitution.
- Proof/term encoding is a clear specification path, but it currently looks too
  expensive and feature-incomplete for production lowering.
