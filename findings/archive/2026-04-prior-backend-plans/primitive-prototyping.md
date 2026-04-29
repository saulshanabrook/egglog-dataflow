# Primitive Prototyping Notes

Date: 2026-04-29

Status: focused companion to
[`dd-refactor-high-level-fixes.md`](dd-refactor-high-level-fixes.md). This note
collects detailed syntax/prototype mechanics for named primitives, inline
lambdas, query-backed returns, merge/reduce behavior, and the A/C experiments
referenced by the high-level note. It is not a full language proposal.

## Goal

Prototype a small primitive surface that unifies:

- pure scalar primitives;
- query-backed named primitives;
- inline lambdas for `vec-map`, `vec-filter`, and `vec-flat-map`;
- e-class pattern matching through existing equality syntax;
- recursive query views inside functional-style container operations;
- three A/C prototype shapes: container folds, indexed providers, and binary
  recursive residuals; and
- deterministic reduction when a query returns multiple candidate outputs.

The key design constraint is that primitive bodies should be planner-visible
when they read egglog state. A primitive can look like a function at the surface
but must lower to query IR when it has a `:when` clause.

## Syntax Sketch

### Pure Named Primitive

```lisp
(primitive add1-div (i64 i64) i64
  (+ (/ _0 _1) 1))
```

Meaning:

- `_0` and `_1` are the primitive inputs.
- The final expression is the return value.
- With no `:when`, this is a pure expression primitive.
- Default merge is irrelevant because there is exactly one expression result.

### Query-Backed Named Primitive

```lisp
(let x (Add (num 0) (num 1)))
(get-args (unstable-fn "Add") x) ;; -> Pair[Num, Num]
;; === Pair()


(primitive add-children (Expr) (Pair Expr Expr)
  (Pair a b)
  :when ((= _0 (Add a b)))
  :merge assert-eq)
```

Meaning:

- Match facts in `:when`.
- Evaluate `(Pair a b)` for every match.
- Reduce all candidate outputs for the same input with `:merge`.
- `assert-eq` accepts one result or equal results and errors on conflicting
  outputs.

This makes direct e-class matching use the ordinary query equality syntax:

```lisp
(= _0 (Add a b))
```

The backend can lower this to e-class/e-node provider reads, but the user does
not need a separate class-matching form.

### Inline Lambda

```lisp
(lambda (Expr) (Pair Expr Expr)
  (Pair a b)
  :when ((= _0 (Add a b)))
  :merge assert-eq)
```

The lambda can lower to a generated primitive id plus captured arguments. The
important difference from an opaque host callback is that its `:when` body is
still query IR.

Example use in a vector operation:

```lisp
(vec-flat-map xs
  (lambda (Expr) (Pair Expr Expr)
    (Pair a b)
    :when ((= _0 (Add a b)))))
```

The query plan is visible:

```text
VecElem(xs, i, x)
ENodeProvider(x, Add(a, b))
=> OutputElem(out, i, Pair(a, b))
```

The first prototype does not need `VecElem`; it can run a dependency-tracked
Rust map and record reads. The query-backed lambda shape is still the desired
lowering target for later fine-grained maintenance.

### Recursive Query Use

```lisp
(query-fixpoint Reach ((a Expr) (b Expr))
  ((Edge a b))
  ((Reach a c) (Edge c b)))

(vec-flat-map xs
  (lambda (Expr) (Pair Expr Expr)
    (Pair a b)
    :when ((= _0 (Add a b))
           (Reach a b))))
```

The recursive view must expose its read set and complete to a frontier before
the lambda observes its results.

## A/C Prototype Shapes

These examples mirror the high-level note. They are all draft syntax unless a
paragraph explicitly names current Python/core behavior.

### Container Fold

This is the query-defined version of the current multiset-fold article pattern:
`constants = xs.map(get_i64)`, `remaining = xs - constants.map(UnstableFn(Num))`,
and `folded = multiset_fold(i64.__add__, i64(0), constants)`.

```lisp
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

Prototype requirement: the map, reverse constructor map, and fold must expose
their reads. A coarse Rust implementation can recompute the whole container map
when `get-i64` changes; a later provider-backed version can maintain element
dependencies.

### Direct Multiset Pattern / Indexed Provider

Desired surface syntax:

```lisp
(rewrite
  (sum (multiset-of (Num y) (Num z) ...xs))
  (sum (multiset-of (Num (+ y z)) ...xs)))
```

Possible provider lowering:

```lisp
(rewrite (sum terms)
  (sum (multiset-replace-two terms i j (Num (+ y z))))
  :when ((multiset-elem terms i (Num y))
         (multiset-elem terms j (Num z))
         (where (< i j))))
