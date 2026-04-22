# Objective
Capture the schedule-overlap baseline for option 3 by running the harness tests plus the semantic and scaling suites, then summarize the oracle, broken-global, dd-barrier, and dd-overlap lanes from the measured output.

# Commands
- `cargo test --manifest-path code/option3-overlap/Cargo.toml`
- `cargo run --release --manifest-path code/option3-overlap/Cargo.toml -- --suite semantic --json-out findings/artifacts/option-3/raw/01-schedule-overlap-semantic.json`
- `cargo run --release --manifest-path code/option3-overlap/Cargo.toml -- --suite scaling --json-out findings/artifacts/option-3/raw/01-schedule-overlap-scaling.json`

# Results
The semantic mini workload passed for `oracle`, `dd-barrier`, and `dd-overlap`, while `broken-global` failed semantic equivalence and freshness as expected. Across the 60 scaling runs on `chain` and `fanout` at scales 8, 32, and 128, every DD run stayed semantically correct, every run had zero early visibility violations, candidate counts stayed bounded, and the maximum frontier lag observed was 3 tasks.

# Verdict
The schedule-overlap baseline is behaving as intended: the overlap lane preserves correctness, the broken-global lane demonstrates the intended failure, and the scaling sweep stays clean.

# Option implication
Option 3 has a stable baseline for comparing overlap against explicit barrier scheduling, with the broken-global case acting as a useful negative control.
