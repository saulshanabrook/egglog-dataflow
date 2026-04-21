# Findings

This directory stores durable research notes for deciding whether `egglog`
should be moved onto Differential Dataflow or a related dataflow/database
substrate.

The README remains the project orientation. These files preserve the reading
evidence and synthesis context for later planning.

## Reading Status

| Workstream | Note | Status |
| --- | --- | --- |
| Egglog Core + Proof Encoding | `source-notes/egglog-core-proof.md` | Complete |
| Containers + Frontends | `source-notes/containers-frontends.md` | Complete |
| Differential/Timely Substrate | `source-notes/differential-timely.md` | Complete |
| Datalog/WCOJ/Planning Systems | `source-notes/datalog-wcoj-planning.md` | Complete |
| Comparative Extension Models | `source-notes/extension-models.md` | Complete |
| Conversations + Social Motivation | `source-notes/conversations-social.md` | Complete |

## Synthesis

- `synthesis.md`: consolidated evidence, provisional conclusion, and next
  experiments.

Current provisional conclusion: do not attempt a full egglog-on-DD rewrite yet.
Continue with a narrow prototype centered on rule evaluation while keeping
equality maintenance, rebuilding, containers, analyses, and extraction native.

## Coverage

The first reading pass covers every source cluster listed in the top-level
README: core `egglog`, Python and experimental frontends, Differential/Timely,
FlowLog/datatoad/WCOJ sources, comparative systems, local papers, local
discussion transcripts, and high-signal blog posts.
