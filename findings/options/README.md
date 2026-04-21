# Backend Boundary Option Findings

This directory stores the second-pass viability analysis for the four backend
boundary options identified in `../synthesis.md`.

## Option Status

| Option | Note | Viability | Recommendation | Status |
| --- | --- | --- | --- | --- |
| 1. Native equality + DD/FlowLog rule evaluation | `option-1-native-equality-dd-rule-eval.md` | Medium | Continue as first hybrid prototype | Complete |
| 2. Proof/term encoding to DD | `option-2-proof-term-encoding-dd.md` | Low / Medium | Defer as main lowering; keep as specification/prototype oracle | Complete |
| 3. FlowLog/datatoad-like middle layer | `option-3-flowlog-datatoad-middle-layer.md` | Medium | Defer as long-term architecture | Complete |
| 4. No DD backend, borrow ideas | `option-4-no-dd-backend-borrow-ideas.md` | High as fallback | Continue as low-risk native improvement track | Complete |

## Relative Ranking

1. **Likely first experiment:** Option 1, because it tests DD/FlowLog where the
   fit is strongest while keeping equality/rebuild/container machinery native.
2. **Promising but deferred:** Option 3, because a FlowLog/datatoad-like middle
   layer is coherent but too large before the smaller data-exchange boundary is
   proven.
3. **High-risk research path:** Option 2, because proof/term encoding is a
   useful relational specification but incomplete and likely too expensive as a
   production lowering.
4. **Fallback/non-migration path:** Option 4, because borrowing WCOJ, provider,
   columnar, and profiling ideas is the safest path if backend migration does
   not justify itself.
