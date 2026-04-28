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
- `messages/eli-dd-overlapped-scheduling.md`: clarification that DD's many-small-iteration strategy may preserve egglog semantics via multidimensional time/frontiers rather than requiring relaxed schedules.
- `repos/scaling-equality-saturation/egglog-new-backend.md`: egglog backend draft covering per-rule seminaive timestamps, timestamp-ordered table layout, Free Join, lazy subsets/indexes, dynamic variable ordering, and binary/bushy join future work.
- `papers/FlowLog Efficient and Extensible Datalog via Incrementality.pdf`: paper text extracted with `uv run --with pypdf`; used for FlowLog's claimed IR/control split, DD substrate, SIP, and robust planning goals.
- `papers/Free Join Unifying Worst-Case Optimal and Traditional Joins.pdf`: paper text extracted with `uv run --with pypdf`; used for WCOJ/binary-join unification and cyclic/acyclic tradeoff framing.
- `papers/leapfrog treejoin.pdf`: paper text extracted with `uv run --with pypdf`; used for triejoin variable-order and worst-case-optimal join context.
- `papers/differentialdataflow.pdf`: paper text extracted with `uv run --with pypdf`; used for DD's nested incremental iteration motivation.
- `papers/Scaling Worst-Case Optimal Datalog to GPUs.pdf`: paper text extracted with `uv run --with pypdf`; used for SRDatalog's GPU WCOJ architecture, flat columnar storage, skew handling, stream-parallel scheduling, and evaluation evidence.

## Key Findings
- Egglog rule evaluation maps best to FlowLog-style planning as the outer architecture: FlowLog explicitly separates per-rule logical plans from recursive control, then lowers planned transformations into DD iterative scopes (`repos/flowlog/crates/planner/src/stratum_planner.rs`, `repos/flowlog/crates/compiler/src/flow/recursive.rs`). That makes FlowLog a planner-shape reference, not a drop-in engine.
- FlowLog is not just "direct DD collections"; it adds Datalog-specific metadata before DD lowering: recursive vs non-recursive transformations, recursion enter/leave collections, accumulative vs iterative recursive relations, IDB-to-head maps, and aggregation metadata (`repos/flowlog/crates/planner/src/stratum_planner.rs`). An egglog backend would need its own adapter and invalidation model around that shape.
- FlowLog also suggests that the physical execution schedule can be part of the substrate design. The local Slack notes report that FlowLog tries to spread work across many small DD iterations so operators can overlap work across iterations, instead of presenting one large batch to the incremental engine (`messages/dec-17-2025-slack.md`). Eli's later clarification makes this more interesting for egglog: the overlap may be a physical execution optimization under the same logical egglog schedule, not a semantic relaxation by default (`messages/eli-dd-overlapped-scheduling.md`).
- FlowLog's current code is useful but not sufficient as a planner: `PlanTree::from_catalog` builds a left-deep chain in source order and `get_first_join_tuple_index` returns `(0, 1)`, with cost-based/heuristic reordering left as future work (`repos/flowlog/crates/optimizer/src/plan_tree.rs`). For egglog, naive binary join ordering is likely too brittle for large e-matching queries.
- Datatoad is the strongest evidence for a WCOJ join kernel: `ExecAtom` exposes count/propose/join, and `wco_join_inner` picks the atom with fewest extensions per prefix, proposes from it, then validates against the others (`repos/datatoad/src/rules/exec.rs`). This is join-kernel inspiration for cyclic/multiway bodies, not a reusable engine.
- Datatoad's representation is a bigger commitment than a plug-in operator: its facts are `Salad`/`FactLSM<Forest<Terms>>` values with column permutation, trie layers, pruning, exchange, and bulk layout changes (`repos/datatoad/src/rules/exec.rs`, `repos/datatoad/src/facts/trie.rs`). Egglog would need an egglog-specific adapter, index layout, and invalidation model to make that work.
- `dataflow-join` shows WCOJ can be expressed as timely/dataflow operators: prefix extenders implement `count`, `propose`, and `intersect`, and `GenericJoin::extend` partitions prefixes by the cheapest proposer before intersection (`repos/dataflow-join/src/lib.rs`). But it is a low-level streaming join library, not a Datalog planner or recursive rule engine, so it is also inspiration rather than drop-in reuse.
- The FlowLog paper argues for a relational IR per rule, Datalog-aware optimizations, SIP, and DD execution; the Free Join paper argues WCOJ and binary joins should be unified rather than treated as mutually exclusive. Together they support a hybrid design: FlowLog-style rule planning/control with datatoad/free-join-style multiway join operators for selected bodies.
- Eli's backend draft adds direct egglog evidence for this hybrid direction: current egglog uses a Free Join variant with lazy subsets, cached hash indexes, fused scans, batching, vectorized actions, morsel-driven parallelism, and dynamic variable ordering. It also identifies binary and bushy plans as future work for common queries where Free Join/GJ has overhead (`repos/scaling-equality-saturation/egglog-new-backend.md`, `findings/source-notes/scaling-equality-saturation.md`).
- SRDatalog strengthens the WCOJ evidence for GPU Datalog specifically. It avoids binary-join intermediate blowups with a WCOJ pipeline over flat sorted Structure-of-Arrays relations, deterministic two-phase count/materialize allocation, root-level histogram load balancing, targeted helper-relation splitting for buried skew, and phase-aligned CUDA stream scheduling (`papers/Scaling Worst-Case Optimal Datalog to GPUs.pdf`, pages 1-8 extracted locally).
- The SRDatalog evaluation reports end-to-end speedups over FlowLog, Souffle, and Ascent on program-analysis workloads and also compares against cuMatch and VFLog on narrower GPU baselines. This makes "GPU WCOJ for recursive Datalog" a demonstrated architecture direction, but not yet an egglog backend result because the evaluated core omits peripheral language features and does not model equality maintenance, rebuild, containers, custom schedulers, or proof/term encoding (`papers/Scaling Worst-Case Optimal Datalog to GPUs.pdf`, pages 8-12 extracted locally).
- The April 24 proof-query note adds a concrete planner requirement: proof
  tables and similar functionally dependent atoms must be represented as
  dependent lookups in the relational IR, not only as ordinary join atoms. In
  the `A/B/C` shape, `C` can depend on bindings produced by `A`; a planner must
  know when `C[t..]` should drive an old/old/new case and when it should be
  late-bound after `A` and `B`.

