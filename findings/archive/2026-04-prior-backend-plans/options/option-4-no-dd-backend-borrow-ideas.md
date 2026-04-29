# Option 4: No DD Backend, Borrow Ideas

## Viability
- High, and strengthened by the Option 3 follow-up gates, as a fallback and
  incremental improvement path. The source notes consistently show that
  DD/FlowLog/datatoad ideas are strongest around maintained indexes, rule
  planning, WCOJ, columnar layout, profiling, and custom relation providers,
  while egglog's hardest semantics remain native-specific: union-by-min
  equality maintenance, rebuild retimestamping, same-id container refresh,
  merge-during-rebuild, arbitrary schedules with per-rule seminaive timestamps,
  Python-facing containers, analyses, and extraction
  (`findings/source-notes/egglog-core-proof.md`,
  `findings/source-notes/containers-frontends.md`,
  `findings/source-notes/differential-timely.md`,
  `findings/source-notes/scaling-equality-saturation.md`). The new Option 3
  lanes confirm the same shape experimentally for adapter designs: native
  regressions pass, but useful overlap through native-authoritative
  rebuild/action/scheduler/container barriers is not demonstrated. Borrowing
  ideas avoids a high-risk semantic migration while still attacking the current
  backend's relational and observability bottlenecks. The provider boundary question is
  not a minor implementation detail here: it may be the real split between
  ordinary DD-backed relations and specialized equality/container/rebuild
  providers.

## General Approach
- Keep the production backend in egglog, but clarify the internal rule-evaluation boundary around `CoreRule`/`BackendRule` and `core-relations` so rule bodies can be planned, profiled, and executed through swappable native strategies.
- Add a native planner that can choose among existing binary/free-join execution, semijoin/SIP-style filtering, and datatoad/free-join-style WCOJ for selected multiway or cyclic patterns (`findings/source-notes/datalog-wcoj-planning.md`).
- If GPU execution becomes a serious target, borrow SRDatalog's WCOJ constraints
  as native/provider experiments first: flat sorted columns, two-phase
  count/materialize allocation, skew histograms, helper-relation splitting, and
  stream multiplexing only behind schedule-safe rule boundaries.
- Preserve and expose the existing seminaive timestamp machinery as a first-class native feature: per-rule last-run timestamps, timestamp-window scans, and row refresh from rebuild should be visible to profiling and planner experiments.
- Experiment with columnar or columnar-inspired row batches for high-volume relation scans and deltas, but only behind relation-storage interfaces so e-class ids, containers, and rebuild mutation keep their current semantics (`findings/source-notes/extension-models.md`).
- Introduce a provider-style interface inspired by Ascent BYODS for special relations: ordinary relations, equality/union-find-backed relations, container indexes, and possibly columnar-backed relations. This should be treated as a first-class boundary option, not just a helper inside the native path: ordinary relations could remain on the default engine while equality/container/rebuild-sensitive relations use specialized providers.
- Improve profiling first-class enough to report per-rule match counts, join-stage cardinalities, rebuild retimestamp counts, dirty-container refresh counts, and scheduler decisions, matching the evidence needed throughout `findings/synthesis.md`.

## What Would Move
- No runtime responsibility moves to DD as a backend.
- FlowLog contributes architectural ideas: explicit rule IR, planner/control separation, recursive/fixpoint boundary naming, profiling hooks, and SIP-style planning.
- Datatoad/free-join contributes join-kernel ideas: variable ordering, count/propose/validate WCOJ stages, delta/base staging, and robust handling of cyclic/high-arity joins.
- SRDatalog contributes GPU-specific execution ideas: flat columnar WCOJ over
  seminaive deltas, deterministic bulk materialization, skew-aware launch
  partitioning, and phase-aligned stream scheduling.
- DD/Timely contributes concepts, not ownership: arrangements as shared maintained indexes, coarse epochs for batched changes, compaction awareness, and operator-boundary layout thinking.
- Eli's backend draft contributes native baseline ideas that should be improved before migration: timestamp-ordered hash tables, staged mutation buffers, deterministic parallel compaction, table-provider hooks, Free Join, dynamic variable ordering, and future binary/bushy planning.
- Ascent/columnar contributes extension and storage patterns: custom provider interfaces and column-oriented fact storage for selected native relations, with the provider boundary itself kept visible as a separate design axis.

