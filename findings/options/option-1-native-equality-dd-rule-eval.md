# Option 1: Native Equality + DD/FlowLog Rule Evaluation

## Viability
- Medium. This is the smallest credible split because egglog's frontend is already close to the needed boundary: `CoreRule` is a conjunctive-query body plus SSA-like actions, and the current backend builder separately lowers body atoms to table/primitive queries and heads to `set`/`delete`/`subsume`/`union` actions (`findings/source-notes/egglog-core-proof.md`, `repos/egglog/src/core.rs`, `repos/egglog/src/lib.rs`). DD/FlowLog fits the rule-matching, seminaive delta, arrangement, and recursive fixed-point side. The viability is not High because equality changes are not ordinary relation inserts, and seminaive freshness is rule-local under arbitrary schedules: native rebuild has value-level table rewriting, incremental/full rebuild choices, explicit rebuild-invalidation events, same-id dirty container refresh, per-rule timestamp windows, and merge hooks that must be reflected back into the dataflow input without semantic misses (`findings/source-notes/scaling-equality-saturation.md`).

## General Approach
- Keep egglog's union-find, congruence/rebuild, containers, merge functions, primitive/action VM, proof/extraction ownership, and scheduler-visible run loop native. Add a DD/FlowLog-like rule-evaluation service beside the native database. At each logical schedule epoch, egglog exports canonical e-node/function relation deltas, explicit rebuild-invalidation events, and the per-rule last-run timestamp information needed for seminaive freshness. The service maintains arranged relation indexes and evaluates rule bodies to produce a stream/table of matches keyed by rule id plus bound variables. Egglog then drains those matches and runs existing native actions through the current action path, including `set`, `union`, `delete`, `subsume`, primitive calls, and failed lookup behavior. The scheduler contract stays native and must preserve full-match collection and delayed-fire semantics: collect all matches first, let the scheduler choose a subset or ordering, then fire actions after that choice. After actions, native rebuild runs and emits the next batch of canonical-id/table invalidations back to dataflow.

## What Would Move
- Rule body matching over function/e-node tables, including joins, filters, repeated-variable constraints, primitive predicates that can be safely evaluated as filters, semijoin/SIP-style pruning, and maintained indexes.
- Seminaive delta scheduling for rule queries, using DD collections/arrangements and FlowLog-style recursive/non-recursive planning as the model (`findings/source-notes/differential-timely.md`, `findings/source-notes/datalog-wcoj-planning.md`).
- Per-rule seminaive freshness filtering, if DD owns rule matching. This requires preserving egglog's logical timestamp windows rather than using only global recent/stable relation state (`findings/source-notes/scaling-equality-saturation.md`).
- Shared query indexes over exported egglog relations: e.g. arrangements by function symbol, result e-class, child e-class positions, and `(op, canonical_children)` keys.
- Optional borrowed ideas from FlowLog/datatoad: stratum metadata, enter/leave collections, transformation sharing, SIP, and later WCOJ for cyclic or high-arity e-matches. FlowLog's current binary join pipeline is a useful shape, but its source-order left-deep optimizer is not enough as-is (`repos/flowlog/crates/optimizer/src/plan_tree.rs`).

## What Stays Native
- Union-find and representative choice, including egglog's locality-sensitive union-by-min behavior.
- Rebuild and canonicalization of table values. `Rebuilder` is explicitly the optimized escape hatch because value-level rebuilds require finding changes in any column (`repos/egglog/core-relations/src/table_spec.rs`).
- Container rebuild semantics and same-id dirty refresh. `refresh_rows_for_values` exists because some ids change semantics without changing identity and parent rows must be retimestamped for seminaive visibility (`repos/egglog/core-relations/src/free_join/mod.rs`, `repos/egglog/core-relations/src/table/rebuild.rs`).
- Logical schedule ownership: egglog should keep the user-visible ruleset/run/saturate schedule and the rule last-run bookkeeping that defines seminaive windows until a DD design proves it can preserve them exactly.
- Action execution and mutation semantics: function lookup, primitive calls, `set`, `delete`, `subsume`, `union`, panic, merge functions, proof instrumentation, and error reporting remain in egglog (`repos/egglog/src/lib.rs`).
- Native table merge/dependency handling after staged mutations (`repos/egglog/core-relations/src/free_join/mod.rs`).

