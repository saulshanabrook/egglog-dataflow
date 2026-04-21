# Conversations + Social Motivation

## Sources Read
- `README.md`: frames the investigation as a falsification pass over technical, maintenance, and social reasons to move egglog onto DD or a nearby substrate.
- `messages/oct-15-2024-zulip.md`: main multi-party design conversation around Frank McSherry's e-graph/DD prototype, WCOJ, rebuilding, analyses, scopes, and collaboration fit.
- `messages/dec-17-2025-slack.md`: follow-up thread on FlowLog, datatoad, DD parallelism, nested fixed points, schedules, and extensibility.
- `repos/blog/posts/2024-10-19.md`: Frank's "Understanding E-Graphs" implementation note that triggered the Zulip thread and surfaced rebuild/indexing questions.
- `repos/blog/posts/2016-06-21.md`: DD/Datalog background showing DD can express Datalog with explicit `iterate`, arbitrary input changes, and incremental correction.
- `repos/blog/posts/2024-10-11.md`: Timely container and `columnar` context, relevant to custom data layouts as a possible shared substrate concern.
- `repos/blog/posts/2025-11-21.md`: datatoad/WCO Datalog note connecting streaming WCOJ, iterative Datalog, GALEN, columnar execution, and FlowLog comparison.
- `repos/blog/posts/2025-12-03.md`: columnar WCOJ follow-up emphasizing robust join plans, delta atoms, and the fragility of bad binary join orders.
- `repos/differential-dataflow/README.md`: DD capability summary: arbitrary updates, `map`/`filter`/`join`/`reduce`, `iterate`, and change-local maintenance.
- `repos/flowlog/README.md`: FlowLog architecture: Datalog pipeline into Timely/DD executables with stratifier, optimizer, planner, compiler, and profiler.
- `repos/datatoad/README.md`: interactive, columnar, WCO Datalog reference with predicate-as-relation extensibility and explicit "exploration" status.
- `repos/dataflow-join/README.md`: streaming GenericJoin in Timely, including parallelism and resource-pressure caveats for large motif updates.

## Key Findings
- The strongest motivation is potential maintenance leverage, not just speed. `README.md` says egglog already owns relational rule evaluation, indexes, equality maintenance, rebuilding, custom sorts, containers, scheduling, analyses, and extraction; the conversations ask whether a DD/FlowLog/datatoad-like substrate can move some of that complexity into an independently maintained systems project. This remains a hypothesis, not an observed outcome.
- There is collaborator interest from the DD side. In `messages/oct-15-2024-zulip.md`, Frank arrived with a short e-graph implementation, a DD EqSat PR, concrete questions about union-find, rebuild costs, WCOJ, equality retractions, nested recursion, multiple outputs, and a willingness to improve the underlying code if clear cases exist.
- Egglog maintainers are interested but not credulous. Saul frames the upside as pushing harder DB problems into a production-quality system, while Eli and Max repeatedly ask whether DD preserves egglog's extra expressivity, arbitrary schedules, WCOJ advantages, rebuilding performance, analyses, and efficient congruence closure (`messages/oct-15-2024-zulip.md`).
- The trust assumption is limited: Frank/DD may be able to solve low-level dataflow/database issues if egglog supplies sharp reproductions, but the conversations do not prove sustained upstream maintenance transfer or that DD automatically gives the right egglog backend. Frank explicitly says DD can express many things and the remaining question is "whether it sucks"; he also warns that the PR maintains too much history unless iteration/timestamps are chosen carefully (`messages/oct-15-2024-zulip.md`).
- Equality maintenance is the central risk. Eli identifies direct union-find and mutable table updates as prior performance wins and asks whether DD's transitive-closure-like iteration should be replaced by union-find; Frank says DD can do union-find-like updates but the naive `iterate` form retains too much update history (`messages/oct-15-2024-zulip.md`).
- Rebuild deltas are a social/technical bridge. Frank's streaming WCOJ intuition matches egglog's semi-naive e-matching, but both sides call out that e-class consolidation changes labels and creates representation-level retractions even when logical facts only grow (`messages/oct-15-2024-zulip.md`, `repos/blog/posts/2025-11-21.md`).
- FlowLog is a plausible implementation reference, not merely related reading. The Slack thread says FlowLog is built on DD, benefits from columnar representation, spreads work across many small iterations for parallelism, and exposes nested fixed points that look like egglog's outer rule loop plus inner congruence closure; `repos/flowlog/README.md` confirms a compiler pipeline with stratification, planning, optimization, profiling, and execution modes including explicit loop blocks.
- The nearby ecosystem offers alternatives if DD is too costly. `repos/datatoad/README.md` and `repos/blog/posts/2025-11-21.md` show active work on interactive columnar WCO Datalog, predicate-backed relations, GALEN-scale comparisons, and plans to transport lessons back to DD, suggesting egglog could collaborate on join/planning/layout ideas without moving all semantics into DD.
- Custom extensibility is not optional. The Slack thread ends with Eli interested in replacing `core-relations` with something FlowLog-esque only if arbitrary schedules and custom tables like union-finds or containers remain feasible; `repos/blog/posts/2024-10-11.md` and Timely's container story make custom layouts plausible, but not egglog's container semantics.