## Relevance To The Main Objective
- This supports moving egglog onto DD only if the design includes a planning layer above DD collections. DD gives incremental iteration and arrangements, but the inspected systems suggest rule evaluation needs explicit planning for recursion boundaries, arrangements, join order, semijoins, and aggregation-like postprocessing.
- A FlowLog-style substrate is a plausible shape for egglog-on-DD: compile egglog rules/e-matches into a relational IR, stratify or otherwise separate recursive saturation control, then lower to DD transformations. That still leaves adapter, index, and invalidation work specific to egglog.
- The substrate could also own physical schedule lowering: a user-facing egglog ruleset could be split into many smaller DD iterations or delta tasks if DD frontiers preserve logical schedule semantics, per-rule timestamp windows, and rebuild invalidation visibility.
- Datatoad-style WCOJ should be considered a join operator family inside that planner, not the whole substrate. It is most relevant for cyclic/high-arity rule bodies where binary joins create large intermediates.
- SRDatalog suggests what a GPU-oriented WCOJ family would require if Option 3 grows toward GPUs: the planner/storage contract cannot stop at "use WCOJ." It must also choose column orders, maintain flat sorted columns across seminaive deltas, estimate skew enough for histogram partitioning or helper splitting, and expose deterministic output sizing for bulk materialization.
- Direct DD collections are still the execution target, but using them directly as the design abstraction would under-specify rule planning and likely recreate FlowLog's planner/compiler piecemeal.
- Proof-aware planning is part of that missing layer. DD can maintain the
  arrangements and deltas, but egglog-specific rule planning must encode
  functional dependencies, dependent lookup atoms, and cardinality-aware choices
  for seminaive delta cases.

## Likely Blockers
- Egglog rules include equality maintenance, e-class canonicalization, rebuild effects, and congruence-like updates; none of FlowLog, datatoad, or `dataflow-join` directly models that lifecycle, so a separate invalidation channel still needs to be designed.
- FlowLog's current optimizer is too simple for egglog workloads: source-order left-deep joins are unlikely to be robust for e-matching patterns with repeated variables and changing relation cardinalities.
- Datatoad's WCOJ kernel assumes trie/columnar fact containers and term-order-aware execution; integrating it with DD arrangements or egglog relation storage may require substantial data-structure translation and an egglog-specific index layout.
- WCOJ helps multiway joins, but many egglog costs may come from equality/rewrite churn, canonicalization, and maintaining derived relations across rebuilds rather than join asymptotics alone.
- Incremental WCOJ inside recursive DD feedback needs careful timestamp/progress semantics; `dataflow-join` handles indexed stream extenders, but not Datalog stratification, egglog saturation control, or rebuild invalidation.
- DD-overlapped physical scheduling can conflict with egglog custom schedulers and side-effectful actions, because those may require all matches for a logical step before choosing which actions fire. That may force barriers even if the DD substrate can keep some work in flight.
- Per-rule seminaive timestamp windows add an index-planning constraint: timestamp constraints need efficient pushdown before joins, and value-ordered join indexes may need auxiliary time slicing.
- SRDatalog's stream-parallel schedule leans on monotone Datalog convergence and phase-aligned bulk GPU kernels. Egglog cannot assume that rule order is freely interchangeable because bounded schedules, custom scheduler admission, staged actions, rebuild visibility, and container refresh are observable compatibility requirements.
- SRDatalog's flat columnar storage avoids static WCOJ index rebuild overhead, but egglog equality rebuild can rewrite canonical ids inside existing rows. That means a GPU WCOJ storage plan needs an explicit story for canonical-id churn and same-id dirty refresh, not only cheap seminaive delta merges.
- Proof and explanation tables add a second planning constraint: driving from a
  newly fresh proof atom can be correct but wasteful unless cardinality and
  functional-dependency information show that `C[t..]` is selective enough.

