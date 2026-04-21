# Egglog on Differential Dataflow?

This repository is a source bundle for deciding whether `egglog` should move
some of its database-like runtime onto Differential Dataflow, FlowLog,
datatoad, or a nearby substrate. The possible upside is shared performance and
maintenance work with a serious dataflow/database ecosystem; the risk is that
egglog's equality maintenance, rebuilding, containers, schedules, and frontend
semantics are too specialized for that move to pay off.

## Current State

No backend path has been selected. The current evidence is useful for comparing
the long-term costs and benefits of each option, but it still leaves open
blockers around equality maintenance, rebuilding, containers, schedules,
extension APIs, and frontend compatibility. The next useful work is to gather
targeted evidence for whichever option looks most important to evaluate.

## Backend Options

| Option | Potential Benefits | Long-Term Costs / Blockers | Link |
| --- | --- | --- | --- |
| Native equality + DD/FlowLog rule evaluation | Tests a shared relational substrate while preserving most existing egglog semantics. DD/FlowLog could own maintained rule indexes and incremental body matching. | Requires a precise delta boundary for canonical ids, rebuild invalidations, same-id container refresh, match deduplication, and action handoff. It may duplicate indexed state across egglog and the substrate. | [Option 1](findings/options/option-1-native-equality-dd-rule-eval.md) |
| FlowLog/datatoad-like middle layer | Could support a long-term relational planning layer with DD execution and WCOJ-style joins. | Requires a substantial new planner, index story, recursive-control model, and egglog-specific equality/rebuild operators before proving that the substrate boundary pays off. | [Option 3](findings/options/option-3-flowlog-datatoad-middle-layer.md) |
| Proof/term encoding to DD | Gives a concrete relational specification for equality, UF/view/rebuild state, and proof-oriented experiments. | Current evidence says it is high-overhead, incomplete for current egglog features, and incompatible with container/presort semantics without a separate native path. | [Option 2](findings/options/option-2-proof-term-encoding-dd.md) |
| No DD backend, borrow ideas | Keeps frontend/container semantics stable while adopting WCOJ planning, provider interfaces, columnar storage, profiling, or clearer rule IR boundaries incrementally. | Gives up much of the shared-substrate maintenance story and leaves egglog owning the hard database/runtime complexity. | [Option 4](findings/options/option-4-no-dd-backend-borrow-ideas.md) |

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
