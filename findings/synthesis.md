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
   - This is the best first prototype because it tests shared substrate value
     without taking on the hardest equality risk immediately.

2. **Proof/term encoding to DD**
   - Use egglog's proof/term encoding as a relational specification of equality
     maintenance, then lower those generated relations to DD.
   - This is semantically clarifying but high risk: the current path is already
     much slower, rejects custom containers/presorts, and shifts equality into
     many generated UF/view/rebuild tables.

3. **FlowLog/datatoad-like middle layer**
   - Build or adapt an intermediate relational planner with DD execution,
     datatoad/WCOJ operators for selected joins, and egglog-specific operators
     for rebuild/equality deltas.
   - This may become the right long-term architecture, but it is too large for
     the first experiment unless reduced to one or two rule patterns.

4. **No DD backend**
   - Keep egglog's backend native and instead borrow ideas: WCOJ planning,
     columnar row storage, custom provider interfaces, better profiling, and
     clearer rule IR boundaries.
   - This becomes the likely answer if equality/container/scheduler support
     consumes most of the design or requires unacceptable frontend changes.

## Current Best Conclusion

Provisional conclusion: do not attempt a full egglog-on-DD rewrite yet, but do
continue with a narrow prototype centered on rule evaluation with native equality.

The source evidence supports DD/FlowLog/datatoad as a plausible shared substrate
for maintained relational matching, arrangements, and maybe WCOJ-style planning.
It does not yet support moving all equality maintenance, rebuilding, containers,
or scheduling into DD. The strongest next step is to test whether the rule
evaluation boundary delivers enough value while preserving current semantics.

If that boundary fails to reduce complexity or shows poor performance on
rebuild/e-matching workloads, the project should pivot to borrowing specific
ideas rather than pursuing a backend migration.

## Option Viability Update

The second-pass option analysis preserves the provisional conclusion and makes
the implementation order sharper.

1. **Likely first experiment: Option 1, native equality + DD/FlowLog rule
   evaluation.** Continue as a hybrid prototype. It keeps union-find, rebuild,
   containers, actions, analyses, and extraction native while testing DD/FlowLog
   for maintained rule indexes and incremental body matching
   (`options/option-1-native-equality-dd-rule-eval.md`).
2. **Promising but deferred: Option 3, FlowLog/datatoad-like middle layer.**
   Defer until the smaller data-exchange boundary is proven. It is the most
   coherent long-term DD-backed architecture, but it requires a new relational
   planner, WCOJ index story, recursive DD integration, and unresolved
   equality/rebuild diff semantics
   (`options/option-3-flowlog-datatoad-middle-layer.md`).
3. **High-risk research path: Option 2, proof/term encoding to DD.** Defer as
   the main production lowering, but keep it as a relational specification and
   prototype oracle. It names the UF/view/rebuild state DD would need, but the
   current encoding is high-overhead, incomplete for current egglog features,
   and incompatible with presort/container semantics
   (`options/option-2-proof-term-encoding-dd.md`).
4. **Fallback/non-migration path: Option 4, no DD backend.** Continue as a
   low-risk native improvement track and fallback. It borrows WCOJ/SIP planning,
   provider interfaces, columnar storage experiments, and profiling without
   risking frontend or container semantics
   (`options/option-4-no-dd-backend-borrow-ideas.md`).

Evidence that would change the ranking:

- Option 1 moves down if rebuild invalidation requires nearly full state
  retraction/reinsertion into DD on realistic equality merges, or if the native
  action handoff creates stale/dedup-heavy match handling.
- Option 3 moves up if a small rule-IR prototype can reuse DD arrangements or
  datatoad/dataflow-join WCOJ kernels without maintaining a second full index
  universe.
- Option 2 moves up only if proof/term encoding overhead can be measured and
  reduced on constructor/rebuild tests, and if containers get a credible native
  side channel or encoding.
- Option 4 becomes the primary path if backend migration fails to reduce
  complexity, but borrowed planning/profiling/provider ideas still improve the
  native backend.

## Next Experiments

1. Build one constructor-only equality witness comparing native equality,
   proof/term encoding, and the local DD EqSat prototype shape. Measure runtime,
   records emitted, retained state, and representative churn.
2. Build one native-equality plus DD rule-evaluation prototype for a small
   e-matching rule. Keep rebuild and union-find native; feed DD batched relation
   deltas and compare against current `core-relations`.
3. Reproduce the documented container witness
   `2 + a + b + b + 3` across binary A/C rules, multiset containers with an
   index, and higher-order multiset functions. Measure whether a DD-backed index
   recreates the blow-up.
4. Classify 3-5 real egglog rules as acyclic, cyclic, repeated-variable, or
   equality-heavy, then compare source-order binary joins with a WCOJ-style
   operator on at least one cyclic pattern.
5. Sketch the minimum custom-provider interface required for equality,
   containers, and columnar relation storage, using Ascent BYODS as the
   comparison point.
