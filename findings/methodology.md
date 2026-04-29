# Source Inventory

This file is only a source inventory and reading map. Current conclusions live
in [`synthesis.md`](synthesis.md), and claim provenance lives in
[`evidence-ledger.md`](evidence-ledger.md).

## Phase-Setting Source

| Path | Role |
| --- | --- |
| `../messages/eli-meeting-april-29-2026.md` | Sets the current external-DD-trial framing: simple subset, new crate/model outside egglog, egglog oracle, benchmark categories, and mapping-first performance interpretation. |

## Conversations

| Path | Role |
| --- | --- |
| `../messages/oct-15-2024-zulip.md` | Early DD/e-graph discussion: EqSat-in-DD prototype, union-find and congruence questions, nested scopes, WCOJ, equality retractions, and analysis stratification. |
| `../messages/dec-17-2025-slack.md` | Follow-up FlowLog/datatoad/DD notes: columnar representation, many small DD iterations, nested fixed points, arbitrary schedules, and custom tables. |
| `../messages/eli-scheduling-seminaive.md` | Preserves Eli's note that arbitrary schedules plus seminaive evaluation are a major egglog design constraint. |
| `../messages/eli-dd-overlapped-scheduling.md` | Clarifies that DD may overlap physical work while preserving egglog's logical schedule semantics through multidimensional time/frontiers. |
| `../messages/april-24-2026-meeting.md` | Notes around DD maintained views, product timestamps, proof-aware dependent lookup, and `loop/fixpoint` as an Option 3-dependent research question. |

## Papers

| Path | Role |
| --- | --- |
| `../papers/egg Fast and Extensible Equality Saturation.pdf` | Core e-graph/rebuild background. |
| `../papers/Better Together Unifying Datalog and Equality Saturation.pdf` | Core egglog language model: Datalog plus equality saturation. |
| `../papers/relational e-matching.pdf` | Direct bridge between e-matching and database join processing. |
| `../papers/differentialdataflow.pdf` | Differential computation model, partially ordered versions, and nested iteration. |
| `../papers/FlowLog Efficient and Extensible Datalog via Incrementality.pdf` | Datalog-to-Timely/DD architecture reference. |
| `../papers/DBSP.pdf` | Adjacent incremental view maintenance model with automatic differentiation of programs. |
| `../papers/Distributed Evaluation of Subgraph Queries.pdf` | Dataflow-style WCOJ / GenericJoin reference. |
| `../papers/Free Join Unifying Worst-Case Optimal and Traditional Joins.pdf` | Hybrid join-planning reference. |
| `../papers/leapfrog treejoin.pdf` | Baseline WCOJ algorithm context. |
| `../papers/Scaling Worst-Case Optimal Datalog to GPUs.pdf` | SRDatalog GPU WCOJ evidence and constraints. |
| `../papers/Parallel and Customizable Equality Saturation.pdf` | Parallel EqSat comparison point independent of DD. |
| `../papers/Seamless DeductiveInference via Macros.pdf` | Ascent/BYODS reference for custom relation providers. |

## Vendored Repositories

| Path | Role |
| --- | --- |
| `../repos/egglog` | Implementation target and native oracle. Key areas: `src/`, `core-relations/`, `egglog-bridge/`, `union-find/`, `src/proofs/`, and tests. |
| `../repos/scaling-equality-saturation` | Eli's backend design draft for schedules, per-rule timestamps, table layout, Free Join, and DD as future work. |
| `../repos/egglog-python` | Python API and strongest local evidence for containers and higher-order container operations. |
| `../repos/egglog-experimental` | Extension surface: parser sugar, custom schedulers, dynamic costs, fresh values, and multi-extraction. |
| `../repos/egglog-tutorial` | Compact language examples for Datalog, analyses, scheduling, extraction, and case studies. |
| `../repos/differential-dataflow` | DD collection, arrangement, join, reduce, trace, and iteration implementation. |
| `../repos/timely-dataflow` | Timely progress, frontiers, nested scopes, product timestamps, scheduling, and container support. |
| `../repos/flowlog` | Datalog parser/planner/compiler architecture on Timely/DD. |
| `../repos/ascent` | Embedded Datalog and BYODS/custom-provider comparison point. |
| `../repos/dynamic-datalog` | Dynamic Datalog workloads and hand-written DD comparisons. |
| `../repos/dataflow-join` | Streaming WCOJ / GenericJoin implementation in Timely. |
| `../repos/datatoad` | Columnar WCOJ Datalog execution reference. |
| `../repos/columnar` | Columnar container library relevant to DD/Timely data layout. |
| `../repos/blog` | Frank McSherry blog archive for DD, WCOJ, datatoad, columnar, and e-graph context. |

## Prototypes And Artifacts

| Path | Role |
| --- | --- |
| `../code/option3-overlap/` | Schedule-overlap prototype used by the Option 3 lanes. |
| `../code/dd-design-spike/` | DD lifecycle/rebuild/scheduler control-plane prototypes. Evidence only, not an active backend plan. |
| `../findings/artifacts/option-3/` | Numbered Option 3 lane artifacts. |
| `../findings/artifacts/dd-full-refactor/` | DD design-spike artifacts used for durable control-plane facts. |

## Source Notes

| Path | Role |
| --- | --- |
| `source-notes/egglog-core-proof.md` | Core egglog, proof encoding, rebuild, union-find, and oracle boundary evidence. |
| `source-notes/containers-frontends.md` | Python containers, higher-order operations, scheduler/frontend compatibility evidence. |
| `source-notes/differential-timely.md` | DD/Timely substrate evidence: arrangements, traces, iteration, progress, and compaction. |
| `source-notes/datalog-wcoj-planning.md` | FlowLog, datatoad, WCOJ, dataflow-join, SRDatalog, and planning evidence. |
| `source-notes/scaling-equality-saturation.md` | Eli backend draft evidence for schedules, seminaive freshness, native Free Join, and DD future work. |
| `source-notes/extension-models.md` | Comparative extension/provider models. |
| `source-notes/conversations-social.md` | Social and collaboration motivation from local conversations. |

## Historical Synthesis

Superseded backend-option and scaffold planning docs are archived under
`archive/2026-04-prior-backend-plans/`. Use the archive to recover history, but
use the active ledger for current conclusions.

## High-Signal Reading Order

1. `../messages/eli-meeting-april-29-2026.md`
2. `minimal-dd-trial.md`
3. `evidence-ledger.md`
4. `source-notes/scaling-equality-saturation.md`
5. `source-notes/egglog-core-proof.md`
6. `source-notes/containers-frontends.md`
7. `source-notes/differential-timely.md`
8. `source-notes/datalog-wcoj-planning.md`
9. `experiments/option-3/README.md`