## What Stays Native
- Union-find, congruence closure, representative choice, and union-by-min locality.
- Rebuild ordering, value-level table rewriting, parent-row retimestamping, merge functions during rebuild, and same-id container refresh.
- Container semantics, including opaque primitive equality/hash, deferred rebuild, higher-order multiset/map/fold functions, and Python-facing conversions.
- Schedules, rulesets, backoff/custom scheduler behavior, `push`/`pop`, deletion/subsumption, analyses, extraction, custom costs, primitive calls, and proof/extraction integration.
- Per-rule seminaive freshness and timestamp-window table access.
- Frontend compatibility for egglog syntax and Python APIs.

## Required Interfaces
- A stable native rule IR below the frontend and above execution: atoms, variables, repeated-variable constraints, primitive filters, action heads, ruleset/scheduler metadata, per-rule timestamp windows, and side-effect boundaries.
- A relation-provider API with at least: row insertion/deletion or delta ingestion, keyed lookup, full scan, cardinality/selectivity estimates, changed-row iteration, rebuild/canonicalization hooks, and profiling counters. This API needs to be able to support a real comparison between a default DD/native split and specialized provider-backed equality/container relations.
- A join-planner API that can request provider estimates and choose binary joins, semijoins, or WCOJ stages without changing rule semantics.
- A rebuild/refresh notification API that makes dirty ids, retimestamped rows, and same-id container semantic changes visible to profiling and to seminaive matching.
- A profiling schema that can attribute time and cardinality to rule, join stage, relation provider, rebuild phase, and scheduler choice.

## Main Risks
- Semantic leakage: provider abstractions may accidentally hide rebuild, dirty-container, or merge-during-rebuild behavior that current code handles explicitly.
- Planner complexity: a native WCOJ/SIP planner could become a second backend-sized subsystem if it needs many special cases for equality churn, primitives, and actions.
- Storage mismatch: columnar borrowed/reference-shaped access may not fit mutable rebuild paths, hash keys, canonical e-class ids, or container values (`findings/source-notes/extension-models.md`).
- Performance ambiguity: WCOJ may help cyclic e-matching but not workloads dominated by equality merges, representative churn, rebuild, or extraction.
- Maintenance dilution: borrowing too many substrate ideas without a clean internal boundary could increase complexity while missing the main DD/FlowLog maintenance benefit.
- Boundary ambiguity: if the provider split is not compared explicitly against the DD/native split, it is too easy to treat specialized equality/container providers as an implementation tweak instead of a design choice with different maintenance and performance tradeoffs.

## Evidence To Gather
- Classify 3-5 real egglog rules as acyclic, cyclic, repeated-variable, or equality-heavy; compare current planning against a prototype WCOJ or semijoin plan on one cyclic/high-arity case.
- For any GPU-oriented prototype, test whether flat sorted column storage
  survives canonical-id rewrites and same-id container refreshes without
  rebuilding most of the relation.
- Add profiling counters for current `core-relations`: per-rule matches,
  intermediate cardinalities, table rebuild rows scanned/retimestamped, dirty
  ids, dirty-container refresh rows, and scheduler skips. The Option 3 lanes
  specifically showed that row-level rebuild/container counters and scheduler
  admission counters are the missing evidence.
- Add a scheduled reachability regression from Eli's draft to protect per-rule seminaive behavior while planner/storage experiments proceed.
- Prototype one native relation-provider boundary with an ordinary relation, a union-find/equality-backed provider, and a container-index provider; verify `container-rebuild.egg` and `merge-during-rebuild.egg` still behave identically.
- Microbenchmark columnar-inspired storage on actual egglog row shapes: constructor rows, parent indexes, symbol-heavy rows, and small container payloads.
- Reproduce the multiset A/C witness under binary A/C rules, index-based containers, and higher-order multiset functions to ensure borrowed indexing does not recreate the blow-up (`findings/source-notes/containers-frontends.md`).
- Compare the provider-based boundary directly against the simpler DD/native split: measure whether specialized providers reduce churn, code complexity, and rebuild risk enough to justify the extra ABI surface.

## Current Assessment
- This is the lowest-migration-risk path for preserving existing semantics
  while testing reusable ideas: better rule IR boundaries, WCOJ/SIP planning,
  provider interfaces, columnar storage experiments, and concrete profiling.
  The Option 3 follow-up gates make this the better incremental next step than
  a broad rewrite because the measured blockers are instrumentation, planner
  data, and provider boundaries. Its weakness is that it may not satisfy the shared-substrate
  motivation unless provider-style boundaries isolate enough reusable relation
  work from native-only equality/container/rebuild behavior.
