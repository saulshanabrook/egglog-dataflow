# Evidence Ledger

Each claim records the source fact, source path, derived conclusion, confidence,
and relevance to the current external DD trial.

## E-001 - Current Phase Is External DD Modeling

- **Source fact:** Eli's April 29 note says to "get simple subset in new crate
  that models egglog" and to keep the code stripped back enough to reason about
  performance profiles.
- **Source path:** `messages/eli-meeting-april-29-2026.md`
- **Derived conclusion:** The active next phase should prepare an external DD
  model, not start by editing the production egglog backend.
- **Confidence:** High.
- **Relevance:** Sets the repo reorganization target.

## E-002 - Native Egglog Is The Oracle

- **Source fact:** The same note says to use egglog as the oracle, use egglog to
  generate test databases, and verify that the model behaves the same.
- **Source path:** `messages/eli-meeting-april-29-2026.md`
- **Derived conclusion:** Oracle comparison is a first-class requirement for the
  future trial and should be documented before implementation planning.
- **Confidence:** High.
- **Relevance:** Defines how correctness evidence should be gathered.

## E-003 - Mapping Comes Before Performance Interpretation

- **Source fact:** The April 29 note says the priority is simplicity to
  understand how core egglog ideas map to DD so performance profiles can be
  understood and compared.
- **Source path:** `messages/eli-meeting-april-29-2026.md`
- **Derived conclusion:** The repo should organize mapping questions and
  evidence before producing an MVP work plan or speed claims.
- **Confidence:** High.
- **Relevance:** Prevents premature implementation milestones in active docs.

## E-004 - Existing Crate Boundaries Are A Learning Risk

- **Source fact:** Eli's note calls the current egglog crate code confusing
  around commands, rules, nested/unnested phases, parameters, and `NCommands`.
- **Source path:** `messages/eli-meeting-april-29-2026.md`
- **Derived conclusion:** A small external model is a better learning vehicle
  than starting with a broad in-repo refactor.
- **Confidence:** Medium / High.
- **Relevance:** Explains why older scaffold docs are archived.

## E-005 - Proof And Frontend Transformations Complicate The Boundary

- **Source fact:** Eli's note says proofs must speak about surface syntax and
  that Python frontend syntax transformations complicate the proof/core split.
- **Source path:** `messages/eli-meeting-april-29-2026.md`
- **Derived conclusion:** The trial should record which representation is used
  as oracle input and should not assume a production proof boundary.
- **Confidence:** Medium.
- **Relevance:** Shapes future interface and oracle questions.

## E-006 - Benchmark Results Need Category Labels

- **Source fact:** The April 29 note separates high-throughput workloads, many
  tiny-iteration workloads, and hard joins/hardboiled-style benchmarks.
- **Source path:** `messages/eli-meeting-april-29-2026.md`
- **Derived conclusion:** Future performance data should be labeled by workload
  shape before being used as backend evidence.
- **Confidence:** High.
- **Relevance:** Organizes benchmark categories in `minimal-dd-trial.md`.

## E-007 - DD/Timely Fits Incremental Relational Maintenance

- **Source fact:** DD source notes record joins, semijoins, reductions,
  consolidation, arrangements, nested iteration, probes, and trace compaction
  as native DD/Timely mechanisms.
- **Source path:** `findings/source-notes/differential-timely.md`
- **Derived conclusion:** DD is plausible for relation maintenance and
  scheduling experiments, but the exact egglog mapping still needs witnesses.
- **Confidence:** Medium / High.
- **Relevance:** Provides the positive substrate evidence.

## E-008 - Trace And Timestamp Policy Remain Open

- **Source fact:** DD/Timely notes warn that progress work depends on timestamp
  granularity, traces retain update history until compaction, and compaction
  loses historical query ability.
- **Source path:** `findings/source-notes/differential-timely.md`
- **Derived conclusion:** The trial must record how logical egglog times map to
  DD times and what is retained for oracle comparisons.
- **Confidence:** Medium / High.
- **Relevance:** Prevents treating DD timestamp policy as settled.

