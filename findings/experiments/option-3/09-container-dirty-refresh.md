# Objective
Check the container dirty-refresh lane, especially the same-id versus changed-id split that feeds `dirty_ids` and the refresh follow-up.

# Commands
`cargo test -p egglog-core-relations nonincremental_dirty_ids_only_include_stable_ids -- --nocapture`
`cargo run -- tests/container-rebuild.egg`
`cargo run -- tests/container-fail.egg`
`cargo run -- tests/vec.egg`

# Results
The unit test passed and confirmed the split: same-id semantic changes produced one dirty id, while changed-id cases produced none. The three direct egglog runs all exited 0, including `container-fail.egg`.

The current binaries do not expose a direct refresh-row counter, so the actual number of refreshed parent rows is still source-inferred from `refresh_rows_for_values`.

# Verdict
The dirty-id lane is implemented as expected, with the same-id refresh path separated from the changed-id rebuild path.

# Option Implication
Container refresh needs an equivalent first-class invalidation signal. The
native `dirty_ids` split is oracle evidence for the behavior, not a required
implementation shape for a replacement backend. Runtime counter coverage is
still thin.
