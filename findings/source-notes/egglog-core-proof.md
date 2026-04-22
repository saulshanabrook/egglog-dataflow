# Egglog Core + Proof Encoding

## Sources Read
- `repos/egglog/src/core.rs`: surface rules lower to conjunctive-query bodies plus SSA-like actions; canonicalization and functional-dependency cleanup live here.
- `repos/egglog/src/lib.rs`: `EGraph` owns the bridge backend, rulesets, seminaive flag, proof state, command resolution, and backend rule construction.
- `repos/egglog/src/scheduler.rs`: optional scheduler path splits each rule into query and action rules with a materialized match table.
- `repos/egglog/core-relations/src/free_join/mod.rs`: database, table notifications, merge fixed point, rebuild hooks, and query substrate.
- `repos/egglog/core-relations/src/table_spec.rs`: table/rebuilder traits and the explicit escape hatch for value-level rebuild and same-id refresh.
- `repos/egglog/core-relations/src/table/rebuild.rs`: incremental/nonincremental table rebuild and retimestamping for rows mentioning dirty ids.
- `repos/egglog/core-relations/src/containers/mod.rs`: container rebuild semantics, dirty ids, and same-id container refresh conditions.
- `repos/egglog/union-find/src/lib.rs`: single-threaded union-find customized for e-graph rebuild work via union-by-min.
- `repos/egglog/union-find/src/concurrent/uf.rs`: concurrent union-find and its stated complexity/heuristic tradeoffs.
- `repos/egglog/src/proofs/proof_encoding.rs`: term/proof instrumentation that reifies UF, views, congruence, merge, and rebuild as egglog rules.
- `repos/egglog/src/proofs/proof_encoding_helpers.rs`: proof datatypes and unsupported proof-mode constructs.
- `repos/egglog/src/proofs/proof_format.rs`: extracted proof representation over a `TermDag`.
- `repos/egglog/src/proofs/proof_checker.rs`: proof checker semantics for rules, unions, merge functions, congruence, and reflexive subterms.
- `repos/egglog/src/proofs/proof_extraction.rs`: proof extraction path from instrumented proof tables through extraction and checking.
- `repos/egglog/src/termdag.rs`: hash-consed term DAG used for extracted terms/proofs and shared-term printing.
- `repos/egglog/tests/proofs/commute-collapse.egg`: small proof example requiring commutation then constant folding.
- `repos/egglog/tests/container-rebuild.egg`: examples where container equality becomes visible through rebuild.
- `repos/egglog/tests/merge-during-rebuild.egg`: regression that non-union merge functions must fire during rebuild.
- `repos/scaling-equality-saturation/egglog-new-backend.md`: maintainer design draft describing arbitrary schedules, per-rule seminaive timestamps, timestamp-ordered tables, two-phase query/merge execution, constructor/function split, rebuild, table providers, and Free Join.

