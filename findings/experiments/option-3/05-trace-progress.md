# Objective
Summarize the trace/progress baseline lane for option 3 from the scaling sweep, focusing on frontier lag, worker/window behavior, candidate counts, and what the harness can and cannot measure.

# Commands
- `cargo run --release --manifest-path code/option3-overlap/Cargo.toml -- --suite scaling --json-out findings/artifacts/option-3/raw/01-schedule-overlap-scaling.json`

# Results
The scaling sweep stayed semantically correct and fresh across all 60 runs. Frontier lag was bounded by the overlap window: window 1 stayed at lag 0, window 2 reached lag 1, and window 4 reached lag 3. The highest-lag sample came from `chain` at scale 128 with 4 workers, where the harness showed `issued_through_task` advancing three tasks ahead of the committed task. Candidate counts ranged from 24 to 8256, stale candidates stayed at 0, and the harness reported no direct trace memory or compaction counters.

# Verdict
This lane is a usable proxy for trace/progress behavior, but it only measures frontier lag and candidate flow, not trace size or compaction.

# Option implication
The overlap window is the main control knob for visible progress in the current harness, and the measured lag bound is small enough to keep the DD overlap variant attractive as the non-barrier baseline.
