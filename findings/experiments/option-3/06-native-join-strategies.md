**Objective**
Compare the native egglog join strategies on representative query shapes.

**Commands**
`cargo test --manifest-path repos/egglog/core-relations/Cargo.toml line_graph_1 -- --nocapture`
`cargo test --manifest-path repos/egglog/core-relations/Cargo.toml line_graph_2 -- --nocapture`
`cargo test --manifest-path repos/egglog/core-relations/Cargo.toml intersection -- --nocapture`

**Results**
All three strategy families passed on all three workloads: `line_graph_1`, `line_graph_2`, and `intersection`. The planner code in `free_join/plan.rs` still distinguishes `Gj` from `PureSize` and `MinCover`; these runs only confirm that the strategies preserve output on the smoke shapes.

**Verdict**
Pass. This lane is a correctness/equivalence probe, not a semantic or freshness evaluation.

**Option Implication**
The comparison here is about plan shape and stage count. Use it to judge efficiency, not result differences.
