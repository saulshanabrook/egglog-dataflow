# Full DD Refactor Design Spike

Date: 2026-04-28

## Executive Decision

The simplest durable architecture is a full execution refactor around the
existing egglog frontend and `CoreRule` IR:

- Keep parsing, desugaring, typechecking, command orchestration, and
  `ResolvedCoreRule`/`CoreRule` lowering in the `egglog` crate.
- Replace the `egglog-bridge` plus `core-relations` execution stack with a
  DD-owned runtime that is the only production backend.
- Use native egglog only as an external oracle during development. Do not keep a
  permanent mirrored native/DD runtime split.
- Start with compiled per-rule or per-ruleset DD fragments over long-lived
  relation inputs and arrangements. Keep a generic tagged-row interpreter only
  as a fallback if churn or drop stress fails.

This changes the earlier bridge-preserving plan: the bridge API is now treated
as migration scaffolding, not as a design constraint.

## Evidence Gathered In This Spike

### Repo boundary evidence

- `EGraph` currently owns `egglog_bridge::EGraph`, function backend ids,
  rulesets, schedulers, and proof state in `repos/egglog/src/lib.rs:241`.
- Rule execution flows from `run_schedule` to `step_rules`, then directly into
  `backend.run_rules` in `repos/egglog/src/lib.rs:866` and
  `repos/egglog/src/lib.rs:935`.
- The stable compiler boundary is already nearby: `CoreRule` is documented as a
  conjunctive query body plus SSA-like actions in `repos/egglog/src/core.rs:1`,
  and resolved rules live at `repos/egglog/src/core.rs:870`.
- `egglog-bridge` documents itself as the layer that elaborates egglog
  seminaive/default/merge behavior onto `core-relations`, not as the algorithm
  layer, in `repos/egglog/egglog-bridge/src/lib.rs:1`.
- The bridge nevertheless owns visible semantics: it runs rules, advances
  seminaive timestamps, rebuilds after union growth, rebuilds containers before
  tables, refreshes dirty parent rows, and loops rebuild to fixed point in
  `repos/egglog/egglog-bridge/src/lib.rs:465` and
  `repos/egglog/egglog-bridge/src/lib.rs:500`.
- The scheduler path is also backend-shaped: it materializes query matches,
  asks a scheduler to choose matches, writes a worklist through `TableAction`,
  flushes, then runs action rules in `repos/egglog/src/scheduler.rs:166`.

Conclusion: preserving `egglog-bridge` as the DD boundary would preserve the
current accidental split and force DD to mirror native backend semantics. The
frontend/backend boundary should move up to command orchestration plus
`ResolvedCoreRule`.

### DD lifecycle experiment

This spike added `code/dd-design-spike/`, a throwaway DD prototype. It validated
the minimal dynamic-install path:

- build one compiled dataflow in a long-lived Timely worker;
- run an `A -> B` rule with `InputSession<u64, Row, isize>`, `row_ts`, a task
  stream, and a probe visibility gate;
- after that worker has advanced, install a second compiled dataflow with new
  inputs and run `B -> C`;
- feed derived rows host-side at the next logical epoch;
- read back a maintained arrangement through a cursor;
- set logical and physical compaction frontiers; and
- drop both dataflows with `Worker::drop_dataflow`.

Verification:

```sh
cargo run --manifest-path code/dd-design-spike/Cargo.toml
```

Observed:

```text
derived_b_rows=[Row { value: 101, row_ts: 2 }]
c_trace_contents={Row { value: 201, row_ts: 4 }: 1}
installed_after_drop=[]
```

This was enough to choose compiled DD fragments for the first design. The
follow-up lifecycle churn and production-shaped lifecycle experiments below
strengthen that choice: churn covers bounded install/drop behavior, and the
production-shaped run imports persistent traces, feeds recursive results back at
barriers, and consolidates signed output diffs before host-side action
visibility. Remaining lifecycle validation should attach this machinery to
`ResolvedCoreRule` lowering and realistic rule bodies.

## Selected Architecture

### Crate and module layout

Initial implementation should be internal to the `egglog` crate:

- Add `src/backend/` with a small execution boundary used by `EGraph`.
- Add `src/backend/dd/` for the DD runtime.
- Move backend-neutral value, primitive, Rust-rule, and action context traits
  into `src/backend/context.rs` or equivalent.
- Keep `egglog-bridge` and `core-relations` in the workspace only as oracle or
  compatibility references while the DD runtime is incomplete.

