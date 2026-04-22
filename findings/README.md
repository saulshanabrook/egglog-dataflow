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
to evaluate before committing to any implementation path. The synthesis now
also treats provider-style relation boundaries, proof-encoding coverage,
scheduler semantics, timestamp/compaction policy, and rebuild invalidation as
first-class evidence gaps rather than separate review notes. Eli's
scaling-equality-saturation draft adds a sharper scheduling constraint:
arbitrary schedules require per-rule seminaive freshness and timestamp-window
table access.

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
- [Option 3b: relaxed small-iteration scheduling](options/option-3b-relaxed-small-iteration-scheduling.md)
  asks whether explicitly relaxed regions could replace egglog's bulk physical
  ruleset iteration with many smaller DD iterations. It may fit DD better, but
  it changes the schedule contract and must be scoped away from programs that
  rely on exact `run`, staged `saturate`, blowup control, manual
  stratification, or custom scheduler behavior.
- [Option 3a: exact FlowLog/datatoad middle layer](options/option-3-flowlog-datatoad-middle-layer.md)
  has broad long-term planning upside, but requires a large new middle layer,
  egglog-specific adapter, index layout, invalidation model, and exact
  schedule/freshness model before the equality/rebuild boundary is proven.
- Proof/term encoding gives a clear relational equality specification, but
  current evidence raises overhead concerns and cannot validate the full
  Python/container/scheduler frontend surface.

## Coverage

The first reading pass covers every source cluster listed in
`methodology.md`: core `egglog`, Python and experimental frontends,
Differential/Timely, FlowLog/datatoad/WCOJ sources, comparative systems, local
papers, local discussion transcripts, the scaling equality saturation draft,
and high-signal blog posts.
