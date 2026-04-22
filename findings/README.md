# Findings

This directory stores the durable research context for deciding whether
`egglog` should move part of its runtime onto Differential Dataflow or a related
dataflow/database substrate.

The top-level README is the short collaborator handoff. This directory is the
index for deeper conclusions, methodology, option analysis, and source notes.

## Entry Points

| Path | Purpose |
| --- | --- |
| [synthesis.md](synthesis.md) | Consolidated evidence, continue/stop criteria, and possible next evidence-gathering work. |
| [option-3-experiments.md](option-3-experiments.md) | Runnable Option 3 scheduling-overlap experiment results and verdict. |
| [options/README.md](options/README.md) | Tradeoff analysis of the backend boundary paths. |
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
| Scaling Equality Saturation | `source-notes/scaling-equality-saturation.md` | Complete |

## Synthesis

- [synthesis.md](synthesis.md): consolidated evidence, provisional assessment, and possible
  evidence-gathering work.

Current provisional assessment: no backend path has been selected. The evidence
is strongest as a map of tradeoffs: DD/FlowLog/datatoad may help with relational
matching and planning, while equality maintenance, rebuilding, containers,
schedules, extension APIs, and frontend compatibility remain the main blockers
to evaluate before committing to any implementation path. The first Option 3
experiment shows that DD-overlapped physical scheduling can preserve per-rule
freshness on the scheduled reachability witness, but synthetic native barriers
and mixed scaling results keep Option 3 as a promising but constrained path.

## Option Tradeoff Pass

- [options/README.md](options/README.md): tradeoff analysis of the backend
  boundary paths from `synthesis.md`.

Current option tradeoff map, ordered by increasing complexity and disruption:

- Native improvement / borrow ideas avoids backend migration risk for existing
  semantics, but gives less maintenance leverage from a shared substrate unless
  provider-style relation boundaries isolate reusable pieces. It is now a
  stronger baseline because the native backend already has timestamp-ordered
  tables, staged mutation, provider hooks, Free Join, and parallel bulk
  execution.
- Exact hybrid DD rule evaluation lowers syntax and frontend disruption by
  keeping equality/rebuild native, but depends on a difficult delta contract
  between egglog and DD, including explicit rebuild invalidation, per-rule
  seminaive freshness, and scheduler match selection.
- [Option 3: FlowLog/datatoad middle layer with DD-overlapped scheduling](options/option-3-flowlog-datatoad-middle-layer.md)
  now has a small positive semantic result: exact per-rule freshness and gated
  visibility survived DD-overlapped physical execution on reachability. It still
  requires a large new middle layer, egglog-specific adapter, index layout,
  invalidation model, and evidence that native actions, rebuild, and custom
  schedulers do not collapse the overlap back into barriers.
- Proof/term encoding gives a clear relational equality specification, but
  current evidence raises overhead concerns and cannot validate the full
  Python/container/scheduler frontend surface.

## Coverage

The first reading pass covers every source cluster listed in
`methodology.md`: core `egglog`, Python and experimental frontends,
Differential/Timely, FlowLog/datatoad/WCOJ sources, comparative systems, local
papers, local discussion transcripts, the scaling equality saturation draft,
and high-signal blog posts.
