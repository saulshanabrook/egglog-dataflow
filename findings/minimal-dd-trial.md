# Minimal External DD Trial

This is the canonical current trail for organizing evidence before a future MVP
plan. It is not an implementation plan.

## Goal

Prepare a minimal external DD trial that models a small egglog subset outside
the production egglog backend, uses native egglog as the oracle, and records how
egglog concepts map into DD concepts before performance results are interpreted
(`E-001`, `E-002`, `E-003`).

The useful output of this phase is a queryable evidence base: which egglog
features are in the subset, which DD concepts they map to, which native
behaviors are oracle-only, and which performance profiles are explainable by
that mapping.

## Non-Goals

- Do not produce the MVP implementation plan in this pass (`E-003`).
- Do not start by changing `repos/egglog` or building an in-repo backend
  scaffold (`E-001`, `E-004`).
- Do not treat proof/term encoding as a complete oracle for the current
  frontend surface (`E-021`).
- Do not interpret WCOJ, GPU, or DBSP evidence as solving egglog-specific
  equality, rebuild, container, scheduler, or primitive semantics (`E-019`,
  `E-020`).
- Do not keep a permanent native/DD mirrored design as the intended shape
  (`E-012`).

## Why This Phase Emerged

The older option-ladder docs established that a permanent middle-layer adapter
would mirror too much native state. The current artifacts downgrade that adapter
reading while leaving a future single-owner backend possible (`E-011`, `E-012`).

The April 29 Eli meeting changed the immediate question. The next useful work
is a stripped-back external model that is simple enough to inspect in profiles,
uses egglog programs and native results as oracle evidence, and helps explain
how top-down patterns, arbitrary joins, multi-patterns, schedules, and selected
benchmarks map into DD (`E-001`, `E-002`, `E-003`).

## Mapping Questions

- **Program boundary:** Which egglog input form should feed the external trial:
  generated `.egg` programs, parsed commands, or a lowered rule representation?
  `CoreRule` / `ResolvedCoreRule` is evidence for a nearby relational boundary,
  but the current phase should treat it as an oracle-facing candidate rather
  than an in-repo scaffold instruction (`E-013`).
- **Relations and functions:** Which subset of relations, function tables,
  constructors, merges, and repeated-variable constraints can be represented
  directly as DD collections, arrangements, joins, filters, and reductions
  (`E-007`, `E-008`)?
- **Schedules and freshness:** How does the trial preserve per-rule last-run
  freshness under arbitrary egglog schedules, and which DD timestamps,
  frontier gates, or data-level `row_ts` fields explain that behavior (`E-009`,
  `E-010`)?
- **Equality and rebuild:** Which equality/canonicalization event is included
  only as an oracle observation, and which row rewrite/retraction facts are
  explicitly modeled in the trial (`E-014`, `E-015`)?
- **Containers:** Which container behavior is in scope for mapping, and how are
  same-id semantic changes represented so seminaive visibility does not miss
  rows (`E-016`)?
- **Schedulers:** If a benchmark needs custom scheduling, does the trial model
  complete match materialization, selected admission, residual matches, and
  barrier-delayed actions (`E-017`)?
- **Primitives and callbacks:** Which host-side primitive or higher-order
  callback reads must be visible as matched inputs, declared dependencies, or
  oracle-only behavior (`E-018`)?
- **Join strategy:** Which workloads are fair DD join/WCOJ trials, and which
  are mainly equality, rebuild, container, or scheduler stressors (`E-019`)?
- **Alternative substrate lens:** Whether DBSP's automatic differentiation
  model explains any part of the mapping better than manual DD lowering remains
  a reading question, not an implementation choice (`E-020`).

## Preflight Mapping Decisions

These decisions are settled enough to start the first relation-only coding
gate:

- **Oracle readback:** Use lower function-table snapshots from
  `EGraph::function_for_each` as the primary oracle. `FunctionRow` exposes row
  values and subsumption status without internal timestamp/subsume columns, so
  the first slice does not require an egglog API change (`E-002`, `E-022`).
