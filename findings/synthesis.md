# Synthesis

This is a provisional synthesis of the first sub-agent reading pass. It should
be treated as a decision aid, not as a final backend design.

## Decision Frame

The investigation should continue only if a small prototype can show that a
DD-related substrate reduces complexity or improves maintainability without
losing egglog's core semantics.

Evidence to continue:

- A native-equality plus DD/FlowLog-style rule-evaluation prototype can preserve
  rebuild semantics, schedules, and containers while moving enough relational
  matching/indexing work out of egglog to justify the dependency.
- DD arrangements or a FlowLog/datatoad-like planner materially improve or
  simplify e-matching-heavy workloads without requiring a private fork of the
  substrate.
- Equality updates can be batched into DD at a coarse enough timestamp granularity
  that trace/progress costs stay bounded on rebuild-heavy cases.

Evidence to stop:

- Equality maintenance requires relationalizing union-find, congruence closure,
  rebuilding, and container refresh in a way that is consistently much slower or
  more complex than the current backend.
- Container rebuild, higher-order container functions, custom schedulers, or
  Python-facing APIs require enough bespoke code that the DD layer no longer
  provides meaningful maintenance leverage.
- The only viable path changes egglog's user-facing language enough to become a
  different system rather than a backend/substrate replacement.

## Arguments For

- The social and maintenance motivation is real: the conversations show active
  interest from the DD ecosystem, especially around e-graph-shaped workloads,
  and the strongest benefit would be sharing hard database engineering rather
  than only chasing speed (`source-notes/conversations-social.md`).
- Egglog already has a relational rule shape. Core rules lower to conjunctive
  query bodies plus actions, and the current backend delegates rule execution to
  a relation layer, making rule matching a plausible substrate boundary
  (`source-notes/egglog-core-proof.md`).
- DD/Timely directly supports maintained joins, arrangements, reductions,
  compaction, and nested iteration. Those match e-node indexes, parent indexes,
  seminaive matching, and some fixed-point structure (`source-notes/differential-timely.md`).
- FlowLog is the closest architecture model for "Datalog on DD": it separates
  rule planning, recursive control, optimization, and DD lowering. That is a
  better fit than treating raw DD collections as the frontend abstraction
  (`source-notes/datalog-wcoj-planning.md`).
- Datatoad, Free Join, and `dataflow-join` suggest a richer join-planning path
  for cyclic or high-arity e-matching queries than source-order binary joins
  (`source-notes/datalog-wcoj-planning.md`).
- Ascent BYODS and `columnar` show that custom relation/storage providers are a
  legitimate design pattern for logic systems, which could preserve specialized
  equality/container behavior while still sharing a rule substrate
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
- A custom-provider design may preserve performance but weaken the benefit of
  moving to DD if most hard behavior remains egglog-specific
  (`source-notes/extension-models.md`).
- Frontend compatibility is broad: Python methods, containers, `push`/`pop`,
  custom costs, schedulers, extraction, preserved Python calls, and
  `egglog-experimental` extensions all constrain the backend boundary
  (`source-notes/containers-frontends.md`).

## Backend Boundary Options

1. **Native equality + DD/FlowLog rule evaluation**
   - Keep union-find, congruence closure, rebuilding, containers, analyses, and
     extraction native.
   - Move selected rule/e-matching relation maintenance to DD arrangements or a
     FlowLog-like planner.
   - Potential benefit: tests shared substrate value at the rule-matching layer
     without immediately moving the hardest equality and container semantics.
   - Main blocker: the egglog/DD boundary would need exact handling for
     canonical-id changes, rebuild invalidations, dirty container refresh,
     duplicate/stale matches, and action handoff
     (`options/option-1-native-equality-dd-rule-eval.md`).

2. **Proof/term encoding to DD**
   - Use egglog's proof/term encoding as a relational specification of equality
     maintenance, then lower those generated relations to DD.
   - Potential benefit: gives a concrete relational account of UF/view/rebuild
     state and could serve as a correctness oracle for other designs.
   - Main blocker: the current path is much slower, rejects custom
     containers/presorts, and shifts equality into many generated tables
     (`options/option-2-proof-term-encoding-dd.md`).

3. **FlowLog/datatoad-like middle layer**
   - Build or adapt an intermediate relational planner with DD execution,
     datatoad/WCOJ operators for selected joins, and egglog-specific operators
     for rebuild/equality deltas.
   - Potential benefit: could support a long-term relational architecture if
     egglog needs planning, recursive control, and WCOJ kernels between the
     frontend and runtime.
   - Main blocker: it requires a substantial new planner, index model, recursive
     control story, and egglog-specific equality/rebuild operators before the
     smaller substrate boundary is proven
     (`options/option-3-flowlog-datatoad-middle-layer.md`).

4. **No DD backend**
   - Keep egglog's backend native and instead borrow ideas: WCOJ planning,
     columnar row storage, custom provider interfaces, better profiling, and
     clearer rule IR boundaries.
   - Potential benefit: preserves existing frontend, container, rebuild, and
     extension semantics while still importing concrete database-engineering
     ideas.
   - Main blocker: it gives less shared-substrate maintenance leverage and
     leaves egglog owning most of the hard runtime complexity
     (`options/option-4-no-dd-backend-borrow-ideas.md`).

