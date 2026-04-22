# Objective
Check the equality/rebuild invalidation path around table rebuild, UF canonicalization, and the `merge-during-rebuild.egg` / `repro-small-rebuild-fail-term-encoding.egg` regressions.

# Commands
`cargo test -p egglog-core-relations incremental_reinsert_canonicalizes_displaced_outer_id -- --nocapture`
`cargo run -- tests/merge-during-rebuild.egg`
`cargo run -- --term-encoding tests/repro-small-rebuild-fail-term-encoding.egg`

# Results
The core-relations unit test passed, and both direct egglog runs exited 0. The term-encoding repro emitted one non-fatal warning about a global that was missing a `$` prefix.

The current binaries do not expose counters for canonical-id rewrites, rebuild-index hits, row retractions, or reinserts, so those values stay source-inferred rather than measured.

# Verdict
The equality/rebuild invalidation lane is functioning on the observed cases.

# Option Implication
The rebuild path is still semantically sound on the regression files, but the repo would need explicit instrumentation if you want per-row canonicalization counts from the runtime.
