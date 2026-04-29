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

This document is not a broad rewrite authorization. The implementation path is
two-step: first build a DD runtime scaffold that proves real `ResolvedCoreRule`
lowering can drive a DD-owned runtime, then run a separate Option 3
replacement-backend ownership gate before claiming the design is ready to replace
native execution for a meaningful semantic slice.

## Meta Goal Alignment

This spike should be read through the same project goals as
[`dd-refactor-high-level-fixes.md`](dd-refactor-high-level-fixes.md):

- **Research platform:** preserve backend hooks for provider/dependency views,
  query-defined primitives, solver constraints, and multiple A/C strategies
  without choosing one representation now.
- **Real-world utility:** preserve current semantics on the selected slice, keep
  compatibility breaks explicit, use native egglog as an oracle during
  migration, and require performance smoke checks before mergeability claims.
- **Maintainability:** move toward one DD-owned production runtime, delete the
  bridge/core-relations duplication over time, and keep backend responsibilities
  modular enough that experiments do not require another execution fork.

## Evidence Gathered In This Spike

This revision also uses
[`pr-856-typed-execution-state-review.md`](pr-856-typed-execution-state-review.md)
and
[`dd-design-spike-alignment-review.md`](dd-design-spike-alignment-review.md) as
design evidence. They inform the semantic boundaries below, but they do not
make PR #856's exact API part of this design.

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

This was enough to keep compiled DD fragments as the first scaffold design. The
follow-up lifecycle churn and production-shaped lifecycle experiments below
strengthen that choice for bounded generated fragments: churn covers
install/drop behavior, and the production-shaped run imports persistent traces,
feeds recursive results back at barriers, and consolidates signed output diffs
before host-side action visibility. They do not prove arbitrary Timely/DD
control-plane viability for real egglog schedules; that remains part of the
Option 3 ownership gate.

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

The Rust API may intentionally shift during this work. The scaffold PR must
inventory and name the replacement concepts for existing extension surfaces that
currently expose `core-relations` or `egglog-bridge`: `ExecutionState`,
`TableAction`, `UnionAction`, `FunctionRow`, `Sort::column_ty`, primitives, and
Rust rules. Compatibility is not required for the scaffold, but breakage must be
explicit rather than accidental.

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
| Function tables | MVP | Store rows as keyed rows with optional output and subsume state. Preserve `DefaultVal::Fail` lookup semantics separately from constructor/default insertion: missing custom-function lookups must take the current panic/failure path, not become optional absence or fresh insertion. |
| Constructor fresh ids | MVP | Host-side action application owns fresh-id allocation and feeds committed rows back into DD. |
| Primitive query filters | MVP | Support pure primitive filters in compiled rule bodies. Rule-query primitives must not perform hidden database reads; such reads must be explicit query atoms or deferred to declared-read/provider work. |
| Declared-read primitives | Extension-ready later | Read-capable primitives or callbacks are sound only when their dependencies are visible to seminaive invalidation: explicit matched inputs, declared dependencies, provider deltas, or another maintained wakeup mechanism. |
| Primitive/action capability contexts | Scaffold contract, API later | PR #856 / issue #772 provide evidence for separating rule-query, rule-action, global-query, and global-action capabilities. The DD design should preserve that semantic split without requiring PR #856's exact public API names or implementation. |
| Primitive actions | MVP narrow | Execute host-side at action application boundaries; fail/panic semantics must match current behavior. Rule actions may write based on matched bindings, but hidden live reads must be rejected, made explicit in the match, or represented as declared dependencies with a wakeup plan. |
| Per-rule seminaive freshness | MVP | Store `row_ts` and `last_run`; filter candidate rows by freshness in compiled fragments. |
| Simple merges | MVP narrow | Implement `Old`, `New`, `AssertEq`, and `UnionId` first. |
| Union/canonicalization | MVP vertical slice | Specialized canonical map emits displaced ids, then DD receives row rewrite/retraction deltas. |
| Rebuild | MVP vertical slice | Rebuild to fixed point for the selected table universe; committed row deltas must replay from initial live rows to final live rows. Collision candidates that only drive merges must not appear as live-row insertions unless they are actually committed. |
| Delete | Action ABI MVP; semantic slice later | `Change::Delete` must be represented in the first backend action boundary so later mutation work does not reopen the ABI. Full negative-diff live-row semantics can land with the mutation slice. |
| Subsume | Action ABI MVP; semantic slice later | `Change::Subsume` must be represented in the first backend action boundary. Full active/all-row/provenance view semantics can land with the mutation slice. |
| Containers | Post-MVP parity | Same-id semantic changes require dirty-refresh events: `-row@old_ts +same_row@next_ts`; the toy contract now passes, but broad native container matching and provider-indexed matching remain later. |
| Higher-order container callbacks | Ownership-gate proof point | Prove one coarse dependency-tracked callback such as safe `unstable-vec-map`. The first acceptable version can record container id, callback identity, captured args, and state reads in Rust; it does not need fine-grained `VecElem`/provider indexing. |
| Scheduler worklists | Interface MVP, parity later | Materialize matches, ask scheduler, write chosen matches, then fire actions behind a barrier; define the backend interface before the scaffold PR. |
| Reports | MVP minimal | Return changed/match/rebuild counts for touched rules; expand after counters stabilize. |
| Serialization | MVP smoke for full file harness | The file-test harness appends `(print-size)` and then exercises graph/serialization paths, so table readback must have a smoke implementation before `tests/files.rs` is used as a gate. Full stable/subsumed parity can follow. |
| Extraction | MVP basic only if extraction tests are in gate | Basic constructor extraction can follow canonical readback; subsume/container/proof extraction should wait until those views are stable. |
| Proof-aware planning | Ownership-gate follow-up | DD does not automatically solve proof-query optimization. Preserve the April 24 planner benchmark: compare naive seminaive delta expansion with dependent lookup on A/B/C-style functional-dependency cases. This is planner work, not only proof extraction/proof API porting. |
| Proof extraction/proof encoding | Known fail in scaffold | Proof term encoding remains a partial specification/oracle, not a production shortcut. Full proof extraction and proof API parity are later work. |
| Push/pop | Separate spike | DD needs snapshot/rollback or epoch-scoped branch state; current `EGraph::push` clones the whole graph. |

