# Synthesis

This is a provisional synthesis of the first sub-agent reading pass. It should
be treated as a decision aid, not as a final backend design.

## Decision Frame

The investigation should continue only if a small prototype can show that a
DD-related substrate reduces complexity or improves maintainability without
losing egglog's core semantics.

Evidence to continue:

- A native-equality plus DD/FlowLog-style rule-evaluation prototype can preserve
  rebuild semantics, custom scheduler behavior, and containers while moving
  enough relational matching/indexing work out of egglog to justify the
  dependency.
- DD arrangements or a FlowLog/datatoad-inspired planner materially improve or
  simplify e-matching-heavy workloads without requiring a private fork or an
  egglog-specific second database engine.
- A WCOJ/GPU-oriented slice can show that SRDatalog-style flat columnar storage,
  deterministic count/materialize allocation, and skew-aware partitioning help
  egglog rule bodies without being swamped by equality rebuild, containers,
  primitives, or scheduler barriers.
- Measurements show that timestamp granularity, trace compaction, rebuild
  invalidations, and same-id dirty refresh events stay bounded on rebuild-heavy
  cases.
- A DD/FlowLog rule-evaluation prototype preserves egglog's per-rule seminaive
  freshness under arbitrary user schedules, including rules that have not run
  since older facts became stable globally.
- A FlowLog/DD-inspired backend slice can overlap physical work across logical
  egglog iterations while preserving exact logical schedule semantics, using
  DD/Timely timestamp and frontier tracking.
- A DD/Timely slice can show that maintained views and product timestamps reduce
  proof/rebuild-rule work without violating egglog's per-rule freshness.
- The first Option 3 scheduling prototype preserved per-rule freshness and
  gated visibility on the scheduled reachability witness while allowing up to
  three future logical tasks in flight (`option-3-experiments.md`).
- Follow-up Option 3 lanes preserved exactness on targeted rebuild, container,
  equality, and scheduler regressions while exposing a high adapter surface and
  barrier-shaped native boundaries (`option-3-experiment-findings.md`,
  `experiments/option-3/README.md`).
- A provider-style boundary can separate ordinary rule relations from
  equality/container/rebuild-sensitive relations without erasing the maintenance
  benefit of the shared substrate.

Evidence to stop:

- Equality maintenance requires relationalizing union-find, congruence closure,
  rebuilding, and container refresh in a way that is consistently much slower or
  more complex than the current backend.
- Container rebuild, higher-order container functions, custom schedulers, or
  Python-facing APIs require enough bespoke code that the DD layer no longer
  provides meaningful maintenance leverage.
- Preserving custom scheduler semantics requires full match materialization,
  subset selection, and delayed action firing in a way that defeats the proposed
  incremental rule-evaluation boundary.
- Preserving arbitrary schedules requires per-rule timestamp-window indexes or
  DD traces that are as complex or expensive as the native timestamp-ordered
  tables.
- DD-overlapped execution cannot deliver useful overlap even inside a
  single-owner backend because rebuild, actions, custom schedulers, or
  per-rule freshness require every logical iteration to finish before useful
  later work can proceed.
- A permanent adapter requires rule IR, timestamp/frontier assignment, rebuild
  invalidation, same-id container refresh, scheduler admission, WCOJ/native
  planner selection, and provider-boundary semantics while native egglog still
  owns the same behavior.

## Arguments For

- The social and maintenance motivation is plausible but unproven: the
  conversations show interest from the DD ecosystem, especially around
  e-graph-shaped workloads, and the hoped-for benefit would be sharing hard
  database engineering rather than only chasing speed
  (`source-notes/conversations-social.md`).
- Egglog already has a relational rule shape. Core rules lower to conjunctive
  query bodies plus actions, and the current backend delegates rule execution to
  a relation layer, making rule matching a plausible substrate boundary
  (`source-notes/egglog-core-proof.md`).
- Eli's scaling-equality-saturation draft makes the native backend a stronger
  comparison point: egglog already has per-rule seminaive timestamps,
  timestamp-ordered hash tables, staged mutation, table-provider hooks, Free
  Join, dynamic variable ordering, and parallel bulk execution
  (`source-notes/scaling-equality-saturation.md`).
- DD/Timely directly supports maintained joins, arrangements, reductions,
  compaction, and nested iteration. Those match e-node indexes, parent indexes,
  seminaive matching, and some fixed-point structure (`source-notes/differential-timely.md`).
