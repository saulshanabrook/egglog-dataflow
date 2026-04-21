# Adversarial Review: Egglog Semantics

## Material Checked
- `findings/source-notes/egglog-core-proof.md`
- `findings/source-notes/containers-frontends.md`
- `findings/options/option-1-native-equality-dd-rule-eval.md`
- `findings/options/option-2-proof-term-encoding-dd.md`
- `findings/options/option-3-flowlog-datatoad-middle-layer.md`
- `findings/options/option-4-no-dd-backend-borrow-ideas.md`
- `repos/egglog/src`
- `repos/egglog/core-relations`
- `repos/egglog/union-find`
- `repos/egglog/src/proofs`
- `repos/egglog-python/docs/explanation/2026_02_containers.md`
- `repos/egglog-python/docs/reference/egglog-translation.md`
- `repos/egglog-python/python/egglog/builtins.py`
- `repos/egglog-experimental`

## Findings
- No P0/P1 issues surfaced in the checked material; the remaining gaps are P2.
- P2: The proof/term-encoding path is not a complete semantic oracle for the frontend surface that containers and Python users actually exercise. `repos/egglog/src/proofs/proof_encoding_helpers.rs` rejects presort/custom container sorts, lookups in actions, primitives in proofs, and non-global `:no-merge` functions, while `repos/egglog-python/python/egglog/builtins.py` and `repos/egglog-python/docs/reference/egglog-translation.md` expose `Map`/`Set`/`MultiSet`/`Vec`, `rebuild`, higher-order container ops, `run`, `saturate`, `seq`, `repeat`, and `run-with`. Issue: a DD migration that leans on proof encoding for validation still cannot check the most user-visible container/scheduler behavior, so the note currently understates the size of the native escape hatch required. Recommended fix: call this out explicitly as a validation blocker in the core/proof notes and add a separate native regression path for unsupported frontend features.
- P2: Custom scheduler semantics are more invasive than "match-count visibility" alone. `repos/egglog/src/scheduler.rs` materializes all matches, lets the scheduler choose a subset, instantiates those matches, and then reruns action rules; `repos/egglog-experimental/src/scheduling.rs` also splits the same schedule into multiple public-API runs because the scheduler is created and driven through the host layer. Issue: any DD-backed rule engine that wants to preserve `run-with`, `saturate`, and backoff behavior has to preserve this full-match/delayed-fire contract, or it will change observable scheduler semantics and likely erase the incremental win. Recommended fix: add this as a first-class blocker in the containers/frontends note and in option 1's risk list, not just as a planner concern.

## Missing Evidence
- A minimal end-to-end reproduction showing one unsupported proof-mode frontend feature, one scheduler feature, and one container rebuild case all on the same branch.
- A measured comparison of native vs proof-encoded execution on a container-heavy program, including how often proof mode forces a fallback to host-side checks.
- A small scheduler trace showing how many matches are materialized and retained per iteration under `run-with` for a backoff workload.

## Corrections Suggested
- `findings/source-notes/egglog-core-proof.md`: add an explicit blocker that proof encoding cannot validate the full container/Python frontend surface.
- `findings/source-notes/containers-frontends.md`: add the scheduler materialization/delayed-fire constraint as a concrete blocker, not just a planner risk.
- `findings/options/option-1-native-equality-dd-rule-eval.md`: expand "Main Risks" with the proof-encoding validation gap and the full-match scheduler contract.
- `findings/options/option-2-proof-term-encoding-dd.md`: tighten the "What Stays Native" and "Main Risks" sections to state that proof mode cannot stand in for container/scheduler validation.

## Confidence
- Medium: the source-backed blockers are real and local, but I did not run a cross-cutting benchmark or full branch-level reproduction to quantify how much each gap matters.