Do not start with a separate `egglog-dd` crate. `CoreRule`, `TypeInfo`, sort
registration, command orchestration, extraction hooks, and proof hooks are still
inside the `egglog` crate. Creating a separate crate first would require
publicizing or moving the IR before the execution design is proven.

End state:

- `egglog` no longer depends on `egglog-bridge` for production execution.
- `egglog-bridge` is deleted or reduced to archived tests/examples.
- `core-relations` is deleted from production dependencies after its reusable
  pieces are moved or replaced.
- `egglog-union-find` may remain if it is the simplest library for the
  specialized canonical map, but it must not remain a native backend authority.

### Runtime ownership

The DD runtime owns:

- relation/function table storage;
- row insertion, deletion, subsume state, and row freshness timestamps;
- per-rule and per-ruleset `last_run` timestamps;
- compiled rule fragments and arrangement registry;
- task streams, output sinks, probes, and visibility gates;
- canonical id map and row rewrite/retraction deltas;
- rebuild and dirty-refresh events;
- scheduler match materialization and admission;
- trace compaction policy and metrics.

The frontend keeps:

- language parsing/desugaring/typechecking;
- command ordering and schedule syntax;
- `CoreRule` construction;
- user-facing errors and command outputs;
- extraction/proof API surfaces until they are ported.

### Data model

Use these DD-side concepts first:

- `Epoch = u64`.
- `Row { rel_id, cols, row_ts, subsumed }`.
- `Task { task_id, ruleset, rule_id, last_run, output_ts, barrier_kind }`.
- `Output { task_id, rel_id, cols, row_ts: output_ts, diff, provenance }`.
- Per-relation `InputSession<Epoch, Row, isize>` where possible.
- Shared arrangements keyed by rule atom access patterns.
- Materialized live-row state keyed by `(rel_id, canonical_key)` for host-side
  action application, duplicate suppression, delete, and merge decisions.
- A specialized canonical map that emits displaced-id events and row rewrite
  deltas. Do not implement equality as generic DD connected components.

Keep `row_ts` as data. DD timestamps should be used for progress, barriers, and
compaction, not as the only encoding of egglog seminaive freshness.

## Semantic Preservation Matrix

| Area | First classification | Required design |
| --- | --- | --- |
| Facts and relation queries | MVP | Insert facts into DD-owned relation sessions; read visible state only after probe gates. |
| Function tables | MVP | Store rows as keyed rows with optional output and subsume state. |
| Constructor fresh ids | MVP | Host-side action application owns fresh-id allocation and feeds committed rows back into DD. |
| Primitive query filters | MVP | Support pure/read-only primitive filters in compiled fragments. |
| Primitive actions | MVP narrow | Execute host-side at action application boundaries; fail/panic semantics must match current behavior. |
| Per-rule seminaive freshness | MVP | Store `row_ts` and `last_run`; filter candidate rows by freshness in compiled fragments. |
| Simple merges | MVP narrow | Implement `Old`, `New`, `AssertEq`, and `UnionId` first. |
| Union/canonicalization | MVP vertical slice | Specialized canonical map emits displaced ids, then DD receives row rewrite/retraction deltas. |
| Rebuild | MVP vertical slice | Rebuild to fixed point for the selected table universe; emit `-old_row +new_row` with fresh `row_ts`. |
| Delete | MVP semantic slice | Negative diff current live row by key; no-op if missing. This does not need to block Gate 1, but it should be present before claiming backend semantics. |
| Subsume | MVP semantic slice | Remove from active view but retain all-row/provenance visibility. |
| Containers | Post-MVP parity | Same-id semantic changes require dirty-refresh events: `-row@old_ts +same_row@next_ts`; the toy contract now passes, but broad container parity remains later. |
| Scheduler worklists | Interface MVP, parity later | Materialize matches, ask scheduler, write chosen matches, then fire actions behind a barrier; define the backend interface before the first DD backend PR. |
| Reports | MVP minimal | Return changed/match/rebuild counts for touched rules; expand after counters stabilize. |
| Serialization | MVP smoke for full file harness | The file-test harness appends `(print-size)` and then exercises graph/serialization paths, so table readback must have a smoke implementation before `tests/files.rs` is used as a gate. Full stable/subsumed parity can follow. |
| Extraction | MVP basic only if extraction tests are in gate | Basic constructor extraction can follow canonical readback; subsume/container/proof extraction should wait until those views are stable. |
| Proofs/proof encoding | Known fail in first PR | Later planner work; DD does not automatically solve proof-query optimization. |
| Push/pop | Separate spike | DD needs snapshot/rollback or epoch-scoped branch state; current `EGraph::push` clones the whole graph. |