- FlowLog is the closest architecture reference for "Datalog on DD": it
  separates rule planning, recursive control, optimization, and DD lowering. It
  is a planning shape, not a drop-in egglog backend
  (`source-notes/datalog-wcoj-planning.md`).
- Datatoad, Free Join, and `dataflow-join` suggest richer join-kernel ideas for
  cyclic or high-arity e-matching queries than source-order binary joins, but
  they need an egglog-specific adapter, index layout, and invalidation model
  (`source-notes/datalog-wcoj-planning.md`).
- SRDatalog is now concrete external evidence that recursive Datalog with WCOJ
  can work well on GPUs. Its architecture combines flat sorted columns,
  deterministic two-phase count/materialize kernels, root-level histogram skew
  balancing, helper-relation splitting, and stream-parallel rule scheduling
  (`source-notes/datalog-wcoj-planning.md`).
- Ascent BYODS and `columnar` show that custom relation/storage providers are a
  legitimate design pattern for logic systems. Provider-style boundaries are a
  cross-cutting architecture axis for preserving specialized equality/container
  behavior while still sharing some rule substrate
  (`source-notes/extension-models.md`).

## Arguments Against

- Equality maintenance is not ordinary monotone Datalog. Rebuild rewrites table
  values, retimestamps dirty parent rows, may run merge functions during rebuild,
  and uses union-by-min for rebuild locality (`source-notes/egglog-core-proof.md`).
- DD can express equality-like maintenance, but the local EqSat prototype uses
  nested iteration and transitive closure with an explicit warning that this is
  not the right connected-components implementation (`source-notes/differential-timely.md`).
- Containers are a first-class user feature, not a corner case. They require
  rebuild of contained e-class references and support higher-order/blockwise
  functions such as map/fold to avoid A/C blow-up (`source-notes/containers-frontends.md`).
- FlowLog's current optimizer is not enough for egglog by itself: the inspected
  code still has source-order left-deep planning, so robust e-matching would need
  extra planner work (`source-notes/datalog-wcoj-planning.md`).
- SRDatalog strengthens the WCOJ case, but it does not solve egglog's full
  backend problem. The evaluated core is recursive Datalog, not equality
  saturation with union-find, rebuild retimestamping, containers/primitives,
  proof/term encoding, arbitrary logical schedules, and custom scheduler
  admission (`source-notes/datalog-wcoj-planning.md`).
- A custom-provider design may preserve performance but weaken the benefit of
  moving to DD if most hard behavior remains egglog-specific
  (`source-notes/extension-models.md`).
- Frontend compatibility is broad: Python methods, containers, `push`/`pop`,
  custom costs, schedulers, extraction, preserved Python calls, and
  `egglog-experimental` extensions all constrain the backend boundary
  (`source-notes/containers-frontends.md`).
- Arbitrary schedules are not only frontend syntax. Standard global
  recent/stable/new seminaive evaluation can be wrong when different rulesets
  run at different times; egglog uses per-rule last-run timestamps and
  timestamp-window table scans to preserve correctness
  (`source-notes/scaling-equality-saturation.md`).
- Proof/term encoding is only a partial validation oracle. It names useful
  equality/rebuild relations, but it cannot validate the full Python,
  container, scheduler, presort, primitive, and custom frontend surface
  (`source-notes/egglog-core-proof.md`, `source-notes/containers-frontends.md`).

## Logical vs Physical Scheduling

The corrected scheduling frame separates egglog's logical schedule from the
backend's physical execution schedule. Exact logical scheduling preserves
per-rule timestamp windows, custom scheduler behavior, bounded `run`, staged
`saturate`, ruleset order, and manual stratification. A DD/FlowLog-backed design
that claims compatibility must preserve those observations
(`options/option-1-native-equality-dd-rule-eval.md`,
`options/option-3-new-backend.md`,
`source-notes/scaling-equality-saturation.md`).

DD-overlapped physical scheduling is different from semantic relaxation. Timely
supports nested/product timestamps and frontiers; DD examples explicitly show
multiple input rounds in flight to improve throughput without changing the
computation's output, apart from batching of observed changes
(`source-notes/differential-timely.md`, `messages/eli-dd-overlapped-scheduling.md`).
The Option 3 hypothesis is that a new backend can use this machinery to start
physical work for logical iteration `N+1` before all of iteration `N` has
finished, then gate visibility of later matches/actions until frontiers prove
the required earlier work is complete.

