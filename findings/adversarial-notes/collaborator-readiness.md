# Adversarial Review: Collaborator Readiness

## Material Checked
- Paths and source clusters reviewed: `README.md`, `findings/README.md`, `findings/synthesis.md`, `findings/options/README.md`, `findings/methodology.md`, and `findings/source-notes/conversations-social.md`.

## Findings
- No P0/P1 findings.
- P2: The motivation section overstates the social payoff as if it were established. Evidence: `README.md:3-8`, `findings/methodology.md:20-27`, and `findings/source-notes/conversations-social.md:18-26`. Issue: phrases like "serious dataflow/database ecosystem," "independently reviewed and maintained project," and "working assumption is that engine-level issues with clear reproductions would receive serious attention" read like settled facts, but the source notes show interest and alignment, not proof of sustained upstream support or maintenance transfer. Recommended fix: rephrase these as hypotheses and attribute them explicitly to the conversations, for example "may improve maintenance leverage if upstream collaboration materializes."
- P2: The docs rely on dense insider jargon without a collaborator-oriented glossary or first-pass definitions. Evidence: `README.md:3-8`, `findings/synthesis.md:14-25`, `findings/synthesis.md:47-57`, `findings/synthesis.md:81-125`, and `findings/methodology.md:72-115`. Issue: DD, FlowLog, datatoad, WCOJ, e-matching, congruence closure, presorts, and "nested fixed points" are used as if the reader already knows the boundary between them. A cold collaborator can misread the options as more concrete than they are. Recommended fix: add a short glossary or inline definitions the first time each term appears, especially in the top-level README and option table.
- P2: The synthesis reads more conclusive than the evidence level warrants. Evidence: `findings/synthesis.md:8-25`, `findings/synthesis.md:129-143`, `findings/synthesis.md:173-185`, and `findings/synthesis.md:187-204`. Issue: the continue/stop criteria and option summaries are framed as if the necessary measurements already mostly exist, but the same document says the pass is provisional and that the decisive claims still need code-level inspection and measurements. Recommended fix: add explicit caveats that these are working hypotheses, not validated conclusions, and label each "evidence needed" item as uncollected.

## Missing Evidence
- Quantitative measurements showing whether DD/FlowLog/datatoad actually reduce runtime, retained state, rebuild work, or maintenance burden on representative egglog workloads.
- A collaborator-facing glossary for the option boundaries so readers can tell which parts stay native and which parts move to the substrate.
- Direct evidence for the social claim that issues would get serious upstream attention, beyond the local conversations cited in the source notes.

## Corrections Suggested
- `README.md`: soften the motivation paragraph and define the backend-option names before the table.
- `findings/synthesis.md`: mark the decision frame and option summaries as provisional hypotheses, not near-final conclusions.
- `findings/methodology.md`: tighten or qualify the social-maintenance claims in the motivation section.
- `findings/source-notes/conversations-social.md`: distinguish observed collaborator interest from inferred maintenance leverage.
- `findings/options/README.md`: add a short boundary glossary or inline definitions for the jargon in the option table.

## Confidence
- Medium. The review is grounded in the local docs and source notes, but it does not add fresh code or benchmark evidence, so the critique is about collaborator-facing clarity and evidentiary weight rather than technical correctness.
