# Objective
Check the custom scheduler/backoff lanes against the current implementation in `repos/egglog/src/scheduler.rs` and `repos/egglog-experimental/src/scheduling.rs`, then run the feasible workloads.

# Commands
`cargo test -p egglog-experimental --test files math_backoff -- --exact --nocapture`

`cargo test -p egglog-experimental --test files repro_scheduler_scopes -- --exact --nocapture`

`cargo test -p egglog --test files schedule_demo -- --exact --nocapture`

# Results
`math_backoff` passed and reached the large final materialization expected from the back-off scheduler. The final print-size summary showed substantial match activity, which is consistent with the scheduler admitting full matches until the limit is crossed and then backing off.

`repro_scheduler_scopes` passed and confirmed that scheduler bindings stay scoped across nested `repeat` and `saturate` blocks.

`schedule_demo` also passed as supporting schedule-boundary evidence.

# Verdict
The custom scheduler/backoff lane is working: admission, banning, unbanning, and scheduler scoping all line up with the source.

# Option implication
Option 3 can rely on the custom scheduler path for controlled backoff instead of baking the throttling into ad hoc schedule rewrites.