## Test Ladder

### Gate 1: basic DD execution

- run the `dd-core-rule-canary` report and require the supported Gate 1 rules to
  lower from actual `ResolvedCoreRule` values, not hand-written toy rules;
- use a reduced no-print path fixture derived from
  `repos/egglog/tests/web-demo/path.egg`; the full file stays in Gate 5 until
  `print-function` readback exists;
- `repos/egglog/tests/relation-query-allowed.egg`
- `repos/egglog/tests/bool.egg`
- `repos/egglog/tests/i64.egg` and `repos/egglog/tests/primitives.egg` as
  top-level primitive/check smoke only; they do not lower any rules in the
  current canary corpus;
- a reduced primitive-filter fixture derived from
  `repos/egglog/tests/repro-primitive-query.egg` that does not use `panic`, or
  the full file only after `Panic` is an explicit supported host-side action.

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
- `repos/egglog/tests/delete.egg`
- `repos/egglog/tests/repro-new-backend-delete.egg`
- `repos/egglog/tests/merge-during-rebuild.egg`

### Gate 5: readback and extraction smoke

Use this gate only after explicit readback/extraction support exists:

- `repos/egglog/tests/web-demo/path.egg`, because it runs `print-function`;
- `repos/egglog/tests/interval.egg`, because it runs `extract`.

### Gate 6: full harness smoke

Only use `tests/files.rs` after `print-size`, graph export, and serialization
smoke paths exist. The harness appends `(print-size)` to each file and then runs
additional output checks, so it is not an execution-only gate.

### Expected scaffold failures

- full `tests/files.rs`;
- extraction/proof tests;
- stable/full serialization parity;
- complex merge functions that read other functions;
- arbitrary schedulers/backoff;
- full container tests, including same-id dirty refresh before the ownership
  gate, broad native container matching, and provider-indexed matching;
- push/pop;
- legacy Rust extension APIs. The scaffold may break APIs that expose
  `ExecutionState`, `TableAction`, `UnionAction`, `FunctionRow`,
  `Sort::column_ty`, primitives, and Rust rules, but it must document the
  intended replacement surfaces.

