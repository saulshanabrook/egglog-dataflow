# Option 3: FlowLog/Datatoad Middle Layer With DD-Overlapped Scheduling

## Viability
- Medium. This remains a large architecture path, but the scheduling story is
  stronger than the earlier separate refinement suggested. The baseline requirement is
  still exact egglog logical scheduling: per-rule seminaive freshness, ruleset
  order, bounded `run`, staged `saturate`, custom scheduler behavior, and native
  rebuild visibility must remain observationally compatible. The DD-specific
  upside is that Timely/DD may overlap physical work across logical iterations
  using multidimensional timestamps and frontiers, rather than forcing egglog's
  current stop/start bulk iteration shape (`messages/eli-dd-overlapped-scheduling.md`,
  `repos/timely-dataflow/mdbook/src/chapter_4/chapter_4_1.md:105`,
  `repos/timely-dataflow/mdbook/src/chapter_3/chapter_3_2.md:3`,
  `repos/differential-dataflow/README.md:172`). The blocker is that this path
  still builds a new relational compiler, adapter layer, index layout,
  recursive-control model, timestamp/frontier design, and rebuild/equality
  invalidation model before proving that the overlap survives egglog's native
  actions and rebuild barriers.

## General Approach
- Build an egglog-specific relational IR between egglog and DD. The IR should
  model rule atoms, equality/canonicalization predicates, filters, projections,
  semijoins, binary joins, WCOJ joins, derived heads, recursion boundaries, and
  a physical schedule distinct from the user-facing egglog schedule.
- Use FlowLog's pipeline as the planner shape: per-rule catalog/preparation,
  SIP/filter pushdown, core join planning, postprocessing, stratum-level
  recursive/non-recursive split, and lowering to Timely/DD iterative scopes
  (`repos/flowlog/README.md`, `repos/flowlog/crates/planner/src/stratum_planner.rs`,
  `repos/flowlog/crates/compiler/src/flow/recursive.rs`).
- Add a WCOJ operator family inspired by datatoad's staged term plans and
  `ExecAtom` count/propose/join loop (`repos/datatoad/src/rules/plan.rs`,
  `repos/datatoad/src/rules/exec.rs`). Keep binary DD joins as the default path
  and choose WCOJ only for cyclic, high-arity, or repeated-variable rule bodies.
- Preserve the current logical schedule exactly. The middle layer may choose a
  different physical schedule only if DD frontiers prove that matches/actions
  for a logical boundary are not made visible too early.
- Treat DD-overlapped scheduling as an optimization of this same middle layer,
  not as a separate option: use DD/Timely multidimensional time to start work
  for later logical iterations while earlier ones are still completing, then
  incrementally finish or correct later work as earlier frontiers advance.
- Keep explicitly relaxed scheduling as a fallback variant only. If exact
  overlap is too constrained or too expensive, a future design could add scoped
  relaxed regions, but that would be a semantic extension rather than the main
  Option 3 hypothesis.

## What Would Move
- DD/Timely: incremental collections, arrangements, iteration scopes, feedback
  variables, consolidation, progress/frontier tracking, and ordinary binary
  joins.
- FlowLog-style middle layer: rule cataloging, planning, SIP/semijoin pushdown,
  join-order choice, recursive stratum metadata, unioning rule heads into IDBs,
  generated DD operator structure, and the adapter logic that turns egglog
  invalidations into dataflow updates.
- DD-overlapped physical scheduler: timestamp/frontier assignment for logical
  ruleset iterations, small physical work units, and visibility gates that
  prevent outputs/actions from crossing egglog schedule boundaries early.
- Datatoad/dataflow-join borrowed ideas: term-introduction plans, adaptive
  count/propose/validate WCOJ kernels, prefix extenders, and trie/columnar or
  multiversion indexes for proposal/validation, but only behind an
  egglog-specific storage layout.
- Egglog backend boundary: translate e-node/e-class/pattern facts into
  relational rows, expose row diffs to the planner, and receive derived
  matches/actions from the dataflow layer.

## What Stays Native
- Union-find/e-class identity, congruence closure, rebuild/canonicalization
  policy, analysis data, extraction, rule action semantics,
  explanations/provenance choices, and user-facing language semantics should
  remain in egglog initially.
- Egglog should own the decision of when a logical schedule boundary starts or
  stops, when native actions become visible, how invalidations are emitted, and
  which canonical facts are exported as DD updates.
- Native code likely still owns custom scheduler APIs that require full-match
  materialization, subset selection, residual-match storage, or delayed action
  firing, unless a DD implementation can reproduce those observations exactly.

## Required Interfaces
- A typed row schema for each egglog relation: constructor/e-node rows, class
  ids, analysis facts, rule-local derived rows, and action outputs.
- A diff protocol from egglog to the middle layer: insertions, retractions,
  representative rewrites, and rebuild-triggered invalidations, with
  timestamps/epochs compatible with DD iteration and an explicit mapping from
  dirty-id refreshes to planner-visible updates.
