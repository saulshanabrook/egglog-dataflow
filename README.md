# Egglog on Differential Dataflow?

This repository is a source bundle for deciding whether `egglog` should move
some of its database-like runtime onto Differential Dataflow, FlowLog,
datatoad, or a nearby substrate. The possible upside hypothesis is shared
performance and maintenance work with maintained dataflow/database projects; the
risk is that egglog's equality maintenance, rebuilding, containers, schedules,
and frontend semantics are too specialized for that move to pay off.

## Current State

No backend path has been selected. The current evidence is useful for comparing
the long-term costs and benefits of each option, but it still leaves open
blockers around equality maintenance, rebuilding, containers, schedules,
extension APIs, and frontend compatibility. The next useful work is to gather
targeted evidence for whichever option looks most important to evaluate.

The benefits below are working hypotheses from the local source review, not
validated benchmark conclusions. `DD` means Differential Dataflow, and `WCOJ`
means worst-case optimal joins.

## Backend Options

The options are ordered as a rough complexity and disruption ladder. Exact
scheduling preserves today's egglog contract, including custom schedules and
per-rule seminaive freshness. Relaxed scheduling may fit DD better by allowing
backend-chosen physical order inside explicit regions, but it changes the
program contract and would need to be scoped.

| Path | Potential Benefits | Long-Term Costs / Blockers | Link |
| --- | --- | --- | --- |
| Native improvement / borrow ideas | Least disruptive baseline. Keeps the backend native while testing WCOJ planning, provider interfaces, columnar storage, profiling, timestamp/index improvements, and clearer rule IR boundaries. | Gives up much of the shared-substrate maintenance story unless provider boundaries isolate reusable relation work from equality, containers, rebuild, and scheduling. | [Option 4](findings/options/option-4-no-dd-backend-borrow-ideas.md) |
| Exact hybrid DD rule evaluation | Tests a shared relational substrate while preserving native equality, rebuild, containers, and user-visible schedules. DD/FlowLog could own selected maintained rule indexes and incremental body matching. | Requires a precise delta boundary for canonical ids, rebuild invalidations, same-id container refresh, per-rule seminaive freshness, custom scheduler match selection, match deduplication, and action handoff. | [Option 1](findings/options/option-1-native-equality-dd-rule-eval.md) |
| Option 3b: relaxed small-iteration DD scheduling | Lets DD choose many smaller overlapping physical iterations inside explicit relaxed regions, which may better match dataflow execution than large ruleset batches. | Higher semantic risk. Existing programs may rely on bounded `run`, staged `saturate`, blowup control, manual stratification, or exact custom scheduler behavior. | [Option 3b](findings/options/option-3b-relaxed-small-iteration-scheduling.md) |
| Option 3a: exact FlowLog/datatoad middle layer | Could support a long-term relational planning layer with DD execution, WCOJ-style joins, and schedule-aware physical planning while preserving the current logical schedule contract. | Requires a substantial new planner, index story, recursive-control model, schedule/freshness model, egglog-specific adapter, and equality/rebuild invalidation model. | [Option 3a](findings/options/option-3-flowlog-datatoad-middle-layer.md) |
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