## Follow-Up Experiment Results

Most follow-up experiments live under `code/dd-design-spike/src/bin/`; the
`ResolvedCoreRule` canary lives in `repos/egglog` because the lowered IR is
crate-private. They emit machine-readable reports under
`findings/artifacts/dd-full-refactor/`. They are bounded design-spike prototypes
and canaries, not production backend code.

| Experiment | Artifact | Result | Design decision |
| --- | --- | --- | --- |
| DD lifecycle churn stress | `findings/artifacts/dd-full-refactor/01-lifecycle-churn.json` | Pass: 1- and 4-worker runs over 10, 100, and 1000 fragments completed with correct outputs, probe completion, compaction/frontier samples, RSS samples, and no retained installed dataflows. | Keep compiled per-rule/per-ruleset fragments as the selected scaffold design, but treat RSS as a smoke signal rather than proof of no retention. |
| Production-shaped lifecycle run | `findings/artifacts/dd-full-refactor/06-production-lifecycle.json` | Pass: persistent edge/path/blocked arrangements were imported by generated fragments over 1- and 4-worker runs at 10, 100, and 1000 fragments; recursive host feedback committed the expected path rows with zero early-visibility violations, one blocked candidate per config, and no retained fragment or base dataflows. | Treat compiled fragments over shared arrangements as viable for the runtime scaffold on this bounded chain/blocked workload. Output sinks must consolidate signed DD diffs before firing actions. Broader control-plane viability remains open for real `ResolvedCoreRule` diversity and arbitrary schedules. |
| Rebuild delta toy | `findings/artifacts/dd-full-refactor/02-rebuild-delta.json` | Pass: a displaced-id event found an affected dependent row; the committed row-delta stream now replays from `initial_live_rows` to `final_live_rows`; the key collision is recorded separately as merge evidence and queues a second displaced-id event. | Treat this as a reverse-index collision toy and a replayable live-row-delta contract, not yet as the equality/rebuild DD-trace protocol. The next proof point must map it onto maintained DD traces. |
| Delete/subsume signed-diff toy | `findings/artifacts/dd-full-refactor/03-delete-subsume.json` | Pass: hard delete removed the live/active row and was idempotent on repeat; subsume removed only from active view while retaining live/all-row/provenance visibility. | Include delete and subsume view semantics in the first semantic mutation slice instead of treating them as speculative. |
| Same-id container dirty refresh toy | `findings/artifacts/dd-full-refactor/04-container-dirty-refresh.json` | Pass: a stable container id with changed canonical contents emitted `-parent@old_ts +same-logical-parent@next_ts`, and the refreshed row passed the next seminaive freshness filter. | Container parity still belongs after the first backend slice, but the required dirty-refresh contract is now explicit and testable. |
| Scheduler materialization toy | `findings/artifacts/dd-full-refactor/05-scheduler-materialization.json` | Pass: complete matches stayed separate from scheduler admission; selected matches wrote to a worklist; actions fired only after barriers; skipped matches remained residual. | Design scheduler admission/worklist/barrier interfaces before the scaffold PR, even if full scheduler parity lands later. |
| ResolvedCoreRule Gate 1 canary | `findings/artifacts/dd-full-refactor/07-resolved-core-rule-canary.json` | Pass with findings: seven rules from five Gate 1 entries parsed, ran, and lowered into stored `ResolvedCoreRule` values; `i64.egg` and `primitives.egg` contributed no rules; the transformed no-panic primitive-filter rule lowered with a conditional pure/admitted primitive requirement; full `repro-primitive-query.egg` still lowered a `Panic` action. | Proceed from toy lifecycle experiments to real-rule scaffold lowering, but tighten Gate 1: use `path-no-print`, relation-query, bool, and the no-panic primitive-filter fixture unless the scaffold deliberately supports `Panic`. Primitive query atoms still need a pure/admitted primitive check because current primitive purity is not visible in the lowered IR. |

### Remaining Unknowns

