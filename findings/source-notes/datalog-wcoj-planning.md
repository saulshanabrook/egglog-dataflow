# Datalog/WCOJ/Planning Systems

## Sources Read
- `repos/flowlog/README.md`: high-level architecture for Datalog parsing, stratification, planning, optimization, and DD code generation.
- `repos/flowlog/crates/planner/src/stratum_planner.rs`: stratum-level planning phases, recursive/non-recursive split, IDB head maps, aggregation metadata, and recursion enter/leave metadata.
- `repos/flowlog/crates/planner/src/rule_planner/core.rs`: binary join planning over positive atoms with key/value layouts.
- `repos/flowlog/crates/planner/src/rule_planner/sip.rs`: FlowLog's current sideways-information-passing implementation.
- `repos/flowlog/crates/optimizer/src/plan_tree.rs`: current join-order implementation, a left-deep source-order chain with future cost-based work noted.
- `repos/flowlog/crates/compiler/src/flow/non_recursive.rs`: lowering planned transformations into DD collections, arrangements, unions, dedup, and aggregation.
- `repos/flowlog/crates/compiler/src/flow/recursive.rs`: lowering recursive strata into DD iterative scopes, entered collections, recursive variables, unions, feedback, and leave outputs.
- `repos/flowlog/crates/compiler/src/transformation.rs`: generated DD operators for maps, joins, antijoins, and arrangements.
- `repos/datatoad/mdbook/src/introduction.md`: design summary for columnar dispatch, sorted-column tries, WCOJ, and logic relations.
- `repos/datatoad/src/rules/plan.rs`: datatoad rule plans as stages of atoms/introduced terms/output order, with `ByTerm` and `ByAtom` strategies.
- `repos/datatoad/src/rules/exec.rs`: WCOJ execution traits and adaptive count/propose/validate join loop.
- `repos/datatoad/src/facts/trie.rs`: trie/forest representation built from sorted columns and prefix layers.
- `repos/dataflow-join/src/lib.rs`: streaming incremental WCOJ API and generic-join prefix extension.
- `repos/dataflow-join/src/extender.rs`: indexed stream extender implementation for count/propose/intersect over timely streams.
- `repos/dataflow-join/src/index.rs`: multiversion keyed index and LSM-like update storage used by `dataflow-join`.
- `messages/dec-17-2025-slack.md`: Yihong/Hangdong notes on FlowLog, DD parallelism through many small iterations, nested fixpoints, and schedule questions.
- `papers/FlowLog Efficient and Extensible Datalog via Incrementality.pdf`: paper text extracted with `uv run --with pypdf`; used for FlowLog's claimed IR/control split, DD substrate, SIP, and robust planning goals.
- `papers/Free Join Unifying Worst-Case Optimal and Traditional Joins.pdf`: paper text extracted with `uv run --with pypdf`; used for WCOJ/binary-join unification and cyclic/acyclic tradeoff framing.
- `papers/leapfrog treejoin.pdf`: paper text extracted with `uv run --with pypdf`; used for triejoin variable-order and worst-case-optimal join context.
- `papers/differentialdataflow.pdf`: paper text extracted with `uv run --with pypdf`; used for DD's nested incremental iteration motivation.

## Key Findings
- Egglog rule evaluation maps best to FlowLog-style planning as the outer architecture: FlowLog explicitly separates per-rule logical plans from recursive control, then lowers planned transformations into DD iterative scopes (`repos/flowlog/crates/planner/src/stratum_planner.rs`, `repos/flowlog/crates/compiler/src/flow/recursive.rs`). That makes FlowLog a planner-shape reference, not a drop-in engine.
- FlowLog is not just "direct DD collections"; it adds Datalog-specific metadata before DD lowering: recursive vs non-recursive transformations, recursion enter/leave collections, accumulative vs iterative recursive relations, IDB-to-head maps, and aggregation metadata (`repos/flowlog/crates/planner/src/stratum_planner.rs`). An egglog backend would need its own adapter and invalidation model around that shape.
- FlowLog also suggests that the physical execution schedule can be part of the substrate design. The local Slack notes report that FlowLog tries to spread work across many small DD iterations so operators can overlap work across iterations, instead of presenting one large batch to the incremental engine (`messages/dec-17-2025-slack.md`). For egglog, Option 3 could lower a logical ruleset into smaller physical schedule units rather than preserving the current bulk iteration shape.
- FlowLog's current code is useful but not sufficient as a planner: `PlanTree::from_catalog` builds a left-deep chain in source order and `get_first_join_tuple_index` returns `(0, 1)`, with cost-based/heuristic reordering left as future work (`repos/flowlog/crates/optimizer/src/plan_tree.rs`). For egglog, naive binary join ordering is likely too brittle for large e-matching queries.
- Datatoad is the strongest evidence for a WCOJ join kernel: `ExecAtom` exposes count/propose/join, and `wco_join_inner` picks the atom with fewest extensions per prefix, proposes from it, then validates against the others (`repos/datatoad/src/rules/exec.rs`). This is join-kernel inspiration for cyclic/multiway bodies, not a reusable engine.
- Datatoad's representation is a bigger commitment than a plug-in operator: its facts are `Salad`/`FactLSM<Forest<Terms>>` values with column permutation, trie layers, pruning, exchange, and bulk layout changes (`repos/datatoad/src/rules/exec.rs`, `repos/datatoad/src/facts/trie.rs`). Egglog would need an egglog-specific adapter, index layout, and invalidation model to make that work.
- `dataflow-join` shows WCOJ can be expressed as timely/dataflow operators: prefix extenders implement `count`, `propose`, and `intersect`, and `GenericJoin::extend` partitions prefixes by the cheapest proposer before intersection (`repos/dataflow-join/src/lib.rs`). But it is a low-level streaming join library, not a Datalog planner or recursive rule engine, so it is also inspiration rather than drop-in reuse.
- The FlowLog paper argues for a relational IR per rule, Datalog-aware optimizations, SIP, and DD execution; the Free Join paper argues WCOJ and binary joins should be unified rather than treated as mutually exclusive. Together they support a hybrid design: FlowLog-style rule planning/control with datatoad/free-join-style multiway join operators for selected bodies.

