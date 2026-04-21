# Adversarial Review: Option Completeness

## Material Checked
- `findings/options/option-1-native-equality-dd-rule-eval.md`
- `findings/options/option-2-proof-term-encoding-dd.md`
- `findings/options/option-3-flowlog-datatoad-middle-layer.md`
- `findings/options/option-4-no-dd-backend-borrow-ideas.md`
- `findings/synthesis.md`
- `findings/source-notes/egglog-core-proof.md`
- `findings/source-notes/containers-frontends.md`
- `findings/source-notes/differential-timely.md`
- `findings/source-notes/datalog-wcoj-planning.md`
- `findings/source-notes/extension-models.md`

## Findings
- No P0/P1 finding. P2: the option set misses a materially different backend boundary by not making the provider-style split a first-class choice. Evidence: the source notes treat custom relation providers as a distinct ABI and design pattern (`findings/source-notes/extension-models.md`), but the synthesis compares only monolithic DD-vs-native paths and buries providers inside option 4. Issue: this collapses a separate architecture axis, because the hardest blockers here are not just "how much goes to DD" but whether equality, container, and rebuild-sensitive relations need specialized providers while ordinary relations move to DD. Recommended fix: add an explicit option or sub-option for a provider-based boundary, and compare it against the current DD/native split on dirty-id refresh, rebuild invalidation, and merge-during-rebuild behavior.

## Missing Evidence
- No direct comparison was found between a provider-based backend split and the current DD/native options.
- No measurement was found showing whether dirty-id refresh and rebuild invalidation can be handled by a provider boundary without recreating a full backend.
- No example was found that separates ordinary relation maintenance from equality/container maintenance in a way that would justify collapsing provider support into the existing options.

## Corrections Suggested
- `findings/synthesis.md`: add a separate backend-boundary entry for a provider-style split, rather than treating provider support as a subpoint of "no DD backend".
- `findings/options/option-4-no-dd-backend-borrow-ideas.md`: split the provider-interface discussion into its own boundary claim and blocker list.
- `findings/options/option-1-native-equality-dd-rule-eval.md` and `findings/options/option-3-flowlog-datatoad-middle-layer.md`: state explicitly that their DD scope excludes provider-based equality/container backends, so the comparison is not silently overlapping.

## Confidence
- Medium. The omission is clear from the source-note framing, but the right way to factor it into the decision tree still needs an explicit comparison or prototype.
