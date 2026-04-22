# Containers + Frontends

## Sources Read
- `repos/egglog-python/docs/explanation/2026_02_containers.md`: main container motivation, multiset polynomial case study, and current limitations.
- `repos/egglog-python/docs/reference/egglog-translation.md`: Python-to-egglog syntax surface for sorts, functions, actions, rules, schedules, extraction, and introspection.
- `repos/egglog-python/python/egglog/builtins.py`: concrete Python-facing container, higher-order function, primitive, and conversion APIs.
- `repos/egglog-python/python/egglog/examples/multiset.py`: current multiset idioms for AC-normalized arithmetic from Python.
- `repos/egglog-python/python/egglog/examples/schedule_demo.py`: Python schedule composition syntax.
- `repos/egglog-python/python/tests/test_high_level.py`: tests for container conversions, preserved Python methods, mutation, `PyObject`, and custom cost models.
- `repos/egglog-tutorial/02-datalog.egg`: tutorial semantics for relations/functions, merge expressions, constructors as function tables, `push`/`pop`, and saturation.
- `repos/egglog-tutorial/04-scheduling.egg`: tutorial scheduling, ruleset partitioning, backoff scheduler motivation, and AC blow-up examples.
- `repos/egglog-experimental/src/lib.rs`: extension registry for parser sugar, rationals, set-cost, size, fresh, schedulers, and multi-extract.
- `repos/egglog-experimental/src/scheduling.rs`: implementation of `run-schedule`, `let-scheduler`, `run-with`, and backoff scheduler hooks.
- `repos/scaling-equality-saturation/egglog-new-backend.md`: backend design draft explaining why arbitrary schedules are needed for blowup control and manual stratification, and why they affect seminaive correctness.

## Key Findings
- Python users define the frontend language through `Expr` subclasses, class methods, overloaded Python operators, `@function`, `@method`, `@ruleset`, `rewrite`, `birewrite`, `rule`, `set_`, `union`, `delete`, and typed variables; this is the compatibility surface, not just the S-expression syntax (`repos/egglog-python/docs/reference/egglog-translation.md:57`, `:90`, `:126`, `:210`, `:327`).
- Generic containers are first-class Python-facing sorts: `Map`, `Set`, `MultiSet`, and `Vec` expose typed constructors, conversion aliases from Python `dict`/`set`/`tuple`/`list`, membership/removal/length/get operations, and explicit `rebuild` methods for congruence (`repos/egglog-python/python/egglog/builtins.py:440`, `:480`, `:548`, `:983`).
- Container semantics are essential, not incidental. The containers write-up says containers are opaque primitives except for equality/hash, primitive functions, and a deferred rebuild operation that updates contained e-class ids to preserve congruence after unions (`repos/egglog-python/docs/explanation/2026_02_containers.md:141`, `:157`, `:164`).
- Multisets are used to replace explicit A/C exploration with normalized representations: `sum_(MultiSet(a, b))` equals `sum_(MultiSet(b, a))`, and after `union(x).with_(y)`, the multiset-backed expression rebuilds to reflect duplicate representatives (`repos/egglog-python/docs/explanation/2026_02_containers.md:168`, `:176`, `:183`, `:194`).
- Matching inside containers is currently indirect. The documented workaround is to fill user-defined index functions such as `ms_num_index(xs, x)` with `xs.fill_index(...)`, then match against those facts; this can itself add e-graph nodes and become a secondary blow-up source (`repos/egglog-python/docs/explanation/2026_02_containers.md:217`, `:221`, `:260`).
- Higher-level container functions are already part of current Python practice: `MultiSet.map`, `filter`, `fold` via `multiset_fold`, `flat_map`, count/pick/pick_max/reset-counts, and `Vec.map` are exposed through `UnstableFn`/`Callable` conversion, including lambda support (`repos/egglog-python/python/egglog/builtins.py:613`, `:628`, `:639`, `:659`, `:1052`, `:1090`, `:1122`).
- The container blog's preferred path is block-wise higher-order processing: map constants out of a multiset, subtract reconstructed constants, fold all constants, and rewrite once, avoiding pairwise index matches (`repos/egglog-python/docs/explanation/2026_02_containers.md:267`, `:289`, `:318`).
- Scheduling must survive as both a frontend feature and runtime semantic constraint. Users rely on rulesets, `run`, `saturate`, `seq`, `repeat`, `run(..., scheduler=...)`, and scheduler scoping to separate analysis from explosive optimization rules (`repos/egglog-python/docs/reference/egglog-translation.md:371`, `:398`, `:475`; `repos/egglog-tutorial/04-scheduling.egg:121`, `:147`, `:203`). Eli's backend draft gives two deeper reasons: schedules control blowup/non-saturating rules, and they provide manual stratification for reasoning that depends on a canonical e-graph shape (`repos/scaling-equality-saturation/egglog-new-backend.md`, `findings/source-notes/scaling-equality-saturation.md`).
- Extension APIs are broad: `egglog-experimental` layers parser macros (`for`, `with-ruleset`), rational primitives, dynamic `set-cost`, `get-size!`, `unstable-fresh!`, custom `run-schedule`, and `multi-extract` onto a base egraph (`repos/egglog-experimental/src/lib.rs:8`, `:45`, `:77`).
- Custom scheduler extension requires more than match-count visibility. The current paths can materialize all matches, let the scheduler choose a subset, and delay action firing until after that choice, so `run-with` and backoff behavior depend on full match materialization plus selective admission (`repos/egglog/src/scheduler.rs`; `repos/egglog-experimental/src/scheduling.rs:33`, `:94`, `:133`, `:302`, `:359`).
- The tutorial's database story maps well to DD vocabulary: relations, functions with merge expressions, constructors as function tables, and functional dependencies are explicit user-visible semantics (`repos/egglog-tutorial/02-datalog.egg:87`, `:97`, `:228`, `:238`).

