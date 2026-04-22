# Objective
Check whether native action/rebuild boundaries behave like hard visibility barriers in Egglog by running the concrete regressions under `repos/egglog/tests`.

# Commands
`cargo test -p egglog --test files container_rebuild -- --exact --nocapture`

`cargo test -p egglog --test files merge_during_rebuild -- --exact --nocapture`

`cargo test -p egglog --test files container_fail -- --exact --nocapture`

`cargo test -p egglog --test files repro_should_saturate -- --exact --nocapture`

`cargo test -p egglog --test files schedule_demo -- --exact --nocapture`

# Results
All five runs passed. `container_rebuild` showed rebuilt constructor materialization after rebuild, `merge_during_rebuild` preserved the distance merge through rebuild, `container_fail` passed only after the rebuilt container child existed, and `repro_should_saturate` stabilized after the single eligible firing.

The `schedule_demo` run also passed and confirmed the expected alternating schedule counts, but it is supporting evidence rather than the core rebuild-boundary regression.

# Verdict
Native action/rebuild boundaries act like hard visibility barriers for these cases.

# Option implication
Adapter and hybrid DD/FlowLog designs should treat native rebuild/action
behavior as an explicit barrier or handoff point. A replacement backend must
model equivalent visibility boundaries internally rather than depending on
native state for moved responsibilities.