## Relevance To The Main Objective
- The conversations support investigating a DD/FlowLog-related backend because the people and projects are unusually aligned: DD already targets incremental recursive dataflow, FlowLog targets Datalog on DD, datatoad targets robust Datalog joins, and Frank is actively looking for e-graph-shaped cases. The evidence supports exploration and concrete reproductions, not a guarantee of shared maintenance payoff.
- The same evidence weakens any simple "port egglog to DD" story. The likely hard parts are exactly egglog's differentiators: congruence closure, representative churn, arbitrary schedules, analyses through equality, custom containers, and extension points.
- A realistic objective should be narrower at first: prototype a `core-relations` or rule-evaluation substrate boundary, then measure whether equality maintenance stays native, becomes DD collections, or uses a custom table/provider abstraction.

## Likely Blockers
- DD's default history retention may be too expensive for egglog-style saturation, especially if it tracks arbitrary undo of input equivalences that most egglog workloads do not need (`messages/oct-15-2024-zulip.md`).
- Efficient congruence closure may still require mutable union-find-like state or a custom table; a pure DD relational encoding could be correct but lose the performance reason to migrate.
- Egglog schedules are richer than one global Datalog fixed point, and Eli explicitly asks whether FlowLog/DD can encode arbitrary schedules (`messages/dec-17-2025-slack.md`).
- Analyses and contextual/nested equality are unresolved. Max says egglog has only a fuzzy story for stratifying analyses through equivalence, and the Zulip thread branches into colored e-graphs, assume nodes, RVSDG, and pessimistic analyses rather than a settled backend mapping.
- WCOJ robustness does not remove resource risk. `repos/dataflow-join/README.md` reports a 6-clique streaming update that climbs toward 60GB because too much intermediate work is staged at once.
- datatoad and FlowLog are high-signal but not drop-in answers: datatoad is explicitly exploratory, and FlowLog compiles Datalog to executables rather than exposing egglog's custom runtime surface directly (`repos/datatoad/README.md`, `repos/flowlog/README.md`).

## Promising Connections
- Use Frank's DD EqSat PR and `repos/blog/posts/2024-10-19.md` as a shared minimal vocabulary for equality-as-derived-views, then replace the naive parts with the conversation's later lessons about timestamps, manual loops, and compaction.
- Compare egglog semi-naive e-matching against streaming WCOJ/datatoad plans on rules with cyclic bodies; `repos/blog/posts/2025-12-03.md` gives concrete delta-atom planning language that maps well to e-matching deltas.
- Treat FlowLog as the closest architecture template for a rule-evaluation backend: parser/stratifier/planner/compiler separation, profiling hooks, and DD execution are visible in `repos/flowlog/README.md`.
- Explore Timely custom containers and `columnar` as a lower-level collaboration point for relation storage, indexes, and batch movement even if egglog containers remain semantic objects above the substrate.
- Use datatoad's predicate-as-relation abstraction as a possible analogue for egglog primitive functions or custom relation providers (`repos/datatoad/README.md`).
- Bring concrete egglog workloads to the DD/FlowLog/datatoad maintainers: representative churn, rebuild deltas, container-heavy rules, and schedules are likely to produce useful upstreamable engine cases.

## Evidence Needed Next
- A small measured prototype that keeps equality native but runs selected rule/e-matching relations through DD/FlowLog-like dataflows, with runtime, relation sizes, retained state, and rebuild deltas.
- A competing prototype that represents equality/congruence as DD collections, using manual loop/timestamp choices rather than the naive PR shape, to quantify history-retention cost.
- Inspection of `repos/egglog/core-relations/`, `repos/egglog/union-find/`, and proof/term encoding paths to identify the minimum backend interface needed for custom tables, containers, and schedules.
- Benchmarks that include cyclic multi-atom rules, representative churn, and container examples; GALEN-style Datalog alone is not representative of egglog semantics.
- A concrete schedule-lowering example showing nested egglog schedules mapped to DD loops or FlowLog extended loops, including what can run concurrently and what must be staged.
- A focused small-iteration experiment comparing current bulk ruleset iteration with a FlowLog/DD-style physical schedule that spreads a logical ruleset across many small DD iterations.
- A collaboration test case small enough for DD/FlowLog/datatoad maintainers to reason about but faithful enough to expose egglog's rebuild/equality problems.

## Confidence
- Medium. The conversation and local blog/repo evidence are strong for motivations, risks, and collaboration fit, but the decisive claims require code-level inspection and measurements that this pass did not perform.
