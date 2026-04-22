**Objective**
Classify representative egglog rule shapes from `repos/egglog/core-relations/src/tests.rs` plus the sibling container regressions.

**Commands**
`rg` and `sed` over `repos/egglog/core-relations/src/tests.rs`, `repos/egglog/core-relations/src/free_join/plan.rs`, `repos/egglog/core-relations/src/free_join/mod.rs`, `repos/egglog/core-relations/src/containers/mod.rs`, and `repos/egglog/tests/container-fail.egg` and `container-rebuild.egg`.

**Results**
`line_graph_1`, `line_graph_2`, and `intersection` are small binary-join shapes; `minimal_ac` and the `ac_test_inner` rebuild loop are join-shaped but dominated by timestamped rebuild and saturation control; `container-fail` and `container-rebuild` are unary container-rewrite cases where rebuild visibility is the real issue.

**Verdict**
Static classification only. Semantic and freshness fields are null because this lane is about shape taxonomy, not a runtime claim.

**Option Implication**
Use the binary shapes for join-strategy comparisons. Treat the AC and container cases as rebuild-sensitive stressors, not fair WCOJ baselines.
