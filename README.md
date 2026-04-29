# Egglog Dataflow Evidence Bundle

This repository is now organized around one current question:

Can we prepare a minimal external Differential Dataflow trial that models a
small egglog subset, uses native egglog as the oracle, and explains how egglog
program concepts map into DD before any performance result is interpreted?

The phase-setting source is
[`messages/eli-meeting-april-29-2026.md`](messages/eli-meeting-april-29-2026.md).
That note supersedes the older "start inside egglog and build a backend
scaffold" handoff as the active next-phase framing.

## Current Phase

The repo is an evidence bundle, not an implementation plan. The active docs
should make a later MVP plan possible by keeping claims deduplicated, cited, and
separable from raw evidence.

Start here:

- [`findings/minimal-dd-trial.md`](findings/minimal-dd-trial.md): canonical
  current trail for the external DD trial question.
- [`findings/evidence-ledger.md`](findings/evidence-ledger.md): claim ledger
  connecting synthesized conclusions to source facts.
- [`findings/synthesis.md`](findings/synthesis.md): compact current synthesis
  that cites ledger IDs.
- [`findings/methodology.md`](findings/methodology.md): source inventory only.
- [`findings/README.md`](findings/README.md): findings index.

## Raw Evidence

Raw and low-level evidence is preserved in place:

- `messages/`: design conversations and meeting notes.
- `papers/`: local papers.
- `repos/`: vendored source checkouts used as evidence.
- `code/`: prototypes, including the DD design spike and schedule-overlap code.
- `findings/artifacts/`: generated JSON artifacts and measured lane outputs.
- `findings/source-notes/`: source-cluster notes that record what was read and
  why it matters.

The older backend-option and scaffold planning docs are archived under
[`findings/archive/2026-04-prior-backend-plans/`](findings/archive/2026-04-prior-backend-plans/).
They remain historical evidence, but active conclusions should cite the ledger
or raw sources instead of repeating those plans.

## Update Rule

When adding or changing synthesis, add or update a claim in
`findings/evidence-ledger.md` first. Active docs should cite ledger IDs such as
`E-001` and avoid reviving stale implementation framing unless it is clearly
marked as archived history.