## E-009 - Arbitrary Schedules Require Per-Rule Freshness

- **Source fact:** Eli's backend draft and source notes say globally
  recent/stable/new seminaive evaluation is incorrect under arbitrary egglog
  schedules; each rule tracks rows new since that rule last ran.
- **Source path:** `findings/source-notes/scaling-equality-saturation.md`
- **Derived conclusion:** Per-rule freshness is a semantic oracle requirement,
  not only an optimization.
- **Confidence:** High.
- **Relevance:** Defines a mandatory mapping question.

## E-010 - DD Overlap Is A Logical/Physical Split

- **Source fact:** The DD-overlap notes and Option 3 schedule lane show
  `dd-barrier` and `dd-overlap` matched the scheduled reachability oracle with
  zero early visibility violations, while a broken global-seminaive model
  missed the witness.
- **Source path:** `findings/source-notes/scaling-equality-saturation.md`,
  `findings/experiments/option-3/README.md`
- **Derived conclusion:** Exact logical scheduling with overlapped physical work
  is plausible on the small witness, but must be rechecked for the trial subset.
- **Confidence:** Medium / High.
- **Relevance:** Supplies the schedule/freshness baseline.

## E-011 - Existing Option 3 Artifacts Are Evidence, Not A Plan

- **Source fact:** The Option 3 lane index records passing schedule, rebuild,
  scheduler, and container lanes, but also says WCOJ is compile-only and trace
  memory/compaction counters are still missing.
- **Source path:** `findings/experiments/option-3/README.md`,
  `findings/artifacts/option-3/`
- **Derived conclusion:** The artifacts should inform the external trial, but
  should not be repeated as a direct backend implementation sequence.
- **Confidence:** High.
- **Relevance:** Justifies keeping artifacts while deduplicating synthesis.

## E-012 - Adapter Mirroring Is Downgraded

- **Source fact:** Lanes 10 and 11 say synthetic/native barriers and high
  adapter surface downgrade a permanent middle-layer adapter, while a future
  single-owner backend remains possible if it owns equivalent semantics.
- **Source path:** `findings/experiments/option-3/README.md`,
  `findings/experiments/option-3/10-real-barrier-survival.md`,
  `findings/experiments/option-3/11-adapter-complexity-audit.md`
- **Derived conclusion:** The current phase should not revive a mirrored
  native/DD architecture.
- **Confidence:** High.
- **Relevance:** Explains the archive of old option-ladder framing.

## E-013 - `CoreRule` / `ResolvedCoreRule` Is A Candidate Boundary

- **Source fact:** Core/proof notes say the normal compiler boundary is already
  relational: `CoreRule` is a conjunctive-query body plus SSA-like action head,
  and current execution delegates lowered rules through the backend.
- **Source path:** `findings/source-notes/egglog-core-proof.md`
- **Derived conclusion:** This is a strong candidate for oracle-facing rule
  extraction, but the external trial should decide the boundary deliberately.
- **Confidence:** Medium / High.
- **Relevance:** Preserves a durable fact from archived scaffold docs without
  keeping the scaffold instruction active.

## E-014 - Signed DD Diffs Must Be Netted Before Visibility

- **Source fact:** The production-shaped DD lifecycle artifact used imported
  arrangements, recursive host feedback, explicit signed negation, and
  host-side net-delta consolidation before committing visible rows.
- **Source path:** `findings/artifacts/dd-full-refactor/06-production-lifecycle.json`
- **Derived conclusion:** Any trial output stream with signed diffs needs
  netting before oracle-visible host effects are compared.
- **Confidence:** Medium.
- **Relevance:** Preserves a durable control-plane invariant from archived
  design-spike work.

## E-015 - Rebuild Deltas Need Replayable Live-Row Semantics

- **Source fact:** The rebuild-delta artifact records
  `delta_replay_matches_final: true`; collision candidates are tracked as
  merge evidence while only committed live-row changes enter `row_deltas`.
