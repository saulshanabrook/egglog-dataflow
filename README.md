# Egglog on Differential Dataflow?

This repository is a source bundle for deciding whether `egglog` should move
some of its database-like runtime onto Differential Dataflow, FlowLog,
datatoad, or a nearby substrate. The possible upside is shared performance and
maintenance work with a serious dataflow/database ecosystem; the risk is that
egglog's equality maintenance, rebuilding, containers, schedules, and frontend
semantics are too specialized for that move to pay off.

## Current Conclusion

- Do not attempt a full egglog-on-DD rewrite yet.
- First experiment: native equality plus DD/FlowLog rule evaluation.
- Keep equality maintenance, rebuilding, containers, analyses, extraction, and
  user-facing syntax native for now.
- Treat DD/FlowLog/datatoad as plausible substrates for maintained relational
  matching, arrangements, and join planning, not yet as replacements for the
  whole egglog backend.

## Backend Options

| Option | Status | Recommendation | Link |
| --- | --- | --- | --- |
| Native equality + DD/FlowLog rule evaluation | Most plausible first experiment | Continue | [Option 1](findings/options/option-1-native-equality-dd-rule-eval.md) |
| FlowLog/datatoad-like middle layer | Coherent long-term architecture | Defer | [Option 3](findings/options/option-3-flowlog-datatoad-middle-layer.md) |
| Proof/term encoding to DD | Useful research/specification path | Defer | [Option 2](findings/options/option-2-proof-term-encoding-dd.md) |
| No DD backend, borrow ideas | Fallback/native improvement path | Continue in parallel | [Option 4](findings/options/option-4-no-dd-backend-borrow-ideas.md) |

## How To Pick This Up

- Read [findings/synthesis.md](findings/synthesis.md) for the current conclusion and stop/continue
  criteria.
- Read [findings/options/README.md](findings/options/README.md) for the ranked option summary.
- If implementing next, start from Option 1's evidence-to-gather list.
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