## Test Ladder

### Gate 1: basic DD execution

- `repos/egglog/tests/web-demo/path.egg`
- `repos/egglog/tests/relation-query-allowed.egg`
- `repos/egglog/tests/bool.egg`
- `repos/egglog/tests/i64.egg`
- `repos/egglog/tests/primitives.egg`
- `repos/egglog/tests/repro-primitive-query.egg`

### Gate 2: equality and direct rewrites

- `repos/egglog/tests/web-demo/eqsat-basic.egg`
- `repos/egglog/tests/before-proofs.egg`
- `repos/egglog/tests/repro-querybug2.egg`
- `repos/egglog/tests/repro-querybug4.egg`

### Gate 3: schedule freshness

- `repos/egglog/tests/stratified.egg`
- `repos/egglog/tests/test-combined.egg`
- `repos/egglog/tests/test-combined-steps.egg`
- `repos/egglog/tests/web-demo/schedule-demo.egg`
- `repos/egglog/tests/until.egg`

### Gate 4: scalar merge/rebuild/delete smoke

- `repos/egglog/tests/repro-empty-query.egg`
- `repos/egglog/tests/merge-saturates.egg`
- `repos/egglog/tests/repro-equal-constant.egg`
- `repos/egglog/tests/repro-equal-constant2.egg`
- `repos/egglog/tests/interval.egg`
- `repos/egglog/tests/delete.egg`
- `repos/egglog/tests/repro-new-backend-delete.egg`
- `repos/egglog/tests/merge-during-rebuild.egg`

### Gate 5: full harness smoke

Only use `tests/files.rs` after `print-size`, graph export, and serialization
smoke paths exist. The harness appends `(print-size)` to each file and then runs
additional output checks, so it is not an execution-only gate.

### Expected first-PR failures

- full `tests/files.rs`;
- extraction/proof tests;
- stable/full serialization parity;
- complex merge functions that read other functions;
- arbitrary schedulers/backoff;
- full container tests, especially same-id dirty refresh;
- push/pop and APIs that expose `ExecutionState`, `TableAction`, or
  `UnionAction`.

## Follow-Up Experiment Results

The follow-up experiments now live under `code/dd-design-spike/src/bin/` and
emit machine-readable reports under `findings/artifacts/dd-full-refactor/`.
They are bounded design-spike prototypes, not production backend code.

| Experiment | Artifact | Result | Design decision |
| --- | --- | --- | --- |
| DD lifecycle churn stress | `findings/artifacts/dd-full-refactor/01-lifecycle-churn.json` | Pass: 1- and 4-worker runs over 10, 100, and 1000 fragments all completed with correct outputs, probe completion, compaction/frontier samples, RSS metrics, and no retained installed dataflows. | Keep compiled per-rule/per-ruleset fragments as the selected first design; use the production-shaped lifecycle run below to close the imported-trace and recursive-feedback control-plane risk. |
| Production-shaped lifecycle run | `findings/artifacts/dd-full-refactor/06-production-lifecycle.json` | Pass: persistent edge/path/blocked arrangements were imported by generated fragments over 1- and 4-worker runs at 10, 100, and 1000 fragments; recursive host feedback committed the expected path rows with zero early-visibility violations, one blocked candidate per config, and no retained fragment or base dataflows. | Treat compiled fragments over shared arrangements as viable for the first runtime shell. Output sinks must consolidate signed DD diffs before firing actions; the remaining lifecycle risk is CoreRule lowering and realistic rule diversity, not Timely/DD control-plane viability. |
| Rebuild delta toy | `findings/artifacts/dd-full-refactor/02-rebuild-delta.json` | Pass: one displaced-id event rewrote a dependent row with `-old_row +new_row`; the key collision ran merge and queued a second displaced-id event; fixed point completed in two iterations. | Use specialized canonical-map displaced-id events plus reverse-index row rewrites as the equality/rebuild delta protocol. |
| Delete/subsume signed-diff toy | `findings/artifacts/dd-full-refactor/03-delete-subsume.json` | Pass: hard delete removed the live/active row and was idempotent on repeat; subsume removed only from active view while retaining live/all-row/provenance visibility. | Include delete and subsume view semantics in the first semantic mutation slice instead of treating them as speculative. |
| Same-id container dirty refresh toy | `findings/artifacts/dd-full-refactor/04-container-dirty-refresh.json` | Pass: a stable container id with changed canonical contents emitted `-parent@old_ts +same-logical-parent@next_ts`, and the refreshed row passed the next seminaive freshness filter. | Container parity still belongs after the first backend slice, but the required dirty-refresh contract is now explicit and testable. |
| Scheduler materialization toy | `findings/artifacts/dd-full-refactor/05-scheduler-materialization.json` | Pass: complete matches stayed separate from scheduler admission; selected matches wrote to a worklist; actions fired only after barriers; skipped matches remained residual. | Design scheduler admission/worklist/barrier interfaces before the first DD backend PR, even if full scheduler parity lands later. |

