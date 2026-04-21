# Adversarial Review: DD/Timely/FlowLog Substrate

## Material Checked
- `findings/source-notes/differential-timely.md`
- `findings/source-notes/datalog-wcoj-planning.md`
- `findings/options/option-1-native-equality-dd-rule-eval.md`
- `findings/options/option-3-flowlog-datatoad-middle-layer.md`
- `repos/differential-dataflow/differential-dataflow/src/operators/iterate.rs`
- `repos/differential-dataflow/differential-dataflow/src/operators/join.rs`
- `repos/differential-dataflow/differential-dataflow/src/operators/reduce.rs`
- `repos/differential-dataflow/differential-dataflow/src/trace/mod.rs`
- `repos/differential-dataflow/mdbook/src/chapter_5/chapter_5.md`
- `repos/differential-dataflow/mdbook/src/chapter_5/chapter_5_3.md`
- `repos/differential-dataflow/mdbook/src/chapter_5/chapter_5_4.md`
- `repos/timely-dataflow/mdbook/src/chapter_3/chapter_3_1.md`
- `repos/timely-dataflow/mdbook/src/chapter_4/chapter_4_2.md`
- `repos/flowlog/README.md`
- `repos/flowlog/crates/planner/src/stratum_planner.rs`
- `repos/flowlog/crates/planner/src/rule_planner/core.rs`
- `repos/flowlog/crates/planner/src/rule_planner/sip.rs`
- `repos/flowlog/crates/optimizer/src/plan_tree.rs`
- `repos/flowlog/crates/compiler/src/flow/recursive.rs`
- `repos/datatoad/README.md`
- `repos/datatoad/src/rules/plan.rs`
- `repos/datatoad/src/rules/exec.rs`
- `repos/datatoad/src/facts/trie.rs`
- `repos/dataflow-join/src/lib.rs`
- `repos/dataflow-join/src/extender.rs`
- `repos/dataflow-join/src/index.rs`
- `code/dd-pr-525-eqsat.rs`

## Findings
- No P0/P1 findings. The issues below are P2, but they are still worth tightening before the substrate story is treated as settled.
- P2: The planner-reuse claim is stronger than the code supports. Evidence: FlowLog's `PlanTree::from_catalog` is explicitly a left-deep source-order chain with `get_first_join_tuple_index()` hard-coded to `(0, 1)` (`repos/flowlog/crates/optimizer/src/plan_tree.rs:47-82`), SIP is an ad hoc pairwise projection+semijoin pass that stops short of a general strategy (`repos/flowlog/crates/planner/src/rule_planner/sip.rs:14-16`, `:31-70`), and datatoad's WCOJ path depends on a separate `Salad`/`Forest<Terms>` layout plus `count`/`propose`/`intersect` passes over that layout (`repos/datatoad/src/rules/exec.rs:9-37`, `:109-165`, `repos/datatoad/src/facts/trie.rs:23-33`, `:148-255`). Issue: the notes currently read as though FlowLog/datatoad can be "reused" as-is, when the checked sources show they are only design references and require an egglog-specific adapter and data layout. Recommended fix: rephrase the option notes to say "planner shape" and "join-kernel inspiration," not planner/kernel reuse, and move any stronger reuse claims behind a concrete adapter prototype.
- P2: The timestamp/compaction guidance is still a hypothesis, not a conclusion. Evidence: Timely only says progress work scales with introduced timestamps and warns that using a new timestamp per record can hurt badly (`repos/timely-dataflow/mdbook/src/chapter_3/chapter_3_1.md:17-19`); DD `iterate` warns that non-consolidated differences can circulate indefinitely and that `Variable::new_from` can produce non-positive differences that make non-linear operators tricky (`repos/differential-dataflow/differential-dataflow/src/operators/iterate.rs:12-15`, `:52-55`, `:232-245`); and trace compaction is frontier-driven, with logical/physical compaction tied to what future queries remain possible (`repos/differential-dataflow/differential-dataflow/src/trace/mod.rs:111-150`, `repos/differential-dataflow/mdbook/src/chapter_5/chapter_5_3.md:89-95`). Issue: the notes currently imply that per-epoch or per-rebuild batching is the right timestamp granularity, but the sources only justify "batch aggressively and measure" rather than a settled granularity. Recommended fix: say explicitly that timestamp granularity is an empirical tuning choice and require workload measurements before claiming phase-level batching is sufficient.
- P2: Retractions and rebuild invalidations are under-specified if they are treated as ordinary tuple deletes. Evidence: DD traces model algebraic differences plus frontier-controlled compaction, not semantic rebuild events (`repos/differential-dataflow/differential-dataflow/src/trace/mod.rs:111-150`); FlowLog's recursive lowering is built around generated IDB unions, feedback variables, and leave outputs, but it does not model egglog's rebuild-triggered invalidation stream (`repos/flowlog/crates/compiler/src/flow/recursive.rs:31-170`); and datatoad/dataflow-join likewise only expose fact insertion/update and streaming join operators, not representative churn or same-id semantic refresh (`repos/datatoad/src/rules/exec.rs:9-37`, `repos/dataflow-join/src/lib.rs:40-91`, `repos/dataflow-join/src/index.rs:17-29`). Issue: without an explicit invalidation class, the substrate story can miss rows that become newly matchable after a rebuild even when tuple identity did not change. Recommended fix: add an explicit rebuild-invalidations or same-id-dirty event type to the interface docs and stop describing ordinary retractions as sufficient coverage for equality churn.

## Missing Evidence
- No measured comparison between DD arrangements and a trie/columnar WCOJ backend on the same e-matching query shape.
- No workload data showing which timestamp granularity minimizes progress overhead without blowing up trace history.
- No end-to-end example that exercises rebuild-triggered invalidations, same-id dirty refresh, and recursive control together.
- No direct evidence that FlowLog's current recursive lowering can absorb egglog equality churn without an external invalidation channel.

## Corrections Suggested
- `findings/source-notes/datalog-wcoj-planning.md:25-31` should qualify "reuse" language and distinguish planner shape from actual reusable machinery.
- `findings/source-notes/differential-timely.md:21-27` and `:42-47` should mark timestamp batching and arrangement reuse as hypotheses that still need workload measurements.
- `findings/options/option-1-native-equality-dd-rule-eval.md:7-13` and `:22-28` should mention rebuild-invalidations explicitly instead of relying on inserts/deletes/retractions alone.
- `findings/options/option-3-flowlog-datatoad-middle-layer.md:6-23` should say the middle layer needs an egglog-specific adapter and separate index layout, not just DD-backed reuse of FlowLog/datatoad components.

## Confidence
- Medium. The checked code clearly supports the cautionary points about planner simplicity, timestamp overhead, compaction frontiers, and WCOJ data layout, but I did not run an end-to-end workload to quantify the cost of each mismatch.