```

Prototype requirement: start with a provider that can enumerate element
positions or value counts for a multiset without requiring the user to maintain
the current `fill_index(ms_num_index)` helper relation in the e-graph.

### Binary Tree Recursive Residual

This keeps binary tree storage and expresses A/C-style matching as recursive
views:

```lisp
(query-fixpoint plus-term ((root Expr) (term Expr))
  ((= root term))
  ((= root (+ l r)) (plus-term l term))
  ((= root (+ l r)) (plus-term r term)))

(query-fixpoint plus-remove ((root Expr) (picked Expr) (rest Expr))
  ((= root (+ picked rest)))
  ((= root (+ rest picked)))
  ((= root (+ l r))
   (plus-remove l picked l-rest)
   (= rest (+ l-rest r)))
  ((= root (+ l r))
   (plus-remove r picked r-rest)
   (= rest (+ l r-rest))))

(rewrite expr
  (+ (Num (+ y z)) rest2)
  :when ((plus-remove expr (Num y) rest1)
         (plus-remove rest1 (Num z) rest2)))
```

Prototype requirement: the recursive view needs a finite frontier before the
rewrite consumes it, and the residual needs duplicate/identity handling that is
explicit enough to cost and debug.

## Merge And Reduce Semantics

A query-backed primitive may produce zero, one, or many candidate outputs for a
single input tuple.

Use `:merge` to collapse those outputs when the primitive's type promises one
result:

```lisp
:merge assert-eq
:merge min
:merge max
:merge union
:merge vec-append
:merge custom-merge-fn
```

Use `:default` when zero matches should return a value:

```lisp
(primitive maybe-add-children (Expr) (Option (Pair Expr Expr))
  (Some (Pair a b))
  :when ((= _0 (Add a b)))
  :merge assert-eq
  :default None)
```

Important implementation point: `:merge` is a maintained reduce, not just a
one-shot callback. If candidate outputs can retract, the backend may need the
full candidate multiset to recompute the reduced value.

`assert-eq` should be the default for query-backed single-result primitives
because it preserves the existing "panic/error on conflicting result" style.

## Cardinality By Combinator

Avoid a separate `:cardinality` option at first. Let the caller determine how
many results are acceptable.

- `vec-map` expects exactly one output per input unless a default is provided.
- `vec-filter` treats existence of at least one match as true.
- `vec-flat-map` accepts zero or many outputs per input.
- A named primitive returning a scalar reduces candidates with `:merge`.
- A named primitive returning a container can use `:merge vec-append` or a
  domain-specific merge.

## Actionful Variant

Keep actionful primitive callbacks separate:

```lisp
(primitive-action record-add (Expr)
  :when ((= _0 (Add a b)))
  :do ((set (has-add _0) true)))
```

Actionful callbacks must run behind the normal rule/action barrier. They should
not be valid inside pure `vec-map` or scalar primitive evaluation unless the
language explicitly introduces a scheduled/actionful container operation.

## Prototype Plan

1. Parse a minimal named `primitive` form with positional arguments and a pure
   expression body.
2. Parse `:when` and lower it to the existing query/fact IR.
3. Treat `(= _0 (Ctor a b))` in `:when` as provider-backed e-class matching in
   the prototype, even if the first backend implementation is an oracle or
   interpreter.
4. Implement `:merge assert-eq` and one nontrivial reduce such as `min`.
5. Add inline `lambda` as syntax sugar for a generated primitive id plus
   captures.
6. Wire the lambda into one container combinator, preferably `vec-flat-map`
   first because it avoids forcing one result per input.
7. Add dependency recording for any Rust-side fallback implementation so hidden
   reads invalidate or retimestamp derived results.
8. Add one recursive query-view example only after the basic `:when` lowering and
   merge behavior are working.

## First Test Cases

### Pure Primitive

```lisp
(primitive add1-div (i64 i64) i64
  (+ (/ _0 _1) 1))
```

Checks parsing, typechecking, positional args, and pure expression evaluation.

### E-Class Pattern Primitive

```lisp
(primitive add-children (Expr) (Pair Expr Expr)
  (Pair a b)
  :when ((= _0 (Add a b)))
  :merge assert-eq)
```

Checks provider-backed equality pattern matching and conflict behavior.

### Flat Map Query Lambda

```lisp
(vec-flat-map xs
  (lambda (Expr) (Pair Expr Expr)
    (Pair a b)
    :when ((= _0 (Add a b)))))
```

Checks lambda lowering and zero/many result behavior.

### Merge Retraction

```lisp
(primitive smallest-child (Expr) Expr
  a
  :when ((= _0 (Add a b)))
  :merge min)
```

Checks that the backend keeps enough candidate state to recover if the current
minimum retracts.

## Open Questions

- Should `primitive` bodies allow local `let` bindings before the return
  expression?
- Should `:when` allow nested schedules immediately, or only after query views
  exist?
- What is the exact conflict error for `:merge assert-eq`?
- How should generated lambda primitive names appear in diagnostics and
  extraction output?
- Which current `UnstableFn` behaviors should remain as compatibility shims
  while query-defined primitives are prototyped?