- Lifecycle is validated for synthetic compiled fragments, bounded churn,
  imported shared arrangements, recursive host feedback, and signed-diff output
  materialization on one narrow workload. Real `ResolvedCoreRule` lowering is now
  smoke-validated for the Gate 1 canary corpus, but the DD compiler still has to
  lower and execute those rules; broader rule shapes and arbitrary schedules
  remain unvalidated.
- Rebuild has a replayable in-memory reverse-index collision model, but it still
  must be mapped onto maintained DD traces before becoming the equality/rebuild
  protocol.
- Delete/subsume/container/scheduler prototypes prove control semantics, not
  throughput or concurrency behavior.
- Push/pop, extraction/proofs, Rust API migration, and stable serialization
  parity still need separate design work.
- Query-defined primitives, inline lambdas, native/provider-backed matching,
  declared-read primitives, solver constraints, and merge/reduce semantics are
  follow-up language/runtime design work, not part of the first DD scaffold. The
  current design trail is
  [`dd-refactor-high-level-fixes.md`](dd-refactor-high-level-fixes.md) and
  [`primitive-prototyping.md`](primitive-prototyping.md).
- PR #856 / issue #772 are useful capability-boundary evidence, but the DD
  scaffold should describe abstract rule/global query/action semantics rather
  than copying that API directly.

## DD Runtime Scaffold Handoff

Status: ready to begin a scaffold sequence, not a production replacement. This
does not mean semantic parity, a performance win, or the Option 3
replacement-backend gate is proven. The scaffold should attach the DD runtime
shape to real `ResolvedCoreRule` lowering and expose the missing design risks
behind explicit gates.

The next agent should start in `repos/egglog`, using this document and the
prototype artifacts as evidence. Production `repos/egglog` code should move
toward one active backend: DD. Native egglog can remain temporarily as a test
oracle, local baseline, or migration reference, but the implementation branch
should not introduce a long-lived user-facing "native versus DD" backend split.

### First implementation target: DD runtime scaffold

Build a narrow DD runtime scaffold, not the Option 3 ownership gate:

- Add an internal backend boundary under `repos/egglog/src/backend/`.
- Add `repos/egglog/src/backend/dd/` for a long-lived Timely/DD runtime.
- Keep parse, desugar, typecheck, schedule syntax, command orchestration, and
  `ResolvedCoreRule` construction in the existing frontend.
- Compile a small subset of `ResolvedCoreRule` into DD fragments over shared
  relation arrangements.
- Support facts, relation joins, repeated variables, pure primitive query
  filters only, constructor/default insert, `DefaultVal::Fail` missing-lookup
  behavior, host-side action application after output netting, and per-rule
  seminaive freshness.
- Reject or defer primitive/callback forms that perform hidden rule-query reads;
  those require explicit matched inputs, declared dependencies, or provider
  deltas before they are admitted to seminaive rule bodies.
- Include `Delete` and `Subsume` in the action ABI from the start, even if their
  full live/active/all-row semantics land in the mutation slice.
- Keep rule actions write-oriented: any action-side live read must already be in
  the matched inputs, be rejected, or have a declared-dependency wakeup plan.
- Materialize DD outputs by net signed diff before actions fire.
- Track known failures explicitly. It is acceptable for many tests to fail in
  the scaffold branch if the failures match the expected-failure list below.
- Inventory intentional Rust API breaks and replacement concepts for
  `ExecutionState`, `TableAction`, `UnionAction`, `FunctionRow`,
  `Sort::column_ty`, primitives, and Rust rules.

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
   - Start with the Gate 1 files in this document, excluding tests with hidden
     `print-function`, extraction, serialization, proof, or push/pop
     requirements.
   - Add focused runtime tests before trying the whole `tests/files.rs` harness.
   - Use native egglog only as an oracle when it helps explain a mismatch.
   - Record expected failures instead of expanding the scaffold to all parity
     areas.

### Scaffold PR acceptance criteria

The DD runtime scaffold PR is successful if it:

- runs a small relation/fact/join/action subset from real egglog input through
  `ResolvedCoreRule`, not only through toy prototype code;
- carries the `dd-core-rule-canary` as a regression report and documents any
  unsupported lowered atom/action kind before adding it to the scaffold gate;
- has a long-lived DD runtime with probes, output netting, and relation
  readback;