## Current Assessment

No backend path is selected yet. The evidence is better read as a tradeoff map
than as a prescriptive implementation queue.

The source evidence supports DD/FlowLog/datatoad as plausible shared substrates
for maintained relational matching, arrangements, and maybe WCOJ-style planning.
It does not yet show that all equality maintenance, rebuilding, containers, or
scheduling can move into that substrate without losing core egglog semantics or
recreating the current backend complexity at a different layer.

The central question for any option is whether it can move enough real
database/runtime responsibility out of egglog to justify its long-term cost. If
the substrate only handles a small amount of ordinary joining while egglog still
owns equality, rebuild, containers, scheduling, custom providers, and most
indexing, the maintenance argument becomes weak even if individual prototypes
work.

## Adversarial Review Update

A pre-sharing adversarial review found no P0/P1 contradictions in this synthesis
or the option framing (`adversarial-review.md`). The accepted corrections are
mostly caveats about evidentiary weight:

- The social and maintenance payoff should be treated as a hypothesis from the
  conversations, not a validated outcome (`adversarial-notes/collaborator-readiness.md`).
- FlowLog, datatoad, and `dataflow-join` are planner shapes and join-kernel
  references, not drop-in reusable engines for egglog
  (`adversarial-notes/dd-flowlog-substrate.md`).
- Timestamp granularity, trace compaction, rebuild invalidation, and same-id
  dirty refresh remain empirical questions; ordinary tuple deletes are not a
  complete description of rebuild-triggered invalidation
  (`adversarial-notes/dd-flowlog-substrate.md`).
- Proof/term encoding is useful as a partial relational specification, but it
  cannot validate the full Python/container/scheduler surface
  (`adversarial-notes/egglog-semantics.md`).
- Provider-style relation boundaries are a cross-cutting design axis. They may
  deserve a separate option or sub-option once there is a concrete comparison
  against the DD/native split (`adversarial-notes/option-completeness.md`).

## Option Tradeoff Update

The second-pass option analysis reframes the options by long-term benefit and
blocker rather than by a preferred order.

- **Option 1: native equality + DD/FlowLog rule evaluation.** Long-term benefit:
  a limited migration surface that tests DD/FlowLog on maintained rule indexes
  and incremental body matching while keeping current equality/rebuild/container
  behavior native. Main blocker before trying: define and measure the delta
  contract for canonical ids, rebuild invalidation, container refresh, and match
  handoff (`options/option-1-native-equality-dd-rule-eval.md`).
- **Option 2: proof/term encoding to DD.** Long-term benefit: a concrete
  relational specification for equality maintenance and proof experiments. Main
  blocker before trying: show that generated UF/view/rebuild tables can be made
  much cheaper and that containers/presorts have a credible encoding or native
  side channel (`options/option-2-proof-term-encoding-dd.md`).
- **Option 3: FlowLog/datatoad-like middle layer.** Long-term benefit: a richer
  planner architecture with DD execution, recursive control, and WCOJ kernels
  available behind egglog's frontend. Main blocker before trying: avoid building
  a second full database engine before proving which egglog relations and
  indexes actually belong outside the native backend
  (`options/option-3-flowlog-datatoad-middle-layer.md`).
- **Option 4: no DD backend, borrow ideas.** Long-term benefit: lower migration
  risk for existing semantics, with incremental adoption of WCOJ, provider,
  columnar, profiling, and rule-IR ideas. Main blocker before trying: this may
  fail the social/maintenance goal by leaving egglog as the sole owner of the
  hard runtime machinery (`options/option-4-no-dd-backend-borrow-ideas.md`).

Evidence that would clarify the choice:

- Option 1 needs data on whether rebuild invalidation causes near-full
  retraction/reinsertion into DD on realistic equality merges, and whether
  native action handoff creates stale or duplicate-heavy match streams.
- Option 2 needs measurements of proof/term encoding overhead on small
  constructor/rebuild tests, plus a concrete story for containers and presorts.
- Option 3 needs a small rule-IR sketch showing whether DD arrangements or
  datatoad/dataflow-join WCOJ kernels can be reused without maintaining a second
  full index universe.
- Option 4 needs evidence that borrowed planning/profiling/provider ideas can
  address important egglog workloads well enough that the shared-substrate
  migration is not worth the cost.

## Possible Evidence-Gathering Work

- Build one constructor-only equality witness comparing native equality,
  proof/term encoding, and the local DD EqSat prototype shape. Measure runtime,
  records emitted, retained state, and representative churn.
- Build one native-equality plus DD rule-evaluation prototype for a small
  e-matching rule. Keep rebuild and union-find native; feed DD batched relation
  deltas and compare against current `core-relations`.
- Reproduce the documented container witness
  `2 + a + b + b + 3` across binary A/C rules, multiset containers with an
  index, and higher-order multiset functions. Measure whether a DD-backed index
  recreates the blow-up.
- Classify 3-5 real egglog rules as acyclic, cyclic, repeated-variable, or
  equality-heavy, then compare source-order binary joins with a WCOJ-style
  operator on at least one cyclic pattern.
- Sketch the minimum custom-provider interface required for equality,
  containers, and columnar relation storage, using Ascent BYODS as the
  comparison point.
