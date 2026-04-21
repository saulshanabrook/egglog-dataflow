# Exploration of Egglog on top of Differential Dataflow

This repository is a source bundle for exploring whether some or all of
`egglog` can be built on top of Timely / Differential Dataflow.

It is not a single buildable workspace. The top-level tree collects papers,
discussion transcripts, and pinned source checkouts that are useful for
answering the design questions before starting implementation work.

## Main Question

Can `egglog`'s current evaluation model be mapped onto Differential Dataflow in
a way that preserves the features that matter for egglog workloads?

The concrete questions to answer from these sources are:

- How much of egglog rule evaluation, scheduling, rebuilding, congruence
  closure, and analysis can be expressed as maintained dataflow views?
- Can Differential Dataflow's support for arbitrary updates, nested iteration,
  and lattice timestamps help with egglog schedules, nested fixed points, or
  speculative / backtracking use cases?
- Where would a Differential-backed design need custom state, such as
  union-find, containers, indexes, or sort-specific tables?
- Can streaming worst-case optimal joins, columnar layouts, or FlowLog-style
  planning provide a better substrate for egglog's relational e-matching?

## Source Inventory

### Papers

| Path | Role |
| --- | --- |
| `papers/differentialdataflow.pdf` | The core Differential Dataflow paper. It introduces differential computation as an incremental dataflow model that supports arbitrarily nested iteration. This is the foundation for reasoning about whether recursive egglog workloads can be maintained incrementally rather than re-run from scratch. |
| `papers/FlowLog Efficient and Extensible Datalog via Incrementality.pdf` | Local copy of the FlowLog paper. FlowLog is the closest included system to "Datalog on Differential Dataflow": it compiles Datalog into Timely / Differential executables, separates recursive control from per-rule relational plans, and emphasizes robust recursive query optimization. The local PDF appears to be an earlier copy; the final PVLDB version is linked in "Suggested additions" below. |

### Conversations

| Path | Role |
| --- | --- |
| `messages/oct-15-2024-zulip.md` | Main design discussion between Frank McSherry, Saul Shanabrook, Eli Rosenthal, Max Willsey, Oliver Flatt, and others. Important topics: an EqSat-in-Differential prototype, whether DD can express union-find and congruence closure, streaming WCOJ as semi-naive e-matching, equality retractions from changing e-class labels, nested scopes, contextual equality, multiple outputs, and analysis stratification. |
| `messages/dec-17-2025-slack.md` | Follow-up notes on FlowLog and datatoad. Important topics: FlowLog on DD, columnar representation, spreading work across many small iterations for DD parallelism, DD's nested fixed points as a possible match for egglog's outer rule loop plus inner congruence closure, lattice timestamps, arbitrary schedules, and the question of adding custom tables such as union-finds or containers. |

### Vendored Repositories