### Remaining Unknowns

- Lifecycle is validated for synthetic compiled fragments, bounded churn,
  imported shared arrangements, recursive host feedback, and signed-diff output
  materialization; still validate real `ResolvedCoreRule` lowering and broader
  rule shapes once the DD runtime shell exists.
- Rebuild must still be mapped onto maintained DD traces rather than the
  in-memory reverse-index model.
- Delete/subsume/container/scheduler prototypes prove control semantics, not
  throughput or concurrency behavior.
- Push/pop, extraction/proofs, broad Rust API compatibility, and stable
  serialization parity still need separate design work.

## Ready-To-Begin Implementation Handoff

Status: ready to begin the first implementation sequence. This does not mean
semantic parity or a performance win is proven. It means the remaining risk is
now implementation risk: attach the DD runtime shape to real `ResolvedCoreRule`
lowering, then expand semantic coverage behind explicit gates.

The next agent should start in `repos/egglog`, using this document and the
prototype artifacts as evidence. Production `repos/egglog` code should move
toward one active backend: DD. Native egglog can remain temporarily as a test
oracle, local baseline, or migration reference, but the implementation branch
should not introduce a long-lived user-facing "native versus DD" backend split.

### First implementation target

Build a narrow DD backend vertical slice, not a full parity rewrite:

- Add an internal backend boundary under `repos/egglog/src/backend/`.
- Add `repos/egglog/src/backend/dd/` for a long-lived Timely/DD runtime.
- Keep parse, desugar, typecheck, schedule syntax, command orchestration, and
  `ResolvedCoreRule` construction in the existing frontend.
- Compile a small subset of `ResolvedCoreRule` into DD fragments over shared
  relation arrangements.
- Support facts, relation joins, repeated variables, pure primitive query
  filters, constructor/default insert, host-side action application, and
  per-rule seminaive freshness.
- Materialize DD outputs by net signed diff before actions fire. The
  production-shaped lifecycle test showed that logging only positive diffs is
  wrong for antijoin-style outputs.
- Track known failures explicitly. It is acceptable for many tests to fail in
  the first branch if the failures match the expected-failure list below.

### Starting work plan

1. **Orient in `repos/egglog`**
   - Re-read `src/lib.rs` around `EGraph`, `run_schedule`, and `step_rules`.
   - Re-read `src/core.rs` around `CoreRule` and `ResolvedCoreRule`.
   - Re-read `src/scheduler.rs` only far enough to preserve the barrier shape
     and avoid baking in an incompatible match-admission API.
   - Treat `egglog-bridge/src/lib.rs` as semantic reference material, not as the
     new architectural boundary.

2. **Introduce the backend scaffold**
   - Create a small internal execution boundary that `EGraph` can call for:
     fact insertion, ruleset execution, action application, rebuild/barrier
     hooks, relation readback, and counters.
   - Keep the boundary narrow and private until the DD shape stabilizes.
   - Avoid exposing a permanent backend selection API.

3. **Build the DD runtime shell**
   - Own a long-lived Timely worker.
   - Register relation/function tables with `InputSession<Epoch, Row, isize>`.
   - Maintain shared arrangements for access patterns required by the first
     lowered rules.
   - Add task/barrier execution, probes, output sinks, readback, compaction, and
     metrics.
   - Use the design-spike prototypes as contracts:
     `01-lifecycle-churn.json` for fragment churn and
     `06-production-lifecycle.json` for imported traces, recursive feedback,
     and signed-diff materialization.

4. **Lower the first `ResolvedCoreRule` subset**
   - Compile relation atoms into joins over arranged collections.
   - Enforce repeated-variable equality and simple constant filters.
   - Support pure primitive query filters.
   - Carry `row_ts` as data and use DD timestamps for progress/barriers.
   - Apply host-side actions only after output netting and probe completion.

5. **Run the first correctness gate**
   - Start with the Gate 1 files in this document.
   - Add focused runtime tests before trying the whole `tests/files.rs` harness.
   - Use native egglog only as an oracle when it helps explain a mismatch.
   - Record expected failures instead of expanding the first PR to all parity
     areas.