- **Debug output:** Keep `print-function` only as a display/debug fallback
  because it routes through `function_to_dag` / `TermDag`, not as the primary
  lower-row oracle (`E-023`).
- **Row identity:** For the v0 relation slice, compare logical `i64` input
  tuples and retain raw lower-row values, output value, sort names, and
  `subsumed` as debug evidence. Synthetic constructor/e-class output ids are
  not the logical relation identity (`E-022`, `E-024`).
- **Initial subset:** Start with `i64` relation facts, relation atoms,
  repeated-variable filters, joins, and explicit staged `(run ...)` boundaries.
  Scope out merge/equality, rebuild, containers, custom schedulers, host
  callbacks, extraction, and proof encoding until the row oracle and relation
  DD model are working (`E-018`, `E-021`, `E-024`).
- **Program boundary:** Pair a native `.egg` fixture with a small external trial
  scenario spec. Do not block the first slice on direct `ResolvedCoreRule`
  export; use the existing canary evidence to keep the later rule-boundary
  mapping honest (`E-013`, `E-024`).
- **Schedule boundary:** Snapshot after explicit native stages and compare
  final visible rows. Per-rule freshness internals remain a follow-up schedule
  witness once the relation slice can already compare against native rows
  (`E-009`, `E-010`, `E-024`).

These questions should be explored after the first row-oracle comparison is
running:

- equality/rebuild row rewrites and replayable live-row deltas (`E-014`,
  `E-015`);
- same-id container refresh (`E-016`);
- scheduler materialization, admission, residual matches, and delayed actions
  (`E-017`);
- hidden primitive or callback reads (`E-018`);
- WCOJ/hard-join planning and DBSP as an explanatory lens (`E-019`, `E-020`).

## Benchmark Categories

The April 29 meeting note separates benchmark interpretation into at least
three categories (`E-006`):

- High-throughput workloads: few rules over large data, where a DD-like
  substrate might plausibly win.
- Tiny-iteration workloads: many small iterations, where fixed setup/build cost
  may dominate.
- Hard joins: benchmarks such as hardboiled-style workloads or graph/WCOJ
  shapes where join planning and data layout are the main question.

Additional stress categories are needed for explanation, not necessarily for
the first performance comparison:

- schedule/freshness witnesses (`E-010`, `E-011`);
- equality/rebuild and replayable row-delta witnesses (`E-014`, `E-015`);
- container dirty-refresh and higher-order container witnesses (`E-016`);
- scheduler materialization/admission witnesses (`E-017`).

## Oracle Requirements

- Native egglog supplies the supported-program oracle and can generate test
  databases or expected step-visible state (`E-002`).
- The primary preflight oracle is lower function-table export through
  `function_for_each`, not rendered `print-function` output (`E-022`, `E-023`).
- The trial should compare at logical boundaries that matter to egglog:
  per-rule freshness, visible rows, match/action admission, rebuild visibility,
  and final relation/function table state (`E-009`, `E-010`, `E-017`).
- DD outputs that include signed diffs must be netted before committing visible
  host-side effects (`E-014`).
- Rebuild/canonicalization evidence should distinguish committed live-row
  deltas from merge-only collision evidence (`E-015`).
- Proof/term encoding can inform supported equality/rebuild mappings, but it is
  not a complete oracle for containers, schedulers, presorts, primitives,
  Python-facing APIs, or custom frontend behavior (`E-021`).

## Open Planning Inputs

The mapping preflight resolves these v0 inputs:

- first subset: relation-only `i64` reachability;
- external artifact shape: `code/minimal-dd-trial/`;
- oracle interface: lower rows exported with `function_for_each`;
- first comparison family: path/reachability;
- first comparison identity: sorted logical input tuples plus raw lower-row
  debug evidence (`E-022`, `E-024`).

A future MVP plan still needs to choose:

- the measurement schema for runtime, build/setup cost, rows, diffs, frontiers,
  trace/compaction, and oracle mismatches;
- the rule shape expansion beyond relation reachability: top-down patterns,
  arbitrary joins, multi-patterns, schedules, containers, or hard joins;
- whether later rule extraction should consume generated `.egg`, parsed
  commands, canary JSON, or a public lowered-rule export.