## Key Findings
- The normal compiler boundary is already relational: `CoreRule` is documented as conjunctive-query body plus SSA-like action head, with query compilation to GJ and action compilation to a small VM (`repos/egglog/src/core.rs:1`). `BackendRule` maps function atoms to table queries, primitives to external calls, and heads to `set`/`delete`/`subsume`/`union` actions (`repos/egglog/src/lib.rs:1986` and `repos/egglog/src/lib.rs:2006`).
- A DD-like substrate could plausibly absorb rule matching, seminaive deltas, indexes, and recursive fixed-point scheduling. Current `step_rules` just collects rule ids and delegates to `backend.run_rules` (`repos/egglog/src/lib.rs:929`), while `core-relations` already tracks modified tables via a notification list and merges pending updates to a fixed point (`repos/egglog/core-relations/src/free_join/mod.rs:282` and `repos/egglog/core-relations/src/free_join/mod.rs:488`).
- Eli's backend draft sharpens what "seminaive deltas" means for egglog: arbitrary user schedules break the ordinary global recent/stable/new criterion, so rows carry logical timestamps and each rule tracks the last timestamp it ran at. An exact DD-backed rule layer must preserve per-rule timestamp windows, not only global relation deltas; a relaxed scheduling mode would need an explicit opt-in contract for where that exact behavior is not promised (`repos/scaling-equality-saturation/egglog-new-backend.md`, `findings/source-notes/scaling-equality-saturation.md`).
- The current native backend is structured for two-phase execution: rule queries read immutable table state and stage mutations, then table merges apply staged updates in bulk. The constructor/function split exists partly to make this low-coordination query phase possible while retaining nested constructor insertion and merge semantics (`repos/scaling-equality-saturation/egglog-new-backend.md`).
- The hard part is not only joins; equality changes mutate the meaning of existing ids. Rebuild is a value-level operation that rewrites table contents using a `Rebuilder`, and the trait explicitly says doing this efficiently with rules is difficult because it requires finding changes in any column (`repos/egglog/core-relations/src/table_spec.rs:103`). This is exactly the kind of non-monotone, update-every-reference maintenance that may not map cleanly to ordinary DD collections.
- Existing rebuild has several specialized fast paths: table rebuild chooses incremental vs full scan (`repos/egglog/core-relations/src/table/rebuild.rs:58`), maintains a rebuild index (`repos/egglog/core-relations/src/table/rebuild.rs:33`), and retimestamps dirty parent rows so seminaive treats them as fresh deltas (`repos/egglog/core-relations/src/table/rebuild.rs:77`). A DD port would need an equivalent invalidation story, not just differential joins.
- Containers expose a likely failure mode for a naive relational encoding. `ContainerRebuildSummary::dirty_ids` records when container semantics change while the outer id stays stable; ordinary table rebuild handles changed-id cases, but stable dirty ids need parent-row refresh or seminaive misses newly matchable rows (`repos/egglog/core-relations/src/containers/mod.rs:76` and `repos/egglog/core-relations/src/free_join/mod.rs:371`).
- Union-find is deliberately not just an abstract equivalence relation. Both UF implementations use union-by-min to perturb fewer ids during congruence closure, while comments admit this does not provide union-by-rank asymptotics and is a heuristic for e-graph rebuild locality (`repos/egglog/union-find/src/lib.rs:6` and `repos/egglog/union-find/src/concurrent/uf.rs:96`). DD would need to preserve or replace that locality benefit.
- Proof/term encoding is the strongest design hint: it reifies each sort's parent table, each function's term/view tables, congruence, merge, path compression, and rebuilding as ordinary egglog rules (`repos/egglog/src/proofs/proof_encoding.rs:96`, `repos/egglog/src/proofs/proof_encoding.rs:335`, `repos/egglog/src/proofs/proof_encoding.rs:403`, and `repos/egglog/src/proofs/proof_encoding.rs:1128`). This suggests a relational equality-maintenance design is semantically possible, but probably expensive.
- Proof mode shows which features resist a fully relational/proof-carrying substrate. It rejects primitives without validators, function lookups in actions, presort/custom sort containers, user commands, input commands, and non-global no-merge functions (`repos/egglog/src/proofs/proof_encoding_helpers.rs:463` and `repos/egglog/src/proofs/proof_encoding_helpers.rs:535`). These are concrete compatibility blockers for any substrate that wants proof or replay semantics.
- Equality proofs are not mere provenance labels. `ProofStore` models egglog as a partial equality relation closed under symmetry, transitivity, and congruence, and explicitly does not assume reflexivity for arbitrary terms (`repos/egglog/src/proofs/proof_format.rs:80`). The checker reconstructs propositions from rule actions, unions, sets, merge functions, primitive validators, and reflexive subterms (`repos/egglog/src/proofs/proof_checker.rs:76` and `repos/egglog/src/proofs/proof_checker.rs:143`).
- Proof/term encoding is useful as a partial relational specification and validation oracle, but it does not cover the full container, Python, scheduler, presort, primitive, or custom frontend surface. Unsupported proof-mode constructs still need native regressions and cannot be treated as a complete end-to-end substitute (`repos/egglog/src/proofs/proof_encoding_helpers.rs:463` and `repos/egglog/src/proofs/proof_encoding_helpers.rs:535`).

