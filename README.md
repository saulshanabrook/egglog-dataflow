# Egglog on Differential Dataflow?

This repository is a source bundle for deciding whether `egglog` should move
some of its database-like runtime onto Differential Dataflow, FlowLog,
datatoad, or a nearby substrate. The possible upside hypothesis is shared
performance and maintenance work with maintained dataflow/database projects; the
risk is that egglog's equality maintenance, rebuilding, containers, schedules,
and frontend semantics are too specialized for that move to pay off.

## Current State

No backend path has been selected. A first Option 3 scheduling experiment now
shows that a DD-backed matcher can preserve per-rule freshness on the scheduled
reachability witness while issuing later logical tasks early, but the performance
signal is mixed and synthetic native barriers collapse the benefit toward
stop/start execution. The larger blockers around equality maintenance,
rebuilding, containers, extension APIs, frontend compatibility, and custom
scheduler/action barriers remain open.

The benefits below are still mostly working hypotheses from local source review.
The Option 3 scheduling row includes one small measured prototype result.
`DD` means Differential Dataflow, and `WCOJ` means worst-case optimal joins.

## Backend Options

The options are ordered as a rough complexity and disruption ladder. The
current scheduling hypothesis is that DD/Timely may allow overlapped physical
execution across logical egglog iterations without changing egglog's observable
schedule semantics. A semantic relaxation remains a possible fallback variant,
but it is not the primary Option 3 framing.

| Path | Potential Benefits | Long-Term Costs / Blockers | Link |
| --- | --- | --- | --- |
| Native improvement / borrow ideas | Least disruptive baseline. Keeps the backend native while testing WCOJ planning, provider interfaces, columnar storage, profiling, timestamp/index improvements, and clearer rule IR boundaries. | Gives up much of the shared-substrate maintenance story unless provider boundaries isolate reusable relation work from equality, containers, rebuild, and scheduling. | [Option 4](findings/options/option-4-no-dd-backend-borrow-ideas.md) |
| Exact hybrid DD rule evaluation | Tests a shared relational substrate while preserving native equality, rebuild, containers, and user-visible schedules. DD/FlowLog could own selected maintained rule indexes and incremental body matching. | Requires a precise delta boundary for canonical ids, rebuild invalidations, same-id container refresh, per-rule seminaive freshness, custom scheduler match selection, match deduplication, and action handoff. | [Option 1](findings/options/option-1-native-equality-dd-rule-eval.md) |
| FlowLog/datatoad middle layer with DD-overlapped scheduling | Prototype evidence shows exact per-rule freshness can be preserved while future logical tasks are in flight on a reachability workload. Could support a long-term relational planning layer with DD execution and WCOJ-style joins. | Requires a substantial new planner, index story, recursive-control model, timestamp/frontier design, egglog-specific adapter, and equality/rebuild invalidation model. Synthetic native barriers already show that native actions, rebuild, and custom schedulers may erase the overlap benefit. | [Option 3](findings/options/option-3-flowlog-datatoad-middle-layer.md) |
| Proof/term encoding to DD | Gives a concrete relational specification for equality, UF/view/rebuild state, and proof-oriented experiments. | Current evidence says it is high-overhead, incomplete for current egglog features, only a partial validation oracle, and incompatible with container/presort/scheduler semantics without a separate native path. | [Option 2](findings/options/option-2-proof-term-encoding-dd.md) |

A provider-style relation boundary cuts across several options: ordinary rule
relations might use DD/FlowLog while equality, containers, and rebuild-sensitive
relations stay behind specialized providers. This is tracked as a first-class
uncertainty in [findings/synthesis.md](findings/synthesis.md).

## How To Pick This Up

- Read [findings/synthesis.md](findings/synthesis.md) for the consolidated evidence and
  continue/stop criteria.
- Read [findings/options/README.md](findings/options/README.md) for the detailed option
  tradeoffs.
- Pick an option based on the uncertainty you want to reduce, then use that
  option note's evidence-to-gather list before implementation.
- If evaluating whether to stop, read the "Evidence to stop" section in
  [findings/synthesis.md](findings/synthesis.md).
- If you need the detailed research framing, source inventory, or reading order,
  read [findings/methodology.md](findings/methodology.md).

## Repo Map

- `findings/`: conclusions, option analysis, methodology, and research notes.
- `repos/`: pinned source submodules.
- `papers/`: local papers.
- `messages/`: design conversations.
- `code/`: small prototypes.