## Promising Connections
- Reuse FlowLog's conceptual pipeline: parse/rules -> catalog/IR -> optimizer/planner -> DD compiler, with egglog-specific operators for e-class canonicalization and rebuild-aware relation maintenance.
- Use FlowLog's recursive split as a model for factoring EDB-only or stable e-node relation work outside saturation loops while rerunning only IDB/equality-dependent transformations inside feedback.
- Replace or augment FlowLog's binary join core with a datatoad-like `count/propose/validate` operator for rule bodies that form cyclic joins or have repeated shared variables.
- Borrow SRDatalog's GPU-specific WCOJ lessons as design constraints for any GPU path: two-phase deterministic allocation, flat sorted columns, histogram-guided skew partitioning, targeted helper-relation splitting, and stream multiplexing only where egglog's logical schedule permits it.
- Use SIP/semijoin filtering from FlowLog before expensive e-matches, especially when one atom has selective bindings from a pattern root, symbol, or e-class id.
- Add proof-aware dependent lookup to the planner IR so atoms keyed by earlier
  bindings can be late-bound instead of always participating as independent
  delta drivers.
- Use Free Join's framing as the long-term target: a unified planner that can choose binary joins, semijoins, or WCOJ without forcing all rules into one execution style.
- Use current egglog Free Join as the baseline when evaluating DD/FlowLog/datatoad plans; the comparison should include dynamic variable ordering and binary/bushy alternatives, not only source-order binary joins.

## Evidence Needed Next
- Build 3-5 concrete egglog rule/e-match patterns as relational queries and classify them as acyclic, cyclic, repeated-variable, or equality-heavy.
- Measure binary DD join plans vs a WCOJ prototype on at least one cyclic e-matching query with realistic e-class/e-node cardinalities.
- Inspect how egglog rebuild/canonicalization updates would appear as DD diffs: relation updates only, key rewrites, or full retractions/reinsertions.
- Prototype a minimal FlowLog-like relational IR for egglog atoms and check whether every needed rule side condition can become map/filter/join/antijoin/WCOJ.
- Determine whether DD arrangements can serve as the backing indexes for WCOJ proposals, or whether trie/COLT-style indexes must be maintained separately.
- Compare a bulk ruleset run against a DD-overlapped physical schedule on the same egglog rule cluster, measuring throughput, progress traffic, retained trace state, per-rule freshness, final e-graph equivalence, and whether later-iteration outputs/actions are gated until the logical schedule permits them.
- Add the scheduled reachability example from Eli's draft to rule-planner tests so any FlowLog/DD lowering proves per-rule timestamp freshness before measuring speed.
- Recast one SRDatalog-style workload shape as an egglog rule body and separately classify whether its cost is dominated by WCOJ, equality rebuild, primitive actions, containers, or scheduler materialization.
- Check whether an egglog WCOJ prototype can maintain SRDatalog-style flat sorted columns after canonical-id rewrites and same-id container refreshes without rebuilding most relation storage.
- Add the April 24 `A/B/C` proof-query shape as a planner benchmark. Compare
  naive seminaive delta expansion against functional-dependency-aware dependent
  lookup, including the old/old/new case where `C[t..]` is the only new input.

## Local Experiment Update
- The rule-classification lane separates fair join/planner targets from
  rebuild/container/scheduler stressors: `line_graph_1`, `line_graph_2`, and
  `intersection` are the current native join baselines, while AC/rebuild and
  container cases should not be treated as fair WCOJ comparisons
  (`findings/experiments/option-3/README.md`).
- Native `Gj`, `PureSize`, and `MinCover` strategy smoke tests passed on the
  representative line graph and intersection workloads. These are correctness
  checks, not a plan-efficiency benchmark.
- The `dataflow-join` WCOJ examples compile, but runtime measurement is blocked
  in this checkout because no graph input dataset is mounted. WCOJ remains a
  component follow-up for the local prototype. The new SRDatalog paper supplies
  external GPU Datalog runtime evidence, but not a replacement for an
  egglog-specific WCOJ/equality/container/scheduler measurement.

## Confidence
- Medium: local code and PDF extraction strongly support the architectural comparison, but no egglog workload was measured against these join strategies in this pass.
