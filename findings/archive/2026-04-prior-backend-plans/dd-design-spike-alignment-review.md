# DD Design Spike Alignment Review

Date: 2026-04-29

Status: review of
[`dd-full-refactor-design-spike.md`](dd-full-refactor-design-spike.md) against
[`dd-refactor-high-level-fixes.md`](dd-refactor-high-level-fixes.md) and the PR
#856 / issue #772 typed execution-state evidence summarized in
[`pr-856-typed-execution-state-review.md`](pr-856-typed-execution-state-review.md).

## Overall Assessment

The design spike is strongest on maintainability: it moves execution ownership
to one DD-owned backend, rejects a permanent bridge/native mirror, and keeps the
first implementation narrow enough to start. It also captures key backend
semantics such as per-rule freshness, signed output netting, rebuild deltas,
dirty-refresh events, and scheduler barriers.

The main gap is that the spike treats primitive/Rust API replacement mostly as
a migration inventory item. PR #856 and issue #772 show that this is also a
semantic boundary: the backend must know the context in which a primitive runs,
and seminaive safety requires hidden reads to become either explicit query atoms,
declared dependencies, or visible deltas.

## Findings

### P1: Rule Query Primitives Are Too Broadly Described

The semantic matrix says the MVP should support "pure/read-only primitive
filters in compiled fragments," and the scaffold repeats "pure primitive query
filters" later:

- [`dd-full-refactor-design-spike.md#L177`](dd-full-refactor-design-spike.md)
- [`dd-full-refactor-design-spike.md#L317`](dd-full-refactor-design-spike.md)
- [`dd-full-refactor-design-spike.md#L361`](dd-full-refactor-design-spike.md)

PR #856 / issue #772 sharpen this distinction: seminaive rule queries can run
pure primitives, but read-only database primitives are not sound unless their
reads are explicit in the query plan or reported as declared dependencies. The
high-level note makes the same point as "dependency visibility" for
`unstable-vec-map` and callbacks.

Recommended fix: change the scaffold language to "pure primitive filters only"
for seminaive rule bodies. Add a separate future/extension row for
declared-read primitives, with dependency recording as the admission condition.

### P1: Higher-Order Container Safety Is Later Than The High-Level Gate Allows

The high-level note says the immediate semantic fix is dependency visibility for
currently possible higher-order/container operations, and the mergeability gate
includes dependency-tracked `unstable-vec-map` or equivalent:

- [`dd-refactor-high-level-fixes.md#L63`](dd-refactor-high-level-fixes.md)
- [`dd-refactor-high-level-fixes.md#L77`](dd-refactor-high-level-fixes.md)
- [`dd-refactor-high-level-fixes.md#L326`](dd-refactor-high-level-fixes.md)

The design spike instead classifies containers as post-MVP parity, marks full
container tests as expected failures, and leaves container/API parity to PR 7:

- [`dd-full-refactor-design-spike.md#L185`](dd-full-refactor-design-spike.md)
- [`dd-full-refactor-design-spike.md#L245`](dd-full-refactor-design-spike.md)
- [`dd-full-refactor-design-spike.md#L389`](dd-full-refactor-design-spike.md)
- [`dd-full-refactor-design-spike.md#L463`](dd-full-refactor-design-spike.md)

That is defensible for the first scaffold, but the spike currently reads as if
all container/HOF safety can wait. PR #856 makes this risky because safe
containers are not only "container parity"; they are the current motivating
example for hidden dependencies.

Recommended fix: keep broad native container matching post-MVP, but split out an
early narrow "dependency-tracked HOF container callback" slice. It can be coarse
Rust-level invalidation, not provider-level `VecElem`, but it should be named as
a mergeability or ownership-gate proof point.

### P1: Typed Execution Contexts Are Missing From The Backend Contract

The spike says to move backend-neutral primitive/Rust-rule/action context traits
and inventory API breaks, but it does not require the backend boundary to
preserve PR #856's four execution contexts:

- [`dd-full-refactor-design-spike.md#L104`](dd-full-refactor-design-spike.md)
- [`dd-full-refactor-design-spike.md#L114`](dd-full-refactor-design-spike.md)
- [`dd-full-refactor-design-spike.md#L325`](dd-full-refactor-design-spike.md)
- [`dd-full-refactor-design-spike.md#L423`](dd-full-refactor-design-spike.md)

The PR #856 review concludes that `ResolvedCoreRule` lowering should know
whether it is compiling a rule body, rule action, global query, or global
action, and should treat capability safety separately from freshness safety:

- [`pr-856-typed-execution-state-review.md#L191`](pr-856-typed-execution-state-review.md)
- [`pr-856-typed-execution-state-review.md#L202`](pr-856-typed-execution-state-review.md)

Recommended fix: make the four-context model part of the backend scaffold
contract, even if the initial implementation only supports pure filters and
host-side actions. The action ABI should explicitly say rule actions may write
but cannot perform hidden live reads unless those reads were matched or declared.

### P2: The Top-Level Values Are Not Reflected Explicitly In The Spike