## Relevance To The Main Objective
- Supports the objective: the core language and proof encoding already describe egglog as relations plus rules over term/view/UF tables, so DD could be used as the substrate for matching, incremental joins, and some equality-maintenance relations.
- Weakens the objective: the production backend contains custom mutation, rebuilding, per-rule timestamp freshness, timestamp-range table access, container, merge-function, two-phase execution, and union-by-min locality mechanisms that are not obviously DD primitives. Replacing them risks semantic misses like the dirty-id/container case or performance regressions from over-materializing equality maintenance.

## Likely Blockers
- Rebuild is non-local: a UF merge can require rewriting every table column that stores affected ids, plus container contents and parent rows.
- Same-id semantic changes are not ordinary insert/delete deltas; DD would need explicit invalidation/retimestamping for rows whose keys did not change.
- Arbitrary schedules make seminaive correctness rule-local. A backend that only tracks globally recent tuples can miss facts under schedules where one ruleset saturates before another.
- Merge functions can fire during rebuild, not only during user `set` actions (`repos/egglog/tests/merge-during-rebuild.egg:1`), so a DD design must handle user-defined merge code inside equality maintenance.
- Proof/term encoding is incomplete for current egglog features, including action lookups, custom sort containers, some primitive cases, input commands, and no-merge functions.
- Proof/term encoding is only a partial validation path: it can specify supported equality and rebuild behavior, but it does not validate the full frontend surface around containers, Python commands, schedulers, presorts, primitives, or custom frontend extensions.
- Union-by-min is a semantic/performance heuristic coupled to rebuild locality; a pure equivalence-closure relation might produce too much churn.
- Proof extraction depends on finding proof terms in instrumented tables, extracting them even when unextractable flags would normally apply, then checking and simplifying (`repos/egglog/src/proofs/proof_extraction.rs:44` and `repos/egglog/src/proofs/proof_extraction.rs:128`).

## Promising Connections
- Treat `CoreRule` as the stable frontend IR and target DD from the same query/action split that `BackendRule` currently targets.
- Use proof term encoding as an executable specification for relational equality maintenance: per-sort UF tables, per-function view tables, congruence rules, merge cleanup, and scheduled rebuild.
- Model dirty-id handling as a first-class differential invalidation stream, not as an afterthought.
- Treat per-rule last-run timestamps and timestamp-window scans as part of the rule-evaluation API, not as a private table optimization.
- Preserve `TermDag`-style hash-consing for proof/extraction output while using relations for maintenance.
- Reuse proof checker requirements as a test oracle: a DD-backed proof mode should produce proofs accepted by `ProofStore::check_proof`.

## Evidence Needed Next
- Measure normal backend vs proof/term encoding on rebuild-heavy tests to estimate the overhead of fully relationalized equality maintenance.
- Build a tiny DD prototype for constructors, union-by-min parent relation, view relation, congruence, and rebuild; compare against `container-rebuild.egg` and `merge-during-rebuild.egg`.
- Inspect `egglog-bridge` rule execution and `core-relations/src/free_join/execute.rs` to separate join-planning cost from mutation/rebuild cost.
- Reproduce the scheduled reachability example from `repos/scaling-equality-saturation/egglog-new-backend.md` and use it as a semantic regression for per-rule seminaive freshness.
- Find or create a minimal benchmark where dirty same-id container changes are required for seminaive visibility, then check whether a DD invalidation stream handles it.
- Check whether DD can express path compression/parent canonicalization without repeatedly churning large parts of the collection.

## Confidence
- Medium: the path-level evidence is strong for core/proof/rebuild mechanisms, but I did not run performance measurements or inspect every bridge execution path needed to quantify feasibility.
