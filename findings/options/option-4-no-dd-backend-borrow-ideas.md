# Option 4: No DD Backend, Borrow Ideas

## Viability
- High, as a fallback and incremental improvement path. The source notes consistently show that DD/FlowLog/datatoad ideas are strongest around maintained indexes, rule planning, WCOJ, columnar layout, profiling, and custom relation providers, while egglog's hardest semantics remain native-specific: union-by-min equality maintenance, rebuild retimestamping, same-id container refresh, merge-during-rebuild, arbitrary schedules, Python-facing containers, analyses, and extraction (`findings/source-notes/egglog-core-proof.md`, `findings/source-notes/containers-frontends.md`, `findings/source-notes/differential-timely.md`). Borrowing ideas avoids a high-risk semantic migration while still attacking the current backend's relational and observability bottlenecks.

## General Approach
- Keep the production backend in egglog, but clarify the internal rule-evaluation boundary around `CoreRule`/`BackendRule` and `core-relations` so rule bodies can be planned, profiled, and executed through swappable native strategies.
- Add a native planner that can choose among existing binary/free-join execution, semijoin/SIP-style filtering, and datatoad/free-join-style WCOJ for selected multiway or cyclic patterns (`findings/source-notes/datalog-wcoj-planning.md`).
- Experiment with columnar or columnar-inspired row batches for high-volume relation scans and deltas, but only behind relation-storage interfaces so e-class ids, containers, and rebuild mutation keep their current semantics (`findings/source-notes/extension-models.md`).
- Introduce a provider-style interface inspired by Ascent BYODS for special relations: ordinary relations, equality/union-find-backed relations, container indexes, and possibly columnar-backed relations. This should be a narrow native ABI, not a DD abstraction layer.
- Improve profiling first-class enough to report per-rule match counts, join-stage cardinalities, rebuild retimestamp counts, dirty-container refresh counts, and scheduler decisions, matching the evidence needed throughout `findings/synthesis.md`.

## What Would Move
- No runtime responsibility moves to DD as a backend.
- FlowLog contributes architectural ideas: explicit rule IR, planner/control separation, recursive/fixpoint boundary naming, profiling hooks, and SIP-style planning.
- Datatoad/free-join contributes join-kernel ideas: variable ordering, count/propose/validate WCOJ stages, delta/base staging, and robust handling of cyclic/high-arity joins.
- DD/Timely contributes concepts, not ownership: arrangements as shared maintained indexes, coarse epochs for batched changes, compaction awareness, and operator-boundary layout thinking.
- Ascent/columnar contributes extension and storage patterns: custom provider interfaces and column-oriented fact storage for selected native relations.

## What Stays Native
- Union-find, congruence closure, representative choice, and union-by-min locality.
- Rebuild ordering, value-level table rewriting, parent-row retimestamping, merge functions during rebuild, and same-id container refresh.
- Container semantics, including opaque primitive equality/hash, deferred rebuild, higher-order multiset/map/fold functions, and Python-facing conversions.
- Schedules, rulesets, backoff/custom scheduler behavior, `push`/`pop`, deletion/subsumption, analyses, extraction, custom costs, primitive calls, and proof/extraction integration.
- Frontend compatibility for egglog syntax and Python APIs.

## Required Interfaces
- A stable native rule IR below the frontend and above execution: atoms, variables, repeated-variable constraints, primitive filters, action heads, ruleset/scheduler metadata, and side-effect boundaries.
- A relation-provider API with at least: row insertion/deletion or delta ingestion, keyed lookup, full scan, cardinality/selectivity estimates, changed-row iteration, rebuild/canonicalization hooks, and profiling counters.
- A join-planner API that can request provider estimates and choose binary joins, semijoins, or WCOJ stages without changing rule semantics.
- A rebuild/refresh notification API that makes dirty ids, retimestamped rows, and same-id container semantic changes visible to profiling and to seminaive matching.
- A profiling schema that can attribute time and cardinality to rule, join stage, relation provider, rebuild phase, and scheduler choice.

## Main Risks
- Semantic leakage: provider abstractions may accidentally hide rebuild, dirty-container, or merge-during-rebuild behavior that current code handles explicitly.
- Planner complexity: a native WCOJ/SIP planner could become a second backend-sized subsystem if it needs many special cases for equality churn, primitives, and actions.
- Storage mismatch: columnar borrowed/reference-shaped access may not fit mutable rebuild paths, hash keys, canonical e-class ids, or container values (`findings/source-notes/extension-models.md`).
- Performance ambiguity: WCOJ may help cyclic e-matching but not workloads dominated by equality merges, representative churn, rebuild, or extraction.
- Maintenance dilution: borrowing too many substrate ideas without a clean internal boundary could increase complexity while missing the main DD/FlowLog maintenance benefit.

## Evidence To Gather
- Classify 3-5 real egglog rules as acyclic, cyclic, repeated-variable, or equality-heavy; compare current planning against a prototype WCOJ or semijoin plan on one cyclic/high-arity case.
- Add profiling counters for current `core-relations`: per-rule matches, intermediate cardinalities, table rebuild rows scanned/retimestamped, dirty ids, dirty-container refresh rows, and scheduler skips.
- Prototype one native relation-provider boundary with an ordinary relation, a union-find/equality-backed provider, and a container-index provider; verify `container-rebuild.egg` and `merge-during-rebuild.egg` still behave identically.
- Microbenchmark columnar-inspired storage on actual egglog row shapes: constructor rows, parent indexes, symbol-heavy rows, and small container payloads.
- Reproduce the multiset A/C witness under binary A/C rules, index-based containers, and higher-order multiset functions to ensure borrowed indexing does not recreate the blow-up (`findings/source-notes/containers-frontends.md`).

## Recommendation
- Continue, as the low-risk parallel track and likely fallback if the DD backend prototype does not prove clear value. Option 4 should not block a narrow native-equality-plus-DD rule-evaluation experiment, but it is the safest path for near-term egglog improvements because it preserves frontend and container semantics while testing the most reusable ideas: better rule IR boundaries, WCOJ/SIP planning, provider interfaces, columnar storage experiments, and concrete profiling.