- A planner ABI for relation providers: keyed lookup, cardinality estimates,
  keyed iteration, arrangement reuse, and optional WCOJ `count/propose/validate`
  support.
- A logical schedule API: ruleset order, bounded `run`, staged `saturate`,
  custom scheduler decisions, per-rule last-run timestamps, and visibility
  barriers.
- A physical schedule API: DD timestamp/frontier assignment for small work
  units, rules for starting later logical iterations early, and frontier gates
  for when later-iteration results can be observed by egglog.
- A seminaive freshness API: represent each rule's logical last-run timestamp
  and each row's insertion/refresh timestamp, or provide an equivalent
  time-window predicate that can be pushed into planned joins.
- A recursive-control API: declare which inputs are stable EDB-like facts, which
  relations are recursive/IDB-like, which outputs feed back, and which outputs
  leave a saturation/iteration scope.
- A join-kernel API that can use either DD arrangements or separate
  trie/columnar indexes; this must be explicit because datatoad's
  `Salad`/`FactLSM<Forest<Terms>>` representation is not a drop-in DD
  collection (`findings/source-notes/datalog-wcoj-planning.md:28-30`).

## Main Risks
- Scope risk: this is effectively a new relational engine layer, not just a DD
  integration. FlowLog's planner/compiler split and datatoad's WCOJ data
  structures are both substantial, and the egglog adapter still needs a custom
  invalidation and row-layout model.
- Exact-overlap risk: DD may be able to start later logical iterations early,
  but native actions, rebuild waves, merge functions, and custom schedulers may
  force frontier gates that collapse the benefit back into stop/start execution.
- Timestamp/progress risk: DD can track rich multidimensional times, but Timely
  progress has overhead per timestamp. Too many physical tasks or frontiers can
  erase the expected parallelism benefit.
- Index risk: WCOJ wants prefix/term-order indexes; DD arrangements are keyed
  binary views. Maintaining both may double memory and update cost.
- Recursive semantics risk: dataflow-join handles timely prefix extenders, but
  not Datalog stratification or egglog saturation control; incremental WCOJ
  inside DD feedback needs careful timestamp/progress semantics.
- Seminaive scheduling risk: physical overlap is wrong if it allows a rule to
  observe facts outside the rows inserted or refreshed since that rule last ran
  under the logical egglog schedule.
- Planner risk: FlowLog's current core planner is still binary-join-oriented
  and the notes call out brittle source-order/left-deep planning as insufficient
  for egglog-sized e-matching (`findings/source-notes/datalog-wcoj-planning.md:27`).
- Equality churn risk: representative changes may appear as mass
  retractions/reinsertions or retained DD history unless native egglog equality
  remains outside the relational layer and the adapter preserves the
  invalidation model explicitly.
- Fallback-semantics risk: if exact overlap is too constrained, adding relaxed
  regions would require a separate user-visible contract and schedule
  eligibility analysis.

## Evidence To Gather
- Lower 3-5 real egglog rules into a relational IR and classify them as
  binary-friendly, cyclic/WCOJ-friendly, repeated-variable, or equality-heavy.
- Prototype one non-recursive e-matching rule with both DD binary joins and a
  datatoad/dataflow-join-style WCOJ kernel; measure intermediates, update cost,
  and index memory.
- Model one rebuild/canonicalization epoch as DD diffs and count how many rows
  retract/reinsert under representative changes.
- Test whether existing DD arrangements can answer WCOJ proposals cheaply
  enough, or whether separate trie/columnar indexes are required.
- Build a tiny recursive stratum with one stable e-node input and one derived
  relation to validate enter/feedback/leave control before trying full
  saturation.
- Prototype the DD-overlapped schedule on one ruleset: compare current
  stop/start bulk iteration, exact non-overlapped middle-layer execution, and
  DD-overlapped physical execution on throughput, operator utilization, progress
  traffic, retained traces, and final semantic equivalence.
- Include the scheduled reachability example as the minimum semantic test for
  schedule lowering and per-rule seminaive freshness.
- Test a custom scheduler/backoff example to measure whether full-match
  materialization forces a barrier.
- Compare the egglog-specific adapter boundary against a simpler DD/native
  split: measure whether the middle layer actually removes enough planner,
  index, and invalidation complexity to justify itself.

## Current Assessment
- This is a coherent architecture sketch, but it is a large system design
  project rather than a small backend substitution. The earlier split is
  better treated as one path: exact middle-layer semantics first, then
  DD-overlapped physical execution as the DD-specific optimization to test.
  Explicit semantic relaxation should remain a fallback experiment, not the
  primary claim. The first useful evidence is whether a small exact middle-layer
  scaffold can demonstrate overlap across logical iterations without exposing
  matches/actions before egglog's logical schedule permits them.