- includes metrics for fragment build latency, probe/barrier count, row
  inserts, output diffs, compaction frontier, and RSS or trace-memory proxy;
- defines the action ABI for delete/subsume and the migration surface for the
  legacy Rust API types, even if those APIs break;
- defines the abstract primitive/action capability boundary for rule queries,
  rule actions, global queries, and global actions without locking in PR #856's
  exact API;
- documents known failing tests and why they are out of scope;
- avoids committing to a permanent bridge/native mirrored runtime.

### Scaffold PR non-goals

Do not block the scaffold branch on:

- full equality/rebuild parity beyond the smallest vertical slice;
- broad container parity, native container matching, and provider-indexed
  matching;
- arbitrary scheduler/backoff parity;
- extraction and proofs;
- stable/full serialization;
- push/pop;
- preserving legacy Rust API compatibility;
- proving a performance win.

### Immediate next milestone: Option 3 ownership gate

The next milestone after the scaffold is the Option 3 replacement-backend gate,
not another toy lifecycle experiment. It must use a small relation/function-table
universe and prove:

- DD-owned state for the selected universe, with native egglog used only as an
  oracle;
- per-rule freshness and step-visible state diffs against native egglog;
- one rebuild/canonicalization event over maintained DD traces;
- one same-id dirty-refresh-style invalidation or equivalent;
- one coarse dependency-tracked higher-order container callback, such as safe
  `unstable-vec-map`, that records container id, callback identity, captured
  args, and relevant state reads;
- one scheduler materialization boundary;
- counters for row rewrites, retractions/reinsertions, refresh rows, scheduler
  admissions/skips, frontier lag, trace memory, and compaction.

The gate should also include, either in the same slice or immediate follow-up,
the proof-aware planner benchmark: compare naive seminaive delta expansion with
a dependent lookup plan on A/B/C-shaped functional-dependency cases.

After the ownership gate, run an extension-readiness probe rather than selecting
stable syntax immediately. That probe should show the backend can host at least
one container-HOF strategy, one provider-indexed container strategy, one binary
recursive/fixpoint strategy, one query-defined primitive or lambda, one
solver-backed scalar constraint, and one schedule-aware derived view, as scoped
in [`dd-refactor-high-level-fixes.md`](dd-refactor-high-level-fixes.md).

## Implementation PR Sequence

1. **Backend scaffold PR**
   - Introduce `src/backend/` and backend-neutral context traits.
   - Keep native/oracle support only for tests or local comparison.
   - No production two-backend switch is required for the final branch.
   - Define abstract capability contexts for rule queries, rule actions, global
     queries, and global actions as semantic requirements, treating PR #856 as
     evidence rather than as a required API.
   - Inventory intentional Rust API breaks and replacement backend-facing
     concepts for extension APIs.

2. **DD lifecycle/runtime PR**
   - Add DD runtime shell, relation registry, task streams, probes, output sinks,
     arrangement registry, compaction hooks, and metrics.
   - Start with compiled fragments; carry the lifecycle churn and
     production-shaped lifecycle scenarios as runtime acceptance tests, then
     replace the synthetic fragment builder with `ResolvedCoreRule` lowering.

3. **Core execution vertical slice PR**
   - Compile a subset of `ResolvedCoreRule` into DD fragments.
   - Support facts, relation joins, repeated variables, pure primitive filters
     only, constructor lookup/default insert, `DefaultVal::Fail`, host-side
     action application after output netting, and per-rule freshness.
   - Start from the supported Gate 1 canary rules. Replace the full
     `repro-primitive-query.egg` with a no-panic primitive-filter fixture unless
     the PR intentionally supports `Panic`.
   - Include `Delete` and `Subsume` in the backend action ABI.
   - Include scheduler-facing match output, worklist, and barrier interfaces,
     but only implement the default all-matches admission path at first.

4. **Equality/rebuild PR**
   - Add specialized canonical map, displaced-id events, row rewrite/retraction
     feedback, simple merge support, and fixed-point rebuild for the selected
     universe.

