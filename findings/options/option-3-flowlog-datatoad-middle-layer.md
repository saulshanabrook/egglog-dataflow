# Option 3: FlowLog/Datatoad-Like Middle Layer

## Viability
- Medium. This is the most coherent long-term architecture for a DD-backed egglog evaluator, but it is too much surface area for a first implementation. The source notes point to a useful split: FlowLog-style rule planning and recursive control above DD, with datatoad/dataflow-join-style WCOJ kernels for selected multiway joins (`findings/source-notes/datalog-wcoj-planning.md:24-37`). The blocker is that this option builds a new relational compiler, egglog-specific adapter layer, index layout, recursive execution model, and schedule/freshness model before proving that egglog's equality/rebuild lifecycle and per-rule seminaive timestamps map cleanly to DD diffs (`findings/source-notes/datalog-wcoj-planning.md`, `findings/source-notes/extension-models.md`, `findings/source-notes/scaling-equality-saturation.md`).

## General Approach
- Build an egglog-specific relational IR between egglog and DD. The IR should model rule atoms, equality/canonicalization predicates, filters, projections, semijoins, binary joins, WCOJ joins, derived heads, recursion boundaries, and a physical schedule distinct from the user-facing egglog schedule. Use FlowLog's pipeline as the planner shape: per-rule catalog/preparation, SIP/filter pushdown, core join planning, postprocessing, stratum-level recursive/non-recursive split, and lowering to Timely/DD iterative scopes (`repos/flowlog/README.md`, `repos/flowlog/crates/planner/src/stratum_planner.rs`, `repos/flowlog/crates/compiler/src/flow/recursive.rs`). Add a WCOJ operator family inspired by datatoad's staged term plans and `ExecAtom` count/propose/join loop (`repos/datatoad/src/rules/plan.rs`, `repos/datatoad/src/rules/exec.rs`). Treat FlowLog, datatoad, and dataflow-join as shape references rather than reusable engines: the adapter still has to decide egglog row layout, recursive-control boundaries, and how invalidation flows through the planner. Keep binary DD joins as the default path and choose WCOJ only for cyclic, high-arity, or repeated-variable rule bodies.
- Consider the small-iteration scheduling refinement in [Option 3 Refinement: Small-Iteration Scheduling](option-3-small-iteration-scheduling-refinement.md). Instead of preserving the current physical shape where a ruleset iteration runs all eligible rule matches as one batch, the middle layer could split one logical ruleset into many smaller DD iterations or delta tasks. This is attractive only if the split preserves user-visible schedule semantics, per-rule last-run timestamp windows, and rebuild invalidation while actually improving throughput or memory after DD timestamp/progress overhead.

## What Would Move
- DD/Timely: incremental collections, arrangements, iteration scopes, feedback variables, consolidation, and ordinary binary joins.
- FlowLog-style middle layer: rule cataloging, planning, SIP/semijoin pushdown, join-order choice, recursive stratum metadata, unioning rule heads into IDBs, generated DD operator structure, and the adapter logic that turns egglog invalidations into dataflow updates.
- Datatoad/dataflow-join borrowed ideas: term-introduction plans, adaptive count/propose/validate WCOJ kernels, prefix extenders, and trie/columnar or multiversion indexes for proposal/validation, but only behind an egglog-specific storage layout.
- Egglog backend boundary: translate e-node/e-class/pattern facts into relational rows, expose row diffs to the planner, and receive derived matches/actions from the dataflow layer.

## What Stays Native
- Union-find/e-class identity, congruence closure, rebuild/canonicalization policy, analysis data, extraction, rule action semantics, explanations/provenance choices, and user-facing language semantics should remain in egglog initially. The source notes specifically warn that FlowLog, datatoad, and dataflow-join do not model equality maintenance, representative churn, congruence updates, or rebuild effects (`findings/source-notes/datalog-wcoj-planning.md:39-44`). Egglog should also own the decision of when a saturation round starts/stops, how invalidations are emitted, and which canonical facts are exported as DD updates.

