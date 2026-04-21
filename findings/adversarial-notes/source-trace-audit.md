# Adversarial Review: Source Trace Audit

## Material Checked
- `README.md`, `findings/synthesis.md`, `findings/options/README.md`.
- Source-note clusters in `findings/source-notes/`: `conversations-social.md`, `egglog-core-proof.md`, `differential-timely.md`, `datalog-wcoj-planning.md`, `extension-models.md`, and `containers-frontends.md`.

## Findings
- No P0/P1 findings. P2: the docs repeatedly state high-level conclusions as if settled when the source notes only support them as plausible or conditional. In `README.md:3-8`, `README.md:22-25`, and `findings/options/README.md:10-13`, claims like "shared performance and maintenance work," "DD/FlowLog could own maintained rule indexes," and "Preserves existing semantics" are presented without citations and go a bit beyond the source notes, which describe these as possible benefits, likely tradeoffs, or design patterns rather than proven outcomes. Recommended fix: add explicit source-note citations on those lines or soften the verbs to "suggests", "could", or "appears to".
- P2: `findings/synthesis.md:132-171` blurs synthesis with evidence in several places, especially "The source evidence supports DD/FlowLog/datatoad as plausible shared substrates" and the option tradeoff bullets that read like conclusions rather than inferences. The source notes do support these as candidates, but they do not establish that they are the right long-term substrate or that the listed blockers are definitive. Recommended fix: label these passages as inference from the notes, or attach citations per bullet and qualify the claims more tightly.

## Missing Evidence
- A measured prototype showing whether native equality plus DD rule evaluation preserves rebuild, schedule, and container behavior while actually reducing rule-matching cost.
- Rebuild-heavy and container-heavy comparisons that quantify canonical-id rewrites, same-id dirty-container refresh, and stale/duplicate match behavior.
- A cyclic or repeated-variable e-matching benchmark that compares binary joins against WCOJ/datatoad-style planning.
- A small custom-provider or relation-provider prototype proving whether specialized equality/container/storage behavior can stay native without erasing the maintenance advantage.

## Corrections Suggested
- `README.md:3-8`, `README.md:22-25`
- `findings/synthesis.md:132-171`
- `findings/options/README.md:10-13`

## Confidence
- Medium. The source notes clearly support the broad tradeoff map, but several headline statements in the three docs are phrased more definitively than the evidence in the notes warrants.