| Path | Current checkout | Role |
| --- | --- | --- |
| `repos/egglog` | `egraphs-good/egglog`, `main`, `b27cd225` | The implementation target. Start here for current semantics and engineering constraints. Important subtrees include `src/` for the core language and runtime, `core-relations/` for the current relation layer, `egglog-bridge/` for integration boundaries, `union-find/`, `numeric-id/`, `concurrency/`, and the `tests/` corpus. |
| `repos/egglog-tutorial` | `egraphs-good/egglog-tutorial`, `main`, `4ca08d2` | Compact examples for the language surface: basics, Datalog, analyses, scheduling, cost models, extraction, and a linear algebra compiler case study. Useful for choosing small translation experiments before attempting full egglog programs. |
| `repos/differential-dataflow` | `TimelyDataflow/differential-dataflow`, `master`, `1f348abd` | The target incremental dataflow framework. Key areas: `differential-dataflow/src/` for collection operators, `mdbook/` for conceptual docs, `dogsdogsdogs/` for advanced join patterns, `interactive/` for an interpreted DD IR, and `doop/` for a Datalog-like workload. |
| `repos/timely-dataflow` | `TimelyDataflow/timely-dataflow`, `master`, `b4e5ef9b` | The lower-level runtime under Differential Dataflow. Relevant for progress tracking, cyclic dataflows, scheduling, communication, container support, worker execution, and timestamp behavior. |
| `repos/flowlog` | `flowlog-rs/flowlog`, `main`, `83f3ae4` | The closest architecture study for compiling Datalog to Timely / Differential. Its crates split the pipeline into parser, stratifier, catalog, optimizer, planner, compiler, common utilities, and profiler. This is the main source for Datalog-specific planning on top of DD. |
| `repos/dynamic-datalog` | `frankmcsherry/dynamic-datalog`, `master`, `0fca831` | Benchmark and comparison material for dynamic Datalog engines. Includes CRDT, DOOP, and GALEN workloads, with hand-written Differential implementations under `differential-dataflow/`. Useful for testing whether a proposed design handles nontrivial recursive workloads. |
| `repos/dataflow-join` | `frankmcsherry/dataflow-join`, `master`, `e87677a` | Streaming implementation of GenericJoin / worst-case optimal joins in Timely Dataflow. Relevant to relational e-matching and delta evaluation. Important files include `src/extender.rs`, `src/index.rs`, `src/motif.rs`, and the graph motif examples. |
| `repos/datatoad` | `frankmcsherry/datatoad`, `main`, `d16572f` | Interactive, columnar, worst-case optimal Datalog. It is not DD-based, but it is a high-signal reference for columnar Datalog execution, WCOJ planning, robust cyclic joins, and GALEN-style workloads. |
| `repos/columnar` | `frankmcsherry/columnar`, `master`, `abff5a7` | Columnar container library for converting arrays of structs into structs of arrays. Relevant because FlowLog and datatoad both point at columnar representation as a major performance lever, and Timely now supports custom containers. |
| `repos/blog` | `frankmcsherry/blog`, `master`, `dd0949d` | Frank McSherry's blog archive. It is especially useful for historical and design context around Timely, Differential, Datalog, WCOJ, datatoad, columnar layouts, and the e-graph discussion that led to this exploration. |

## High-Signal Local Reading Order

1. Read `messages/oct-15-2024-zulip.md` and `messages/dec-17-2025-slack.md` to recover the actual design questions.
2. Read `repos/egglog/README.md`, then skim `repos/egglog/core-relations/`, `repos/egglog/union-find/`, and `repos/egglog/src/`.
3. Work through `repos/egglog-tutorial/02-datalog.egg`, `03-analysis.egg`, and `04-scheduling.egg`.
4. Read `papers/differentialdataflow.pdf`, then `repos/differential-dataflow/README.md` and the mdbook.
5. Read the FlowLog paper and `repos/flowlog/README.md`; inspect `crates/planner`, `crates/optimizer`, and `crates/compiler`.
6. Read `repos/dataflow-join/README.md`, `repos/differential-dataflow/dogsdogsdogs/README.md`, and the datatoad README for WCOJ / Datalog execution tradeoffs.
7. Read `repos/columnar/README.md` and the Timely container docs before making representation decisions.

## Useful Blog Posts In The Bundle

The blog repo is large. These posts are the most directly relevant starting
points:

- `repos/blog/posts/2024-10-19.md`: "Understanding E-Graphs"; context for Frank's e-graph implementation and the Zulip thread.
- `repos/blog/posts/2025-12-03.md`: "Columnar Worst-Case Optimal Joins"; datatoad's newer WCOJ direction.
- `repos/blog/posts/2025-11-21.md`: "Worst-Case Optimal Datalog"; Datalog iteration through streaming WCOJ.
- `repos/blog/posts/2024-10-11.md`: "Dataflow and Columns and WASM, Oh My!"; Timely containers plus `columnar`.
- `repos/blog/posts/2016-06-21.md`: "Differential datalog"; direct background on Datalog in Differential Dataflow.
- `repos/blog/posts/2015-04-11.md`: "Worst-case optimal joins, in dataflow"; early dataflow GenericJoin explanation.
- `repos/blog/posts/2016-09-17.md`: "Tracking motifs in evolving graphs"; companion context for `dataflow-join`.
- `repos/blog/posts/2016-08-03.md`: "Differential Dataflow internals"; useful before changing DD-level execution assumptions.