## Required Interfaces
- A typed row schema for each egglog relation: constructor/e-node rows, class ids, analysis facts, rule-local derived rows, and action outputs.
- A diff protocol from egglog to the middle layer: insertions, retractions, representative rewrites, and rebuild-triggered invalidations, with timestamps/epochs compatible with DD iteration and with an explicit mapping from dirty-id refreshes to planner-visible updates.
- A planner ABI for relation providers: keyed lookup, cardinality estimates, keyed iteration, arrangement reuse, and optional WCOJ `count/propose/validate` support.
- A schedule-lowering API: separate logical egglog schedule semantics from physical execution units such as strata, rules, pattern roots, delta families, rebuild waves, and micro-iterations.
- A seminaive freshness API: represent each rule's logical last-run timestamp and each row's insertion/refresh timestamp, or provide an equivalent time-window predicate that can be pushed into planned joins.
- A recursive-control API: declare which inputs are stable EDB-like facts, which relations are recursive/IDB-like, which outputs feed back, and which outputs leave a saturation/iteration scope.
- A join-kernel API that can use either DD arrangements or separate trie/columnar indexes; this must be explicit because datatoad's `Salad`/`FactLSM<Forest<Terms>>` representation is not a drop-in DD collection (`findings/source-notes/datalog-wcoj-planning.md:28-30`).

## Main Risks
- Scope risk: this is effectively a new relational engine layer, not just a DD integration. FlowLog's planner/compiler split and datatoad's WCOJ data structures are both substantial, and the egglog adapter still needs a custom invalidation and row-layout model.
- Index risk: WCOJ wants prefix/term-order indexes; DD arrangements are keyed binary views. Maintaining both may double memory and update cost.
- Recursive semantics risk: dataflow-join handles timely prefix extenders, but not Datalog stratification or egglog saturation control; incremental WCOJ inside DD feedback needs careful timestamp/progress semantics.
- Physical scheduling risk: many small DD iterations may improve throughput, but custom schedulers and side-effectful actions may require full-match materialization, subset selection, and delayed action firing that block pipelining.
- Seminaive scheduling risk: splitting a logical ruleset into smaller physical tasks can be wrong if each rule no longer sees exactly the facts inserted or refreshed since that rule last ran.
- Planner risk: FlowLog's current core planner is still binary-join-oriented and the notes call out brittle source-order/left-deep planning as insufficient for egglog-sized e-matching (`findings/source-notes/datalog-wcoj-planning.md:27`).
- Equality churn risk: representative changes may appear as mass retractions/reinsertions or retained DD history unless native egglog equality remains outside the relational layer and the adapter preserves the invalidation model explicitly.
- Maintenance risk: generated DD code for realistic workloads can become large and hand-optimized-looking; the extension notes flag Dynamic Datalog examples as evidence of planning/codegen complexity (`findings/source-notes/extension-models.md:32-34`).

## Evidence To Gather
- Lower 3-5 real egglog rules into a relational IR and classify them as binary-friendly, cyclic/WCOJ-friendly, repeated-variable, or equality-heavy.
- Prototype one non-recursive e-matching rule with both DD binary joins and a datatoad/dataflow-join-style WCOJ kernel; measure intermediates, update cost, and index memory.
- Model one rebuild/canonicalization epoch as DD diffs and count how many rows retract/reinsert under representative changes.
- Test whether existing DD arrangements can answer WCOJ proposals cheaply enough, or whether separate trie/columnar indexes are required.
- Build a tiny recursive stratum with one stable e-node input and one derived relation to validate enter/feedback/leave control before trying full saturation.
- Compare bulk egglog-style physical iteration against many small DD iterations on one ruleset, measuring throughput, operator utilization, progress traffic, retained traces, and semantic equivalence.
- Include Eli's scheduled reachability example as the minimum semantic test for schedule lowering and per-rule seminaive freshness.
- Compare the egglog-specific adapter boundary against a simpler DD/native split: measure whether the middle layer actually removes enough planner, index, and invalidation complexity to justify itself.

## Current Assessment
- This is a coherent architecture sketch, but it is a large system design project rather than a small backend substitution. It should be evaluated by first proving which egglog relations, indexes, providers, and invalidation events belong outside the native backend, then adding FlowLog-like planning or WCOJ kernels around the subset that survives measurement.