## Relevance To The Main Objective
- Moving onto Differential Dataflow looks plausible where egglog already presents itself as tables, joins, functions, merges, and schedules, but the substrate must preserve e-class congruence, deferred rebuild, and container normalization as observable behavior.
- DD could help with incremental relation maintenance and indexed matching, but the current frontend assumes imperative egraph actions, extraction/cost hooks, stack-like `push`/`pop`, scheduler-controlled match admission, and per-rule seminaive freshness under arbitrary schedules, so a pure relational core is not enough.
- DD-backed rule evaluation still needs an evidence-backed story for custom schedulers: the engine must preserve full match materialization, subset selection, and delayed action firing, not just expose match counts.
- Containers are a major reason to change substrate only if DD can make index/fold/map workloads cheaper without losing the opaque primitive plus rebuild abstraction that Python users already exercise.

## Likely Blockers
- Container rebuild is not just a function call; it must run at the right point after unions so container equality and downstream matches see canonical e-class ids.
- Higher-order container operations depend on `UnstableFn`, partial application, lambdas converted into default rewrites, and generic-ish typing; the blog explicitly calls primitive function composition and generic support current weak spots.
- Backoff and custom schedulers need visibility into per-rule match counts and the ability to skip or delay matches, which is a blocker/evidence gap for DD-backed rule evaluation if full-match materialization and delayed firing are not preserved.
- Even without custom `run-with` schedulers, arbitrary ruleset schedules require per-rule last-run timestamp semantics; a DD-backed matcher that only models global Datalog strata can be wrong for normal egglog schedules.
- `push`/`pop`, deletion/subsumption, dynamic cost tables, Python preserved methods, `PyObject` calls, and extraction with custom Python cost models are side-effectful or host-integrated APIs that need an adapter layer.
- Index-based container matching can recreate blow-up; a DD port that only materializes more indexes may miss the blog's higher-order/block-wise lesson.

## Promising Connections
- The tutorial's "everything is a function" framing gives a clean translation target: function/relation tables with merges and constructor-default behavior can be mapped to maintained arrangements.
- DD arrangements may be a natural implementation for user-defined container indexes such as multiset count tables, if rebuild invalidation and e-class canonicalization are handled incrementally.
- Higher-order container primitives (`map`, `filter`, `fold`, `flat_map`) look like algebraic operators over collections; implementing them below the egraph layer could preserve the Python API while avoiding per-element e-node churn.
- Scheduler APIs from `egglog-experimental` provide concrete hooks for a DD design to expose: rule match counts, match filtering, rule banning, scheduler scopes, and fast-forward-like behavior.
- Eli's scheduled reachability example provides a small semantic witness for ordinary ruleset scheduling, separate from custom scheduler APIs.
- The multiset polynomial case study supplies a decision benchmark: compare binary A/C/D saturation against multiset-of-multisets plus block-wise factoring on the same extracted workload.

## Evidence Needed Next
- Measure node/table counts and runtime for the documented `2 + a + b + b + 3` and polynomial examples under binary A/C rules, index-based containers, and higher-order container rules.
- Inspect the current Rust container implementation in core egglog to pin down rebuild ordering, dirty-container tracking, and how container equality/hash interacts with e-class ids.
- Trace Python `UnstableFn`/lambda lowering into egglog commands for `MultiSet.map` and `multiset_fold` to identify exactly what the substrate must execute.
- Prototype one DD-style maintained index for `MultiSet.count` or `fill_index` and test whether union/rebuild invalidation preserves the documented `sum_(MultiSet(x, y))` after `x == y` behavior.
- Check whether DD can expose per-rule match counts and selective match application without forcing all matches to materialize first.
- Reproduce a schedule where one ruleset saturates before another and verify that a backend preserves per-rule seminaive freshness, not just global recent/stable tuple sets.

## Confidence
- Medium, because the frontend and documentation evidence is strong, but the exact feasibility depends on core Rust container/rebuild internals not read in this pass.