The high-level note starts with the three meta goals: research platform,
real-world utility, and maintainability:

- [`dd-refactor-high-level-fixes.md#L10`](dd-refactor-high-level-fixes.md)

The spike's executive decision strongly expresses maintainability: single owner,
no permanent mirrored runtime, and bridge-as-scaffolding:

- [`dd-full-refactor-design-spike.md#L7`](dd-full-refactor-design-spike.md)
- [`dd-full-refactor-design-spike.md#L20`](dd-full-refactor-design-spike.md)

It expresses real-world utility through semantic preservation gates, test
ladders, and native-oracle comparison. It expresses research-platform value only
indirectly through later unknowns and links to the high-level note.

Recommended fix: add a short "Meta Goal Alignment" section near the top of the
spike. It should say explicitly:

- Maintainability drives the single-owner DD runtime and bridge deletion.
- Real-world utility drives semantic preservation, compatibility inventory, and
  performance/regression gates.
- Research-platform value drives provider/dependency hooks and defers stable
  A/C/query-primitive syntax decisions until experiments can compare them.

### P2: Performance Is A Metric, But Not Yet A Merge Constraint

The high-level value says performance is a merge requirement, while the spike
says the scaffold need not prove a performance win and lists metrics before
performance claims:

- [`dd-refactor-high-level-fixes.md#L18`](dd-refactor-high-level-fixes.md)
- [`dd-full-refactor-design-spike.md#L389`](dd-full-refactor-design-spike.md)
- [`dd-full-refactor-design-spike.md#L473`](dd-full-refactor-design-spike.md)

Those can coexist, but the spike should make the distinction explicit. PR #856
is a concrete warning: a semantically useful API refactor can still regress hot
paths enough to threaten mergeability.

Recommended fix: keep "no performance win required" for the scaffold, but add a
"no unbounded or unexplained hot-path regression" gate. At minimum, require
before/after timing for primitive dispatch, Rust rules, and the selected DD
vertical slice before claiming a mergeable backend path.

### P2: Action Semantics Need The PR #856 Read/Write Split

The spike says primitive actions execute host-side at action application
boundaries and that host-side actions apply after output netting:

- [`dd-full-refactor-design-spike.md#L178`](dd-full-refactor-design-spike.md)
- [`dd-full-refactor-design-spike.md#L322`](dd-full-refactor-design-spike.md)
- [`dd-full-refactor-design-spike.md#L363`](dd-full-refactor-design-spike.md)

That covers the barrier shape, but not the #772 invariant that seminaive rule
actions may write but should not read live database state. The PR #856 review
found this is still a live issue for `unstable-app` over custom functions.

Recommended fix: add an action-context invariant: rule actions are functions of
the matched bindings plus write-only action capabilities. Any action-side read
must be made an explicit matched input, rejected, or represented as a declared
dependency with a wakeup plan.

### P3: Extension Readiness Is Linked, But Not Yet Testable

The spike correctly says query-defined primitives, lambdas,
native/provider-backed matching, and merge/reduce semantics are follow-up
language/runtime design work:

- [`dd-full-refactor-design-spike.md#L287`](dd-full-refactor-design-spike.md)

The high-level note asks for an extension-readiness gate that can host one
container-HOF strategy, one indexed-provider strategy, one binary fixpoint
strategy, one query-defined primitive/lambda, one solver constraint, and one
schedule-aware view:

- [`dd-refactor-high-level-fixes.md#L338`](dd-refactor-high-level-fixes.md)

The spike's PR sequence only has a broad "Container/API parity PRs" bucket. That
keeps the first scaffold clean, but it does not yet give future agents a
concrete extension-readiness check.

Recommended fix: add a post-ownership-gate "extension readiness probe" milestone
or fold the high-level gate into PR 7. This does not require choosing stable
syntax; it only requires demonstrating that the backend hooks can host the
experiments.

## What The Spike Already Handles Well

- **Maintainability:** strong. The single-owner DD runtime, bridge deletion path,
  and no permanent backend switch align directly with the high-level values.
- **Dirty refresh:** addressed explicitly through runtime ownership,
  same-id dirty-refresh experiment results, and the Option 3 ownership gate.
- **Schedule/action barriers:** addressed through output netting, probes,
  scheduler materialization, and host-side action boundaries.
- **Narrow mergeable start:** addressed through the scaffold/non-goals split and
  expected-failure list.

## Suggested Patch Shape

Do not rewrite the whole spike. Add targeted clarifications:

1. Add "Meta Goal Alignment" after the executive decision.
2. In the semantic matrix, change rule-query primitive support from
   "pure/read-only" to "pure; declared-read later."
3. Add a row for typed execution contexts / capability safety.
4. Add a row or ownership-gate bullet for dependency-tracked HOF container
   callbacks.
5. Add a rule-action invariant: write-only unless reads are explicit or
   declared.
6. Add a performance regression smoke gate distinct from "prove a performance
   win."
7. Add an extension-readiness probe after the ownership gate, linked to
   `dd-refactor-high-level-fixes.md`.