The first runnable Option 3 experiment supports the semantic part of this
hypothesis on a small reachability workload: the broken global-seminaive model
misses `reachable(1, 3)`, while `dd-barrier` and `dd-overlap` match the oracle
with zero early visibility violations (`option-3-experiments.md`). The
performance result is not decisive: overlap wins most small/medium runs, but
three of twelve summary rows are neutral or losses, and synthetic native
barriers collapse the setup back toward stop/start behavior.

The follow-up lane pass sharpens the ownership question. Native rebuild/action,
equality/rebuild, container dirty-refresh, and custom scheduler/backoff
regressions pass when their native boundaries are respected, but the integration
lane does not show useful overlap through native-authoritative barriers. That
downgrades a permanent adapter that mirrors native state. It does not reject a
replacement backend that owns those responsibilities directly. The planner
lanes are also incomplete as performance evidence: native join strategies pass
smoke tests, while WCOJ examples compile but lack graph inputs for runtime
comparison.

Explicitly relaxed scheduling remains only a fallback variant. It may still be
worth studying if exact overlap is too constrained, but it would require a
separate scoped contract because existing programs may rely on bounded `run`,
staged `saturate`, blowup control, manual stratification, or full-match
materialization for custom schedulers.

## Backend Boundary Options

These are ordered as a complexity and disruption ladder, not as a recommendation
ranking.

1. **Native improvement / borrow ideas**
   - Keep egglog's backend native and instead borrow ideas: WCOJ planning,
     columnar row storage, custom provider interfaces, better profiling,
     timestamp/index improvements, and clearer rule IR boundaries.
   - Potential benefit: preserves existing frontend, container, rebuild,
     schedule, and extension semantics while still importing concrete
     database-engineering ideas.
   - Main blocker: it gives less shared-substrate maintenance leverage and
     leaves egglog owning most of the hard runtime complexity unless
     provider-style relation boundaries isolate reusable pieces
     (`options/option-4-no-dd-backend-borrow-ideas.md`).

2. **Exact hybrid DD rule evaluation**
   - Keep union-find, congruence closure, rebuilding, containers, analyses,
     extraction, and logical schedules native.
   - Move selected rule/e-matching relation maintenance to DD arrangements or a
     FlowLog-like planner.
   - Potential benefit: tests shared substrate value at the rule-matching layer
     without immediately moving the hardest equality, container, and scheduler
     semantics.
   - Main blocker: the egglog/DD boundary would need exact handling for
     canonical-id changes, explicit rebuild-invalidation and same-id dirty
     events, per-rule seminaive timestamp windows, full-match/delayed-fire
     scheduler semantics, duplicate/stale matches, and action handoff
     (`options/option-1-native-equality-dd-rule-eval.md`).

3. **FlowLog/datatoad/DD-inspired new backend**
   - Build an authoritative backend with FlowLog-like planning, DD/Timely
     execution and progress, datatoad/WCOJ operators for selected joins, and
     backend-owned semantics for the moved relation state.
   - Preserve current logical schedule semantics while letting DD overlap
     physical work across logical iterations when timestamp/frontier tracking
     proves later work cannot become visible too early.
   - Potential benefit: could support a long-term relational backend using
     FlowLog-like planner structure and datatoad/dataflow-join-style join
     kernels, while avoiding permanent duplicate state by replacing rather than
     shadowing native backend responsibilities.
   - Main blocker: the next gate must prove single ownership in a vertical
     slice, including per-rule freshness, rebuild/canonicalization,
     dirty-refresh-style invalidation, scheduler materialization, and
     step-visible native-oracle equivalence (`options/option-3-new-backend.md`).

4. **Proof/term encoding to DD**
   - Use egglog's proof/term encoding as a relational specification of equality
     maintenance, then lower those generated relations to DD.
   - Potential benefit: gives a concrete relational account of UF/view/rebuild
     state and could serve as a partial correctness oracle for other designs.
   - Main blocker: the current path is much slower, rejects custom
     containers/presorts, cannot validate the full Python/container/scheduler
     surface or arbitrary-schedule seminaive behavior, and shifts equality into
     many generated tables
     (`options/option-2-proof-term-encoding-dd.md`).

## Cross-Cutting Provider Boundary

Provider-style relation boundaries are not just an implementation detail of
Option 4. They cut across the DD and non-DD designs: ordinary rule relations
could be DD/FlowLog-backed while equality, containers, rebuild-sensitive tables,
or columnar storage use specialized providers. The upside is preserving
egglog-specific semantics without forcing all equality maintenance into ordinary
relations. The cost is a new provider ABI that may recreate much of the current
backend complexity and reduce the generic value DD can provide
(`source-notes/extension-models.md`).

