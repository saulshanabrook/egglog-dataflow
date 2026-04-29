# DD Refactor High-Level Goals

Date: 2026-04-29

Status: companion note to
[`dd-full-refactor-design-spike.md`](dd-full-refactor-design-spike.md). The
design spike defines the first DD runtime scaffold. This note states the broader
reason to do the refactor and the extension surface it should leave open.

## Meta Goals

The backend refactor should serve three project goals at once:

1. **Research platform.** Egglog should stay flexible enough to try new e-graph
   ideas in user land or thin extension layers, including different
   representations for functions, A/C structure, containers, solvers, and
   scheduling.
2. **Real-world utility.** The rewrite should preserve existing behavior as much
   as possible, keep compatibility breaks explicit, and treat performance as a
   merge requirement rather than a later cleanup problem.
3. **Maintainability.** The architecture should reduce total project complexity,
   avoid a permanent mirrored backend split, and keep responsibilities modular
   enough that new research paths do not require rewriting core execution.

## Core Framing

We do not yet know the single best way to represent functions, A/C structure, or
container-heavy matching in egglog. The refactor should not prematurely choose
one representation and bake it into the backend.

The goal is a backend that is general enough, and still performant enough, to let
users and experiments try several strategies in user land:

- native containers plus higher-order operations;
- native containers plus explicit indexed matching;
- binary e-graph representations plus recursive/fixpoint matching;
- query-defined primitives and lambdas;
- solver-backed constraints; and
- schedule-aware derived views.

The first mergeable backend rewrite should preserve today's semantics and
performance envelope, reduce project complexity, and make existing behavior
safer. The later extension goal is to make experiments easier without committing
now to which syntax or representation should become canonical.

Syntax status: code sketches in this note are draft or idealized unless a
paragraph explicitly says it is describing current Python/core behavior.

## First Priority: Preserve And Simplify

The near-term refactor should be judged by whether it can plausibly be merged.
That means:

- preserve existing egglog semantics, including schedules, rebuild,
  canonicalization, containers, primitives, and action barriers;
- preserve or improve the current performance envelope on the selected slice;
- reduce overall backend complexity instead of adding a permanent second
  execution stack;
- make currently possible higher-order/container operations seminaive-safe; and
- use native egglog as an oracle during migration, not as a permanent mirrored
  production backend.

The most important immediate semantic fix is dependency visibility. Existing
operations such as `unstable-vec-map` can hide reads of function tables,
captured arguments, containers, and canonicalized values. The first slice does
not need native container matching to fix this. A dependency-tracked Rust
implementation can be acceptable:

```text
with_dependency_recorder(task):
  xs = read_container(xs_id)
  for x in xs:
    y = apply_callback(f, x)
    push(y)
```

This is coarse: one changed callback row may rerun the whole map. But it is
sound if every state read is recorded and later table writes, rebuild events,
deletes, subsumes, or dirty refreshes invalidate or retimestamp dependent
outputs. Fine-grained `VecElem`-style views can come later.

## Backend Shape To Preserve Flexibility

The backend should expose a small set of general mechanisms rather than a large
set of special-purpose primitive ops:

- explicit read sets for host primitives and higher-order callbacks;
- provider relations for structured data and e-class/e-node views;
- `:when` query bodies for query-defined primitives and lambdas;
- maintained reduce/merge for multi-result queries;
- recursive `query-fixpoint` views;
- solver/constraint providers with boundness and completeness contracts; and
- schedule-aware derived views with explicit barriers.

These mechanisms should make hidden dependencies visible to seminaive freshness,
planning, rebuild, and scheduling. They should also let experiments stay in
egglog programs or thin extension layers instead of requiring a new backend fork
for every representation idea.

## A/C Matching Should Stay Open

A/C is the clearest reason not to pick one representation too early. At least
three useful designs should remain expressible, without ranking them before
experiments show which one fits a domain.

### 1. Container Representation With Higher-Order Functions

Represent the A/C node as a native container or multiset and operate over it with
query-defined folds, maps, or reductions.