## Relevance To The Main Objective
- This supports moving egglog onto DD only if the design includes a planning layer above DD collections. DD gives incremental iteration and arrangements, but the inspected systems suggest rule evaluation needs explicit planning for recursion boundaries, arrangements, join order, semijoins, and aggregation-like postprocessing.
- A FlowLog-style substrate is a plausible shape for egglog-on-DD: compile egglog rules/e-matches into a relational IR, stratify or otherwise separate recursive saturation control, then lower to DD transformations. That still leaves adapter, index, and invalidation work specific to egglog.
- The substrate could also own physical schedule lowering: a user-facing egglog ruleset could be split into many smaller DD iterations or delta tasks if the split preserves logical schedule semantics and rebuild invalidation.
- Datatoad-style WCOJ should be considered a join operator family inside that planner, not the whole substrate. It is most relevant for cyclic/high-arity rule bodies where binary joins create large intermediates.
- Direct DD collections are still the execution target, but using them directly as the design abstraction would under-specify rule planning and likely recreate FlowLog's planner/compiler piecemeal.

## Likely Blockers
- Egglog rules include equality maintenance, e-class canonicalization, rebuild effects, and congruence-like updates; none of FlowLog, datatoad, or `dataflow-join` directly models that lifecycle, so a separate invalidation channel still needs to be designed.
- FlowLog's current optimizer is too simple for egglog workloads: source-order left-deep joins are unlikely to be robust for e-matching patterns with repeated variables and changing relation cardinalities.
- Datatoad's WCOJ kernel assumes trie/columnar fact containers and term-order-aware execution; integrating it with DD arrangements or egglog relation storage may require substantial data-structure translation and an egglog-specific index layout.
- WCOJ helps multiway joins, but many egglog costs may come from equality/rewrite churn, canonicalization, and maintaining derived relations across rebuilds rather than join asymptotics alone.
- Incremental WCOJ inside recursive DD feedback needs careful timestamp/progress semantics; `dataflow-join` handles indexed stream extenders, but not Datalog stratification, egglog saturation control, or rebuild invalidation.
- Small-iteration scheduling can conflict with egglog custom schedulers and side-effectful actions, because those may require all matches for a logical step before choosing which actions fire.

## Promising Connections
- Reuse FlowLog's conceptual pipeline: parse/rules -> catalog/IR -> optimizer/planner -> DD compiler, with egglog-specific operators for e-class canonicalization and rebuild-aware relation maintenance.
- Use FlowLog's recursive split as a model for factoring EDB-only or stable e-node relation work outside saturation loops while rerunning only IDB/equality-dependent transformations inside feedback.
- Replace or augment FlowLog's binary join core with a datatoad-like `count/propose/validate` operator for rule bodies that form cyclic joins or have repeated shared variables.
- Use SIP/semijoin filtering from FlowLog before expensive e-matches, especially when one atom has selective bindings from a pattern root, symbol, or e-class id.
- Use Free Join's framing as the long-term target: a unified planner that can choose binary joins, semijoins, or WCOJ without forcing all rules into one execution style.

## Evidence Needed Next
- Build 3-5 concrete egglog rule/e-match patterns as relational queries and classify them as acyclic, cyclic, repeated-variable, or equality-heavy.
- Measure binary DD join plans vs a WCOJ prototype on at least one cyclic e-matching query with realistic e-class/e-node cardinalities.
- Inspect how egglog rebuild/canonicalization updates would appear as DD diffs: relation updates only, key rewrites, or full retractions/reinsertions.
- Prototype a minimal FlowLog-like relational IR for egglog atoms and check whether every needed rule side condition can become map/filter/join/antijoin/WCOJ.
- Determine whether DD arrangements can serve as the backing indexes for WCOJ proposals, or whether trie/COLT-style indexes must be maintained separately.
- Compare a bulk ruleset run against a small-iteration DD physical schedule on the same egglog rule cluster, measuring throughput, progress traffic, retained trace state, and final e-graph equivalence.

## Confidence
- Medium: local code and PDF extraction strongly support the architectural comparison, but no egglog workload was measured against these join strategies in this pass.
