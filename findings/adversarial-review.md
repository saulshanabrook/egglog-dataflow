# Adversarial Review Before Sharing

This review checks whether the collaborator-facing README, synthesis, and
backend option notes overstate the local source evidence. It integrates the five
review notes under `adversarial-notes/`.

## Executive Summary

- No P0/P1 contradictions were found. The current neutral tradeoff framing is
  shareable if it remains explicitly provisional.
- The main issue is evidentiary weight: several benefits are plausible
  hypotheses, not measured outcomes.
- The strongest missing design axis is provider-style relation boundaries:
  equality, containers, and rebuild-sensitive relations may need custom
  providers even if ordinary rule relations move to DD/FlowLog.
- The strongest technical caveats are proof-encoding coverage, custom scheduler
  semantics, timestamp/compaction cost, and rebuild invalidation.

## Findings

- **P0: None.** No direct contradiction, incorrect source attribution, or
  must-fix blocker was found.
- **P1: None.** No finding invalidates the current option framing.
- **P2: Source wording is sometimes stronger than the source notes support.**
  `README.md`, `findings/synthesis.md`, and `findings/options/README.md` should
  continue to label benefits as hypotheses and distinguish source evidence from
  inference (`adversarial-notes/source-trace-audit.md`).
- **P2: Social and maintenance payoff is inferred, not proven.** The
  conversations show interest and alignment, but they do not prove sustained
  upstream maintenance transfer or future responsiveness
  (`adversarial-notes/collaborator-readiness.md`,
  `source-notes/conversations-social.md`).
- **P2: Provider-style relation boundaries are under-factored.** The current
  four options mention providers mostly under Option 4, but the source notes
  suggest a separate architecture axis: ordinary relations could use DD/FlowLog
  while equality/container/rebuild-sensitive relations use specialized providers
  (`adversarial-notes/option-completeness.md`,
  `source-notes/extension-models.md`).
- **P2: Proof encoding is not a complete validation oracle.** It is useful as a
  relational specification for supported equality/rebuild behavior, but it does
  not cover the full container, Python, scheduler, presort, and custom frontend
  surface (`adversarial-notes/egglog-semantics.md`,
  `source-notes/egglog-core-proof.md`,
  `source-notes/containers-frontends.md`).
- **P2: Scheduler behavior is more invasive than match counts.** Current
  scheduler paths can materialize all matches, choose a subset, and delay
  action firing, so a DD-backed rule engine must preserve that contract if it
  supports custom schedulers (`adversarial-notes/egglog-semantics.md`).
- **P2: FlowLog/datatoad reuse should be described as inspiration.** The checked
  sources support planner shapes and WCOJ kernels as references, not direct
  reuse without an egglog-specific adapter, index layout, and recursive-control
  boundary (`adversarial-notes/dd-flowlog-substrate.md`,
  `source-notes/datalog-wcoj-planning.md`).
- **P2: Timestamp, compaction, and invalidation policy is unproven.** DD/Timely
  sources justify measuring coarse batching and compaction strategy, but they do
  not establish the right timestamp granularity. Rebuild invalidations and
  same-id dirty refresh should be modeled explicitly, not as ordinary tuple
  deletes alone (`adversarial-notes/dd-flowlog-substrate.md`,
  `source-notes/differential-timely.md`).

## Option Challenge Notes

- **Option 1: Native equality + DD/FlowLog rule evaluation.** Still plausible as
  a limited boundary, but the risk list should be read to include scheduler
  full-match/delayed-fire semantics and explicit rebuild-invalidation event
  types. A successful prototype needs semantic equivalence on rebuild,
  containers, and custom schedules, not just matching speed.
- **Option 2: Proof/term encoding to DD.** Strong as a partial relational
  specification, weak as a complete validation path. Unsupported container,
  presort, Python, primitive, and scheduler behavior must have separate native
  regressions before this option can support stronger conclusions.
- **Option 3: FlowLog/datatoad-like middle layer.** Broadly coherent as an
  architecture sketch, but FlowLog/datatoad/dataflow-join are references rather
  than drop-in engines. The cost includes an egglog-specific adapter, planner,
  index layout, and invalidation model.
- **Option 4: No DD backend, borrow ideas.** Still a low-migration-risk path,
  but it currently hides a bigger question: whether provider-style relation
  boundaries should become their own design option rather than only a native
  improvement technique.

## Unsupported Or Overstated Claims

- "Shared maintenance" should be framed as a hypothesis from collaborator
  interest, not a guaranteed outcome.
- "DD/FlowLog owns maintained rule indexes" should mean "could own" pending a
  prototype that proves the delta and scheduler boundary.
- "FlowLog/datatoad reuse" should mean "planner shape and join-kernel
  inspiration" until an adapter proves otherwise.
- "Proof encoding as oracle" should be qualified as partial because important
  container and frontend behavior is outside its supported subset.

## Missing Evidence Needed Before Recommendations

- A measured native-equality plus DD rule-evaluation prototype on at least one
  rebuild-heavy, one container-heavy, and one scheduler-sensitive case.
- A row-count and runtime comparison of native equality, proof/term encoding,
  and DD-style equality maintenance on constructor/rebuild examples.
- A WCOJ-vs-binary-join comparison on an egglog-shaped cyclic or repeated-
  variable e-matching rule.
- A timestamp/compaction experiment that measures progress traffic, retained
  trace state, and update churn for per-command, per-phase, and per-rule-batch
  epochs.
- A provider-boundary sketch or prototype comparing ordinary DD-backed
  relations with specialized equality/container/rebuild providers.
- A collaborator-facing glossary or equivalent short definitions for DD,
  FlowLog, datatoad, WCOJ, e-matching, rebuild, and presorts if the repo is
  shared beyond people already familiar with these terms.

## Doc Changes Made

- `README.md` now frames the social/maintenance payoff as a hypothesis and adds
  short definitions for DD and WCOJ.
- `README.md` now links to this adversarial review from "How To Pick This Up."
- `findings/README.md` now indexes this review and the per-lens review notes.
- `findings/synthesis.md` now includes an "Adversarial Review Update" caveating
  social payoff, FlowLog/datatoad reuse, timestamp/invalidation policy, proof
  encoding coverage, and provider-style boundaries.
- `findings/options/README.md` now notes that provider-style relation boundaries
  are a cross-cutting axis that may deserve a separate option or sub-option
  after comparison.

## Remaining Recommended Cleanup

- Keep the individual option notes as historical second-pass findings for now,
  but treat their `Recommendation` sections as less authoritative than the
  top-level README and synthesis.
- If this repository is sent to a broader audience, add a small glossary under
  `findings/` or expand the top-level README by one short definitions section.