**Surface rewrite sketch.** For constant folding, this can be one blockwise
rewrite. The current Python container article implements the shape with
`xs.map(get_i64)`, `constants.map(UnstableFn(Num))`, and
`multiset_fold(...)`. This is an idealized query-defined syntax derived from
that pattern:

```lisp
(function sum (MultiSet Expr) Expr)

(primitive get-i64 (Expr) i64
  i
  :when ((= _0 (Num i)))
  :merge assert-eq)

(rewrite (sum xs)
  (sum (multiset-insert remaining (Num folded)))
  :when ((= constants (multiset-map get-i64 xs))
         (= remaining
            (multiset-difference xs
              (multiset-map (lambda (i64) Expr (Num _0)) constants)))
         (= folded (multiset-fold + 0 constants))
         (> (multiset-length constants) 1)))
```

**How it maps to backend mechanisms.** The current core already has unstable
container map/reduce primitives and `UnstableFn` callbacks. The backend work is
to make the container read, callback lookup, captured arguments, and reduction
inputs visible to seminaive invalidation. A first implementation can be coarse
and dependency-tracked in Rust; later provider views can maintain individual
container elements and callback rows.

**Why this strategy is worth testing.** It keeps the normalized bag-like
representation and folds all constants in one rewrite. That is useful for
domains where whole-container operations are natural and repeated pair picking
would add unnecessary e-graph growth.

### 2. Container Representation With Explicit Indexed Matching

Represent the A/C node as a native container but match explicit elements or
indices.

**Surface rewrite sketch.** The desired user-facing rule can look like a direct
multiset pattern:

```lisp
(function sum (MultiSet Expr) Expr)

(rewrite
  (sum (multiset-of (Num y) (Num z) ...xs))
  (sum (multiset-of (Num (+ y z)) ...xs)))
```

One possible provider lowering is an explicit indexed match:

```lisp
(rewrite (sum terms)
  (sum (multiset-replace-two terms i j (Num (+ y z))))
  :when ((multiset-elem terms i (Num y))
         (multiset-elem terms j (Num z))
         (where (< i j))))
```

**How it maps to backend mechanisms.** Today this resembles the
`fill_index(ms_num_index)` workaround from the container article: users create a
helper function from a multiset and value to an index/count-like fact, fill it
for each sum, and then match that helper. The backend goal is provider/index
support that can expose `multiset-elem` or `multiset-count` without forcing the
program to maintain helper facts that grow the e-graph.

**Why this strategy is worth testing.** It is local and explicit. It may take
several iterations to fold all constants, but it can be cheaper or more
controllable than whole-container normalization when the rewrite only needs a
small number of elements.

### 3. Binary Representation With Recursive/Fixpoint Matching

Keep the expression stored as ordinary binary trees, but define recursive query
views that inspect the tree as if it were A/C.

**Surface rewrite sketch.** This is draft query-view syntax over ordinary tree
storage, not an existing feature:

```lisp
(algebraic-op + :assoc true :comm true :identity 0)

(query-fixpoint plus-term ((root Expr) (term Expr))
  ((= root term))
  ((= root (+ l r)) (plus-term l term))
  ((= root (+ l r)) (plus-term r term)))
```

To make a rewrite, the recursive view also needs a residual: remove one matched
term from the tree and return the remaining tree.

```lisp
(query-fixpoint plus-remove ((root Expr) (picked Expr) (rest Expr))
  ;; Pick a direct child.
  ((= root (+ picked rest)))
  ((= root (+ rest picked)))

  ;; Recurse left, then rebuild the original right side.
  ((= root (+ l r))
   (plus-remove l picked l-rest)
   (= rest (+ l-rest r)))

  ;; Recurse right, then rebuild the original left side.
  ((= root (+ l r))
   (plus-remove r picked r-rest)
   (= rest (+ l r-rest))))

(rewrite expr
  (+ (Num (+ y z)) rest2)
  :when ((plus-remove expr (Num y) rest1)
         (plus-remove rest1 (Num z) rest2)))
```

**How it maps to backend mechanisms.** The stored e-graph stays binary. A
recursive `query-fixpoint` view materializes or incrementally maintains the
reachable A/C-style terms and residuals needed by the rewrite. The residual
construction needs design work around identities, duplicates, and cost, but the
backend mechanism is a recursive derived view rather than a special built-in
A/C matcher.