5. **Option 3 ownership-gate PR**
   - Prove the selected universe has single ownership: per-rule freshness,
     rebuild/canonicalization on maintained DD traces, dirty-refresh-style
     invalidation, one dependency-tracked higher-order container callback,
     scheduler materialization, and native-oracle state diffs.
   - Include the proof-aware planner benchmark or record it as the immediate
     follow-up for the gate.

6. **Mutation and scheduler parity PR**
   - Fill in negative-diff delete, active/all-row subsume views, and
     selected-match scheduler admission over the already-defined
     worklist/barrier interface.

7. **Container/API parity PRs**
   - Port broad native container matching, provider-indexed container matching,
     extraction, serialization, proof hooks, push/pop, Rust primitive APIs, Rust
     rules, and replacement public API surfaces.

8. **Extension-readiness probe PR**
   - Demonstrate that the backend hooks can host the A/C, query-primitive,
     solver, and schedule-view experiments from
     [`dd-refactor-high-level-fixes.md`](dd-refactor-high-level-fixes.md)
     without stabilizing their syntax.

9. **Cleanup PR**
   - Remove production dependency on `egglog-bridge`.
   - Remove or archive `core-relations` once all reused pieces have been moved or
     replaced.

## Metrics Required Before Performance Claims

The scaffold does not need to prove a performance win. Before claiming a
mergeable backend path, it must show no unexplained regression larger than 20%
on the named comparison workloads below. A larger regression can still be
acceptable only if it is measured, attributed to a deliberate semantic or
instrumentation cost, and assigned to a concrete follow-up before merge.

- rule-fragment build latency;
- primitive dispatch timing;
- Rust-rule/action hot-path timing;
- per-rule probe lag and logical frontier lag;
- trace memory/RSS;
- logical and physical compaction frontiers;
- row inserts, retractions, rewrites, and refreshes;
- canonical-map union/displaced-id counts;
- delete/subsume counts;
- scheduler admissions, skips, and residual matches;
- barrier count by reason;
- native-oracle state diff at each logical boundary while the oracle exists.

Minimum comparison workload list:

- the supported Gate 1 canary rules from
  `findings/artifacts/dd-full-refactor/07-resolved-core-rule-canary.json`;
- the reduced path fixture, `relation-query-allowed.egg`, `bool.egg`, and the
  no-panic primitive-filter fixture;
- top-level primitive/check smoke from `i64.egg` and `primitives.egg`;
- `repos/egglog/benches/rust_api_benchmarking.rs` cases
  `rust_rule_match_overhead`, `rust_rule_insert_loop`, and
  `rust_rule_tableaction_hot_path`;
- the selected DD vertical-slice workload with native-oracle state diffs at each
  logical boundary.

## Stop Criteria Answered

- Dynamic per-rule DD graph construction is viable for bounded generated
  fragments and for one production-shaped imported-arrangement/recursive-feedback
  run. Output sinks must materialize net signed diffs before actions; broader
  control-plane viability still requires DD execution of the real Gate 1
  `ResolvedCoreRule` canary rules, realistic rule shapes, and arbitrary
  schedules.
- The old bridge/core-relations layers should not survive as permanent
  architecture boundaries.
- The first scaffold must support facts, function rows, constructor defaults,
  `DefaultVal::Fail`, pure primitive filters, host-side actions after output
  netting, per-rule freshness, delete/subsume action ABI shape, and Rust API
  migration inventory. Hidden reads belong in explicit query atoms, declared
  dependencies, provider deltas, or later extension work.
- The separate Option 3 ownership gate must own per-rule freshness, one
  rebuild/canonicalization event over maintained DD traces, one
  dirty-refresh-style invalidation, one dependency-tracked higher-order
  container callback, one scheduler materialization boundary, and native-oracle
  comparison.
- The first scaffold may knowingly fail extraction, proofs, stable/full
  serialization parity, broad containers/native container matching, arbitrary
  schedulers, push/pop, and legacy Rust API compatibility.
- Follow-up prototypes now pass for lifecycle churn, production-shaped
  lifecycle, replayable rebuild deltas, delete/subsume, container dirty refresh,
  scheduler materialization, and real Gate 1 `ResolvedCoreRule` shape
  classification. Further work should turn these contracts into
  production-shaped backend tests rather than repeat the same toy spikes.