### First PR acceptance criteria

The first serious implementation PR is successful if it:

- replaces the active production execution path for its supported subset with
  DD-owned execution;
- runs a small relation/fact/join/action subset from real egglog input through
  `ResolvedCoreRule`, not only through toy prototype code;
- has a long-lived DD runtime with probes, output netting, and relation
  readback;
- includes metrics for fragment build latency, probe/barrier count, row
  inserts, output diffs, compaction frontier, and RSS or trace-memory proxy;
- documents known failing tests and why they are out of scope;
- avoids committing to a permanent bridge/native mirrored runtime.

### First PR non-goals

Do not block the first implementation branch on:

- full equality/rebuild parity beyond the smallest vertical slice;
- containers;
- arbitrary scheduler/backoff parity;
- extraction and proofs;
- stable/full serialization;
- push/pop;
- public Rust API compatibility for `ExecutionState`, `TableAction`, and
  `UnionAction`;
- proving a performance win.

The next useful milestone after the first PR is not another toy lifecycle
experiment. It is a real-rule mismatch log: which Gate 1 or Gate 2 tests fail,
what semantic mechanism is missing, and whether the missing mechanism belongs
in the equality/rebuild PR, mutation/scheduler PR, or a separate spike.

## Implementation PR Sequence

1. **Backend scaffold PR**
   - Introduce `src/backend/` and backend-neutral context traits.
   - Keep native/oracle support only for tests or local comparison.
   - No production two-backend switch is required for the final branch.

2. **DD lifecycle/runtime PR**
   - Add DD runtime shell, relation registry, task streams, probes, output sinks,
     arrangement registry, compaction hooks, and metrics.
   - Start with compiled fragments; carry the lifecycle churn and
     production-shaped lifecycle scenarios as runtime acceptance tests, then
     replace the synthetic fragment builder with `ResolvedCoreRule` lowering.

3. **Core execution vertical slice PR**
   - Compile a subset of `ResolvedCoreRule` into DD fragments.
   - Support facts, relation joins, repeated variables, pure primitive filters,
     constructor lookup/default insert, host-side action application, and
     per-rule freshness.
   - Include scheduler-facing match output, worklist, and barrier interfaces,
     but only implement the default all-matches admission path at first.

4. **Equality/rebuild PR**
   - Add specialized canonical map, displaced-id events, row rewrite/retraction
     feedback, simple merge support, and fixed-point rebuild for the selected
     universe.

5. **Mutation and scheduler PR**
   - Add negative-diff delete, active/all-row subsume views, and selected-match
     scheduler admission over the already-defined worklist/barrier interface.

6. **Container/API parity PRs**
   - Port container dirty refresh, extraction, serialization, proof hooks,
     push/pop, Rust primitive APIs, Rust rules, and public compatibility
     surfaces.

7. **Cleanup PR**
   - Remove production dependency on `egglog-bridge`.
   - Remove or archive `core-relations` once all reused pieces have been moved or
     replaced.

## Metrics Required Before Performance Claims

- rule-fragment build latency;
- per-rule probe lag and logical frontier lag;
- trace memory/RSS;
- logical and physical compaction frontiers;
- row inserts, retractions, rewrites, and refreshes;
- canonical-map union/displaced-id counts;
- delete/subsume counts;
- scheduler admissions, skips, and residual matches;
- barrier count by reason;
- native-oracle state diff at each logical boundary while the oracle exists.

## Stop Criteria Answered

- Dynamic per-rule DD graph construction is viable for bounded generated
  fragments and for a production-shaped imported-arrangement/recursive-feedback
  run. Output sinks must materialize net signed diffs before actions; the
  remaining lifecycle work is `ResolvedCoreRule` lowering and realistic rule
  shapes.
- The old bridge/core-relations layers should not survive as permanent
  architecture boundaries.
- The first serious implementation must support facts, function rows,
  constructor defaults, primitive filters/actions, per-rule freshness, simple
  merges, and a vertical slice of canonicalization/rebuild.
- The first implementation branch may knowingly fail extraction, proofs,
  stable/full serialization parity, broad containers, arbitrary schedulers,
  push/pop, and complex Rust API compatibility.
- Follow-up prototypes now pass for lifecycle churn, production-shaped
  lifecycle, rebuild deltas, delete/subsume, container dirty refresh, and
  scheduler materialization. Further work should turn these contracts into
  production-shaped backend tests rather than repeat the same toy spikes.