This axis needs its own evidence before it becomes a separate option: a minimal
provider-boundary sketch with one ordinary relation, one equality/rebuild-aware
provider, and one container/index provider, compared against both the native
backend and a DD-backed rule-evaluation prototype.

## Scaling Equality Saturation Update

Eli's backend draft changes the scheduling conclusion. Arbitrary schedules are
not just a compatibility feature around `run`, `saturate`, and `run-with`; they
are part of seminaive correctness. Standard global recent/stable/new Datalog
evaluation can miss matches when one ruleset saturates before another. Egglog's
solution is to tag rows with logical timestamps and track each rule's last-run
timestamp, so each rule sees rows inserted or refreshed since it last ran
(`source-notes/scaling-equality-saturation.md`).

That means any exact DD/FlowLog option that owns rule matching must preserve
per-rule freshness windows. It can implement them with DD timestamps, with data
columns plus arrangements, or by keeping the native timestamp-ordered table as a
provider, but it cannot treat seminaive evaluation as a solved generic Datalog
optimization. Eli's later clarification changes the physical-scheduling
interpretation: DD may still overlap work for later logical iterations while
preserving these logical windows, because multidimensional time and frontiers
can track when earlier work is actually complete
(`messages/eli-dd-overlapped-scheduling.md`, `source-notes/differential-timely.md`).
The same source also strengthens Option 4 as a baseline: the current backend
already contains several database-engineering ideas a migration would need to
replace or reuse, including staged mutation, timestamp-ordered hash tables,
provider hooks, Free Join, dynamic variable ordering, and parallel bulk
execution.

## Current Assessment

No backend path is selected yet. The evidence is better read as a tradeoff map
than as a prescriptive implementation queue.

The source evidence supports DD/FlowLog/datatoad as plausible references or
substrates for maintained relational matching, arrangements, and maybe
WCOJ-style planning. The SRDatalog paper makes the WCOJ/GPU side stronger than
the earlier local compile-only evidence, but it also narrows the remaining
question: can those join/storage/scheduling techniques survive egglog's
union-find, rebuild, containers/primitives, and schedule semantics? The current
repo does not yet show that all equality maintenance, rebuilding, containers,
per-rule seminaive scheduling, or provider-specific behavior can move into that
substrate without losing core egglog semantics or recreating the current backend
complexity at a different layer.

The Option 3 experiment suite narrows several uncertainties: per-rule seminaive
freshness can be preserved in a DD-backed matcher for the corrected scheduled
reachability witness, and targeted native rebuild/container/scheduler
regressions pass under the current native semantics. It points away from a
permanent adapter because real native phases remain barrier-shaped, rebuild and
container counters are not yet exposed, WCOJ has no runtime dataset in this
checkout, and an adapter would duplicate too many egglog-specific
responsibilities. It leaves open a replacement backend that owns those
responsibilities directly.

The central question for any option is whether it can move enough real
database/runtime responsibility out of egglog to justify its long-term cost. If
the substrate only handles a small amount of ordinary joining while egglog still
owns equality, rebuild, containers, scheduling, custom providers, and most
indexing, the maintenance argument becomes weak even if individual prototypes
work. The social maintenance payoff should also remain a hypothesis until a
prototype produces issues, fixes, or reusable abstractions that upstream
projects actually want to share.

## Option Tradeoff Update

The option analysis now presents a complexity/disruption ladder. This is still
not a recommendation ranking; each row answers a different question about how
much semantic and architectural change the project is willing to consider.

- **Native improvement / borrow ideas.** Long-term benefit: lowest migration
  risk for existing semantics, with incremental adoption of WCOJ, provider,
  columnar, profiling, timestamp/index, and rule-IR ideas. Main blocker before
  trying: this may fail the social/maintenance goal by leaving egglog as the
  sole owner of the hard runtime machinery
  (`options/option-4-no-dd-backend-borrow-ideas.md`).
- **Exact hybrid DD rule evaluation.** Long-term benefit: a limited migration
  surface that tests DD/FlowLog on maintained rule indexes and incremental body
  matching while keeping current equality/rebuild/container/schedule behavior
  native. Main blocker before trying: define and measure the delta contract for
  canonical ids, rebuild invalidation, same-id dirty refresh, per-rule seminaive
  freshness, scheduler match selection, and action handoff
  (`options/option-1-native-equality-dd-rule-eval.md`).