## Required Interfaces
- Exported relation schema: stable table ids/function ids, arity/type metadata, and row records for each matchable function/e-node table. Rows need canonical e-class ids in all equality-sort columns and enough provenance/table-row identity to retract or refresh old rows.
- Delta stream from egglog to dataflow: inserts, deletes/subsumes, canonical-id rewrites from rebuild, class-id merge deltas old->new, and explicit invalidation/refresh event types for rebuild-triggered changes, same-id dirty containers, and other semantic changes without tuple identity changes.
- Dataflow rule plan API: compile `CoreRule` body atoms into FlowLog/DD plans, with bindings layout compatible with egglog's `QueryEntry`/resolved variable representation and with rule ids preserved.
- Seminaive timestamp API: expose each row's logical insertion/refresh timestamp and each rule's last-run timestamp, or expose an equivalent freshness-window predicate that DD can push into rule-body evaluation.
- Match output API: append-only or epoch-scoped match relation `(rule_id, substitution, match_epoch)` plus dedup policy, so egglog can apply each match once under the same semantics as current seminaive execution.
- Action handoff API: egglog takes match substitutions and invokes the existing native action compiler/VM path; DD should not directly mutate union-find or function tables.
- Epoch/progress boundary: egglog must know when DD has reached a fixed point for the current exported deltas before draining matches, and DD must know when native rebuild has completed before accepting the next canonicalized batch.

## Main Risks
- Rebuild invalidation is the central semantic risk. If DD only sees ordinary tuple inserts/deletes, it can miss rows whose logical contents became newly matchable after a native rebuild or same-id container change.
- Scheduler semantics are another semantic risk. If DD materializes only a subset of matches or fires actions early, it can change observable `run`, `saturate`, and custom schedule behavior even if the final match set looks similar.
- Per-rule seminaive risk: a backend that sees only global deltas can miss matches when different rulesets run at different times. The scheduled reachability example from Eli's draft should fail any design that lacks rule-local freshness windows.
- Churn from class-id rewrites may dominate. A large class merge can require retracting and reinserting many relation rows and arrangement keys, even though native union-find records a compact merge.
- The action split can duplicate work or introduce stale matches: DD may emit matches over rows that native actions/rebuild later delete, subsume, or canonicalize unless epochs and dedup are precise.
- FlowLog is an architectural guide, not a drop-in planner. Its current optimizer builds a left-deep source-order plan, while egglog needs cost/selectivity awareness and likely WCOJ for some rules (`findings/source-notes/datalog-wcoj-planning.md`).
- Timely/DD timestamp overhead and trace compaction can become a bottleneck if every small e-graph mutation becomes its own dataflow time (`findings/source-notes/differential-timely.md`).
- Maintenance cost is high: this creates two indexed views of egglog state, so every native semantic feature that affects matching needs an explicit dataflow delta contract.

## Evidence To Gather
- Implement a tiny adapter for 2-3 function tables: export canonical rows to DD, evaluate one multi-atom rule body, return substitutions, and apply existing native actions.
- Reproduce rebuild-heavy cases including `container-rebuild.egg` and `merge-during-rebuild.egg`; verify DD receives enough invalidations to produce the same matches as native seminaive execution.
- Add semantic-equivalence tests for rebuild, container refresh, and custom schedules, including cases where matches are fully materialized before action fire and cases where delayed-fire scheduling chooses only a subset.
- Reproduce the scheduled reachability example from `repos/scaling-equality-saturation/egglog-new-backend.md` and verify that DD receives enough timestamp-window information to match native seminaive behavior.
- Measure delta volume for class-id merges: number of row retractions/reinsertions and arrangement updates per native union/rebuild, especially with high parent fanout.
- Compare native free-join rule matching against DD binary joins and one WCOJ-style prototype on representative egglog rule shapes: acyclic, cyclic, repeated-variable, and selective-root patterns.
- Decide and measure epoch granularity: per command, per rule batch, per rebuild phase, or per saturation iteration.

## Current Assessment
- This option remains useful to evaluate as a hybrid prototype because it preserves the hardest egglog-specific machinery natively while testing DD/FlowLog on maintained rule indexes and incremental body matching. The first milestone should not attempt relational equality maintenance; it should prove the data-exchange contract by matching native results on rebuild/container/merge/scheduler regressions and quantifying class-id rewrite churn. If that contract is too large or too expensive, the evidence would argue against broader integration.