**Why this strategy is worth testing.** It lets users keep ordinary tree
storage while still trying arbitrary A/C-style patterns. That matters for
domains that do not want to normalize representation but do want query-local
recursive matching.

The backend should support all three styles as experiments. It should not encode
"A/C matching" as one mandatory primitive unless experiments show one design is
clearly the right language feature.

## Query-Defined Primitives And Lambdas

Named primitives and inline lambdas could be a main way users prototype these
ideas without adding backend built-ins.

Pure shorthand:

```lisp
(primitive add1-div (i64 i64) i64
  (+ (/ _0 _1) 1))
```

Query-backed primitive:

```lisp
(primitive add-children (Expr) (Pair Expr Expr)
  (Pair a b)
  :when ((= _0 (Add a b)))
  :merge assert-eq)
```

Inline lambda:

```lisp
(vec-flat-map xs
  (lambda (Expr) (Pair Expr Expr)
    (Pair a b)
    :when ((= _0 (Add a b)))))
```

The key rule is that `:when` lowers to query IR, not an opaque host callback.
Candidate outputs are reduced with `:merge` when one output is required. The
default should be conservative:

```lisp
:merge assert-eq
```

Other reducers such as `min`, `max`, `union`, and `vec-append` should be modeled
as maintained reductions over candidate outputs. Under retractions, the backend
may need to retain the candidate multiset, not just the current reduced value.

Actionful forms should remain separate and barriered:

```lisp
(primitive-action record-add (Expr)
  :when ((= _0 (Add a b)))
  :do ((set (has-add _0) true)))
```

This keeps `vec-map` and scalar primitive evaluation from secretly mutating the
e-graph while they are being queried.

Detailed syntax and prototype steps live in
[`primitive-prototyping.md`](primitive-prototyping.md).

## Extension Readiness, Not Extension Commitment

The rewrite should make these features easier to try, but it should not require
settling them before the backend scaffold lands.

Useful extension hooks to preserve:

- query equality patterns over e-classes, such as `(= x (Add a b))`;
- container provider views, such as `vec-elem`, `map-entry`, and
  `multiset-count`;
- solver constraints, such as `(where (= y (+ x 10)))`;
- recursive query views, such as `query-fixpoint`;
- query-defined `primitive` and `lambda`; and
- schedule-aware views.

The design decision to add any of these as stable user-facing syntax can be
deferred. The backend should still be shaped so prototypes do not require
reopening core execution ownership.

## Gates

### Mergeability Gate

This gate should not require query-defined primitives, inline lambdas, or any of
the A/C extension syntax above. Those belong to extension-readiness tests after
the first scaffold proves the existing semantics can move.

The first serious implementation slice should prove:

- real `ResolvedCoreRule` lowering into the DD-owned runtime;
- per-rule seminaive freshness and exact logical schedule behavior;
- signed output netting before host-side actions;
- one rebuild/canonicalization event over maintained state;
- same-id container dirty refresh for current semantics;
- dependency-tracked `unstable-vec-map` or equivalent higher-order callback; and
- native-oracle comparison for the selected semantic slice.

This gate is about preserving and simplifying what exists.

### Extension-Readiness Gate

After the first slice, test whether the backend can host multiple strategies:

- one container higher-order strategy;
- one container indexed-matching strategy;
- one binary recursive/fixpoint matching strategy;
- one query-defined primitive or lambda with `:when` and `:merge`;
- one solver-backed scalar constraint; and
- one schedule-aware derived view.

This gate is not asking which design should become canonical. It asks whether
the backend is general enough to compare them.

## Open Questions

- How coarse can dependency tracking be before recomputation becomes unusable?
- Which provider views should be public syntax and which should stay internal?
- Can query-defined primitives cover most extension needs, or do some domains
  require dedicated built-ins?
- How much candidate state must maintained `:merge` reducers retain under
  deletes, subsumes, and rebuild?
- Can schedule-aware views preserve exact logical schedule semantics while still
  giving the backend room for physical overlap?