- **FlowLog/datatoad/DD-inspired new backend.** Long-term benefit: a richer
  backend architecture inspired by DD execution, recursive control, WCOJ
  kernels, and DD's ability to keep multiple logical times in flight while
  preserving egglog's logical schedule. The first scheduling experiment passed
  the reachability freshness gate with no early visibility violations. The
  parallel follow-up gates downgrade the adapter path, not the single-owner
  backend path: exactness passes by respecting rebuild/action/scheduler/container
  boundaries, and the next experiment must show a new backend owning those
  boundaries directly. SRDatalog now provides external GPU WCOJ runtime data,
  but local egglog-specific WCOJ/equality/container/scheduler data is still
  missing
  (`options/option-3-new-backend.md`).
- **Proof/term encoding to DD.** Long-term benefit: a concrete relational
  specification for equality maintenance and proof experiments. Main blocker
  before trying: show that generated UF/view/rebuild tables can be made much
  cheaper and accept that containers, schedulers, presorts, and Python
  frontends, plus arbitrary-schedule seminaive behavior, still need separate
  validation paths
  (`options/option-2-proof-term-encoding-dd.md`).

Evidence that would clarify the choice:

- Native improvement needs evidence that borrowed planning/profiling/provider
  ideas can address important egglog workloads well enough that the
  shared-substrate migration is not worth the cost.
- Exact hybrid DD rule evaluation needs data on whether rebuild invalidation
  causes near-full retraction/reinsertion into DD on realistic equality merges,
  and whether native action handoff creates stale or duplicate-heavy match
  streams. It also needs the scheduled reachability witness to pass with
  per-rule freshness.
- Option 3's next gate is a replacement-backend vertical slice, not another
  adapter experiment. It needs row-level rebuild/container counters, trace
  memory and compaction metrics, a mounted WCOJ dataset, and one single-owner
  slice proving that moved responsibilities do not call back into native
  `core-relations`.
- Proof/term encoding needs measurements of proof/term encoding overhead on
  small constructor/rebuild tests, plus a concrete story for containers and
  presorts.
- The provider-boundary axis needs a concrete comparison showing whether custom
  equality/container/rebuild providers preserve semantics without collapsing
  back into a full native backend.

## Possible Evidence-Gathering Work

- Build one constructor-only equality witness comparing native equality,
  proof/term encoding, and the local DD EqSat prototype shape. Measure runtime,
  records emitted, retained state, and representative churn.
- Build one native-equality plus DD rule-evaluation prototype for a small
  e-matching rule. Keep rebuild and union-find native; feed DD batched relation
  deltas and compare against current `core-relations`, including a case that
  requires same-id dirty refresh or explicit rebuild invalidation.
- Reproduce Eli's scheduled reachability example and use it as the minimum
  semantic regression for any DD/FlowLog rule-evaluation or schedule-lowering
  prototype.
- Test the April 24 `A/B/C` proof-query shape as a planner benchmark, measuring
  naive seminaive delta expansion against functional-dependency-aware dependent
  lookup.
- Reproduce the documented container witness
  `2 + a + b + b + 3` across binary A/C rules, multiset containers with an
  index, and higher-order multiset functions. Measure whether a DD-backed index
  recreates the blow-up.
- Classify 3-5 real egglog rules as acyclic, cyclic, repeated-variable, or
  equality-heavy, then compare source-order binary joins with a WCOJ-style
  operator on at least one cyclic pattern. The first classification pass is
  complete; the local WCOJ runtime comparison still needs graph input data, and
  the SRDatalog paper should be used to choose GPU-specific WCOJ constraints
  rather than treated as an egglog measurement.
- Build the Option 3 replacement-backend vertical slice: one small
  relation/function-table universe, per-rule freshness, one
  rebuild/canonicalization event, one dirty-refresh-style invalidation, one
  scheduler materialization boundary, and step-visible comparison against native
  egglog as the oracle.
- Sketch the minimum custom-provider interface required for equality,
  containers, and columnar relation storage, using Ascent BYODS as the
  comparison point.
- Trace one custom scheduler run to measure how many matches are materialized,
  retained, filtered, and delayed before action execution. The first scheduler
  lane passed behavior/scoping tests; it still needs direct match/admission
  counters if this becomes implementation evidence.
