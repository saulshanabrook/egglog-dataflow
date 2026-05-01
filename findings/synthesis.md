# Synthesis

This is the compact current synthesis for the external DD trial evidence bundle.
Detailed claim provenance lives in [`evidence-ledger.md`](evidence-ledger.md).

## Current Answer

The next useful phase is not another broad backend-option comparison and not an
in-place egglog backend scaffold. The evidence now points to a small external DD
trial that models a selected egglog subset, uses native egglog as the oracle,
and records how facts, functions, schedules, equality/rebuild, containers,
schedulers, and joins map into DD concepts before performance results are used
for design decisions (`E-001`, `E-002`, `E-003`).

## Current Conclusions

- The trial should optimize for interpretability first. The April 29 meeting
  note explicitly says the priority is understanding how egglog maps into DD so
  performance profiles are explainable (`E-003`).
- Native egglog should be the oracle and data/source generator for supported
  programs, not a mirrored production half of the trial (`E-002`, `E-011`).
- The preflight oracle should use lower function-table rows exported through
  `EGraph::function_for_each`; rendered `print-function` output is useful for
  debugging but should not be the primary row-comparison surface (`E-022`,
  `E-023`).
- The first runnable acceptance gate is relation-only over `i64`: native lower
  rows projected to logical tuples are compared against DD results for
  recursive path reachability, repeated-variable filtering, and a non-recursive
  three-way join (`E-024`, `E-025`).
- Materialize's production DD/Timely blog evidence sharpens how that gate
  should be interpreted: it is a semantic canary, not a performance benchmark,
  until memory/compaction, arrangement-aware join plans, and runtime/progress
  mechanisms are measured explicitly (`E-026`, `E-027`, `E-028`, `E-029`).
- The external model should stay small because current egglog crate boundaries,
  parser/typechecker/proof complications, and frontend transformations make an
  immediate invasive refactor a poor learning vehicle (`E-004`, `E-005`).
- Per-rule freshness and logical scheduling are mandatory semantic checks for
  any DD mapping, not optional performance details (`E-009`, `E-010`).
- Existing Option 3 artifacts strengthen the schedule-overlap subclaim, but
  they also show that permanent adapter mirroring is the wrong interpretation
  for the next phase (`E-011`, `E-012`).
- Equality rebuild, same-id container refresh, scheduler admission, and hidden
  primitive/callback reads are the main places where naive relational mappings
  can appear correct while missing real egglog behavior (`E-014`, `E-015`,
  `E-016`, `E-017`, `E-018`).
- WCOJ, `dataflow-join`, SRDatalog, and DBSP remain relevant substrate and
  planning references, but they are design inputs rather than proof that the
  egglog backend problem is solved (`E-019`, `E-020`).
- The Materialize blog corpus is useful production-system evidence for
  DD/Timely behavior, but it does not settle egglog-specific equality,
  rebuild, container, scheduler, or primitive semantics (`E-026`).

## Evidence Shape

Use [`minimal-dd-trial.md`](minimal-dd-trial.md) as the current trail for the
future plan. Use [`evidence-ledger.md`](evidence-ledger.md) to answer "why do we
believe this?" without duplicating long option summaries. Use
`code/minimal-dd-trial/` for the runnable preflight, raw artifacts under
`findings/artifacts/` for measured facts, and
`sources/materialize-blog/` for the Materialize blog snapshot plus relevance
index.

The archived docs under
[`archive/2026-04-prior-backend-plans/`](archive/2026-04-prior-backend-plans/)
are historical. They should be mined for evidence, not treated as the active
implementation direction.
