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

A future MVP plan should choose these, but this doc intentionally does not:

- the exact egglog subset;
- the external crate or artifact shape;
- the oracle extraction/readback interface;
- the first benchmark family from the categories above;
- the measurement schema for runtime, build/setup cost, rows, diffs, frontiers,
  trace/compaction, and oracle mismatches;
- the rule shape coverage: top-down patterns, arbitrary joins, multi-patterns,
  schedules, containers, or hard joins.