- **Source path:** `findings/artifacts/dd-full-refactor/02-rebuild-delta.json`
- **Derived conclusion:** Rebuild/canonicalization evidence should distinguish
  committed row diffs from merge-only collision evidence.
- **Confidence:** Medium.
- **Relevance:** Defines how to avoid overclaiming from rewrite/collision data.

## E-016 - Containers Need Same-Id Dirty Refresh Evidence

- **Source fact:** Core/proof and container notes describe
  `ContainerRebuildSummary::dirty_ids` / refresh-row behavior for stable outer
  ids whose container contents change semantically after rebuild.
- **Source path:** `findings/source-notes/egglog-core-proof.md`,
  `findings/source-notes/containers-frontends.md`,
  `findings/experiments/option-3/README.md`
- **Derived conclusion:** The trial cannot treat container changes as ordinary
  key changes only; same-id semantic refresh must be modeled or scoped out.
- **Confidence:** High.
- **Relevance:** Key mapping risk for container benchmarks.

## E-017 - Scheduler Semantics Need Materialization And Admission

- **Source fact:** The scheduler materialization artifact records complete
  matches, selected matches, worklist rows, residual matches, and barrier-
  delayed actions.
- **Source path:** `findings/artifacts/dd-full-refactor/05-scheduler-materialization.json`,
  `findings/source-notes/containers-frontends.md`
- **Derived conclusion:** A scheduler-aware trial must separate match output
  from selected admission and delayed action firing.
- **Confidence:** Medium / High.
- **Relevance:** Prevents match-count-only scheduler modeling.

## E-018 - Hidden Primitive/Callback Reads Are Seminaive Risks

- **Source fact:** The PR #856 review and container/frontend notes show that
  primitive and higher-order container callbacks can hide reads or writes from
  seminaive freshness unless dependencies are explicit.
- **Source path:** `findings/pr-856-typed-execution-state-review.md`,
  `findings/source-notes/containers-frontends.md`
- **Derived conclusion:** Host-side primitives and callbacks in the trial need
  explicit matched inputs, declared dependencies, or oracle-only scoping.
- **Confidence:** Medium / High.
- **Relevance:** Shapes primitive/container subset choices.

## E-019 - WCOJ Evidence Is Component Evidence

- **Source fact:** WCOJ source notes record FlowLog, datatoad, dataflow-join,
  Free Join, and SRDatalog as strong planning/join references, while the local
  WCOJ lane is compile-only because graph inputs are missing.
- **Source path:** `findings/source-notes/datalog-wcoj-planning.md`,
  `findings/experiments/option-3/README.md`
- **Derived conclusion:** WCOJ can guide hard-join trial design, but local
  runtime evidence still needs egglog-shaped data and oracle comparisons.
- **Confidence:** Medium / High.
- **Relevance:** Keeps hard-join benchmarks separate from backend conclusions.

## E-020 - DBSP Is A Reading Input, Not A Chosen Direction

- **Source fact:** The April 29 note says to read DBSP because it is a simpler
  model, supports nested/recursive computation, and automatically
  differentiates code instead of requiring manual DD lowering.
- **Source path:** `messages/eli-meeting-april-29-2026.md`,
  `papers/DBSP.pdf`
- **Derived conclusion:** DBSP should remain a comparison lens for the mapping,
  not a selected substrate until the future plan chooses it.
- **Confidence:** Medium.
- **Relevance:** Preserves a source addition without expanding scope.

## E-021 - Proof Encoding Is Partial Specification, Not Full Oracle

- **Source fact:** Core/proof notes say proof/term encoding reifies UF, view,
  congruence, merge, and rebuild as rules, but rejects primitives without
  validators, action lookups, presort/custom sort containers, user commands, and
  other frontend features.
- **Source path:** `findings/source-notes/egglog-core-proof.md`
- **Derived conclusion:** Proof encoding can inform supported relational
  equality/rebuild mappings but cannot validate the full external trial alone.
- **Confidence:** High.
- **Relevance:** Prevents over-scoping the oracle.
