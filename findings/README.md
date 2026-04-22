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
| [options/README.md](options/README.md) | Tradeoff analysis of the four backend boundary options. |
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
to evaluate before committing to any implementation path. The synthesis now
also treats provider-style relation boundaries, proof-encoding coverage,
scheduler semantics, timestamp/compaction policy, and rebuild invalidation as
first-class evidence gaps rather than separate review notes. Eli's
scaling-equality-saturation draft adds a sharper scheduling constraint:
arbitrary schedules require per-rule seminaive freshness and timestamp-window
table access.

## Option Tradeoff Pass

- [options/README.md](options/README.md): second-pass tradeoff analysis of the four
  backend boundary options from `synthesis.md`.

Current option tradeoff map:

- Option 1 lowers syntax and frontend disruption by keeping equality/rebuild
  native, but depends on a difficult delta contract between egglog and DD,
  including explicit rebuild invalidation, per-rule seminaive freshness, and
  scheduler match selection.
- Option 2 gives a clear relational equality specification, but current
  proof/term encoding evidence raises overhead and cannot validate the full
  Python/container/scheduler frontend surface.
- Option 3 has broad long-term planning upside, but requires a large new
  middle layer, egglog-specific adapter, index layout, and invalidation model
  before the equality/rebuild boundary and schedule/freshness model are proven.
  A new
  [small-iteration scheduling refinement](options/option-3-small-iteration-scheduling-refinement.md)
  asks whether that middle layer should also replace egglog's bulk physical
  ruleset iteration with many smaller DD iterations while preserving per-rule
  timestamp windows.
- Option 4 avoids backend migration risk for existing semantics, but gives less
  maintenance leverage from a shared substrate unless provider-style relation
  boundaries isolate reusable pieces. It is now a stronger baseline because the
  native backend already has timestamp-ordered tables, staged mutation,
  provider hooks, Free Join, and parallel bulk execution.

## Coverage

The first reading pass covers every source cluster listed in the top-level
README: core `egglog`, Python and experimental frontends, Differential/Timely,
FlowLog/datatoad/WCOJ sources, comparative systems, local papers, local
discussion transcripts, the scaling equality saturation draft, and high-signal
blog posts.
