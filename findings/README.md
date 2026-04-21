# Findings

This directory stores the durable research context for deciding whether
`egglog` should move part of its runtime onto Differential Dataflow or a related
dataflow/database substrate.

The top-level README is the short collaborator handoff. This directory is the
index for deeper conclusions, methodology, option analysis, and source notes.

## Entry Points

| Path | Purpose |
| --- | --- |
| [synthesis.md](synthesis.md) | Current consolidated conclusion, continue/stop criteria, and next experiments. |
| [options/README.md](options/README.md) | Ranked analysis of the four backend boundary options. |
| [methodology.md](methodology.md) | Detailed research framing, scientific questions, source inventory, reading order, and design risks. |
| [source-notes/](source-notes/) | First-pass evidence notes by source cluster. |

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

- [synthesis.md](synthesis.md): consolidated evidence, provisional conclusion, and next
  experiments.

Current provisional conclusion: do not attempt a full egglog-on-DD rewrite yet.
Continue with a narrow prototype centered on rule evaluation while keeping
equality maintenance, rebuilding, containers, analyses, and extraction native.

## Option Viability Pass

- [options/README.md](options/README.md): second-pass analysis of the four backend boundary
  options from `synthesis.md`.

Current option ranking:

1. Option 1: native equality plus DD/FlowLog rule evaluation.
2. Option 3: FlowLog/datatoad-like middle layer.
3. Option 2: proof/term encoding to DD.
4. Option 4: no DD backend, borrow ideas as fallback.

## Coverage

The first reading pass covers every source cluster listed in the top-level
README: core `egglog`, Python and experimental frontends, Differential/Timely,
FlowLog/datatoad/WCOJ sources, comparative systems, local papers, local
discussion transcripts, and high-signal blog posts.
