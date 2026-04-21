# Backend Boundary Option Findings

This directory stores the second-pass tradeoff analysis for the four backend
boundary options identified in `../synthesis.md`.

These tradeoffs are working hypotheses. The adversarial review flags
provider-style relation boundaries as a cross-cutting design axis that may
deserve a separate option or sub-option after a concrete comparison
(`../adversarial-review.md`).

## Option Tradeoffs

| Option | Main Benefit | Long-Term Cost / Blocker | Note |
| --- | --- | --- | --- |
| 1. Native equality + DD/FlowLog rule evaluation | Tests maintained relational matching and indexing while keeping equality/rebuild/container behavior native. | Needs a precise delta interface for canonical-id changes, rebuild invalidations, container refresh, match deduplication, and action handoff. | [Option 1](option-1-native-equality-dd-rule-eval.md) |
| 2. Proof/term encoding to DD | Provides a concrete relational specification for equality, UF/view/rebuild tables, and proof-oriented experiments. | Current encoding is slow, incomplete for current egglog features, and incompatible with container/presort semantics without a native side channel. | [Option 2](option-2-proof-term-encoding-dd.md) |
| 3. FlowLog/datatoad-like middle layer | Could become a coherent long-term relational planner with DD execution and WCOJ-style join kernels. | Requires a substantial new planner, index universe, recursive-control model, and egglog-specific rebuild/equality operators. | [Option 3](option-3-flowlog-datatoad-middle-layer.md) |
| 4. No DD backend, borrow ideas | Preserves existing semantics while incrementally adopting WCOJ planning, provider interfaces, columnar storage, profiling, or cleaner rule IR boundaries. | Does not answer the shared-substrate motivation and leaves most database/runtime complexity inside egglog. | [Option 4](option-4-no-dd-backend-borrow-ideas.md) |

## Tradeoff Summary

- Option 1 is the smallest migration surface, but it may force egglog and DD to
  maintain overlapping indexes and carefully synchronized deltas.
- Option 2 is a clear specification path, but it currently looks too
  expensive and feature-incomplete for production lowering.
- Option 3 has broad architecture upside, but it is a large system design
  project rather than a small backend substitution.
- Option 4 avoids migration risk, but it gives up most of the social and
  maintenance benefit of sharing a substrate with DD/FlowLog/datatoad.