## Suggested Additions Before Implementation

These are not all present as local files. Pulling them in, or at least linking
them from this README, would reduce avoidable rediscovery before implementation
work starts.

### Highest Priority

- Final FlowLog paper and artifact:
  - [PVLDB paper](https://www.vldb.org/pvldb/vol19/p361-zhao.pdf)
  - [FlowLog site](https://www.flowlog-rs.com/)
  - [VLDB artifact repository](https://github.com/flowlog-rs/FlowLog-VLDB)
- Prior EqSat-in-Differential attempt:
  - [TimelyDataflow/differential-dataflow#525](https://github.com/TimelyDataflow/differential-dataflow/pull/525)
- Core e-graph / egglog papers:
  - [egg: Fast and Extensible Equality Saturation](https://arxiv.org/abs/2004.03082)
  - [Better Together: Unifying Datalog and Equality Saturation](https://arxiv.org/abs/2304.04332)
  - [Relational E-matching](https://effect.systems/doc/popl-2022-gj/paper.pdf)
- WCOJ and join-planning papers:
  - [Streaming joins over streaming data](https://www.vldb.org/pvldb/vol11/p691-ammar.pdf)
  - [Free Join: Unifying Worst-Case Optimal and Traditional Joins](https://arxiv.org/abs/2301.10841)
  - [Leapfrog Triejoin](https://arxiv.org/abs/1210.0481)

### Medium Priority

- [DBSP: Automatic Incremental View Maintenance for Rich Query Languages](https://www.vldb.org/pvldb/vol16/p1601-budiu.pdf), for formal incrementalization context adjacent to DD.
- [Ascent](https://s-arash.github.io/ascent/), because the Zulip thread mentions Datalog systems with custom data structures as a comparison point.
- DDlog / Differential Datalog sources, if the work needs a compiler-to-DD comparison beyond FlowLog.
- Souffle benchmark and implementation notes, if evaluation will use Datalog workloads already reported in FlowLog or dynamic-datalog.
- Contextual / scoped e-graph sources, if the work touches nested scopes, filters, or letrec-like analyses:
  - [Slotted E-Graphs](https://pldi24.sigplan.org/details/egraphs-2024-papers/10/Slotted-E-Graphs)
  - [Colored E-Graphs](https://arxiv.org/abs/2305.19203)
  - [Equivalence Hypergraphs](https://pldi24.sigplan.org/details/egraphs-2024-papers/9/Equivalence-Hypergraphs-E-Graphs-for-Monoidal-Theories)
  - [RVSDG IR](https://arxiv.org/abs/1912.05036)

## Open Design Risks To Resolve Early

- DD can maintain arbitrary updates, but egglog mostly wants monotone growth plus
  representation-level churn from canonicalization. The cost of maintaining too
  much retraction history needs to be measured.
- A DD expression can encode union-find-like behavior, but it may not match the
  performance of direct mutable union-find unless the iteration and compaction
  strategy are chosen carefully.
- Egglog schedules are richer than "run all rules to a single fixed point".
  Any prototype should first show how arbitrary schedules map to DD loops.
- Relational e-matching and streaming WCOJ line up conceptually, but rebuilding
  and changing e-class labels create deltas that must be handled explicitly.
- Custom tables, containers, and sorts are central to egglog extensibility. A DD
  backend should prove that these extension points remain possible before chasing
  whole-program performance.
