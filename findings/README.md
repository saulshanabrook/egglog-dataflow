# Findings

This directory stores durable research context for the next phase: a minimal
external DD trial that models a small egglog subset, uses native egglog as an
oracle, and records how egglog concepts map to DD concepts before performance
claims are made.

## Active Entry Points

| Path | Purpose |
| --- | --- |
| [`minimal-dd-trial.md`](minimal-dd-trial.md) | Canonical current trail for goal, non-goals, mapping questions, benchmark categories, oracle requirements, and open planning inputs. |
| [`evidence-ledger.md`](evidence-ledger.md) | Claim ledger. Each synthesized conclusion has a source fact, source path, confidence, and relevance. |
| [`synthesis.md`](synthesis.md) | Compact current conclusions that cite ledger IDs. |
| [`methodology.md`](methodology.md) | Source inventory only: raw conversations, papers, vendored repos, artifacts, and reading order. |
| [`source-notes/`](source-notes/) | Detailed source-cluster notes from the reading passes. |
| [`experiments/option-3/README.md`](experiments/option-3/README.md) | Ordered index of existing Option 3 artifacts and measured lane outputs. |
| [`archive/2026-04-prior-backend-plans/`](archive/2026-04-prior-backend-plans/) | Superseded synthesized backend-option and scaffold planning docs. |

## Current Status

The April 29 Eli meeting note changes the active framing from "plan an in-repo
backend scaffold" to "prepare evidence for a small external DD model" (`E-001`,
`E-002`). The new docs therefore prioritize source-backed mapping questions and
oracle requirements over implementation milestones.

Older option-ladder and full-refactor docs are not deleted. They are archived
because they contain useful evidence and historical reasoning, but they also
repeat superseded implementation instructions (`E-011`, `E-012`, `E-013`).

## Update Discipline

- Preserve raw evidence: `messages/`, `papers/`, `repos/`, `code/`, and
  `findings/artifacts/`.
- Put new synthesized conclusions in [`evidence-ledger.md`](evidence-ledger.md)
  before repeating them in other active docs.
- In active docs, cite ledger IDs or raw source paths for non-obvious claims.
- Keep implementation sequences out of the active current-trail docs until a
  future MVP plan is explicitly requested.
