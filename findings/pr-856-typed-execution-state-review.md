# PR 856 Typed Execution State Review

Date: 2026-04-29

Status: review of
[egraphs-good/egglog#856](https://github.com/egraphs-good/egglog/pull/856)
at head `e6d392cc5b0f3c00c4385f565128aa815a299caa`, with context from
[egraphs-good/egglog#772](https://github.com/egraphs-good/egglog/issues/772).
The PR is still draft, so treat this as design evidence and review input, not
as a stable API description.

## Source Basis

- GitHub PR #856 metadata and diff: draft, open, mergeable, base `main` at
  `88c4ea86feb3418ba39131e1487359896d736908`, head `e6d392cc...`.
- GitHub issue #772 body and comments, especially the typed execution-state
  proposal and the later comment connecting it to PR #832's container dirty-id
  repair.
- Local worktree for the PR at `/Users/saul/p/egglog-pr-856`.
- Targeted validation:
  - `cargo test --test typed_primitive` passed: 6 tests.
  - `cargo test --test files --features bin typed_primitive_unstable_app`
    passed: 3 tests.
  - One first parallel test invocation failed while two commands raced rustup's
    toolchain installation; rerunning after the install succeeded.

## Issue 772 Context

Issue #772 frames the bug as an `ExecutionState` capability problem: public
primitive code can read or write database state from rule queries and actions.
That is unsound under seminaive because:

- query-side writes mutate the database while matching is still underway;
- query-side reads can hide dependencies from seminaive freshness; and
- action-side reads make an old match produce different effects later, breaking
  saturation detection.

The issue proposal has four contexts:

| Context | Reads | Writes | Intended use |
| --- | --- | --- | --- |
| `RuleQuery` | no | no | seminaive rule body |
| `RuleAction` | no | yes | seminaive rule action |
| `GlobalQuery` | yes | no | top-level checks/queries |
| `GlobalAction` | yes | yes | top-level actions/eval |

The follow-up comment adds the PR #832 lesson: seminaive safety is not only
about obvious `ExecutionState` reads and writes. A container can keep the same
outer id while rebuild changes its semantic contents. That still needs a
visible row delta or declared dependency, currently through
`ContainerRebuildSummary::dirty_ids` / `refresh_rows_for_values`.

## PR 856 Summary

PR #856 implements a typed primitive surface around that context split.

- Adds `src/exec_state.rs` with `Context`, `PureState`, `WriteState`,
  `ReadState`, `FullState`, and public capability traits `Core` and `Write`.
  The raw `ExecutionState` and table lookup seams are kept behind a
  crate-private `__internal::Internal` trait.
- Replaces the old single primitive trait with `PrimitiveCommon` plus
  `PurePrim`, `WritePrim`, `ReadPrim`, and `FullPrim`.
- Adds `EGraph::add_pure_primitive`, `add_write_primitive`,
  `add_read_primitive`, `add_full_primitive`, and `add_primitive_group`.
  Group registration partitions valid contexts so only one variant is intended
  to survive at a call site.
- Threads `Context` through typechecking and constraint construction, so rule
  bodies use `RuleQuery`, rule actions use `RuleAction`, top-level checks use
  `GlobalQuery`, and top-level actions use `GlobalAction`.
- Splits `TableAction::lookup` into a pure read and `lookup_or_insert` into the
  constructor-minting write path.
- Reworks `unstable-fn` / `unstable-app` with query/action variants:
  `apply_in` refuses constructor minting and rule-query custom lookups, while
  `apply_mut` keeps action-side constructor minting.
- Reworks vec and multiset higher-order primitives into grouped variants:
  pure for rule query, read for global query, write/full for action contexts.
- Moves `unstable-multiset-fill-index` to `FullPrim` / global action only,
  reflecting that it reads existing index rows before writing.

The PR also adds good regression coverage around context acceptance and
`unstable-app` dispatch. The PR's current Codecov comment reports 85.37% patch
coverage and the CodSpeed bot reports regressions, including a 31.42% wall-time
regression on `tests[luminal-llama]` and 6-17% regressions on Rust-rule
benchmarks. The CodSpeed comment notes environment differences, so the exact
percentages need confirmation, but hot-path overhead is a real merge risk.

## Review Findings

### P1: `PureState` Still Exposes Side Effects

`Core` is implemented for `PureState`, and it still exposes `inc_counter`,
`register_container`, and `container_to_value`:

- [`src/exec_state.rs#L123-L153`](https://github.com/egraphs-good/egglog/blob/e6d392cc5b0f3c00c4385f565128aa815a299caa/src/exec_state.rs#L123-L153)
- [`src/exec_state.rs#L173-L177`](https://github.com/egraphs-good/egglog/blob/e6d392cc5b0f3c00c4385f565128aa815a299caa/src/exec_state.rs#L173-L177)

That means a `PurePrim` used in a seminaive rule query can still mutate counter
state and intern containers. The PR treats container interning as idempotent and
therefore pure, but #772's follow-up explicitly called out `register_val` /
container interning as a write-like operation that should not sit in the
unqualified core capability.

This is especially relevant because `unstable-fn` is a `PurePrim` and interns a
`FunctionContainer`:

- [`src/sort/fn.rs#L361-L404`](https://github.com/egraphs-good/egglog/blob/e6d392cc5b0f3c00c4385f565128aa815a299caa/src/sort/fn.rs#L361-L404)

Recommendation: split pure value conversion from allocation/interning. If some
container interning must remain available in query evaluation, make that a
separate runtime-internal facility with an explicit idempotence and dependency
contract, not a public `PureState` method. Move `inc_counter` out of `Core`.

### P1: Rule Actions Can Still Read Custom Function Tables Through `unstable-app`

`ApplyFull` is registered as a `WritePrim` and is valid in `RuleAction`. It calls
`FunctionContainer::apply_mut`, which reads custom functions with
`TableAction::lookup`:

- [`src/sort/fn.rs#L551-L563`](https://github.com/egraphs-good/egglog/blob/e6d392cc5b0f3c00c4385f565128aa815a299caa/src/sort/fn.rs#L551-L563)
- [`src/sort/fn.rs#L631-L636`](https://github.com/egraphs-good/egglog/blob/e6d392cc5b0f3c00c4385f565128aa815a299caa/src/sort/fn.rs#L631-L636)

The new test explicitly locks this behavior in:

- [`tests/typed_primitive_unstable_app.egg#L34-L43`](https://github.com/egraphs-good/egglog/blob/e6d392cc5b0f3c00c4385f565128aa815a299caa/tests/typed_primitive_unstable_app.egg#L34-L43)

That conflicts with #772's core rule that seminaive rule actions may write but
must not read live database state. A rule action whose RHS reads a custom
function can produce different output after the original match is old, with no
fresh match to wake it.

Recommendation: split action application further. Constructor minting in a rule
action is a write; custom-function lookup in a rule action is a read and should
either be rejected, forced into the rule query as an explicit atom, or routed
through a declared-read dependency mechanism that can wake the action when the
read value changes.

### P2: `ReadPrim` Says "Reads Tables" But Exposes No Public Read API

The public trait docs say `ReadPrim` bodies see `ReadState` with table reads:

- [`src/lib.rs#L169-L175`](https://github.com/egraphs-good/egglog/blob/e6d392cc5b0f3c00c4385f565128aa815a299caa/src/lib.rs#L169-L175)

But `ReadState` only implements `Core`; raw table lookup is crate-private under
`__internal::Internal`:

- [`src/exec_state.rs#L72-L97`](https://github.com/egraphs-good/egglog/blob/e6d392cc5b0f3c00c4385f565128aa815a299caa/src/exec_state.rs#L72-L97)
- [`src/exec_state.rs#L357-L367`](https://github.com/egraphs-good/egglog/blob/e6d392cc5b0f3c00c4385f565128aa815a299caa/src/exec_state.rs#L357-L367)

That may be an intentional intermediate design, but the API currently advertises
more than external primitive authors can use. The in-tree higher-order
container wrappers can read indirectly through `FunctionContainer::apply_in`
because they are inside the crate; third-party `ReadPrim` implementations
cannot.

Recommendation: either add a real public read capability for `ReadState`, or
document `ReadPrim` as a reserved/internal extension point until declared reads
exist.

### P2: Grouped Primitive Dispatch Is A Convention, Not A Checked Contract

`add_primitive_group` partitions valid contexts by priority and relies on the
author promise that all grouped variants compute the same result:

- [`src/typechecking.rs#L233-L318`](https://github.com/egraphs-good/egglog/blob/e6d392cc5b0f3c00c4385f565128aa815a299caa/src/typechecking.rs#L233-L318)

That is a useful ergonomic mechanism, but it is not a semantic proof. The PR
also documents a known ambiguity gap where two non-grouped same-name,
same-signature primitives silently pick the first registration:

- [`tests/typed_primitive.rs#L305-L326`](https://github.com/egraphs-good/egglog/blob/e6d392cc5b0f3c00c4385f565128aa815a299caa/tests/typed_primitive.rs#L305-L326)

Recommendation: before stabilizing this API, either reject non-grouped duplicate
same-signature primitives or make "first wins" explicit in the public API. For
groups, diagnostics should name the selected context variant and warn when a
variant loses all contexts during partitioning.

### P2: Performance Needs To Be A Merge Gate

The PR changes hot primitive dispatch paths, adds per-context wrapper
construction, and uses `ArcSwap` for action registry reads. The CodSpeed bot
reported regressions on `luminal-llama`, `rust_rule_match_overhead`,
`rust_rule_insert_loop`, and `rust_rule_tableaction_hot_path`.

Recommendation: keep the typed API direction, but require a before/after profile
on representative Rust-rule and container/HOF workloads before merge. The DD
refactor should treat this as evidence that capability safety must compile down
to cheap per-call paths.

## Implications For The DD Refactor

PR #856 is useful design inspiration, but it should not be copied as the whole
DD answer. The important split is:

- **Capability safety:** which operations are callable from `RuleQuery`,
  `RuleAction`, `GlobalQuery`, and `GlobalAction`.
- **Freshness safety:** which hidden reads or value-level semantic changes must
  produce deltas or declared dependencies so seminaive re-runs the right work.

Typed wrappers help with capability safety. They do not replace dirty container
refresh, provider deltas, or declared-read dependency recording.

Recommended changes to the DD design direction:

1. Preserve the four-context distinction if #856 lands before the DD scaffold.
   `ResolvedCoreRule` lowering should know whether it is compiling a rule body,
   rule action, global query, or global action.
2. Keep dependency recording as a first-scaffold requirement for higher-order
   container operations. A safe `vec-map` over a lookup function can start as a
   coarse Rust implementation, but it must record the container id, callback
   function, captured args, and every state read that affects the output.
3. Treat provider/indexed container matching as the long-term replacement for
   `fill_index`-style helper blowup. PR #856's move of
   `unstable-multiset-fill-index` to global-only action is evidence that
   user-maintained index filling inside rule actions is not the right safe
   seminaive abstraction.
4. Model higher-order function application as effect-polymorphic. In a
   seminaive query, allow only pure callbacks or callbacks with declared reads
   whose dependencies the backend can maintain. In a rule action, forbid hidden
   custom-function reads unless they were already explicit in the match or are
   declared dependencies.
5. Keep PR #832-style dirty refresh in the compatibility surface. Any DD-owned
   container implementation must emit visible refresh deltas when canonicalized
   container contents change while the logical container id remains stable.
6. Do not require the first DD scaffold to implement query-defined primitives or
   native container matching. It should, however, leave a clear place to attach
   typed capability metadata and declared-read/provider dependencies later.

## Bottom Line

PR #856 is pointing in the right direction for the public primitive API: it
turns a broad raw `ExecutionState` escape hatch into a typed capability surface,
and it makes higher-order container primitives confront context-sensitive
dispatch. The remaining risk is that seminaive safety is broader than API
capabilities. For containers and higher-order functions, the DD design should
learn from #856 but center the invariant from #832: any hidden dependency must
be surfaced as a delta or an explicit declared dependency.
