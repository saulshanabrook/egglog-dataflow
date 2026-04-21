# Option 3: FlowLog/Datatoad-Like Middle Layer

## Viability
- Medium. This is the most coherent long-term architecture for a DD-backed egglog evaluator, but it is too much surface area for a first implementation. The source notes point to a useful split: FlowLog-style rule planning and recursive control above DD, with datatoad/dataflow-join-style WCOJ kernels for selected multiway joins (`findings/source-notes/datalog-wcoj-planning.md:24-37`). The blocker is that this option builds a new relational compiler, index provider layer, and recursive execution model before proving that egglog's equality/rebuild lifecycle maps cleanly to DD diffs (`findings/source-notes/datalog-wcoj-planning.md:39-44`, `findings/source-notes/extension-models.md:41-46`).

## General Approach
- Build an egglog-specific relational IR between egglog and DD. The IR should model rule atoms, equality/canonicalization predicates, filters, projections, semijoins, binary joins, WCOJ joins, derived heads, and recursion boundaries. Use FlowLog's pipeline as the planner shape: per-rule catalog/preparation, SIP/filter pushdown, core join planning, postprocessing, stratum-level recursive/non-recursive split, and lowering to Timely/DD iterative scopes (`repos/flowlog/README.md`, `repos/flowlog/crates/planner/src/stratum_planner.rs`, `repos/flowlog/crates/compiler/src/flow/recursive.rs`). Add a WCOJ operator family inspired by datatoad's staged term plans and `ExecAtom` count/propose/join loop (`repos/datatoad/src/rules/plan.rs`, `repos/datatoad/src/rules/exec.rs`). Keep binary DD joins as the default path and choose WCOJ only for cyclic, high-arity, or repeated-variable rule bodies.

## What Would Move
- DD/Timely: incremental collections, arrangements, iteration scopes, feedback variables, consolidation, and ordinary binary joins.
- FlowLog-style middle layer: rule cataloging, planning, SIP/semijoin pushdown, join-order choice, recursive stratum metadata, unioning rule heads into IDBs, and generated DD operator structure.
- Datatoad/dataflow-join borrowed ideas: term-introduction plans, adaptive count/propose/validate WCOJ kernels, prefix extenders, and trie/columnar or multiversion indexes for proposal/validation.
- Egglog backend boundary: translate e-node/e-class/pattern facts into relational rows, expose row diffs to the planner, and receive derived matches/actions from the dataflow layer.

## What Stays Native
- Union-find/e-class identity, congruence closure, rebuild/canonicalization policy, analysis data, extraction, rule action semantics, explanations/provenance choices, and user-facing language semantics should remain in egglog initially. The source notes specifically warn that FlowLog, datatoad, and dataflow-join do not model equality maintenance, representative churn, congruence updates, or rebuild effects (`findings/source-notes/datalog-wcoj-planning.md:39-44`). Egglog should also own the decision of when a saturation round starts/stops and which canonical facts are exported as DD updates.

## Required Interfaces
- A typed row schema for each egglog relation: constructor/e-node rows, class ids, analysis facts, rule-local derived rows, and action outputs.
- A diff protocol from egglog to the middle layer: insertions, retractions, representative rewrites, and rebuild-triggered invalidations, with timestamps/epochs compatible with DD iteration.
- A planner ABI for relation providers: keyed lookup, cardinality estimates, keyed iteration, arrangement reuse, and optional WCOJ `count/propose/validate` support.
- A recursive-control API: declare which inputs are stable EDB-like facts, which relations are recursive/IDB-like, which outputs feed back, and which outputs leave a saturation/iteration scope.
- A join-kernel API that can use either DD arrangements or separate trie/columnar indexes; this must be explicit because datatoad's `Salad`/`FactLSM<Forest<Terms>>` representation is not a drop-in DD collection (`findings/source-notes/datalog-wcoj-planning.md:28-30`).

## Main Risks
- Scope risk: this is effectively a new relational engine layer, not just a DD integration. FlowLog's planner/compiler split and datatoad's WCOJ data structures are both substantial.
- Index risk: WCOJ wants prefix/term-order indexes; DD arrangements are keyed binary views. Maintaining both may double memory and update cost.
- Recursive semantics risk: dataflow-join handles timely prefix extenders, but not Datalog stratification or egglog saturation control; incremental WCOJ inside DD feedback needs careful timestamp/progress semantics.
- Planner risk: FlowLog's current core planner is still binary-join-oriented and the notes call out brittle source-order/left-deep planning as insufficient for egglog-sized e-matching (`findings/source-notes/datalog-wcoj-planning.md:27`).
- Equality churn risk: representative changes may appear as mass retractions/reinsertions or retained DD history unless native egglog equality remains outside the relational layer.
- Maintenance risk: generated DD code for realistic workloads can become large and hand-optimized-looking; the extension notes flag Dynamic Datalog examples as evidence of planning/codegen complexity (`findings/source-notes/extension-models.md:32-34`).

## Evidence To Gather
- Lower 3-5 real egglog rules into a relational IR and classify them as binary-friendly, cyclic/WCOJ-friendly, repeated-variable, or equality-heavy.
- Prototype one non-recursive e-matching rule with both DD binary joins and a datatoad/dataflow-join-style WCOJ kernel; measure intermediates, update cost, and index memory.
- Model one rebuild/canonicalization epoch as DD diffs and count how many rows retract/reinsert under representative changes.
- Test whether existing DD arrangements can answer WCOJ proposals cheaply enough, or whether separate trie/columnar indexes are required.
- Build a tiny recursive stratum with one stable e-node input and one derived relation to validate enter/feedback/leave control before trying full saturation.

## Recommendation
- Defer. Continue investigating it as the likely long-term architecture, but do not make it the first implementation target. A first pass should prove the smaller DD boundary and equality/rebuild diff story, then add a FlowLog-like planner around the successful subset. WCOJ should enter as a measured join-kernel experiment for specific cyclic/high-arity e-matching rules, not as a mandatory foundation.
