# Objective

Audit whether the surface implied by all Option 3 lanes is accidental adapter
duplication or expected replacement-backend scope.

# Commands

- `python3 code/option3-overlap/scripts/validate_option3_lanes.py findings/artifacts/option-3`
- Read the validated lane artifacts `02`, `06`, `07`, and `10`.

# Results

The implementation surface includes rule IR classification, per-rule
timestamps, DD frontiers, rebuild invalidation, same-id container refresh,
custom scheduler admission/backoff, native join strategy selection, WCOJ
kernels, and provider boundaries. Several of those surfaces are still only
partially measured: rebuild/container row counters are not exposed, WCOJ
examples compile but need graph inputs for runtime comparison, and the trace
lane lacks trace memory or compaction metrics.

# Verdict

The surface is too high for a permanent adapter that mirrors native state. It is
plausible backend scope if the new layer is authoritative for the moved
responsibilities.

# Option Implication

Do not build a permanent mirrored adapter. Keep Option 3 as a replacement
backend path gated by a vertical slice with single ownership and native egglog
as an oracle.
