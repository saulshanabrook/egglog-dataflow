# Exploration of Egglog on top of Differential Dataflow

This repository is a source bundle for exploring whether some or all of
`egglog` can be built on top of Timely / Differential Dataflow.

It is not a single buildable workspace. The top-level tree collects papers,
discussion transcripts, pinned source checkouts, and small prototype code that
are useful for answering design questions before implementation work starts.

## Main Scientific Question

Can `egglog`'s current evaluation model be mapped onto Differential Dataflow in
a way that preserves the semantics, performance-critical invariants, and
user-facing abstractions that matter for real egglog workloads?

## Scientific Questions

These are the high-level questions to keep updating as the research proceeds.
The goal is to refine these questions and add evidence to the conclusions
section, not to lock in a specific implementation trail yet.

### Equality Maintenance And Rebuilding

- What should a Differential-backed design own: only relational e-matching, or
  also equality maintenance, congruence closure, rebuilding, and analysis?
- Can equality maintenance be expressed as explicit maintained relations without
  a separate backend union-find data structure, or does practical performance
  require mutable union-find-like state and compaction?
- If equality is represented relationally, how are changing representatives,
  canonicalization, cleanup, and congruence closure represented as updates?
- Does DD's support for arbitrary updates and nested iteration help with
  equality retractions, contextual equality, or nested fixed points, or does it
  retain too much history for normal egglog workloads?

### Rule Evaluation And Scheduling

- How much of egglog rule evaluation, semi-naive scheduling, and relational
  e-matching can be expressed as maintained dataflow views?
- Where do egglog schedules differ from "run all rules to one fixed point", and
  how should arbitrary schedules map onto DD loops or nested scopes?
- Can streaming worst-case optimal joins, columnar layouts, FlowLog-style
  planning, or datatoad-style execution improve egglog's relational e-matching?
- What is the right boundary between native egglog rebuilding and DD-backed rule
  evaluation if equality maintenance remains outside DD?

### Containers And Higher-Level Functions

- Can a redesign support current container sorts such as `Vec`, `Set`, `Map`,
  and `MultiSet`, including rebuilding of e-class references inside containers?
- Can it preserve the Python-facing modeling style where users define
  higher-level functions over containers, such as `sum_(MultiSet[Num]) -> Num`,
  to encode algebraic invariants directly in the representation?
- How should matching on container contents work: explicit index functions such
  as `ms_num_index(xs, x)`, maintained derived views, higher-order/blockwise
  functions such as `map` and `fold`, or some combination?
- Can higher-order container functions avoid materializing the same blow-ups
  that containers are meant to prevent?

### Extensibility And Frontends

- Which extension points must remain available for custom sorts, primitive
  functions, analyses, extraction, plugins, and experimental libraries?
- How much of `egglog-experimental` and `egglog-python` should be treated as
  part of the design target rather than optional surface area?
- Can a DD-backed substrate expose enough control over custom tables,
  containers, indexes, and relation providers to preserve egglog's existing
  extensibility story?

### Evaluation Method

- Which small witnesses best distinguish constructor-only equality maintenance,
  relational e-matching, containers, analyses, schedules, and extraction?
- Which real examples are large enough to reveal DD trace costs, container
  costs, or join-planning benefits without hiding the failure mechanism?
- What measurements are needed before choosing a backend boundary: runtime,
  retained state, iterations, relation sizes, rebuild work, and extracted result
  quality?

## Current Conclusions

These are working conclusions from the current source bundle. They should be
updated as the questions above get sharper.

### Equality Maintenance

- Egglog's proof / term encoding is the most concrete local evidence that
  equality maintenance can be represented at the egglog level rather than only
  as a separate Rust union-find implementation.
- That encoding does not eliminate equality maintenance. It shifts the work into
  generated per-sort UF tables, function indexes, term tables, view tables,
  congruence rules, rebuild rules, and cleanup rules.
- The proof / term encoding is therefore a useful candidate lowering to compare
  against DD collections, but it is not yet evidence that a separate union-find
  can be avoided at acceptable cost.
- The current term encoding is reported to be roughly 100x slower than the
  native path, so any design that starts from it needs a measured explanation of
  where the overhead comes from.

### Containers And Higher-Level Functions

- Containers are not an edge case. The current `egglog-python` container
  write-up uses `MultiSet` plus higher-level functions to avoid associativity /
  commutativity blow-up for algebraic expressions such as `2 + a + b + b + 3`.
- The core modeling pattern is to move invariants into data representation:
  `sum_(MultiSet(a, b))` and `sum_(MultiSet(b, a))` are the same because the
  multiset is order-insensitive, not because a commutativity rewrite fired.
- Container rebuilding is part of congruence. If e-classes inside a container
  merge, the container must rebuild its inner references so terms containing the
  container become congruent.
- Egglog intentionally does not match directly inside container structure. The
  current workaround is either explicit index functions such as
  `ms_num_index(xs, x)` filled from `xs.fill_index(...)`, or higher-order
  container functions that perform blockwise work such as `map`, `fold`,
  `multiset_fold`, `multiset_flat_map`, and filtering-like transformations.
- A redesign that handles constructor terms but cannot support `Vec`, `Set`,
  `Map`, `MultiSet`, container rebuilding, and higher-order container functions
  is not representative of current Python-facing egglog use cases.
- The current proof / term encoding rejects custom container sorts with presorts
  and has unsupported snapshots for container-heavy files. Using that encoding
  as the main lowering therefore requires solving container support or clearly
  scoping it to a constructor-only subset.

### Differential Dataflow Boundary

- DD appears most immediately relevant to incremental relational rule
  evaluation, recursive Datalog-style computation, arbitrary updates, nested
  iteration, and maintained join results.
- It is still open whether DD should also own equality maintenance. The Zulip
  and Slack discussions explicitly raise the possibility that DD can express
  union-find-like behavior, but also raise concerns about representative churn,
  equality retractions, and retained history.
- FlowLog is the closest included architecture reference for compiling Datalog
  to Timely / Differential. It should inform any rule-planning boundary even if
  equality maintenance remains native.
- WCOJ sources are relevant because relational e-matching is join-heavy, but
  rebuilding and changing e-class labels mean the join input relations are not
  ordinary append-only Datalog relations.

### Extensibility

- The source bundle now points to three important user surfaces: core `egglog`,
  `egglog-experimental`, and `egglog-python`. A backend design should be judged
  against all three before claiming to support "egglog".
- Custom tables, custom sort/container implementations, primitive functions,
  analyses, and extraction are central design constraints, not late
  optimizations.

## Source Inventory

### Papers

| Path | Role |
| --- | --- |
| `papers/egg Fast and Extensible Equality Saturation.pdf` | Core `egg` paper. Read for rebuilding, e-class analyses, and why native e-graph maintenance was designed around deferred congruence closure. |
| `papers/Better Together Unifying Datalog and Equality Saturation.pdf` | Core `egglog` paper. Read for the language model, relational e-matching, lattices, and how Datalog and equality saturation are unified. |
| `papers/relational e-matching.pdf` | Relational e-matching paper. This is the direct bridge between e-matching and database join processing. |
| `papers/differentialdataflow.pdf` | Differential Dataflow paper. It introduces differential computation as an incremental dataflow model with arbitrarily nested iteration. |
| `papers/FlowLog Efficient and Extensible Datalog via Incrementality.pdf` | FlowLog paper. This is the closest included system to "Datalog on Differential Dataflow": it compiles Datalog into Timely / Differential executables and separates recursive control from per-rule relational plans. |
| `papers/DBSP.pdf` | DBSP paper. Useful formal context for incremental view maintenance over richer languages adjacent to DD. |
| `papers/Distributed Evaluation of Subgraph Queries.pdf` | Streaming / distributed WCOJ paper. Relevant to dataflow-style GenericJoin and the `dataflow-join` code. |
| `papers/Free Join Unifying Worst-Case Optimal and Traditional Joins.pdf` | Free Join paper. Relevant to hybrid plans that combine binary joins and WCOJ-style stages. |
| `papers/leapfrog treejoin.pdf` | Leapfrog Triejoin paper. Baseline WCOJ algorithm to compare against dataflow-oriented GenericJoin variants. |
| `papers/Parallel and Customizable Equality Saturation.pdf` | Parallel EqSat paper. Relevant for parallelism strategy comparisons independent of DD. |
| `papers/Seamless DeductiveInference via Macros.pdf` | Ascent paper. Relevant to embedded Datalog in Rust and macro-generated rule engines. |

### Code And Prototypes

| Path | Role |
| --- | --- |
| `code/dd-pr-525-eqsat.rs` | Local copy of Frank McSherry's EqSat-in-Differential prototype from `TimelyDataflow/differential-dataflow#525`. It represents AST nodes and exogenous equivalences as DD collections, derives congruence through iteration, and demonstrates equality retraction. Treat this as a sketch to inspect and reduce, not as production design. |

### Conversations

| Path | Role |
| --- | --- |
| `messages/oct-15-2024-zulip.md` | Main design discussion between Frank McSherry, Saul Shanabrook, Eli Rosenthal, Max Willsey, Oliver Flatt, and others. Important topics: an EqSat-in-Differential prototype, whether DD can express union-find and congruence closure, streaming WCOJ as semi-naive e-matching, equality retractions from changing e-class labels, nested scopes, contextual equality, multiple outputs, and analysis stratification. |
| `messages/dec-17-2025-slack.md` | Follow-up notes on FlowLog and datatoad. Important topics: FlowLog on DD, columnar representation, spreading work across many small iterations for DD parallelism, DD's nested fixed points as a possible match for egglog's outer rule loop plus inner congruence closure, lattice timestamps, arbitrary schedules, and the question of adding custom tables such as union-finds or containers. |

### Vendored Repositories

| Path | Current checkout | Role |
| --- | --- | --- |
| `repos/egglog` | `egraphs-good/egglog`, `main`, `b27cd225` | The implementation target. Important subtrees include `src/` for the core language and runtime, `core-relations/` for the current relation layer, `egglog-bridge/` for integration boundaries, `union-find/`, `numeric-id/`, `concurrency/`, `src/proofs/`, and the `tests/` corpus. |
| `repos/egglog-python` | `egraphs-good/egglog-python`, `main`, `8812ec9` | Python bindings and high-level user-facing API. Important sources include `docs/explanation/2026_02_containers.md`, `docs/reference/egglog-translation.md`, `docs/changelog.md`, and `python/tests/test_high_level.py`. This repo is the strongest local evidence that containers and higher-order container functions are part of the design target. |
| `repos/egglog-experimental` | `egraphs-good/egglog-experimental`, `main`, `eae9570` | Experimental extensions / standard-library-like layer on top of core egglog. Relevant features include `for`, `with-ruleset`, rationals, dynamic cost models, custom `run-with` schedulers, `get-size!`, and multi-extraction. Real workloads may depend on this surface. |
| `repos/egglog-tutorial` | `egraphs-good/egglog-tutorial`, `main`, `4ca08d2` | Compact examples for the language surface: basics, Datalog, analyses, scheduling, cost models, extraction, and a linear algebra compiler case study. Useful for calibrating small examples before attempting full egglog programs. |
| `repos/differential-dataflow` | `TimelyDataflow/differential-dataflow`, `master`, `1f348abd` | The target incremental dataflow framework. Key areas: `differential-dataflow/src/` for collection operators, `mdbook/` for conceptual docs, `dogsdogsdogs/` for advanced join patterns, `interactive/` for an interpreted DD IR, and `doop/` for a Datalog-like workload. |
| `repos/timely-dataflow` | `TimelyDataflow/timely-dataflow`, `master`, `b4e5ef9b` | The lower-level runtime under Differential Dataflow. Relevant for progress tracking, cyclic dataflows, scheduling, communication, container support, worker execution, and timestamp behavior. |
| `repos/flowlog` | `flowlog-rs/flowlog`, `main`, `83f3ae4` | The closest architecture study for compiling Datalog to Timely / Differential. Its crates split the pipeline into parser, stratifier, catalog, optimizer, planner, compiler, common utilities, and profiler. This is the main source for Datalog-specific planning on top of DD. |
| `repos/ascent` | `s-arash/ascent`, `master`, `f7b4fa4` | Embedded Rust logic programming system. Relevant for lattices, parallel Datalog, aggregation, and BYODS ("Bring Your Own Data Structures") with union-find-backed relation providers. |
| `repos/dynamic-datalog` | `frankmcsherry/dynamic-datalog`, `master`, `0fca831` | Benchmark and comparison material for dynamic Datalog engines. Includes CRDT, DOOP, and GALEN workloads, with hand-written Differential implementations under `differential-dataflow/`. |
| `repos/dataflow-join` | `frankmcsherry/dataflow-join`, `master`, `e87677a` | Streaming implementation of GenericJoin / worst-case optimal joins in Timely Dataflow. Relevant to relational e-matching and delta evaluation. Important files include `src/extender.rs`, `src/index.rs`, `src/motif.rs`, and the graph motif examples. |
| `repos/datatoad` | `frankmcsherry/datatoad`, `main`, `d16572f` | Interactive, columnar, worst-case optimal Datalog. It is not DD-based, but it is a high-signal reference for columnar Datalog execution, WCOJ planning, robust cyclic joins, and GALEN-style workloads. |
| `repos/columnar` | `frankmcsherry/columnar`, `master`, `abff5a7` | Columnar container library for converting arrays of structs into structs of arrays. Relevant because FlowLog and datatoad both point at columnar representation as a major performance lever, and Timely supports custom containers. |
| `repos/blog` | `frankmcsherry/blog`, `master`, `dd0949d` | Frank McSherry's blog archive. Useful for historical and design context around Timely, Differential, Datalog, WCOJ, datatoad, columnar layouts, and the e-graph discussion that led to this exploration. |

## Proof / Term Encoding Notes

Egglog's proof machinery is worth treating as a core source for the equality
maintenance question. The local design note is
`repos/egglog/src/proofs/proof_encoding.md`.

What it does today:

- The term encoding removes explicit `union` calls from the source program.
- It lowers equality maintenance into generated egglog relations and schedules:
  per-sort `UF_<Sort>` constructor UF tables, per-sort `UF_<Sort>f` function
  indexes, constructor term tables, constructor view tables, congruence rules,
  rebuild rules, and cleanup rules.
- It can be enabled through `EGraph::new_with_term_encoding`,
  `EGraph::with_term_encoding_enabled`, or the CLI `--term-encoding` flag.
- Proof mode builds on the same encoding and adds proof tables for terms, UF
  edges, and views.

Why it matters for this project:

- It suggests a way to express equality maintenance at the egglog level rather
  than in a separate Rust backend union-find and rebuild implementation.
- It is a concrete source-to-source encoding that can be compared against a DD
  encoding or reused as an initial relational lowering target.
- It makes equality and congruence explicit enough to instrument, inspect, and
  potentially translate to Differential Dataflow collections.

Known current constraints:

- The current implementation is roughly 100x slower than the native path; verify
  this on the chosen benchmark before drawing design conclusions.
- It does not support custom container sorts today. The local support check
  rejects sorts with presorts as "custom sort container implementation", and the
  proof unsupported snapshot includes container-heavy files such as
  `container-fail.egg`, `container-rebuild.egg`, `eqsat-basic-multiset.egg`,
  `vec.egg`, `map.egg`, and `set.egg`.
- This encoding does not eliminate equality maintenance. It shifts it into
  generated UF tables, view tables, and rebuilding schedules.

## Container Use Case From Egglog Python

The local file `repos/egglog-python/docs/explanation/2026_02_containers.md` is
the current source to read before designing around containers or higher-level
functions.

The use case:

- Binary algebraic operators plus associativity, commutativity, and
  distributivity can make e-graphs grow quickly, even for small expressions like
  `2 + a + b + b + 3`.
- The container approach moves some algebraic invariants into the representation
  itself. For example, `sum_(MultiSet(a, b))` and `sum_(MultiSet(b, a))` are
  indistinguishable because `MultiSet` ignores order.
- Containers preserve congruence by rebuilding references to e-classes inside
  the container when e-classes merge.
- Egglog does not directly match inside containers. One approach is to create an
  index function, such as `ms_num_index(xs, x) -> i64`, and fill it from the
  container with `xs.fill_index(ms_num_index)`.
- The higher-level approach uses container functions such as `map`, `fold`,
  `multiset_fold`, `multiset_flat_map`, `partial`, and `UnstableFn` to perform
  blockwise operations over whole containers during rule matching.
- The polynomial case study represents sums of products as nested multisets,
  roughly `polynomial(MultiSet[MultiSet[Value]])`, and uses higher-level
  functions to normalize polynomial subtrees without directly matching inside
  every container element.

Design implications:

- Container support must include both rebuild semantics and a story for derived
  indexes or higher-level functions over container contents.
- A relational backend should not assume every useful operation appears as a
  first-order relation over individual constructor terms. Some operations are
  intentionally packaged as primitive or higher-order functions over containers.
- If a DD-backed design materializes all intermediate pairwise matches that
  higher-level functions are meant to avoid, it may recreate the original
  e-graph blow-up in a different representation.
- Generic container APIs and Python conversions matter for the user-facing
  design, because current examples rely on typed containers and helpers such as
  `VecLike`, `SetLike`, `MapLike`, `MultiSet`, and higher-order callables.

## High-Signal Local Reading Order

1. Read `messages/oct-15-2024-zulip.md` and
   `messages/dec-17-2025-slack.md` to recover the actual design questions.
2. Read `repos/egglog/README.md`, then skim `repos/egglog/core-relations/`,
   `repos/egglog/union-find/`, `repos/egglog/src/`, and
   `repos/egglog/src/proofs/`.
3. Read `repos/egglog/src/proofs/proof_encoding.md`, then inspect
   `repos/egglog/src/proofs/proof_encoding.rs`,
   `repos/egglog/src/proofs/proof_encoding_helpers.rs`, and
   `repos/egglog/tests/files.rs`.
4. Read `repos/egglog-python/docs/explanation/2026_02_containers.md`, then
   skim `repos/egglog-python/docs/reference/egglog-translation.md`,
   `repos/egglog-python/docs/changelog.md`, and
   `repos/egglog-python/python/tests/test_high_level.py` for the current
   container and higher-level API surface.
5. Read `repos/egglog-experimental/README.md` and
   `repos/egglog-experimental/src/lib.rs` to understand the non-core surface
   area a future backend may need to support.
6. Work through `repos/egglog-tutorial/02-datalog.egg`,
   `03-analysis.egg`, and `04-scheduling.egg` to calibrate small examples.
7. Read `code/dd-pr-525-eqsat.rs` and identify which equality-maintenance ideas
   it demonstrates.
8. Read `papers/differentialdataflow.pdf`, then
   `repos/differential-dataflow/README.md` and the mdbook.
9. Read the FlowLog paper and `repos/flowlog/README.md`; inspect
   `crates/planner`, `crates/optimizer`, and `crates/compiler`.
10. Read `repos/dataflow-join/README.md`,
    `repos/differential-dataflow/dogsdogsdogs/README.md`, and
    `repos/datatoad/README.md` for WCOJ / Datalog execution tradeoffs.
11. Read `repos/ascent/README.MD` and `repos/ascent/BYODS.MD` before deciding
    whether custom relation providers are a better comparison point than DD.

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

## Remaining Sources To Add If Needed

The high-priority source set is now mostly local. Additional sources should be
added only when a research question needs them:

- The Egglog Python paper, if Python API ergonomics or frontend compatibility
  become central claims.
- The executable notebook or branch behind
  `repos/egglog-python/docs/explanation/2026_02_containers.md`, if reproducing
  the polynomial case study becomes necessary.
- FlowLog VLDB artifact repository, if reproducing FlowLog numbers or examples
  becomes part of the plan.
- DDlog / Differential Datalog sources, if the work needs a compiler-to-DD
  comparison beyond FlowLog.
- Souffle benchmark and implementation notes, if evaluation will use Datalog
  workloads already reported in FlowLog or `dynamic-datalog`.
- Ascent BYODS OOPSLA paper, if custom relation providers become central.
- Contextual / scoped e-graph sources, if the work touches nested scopes,
  filters, or letrec-like analyses: Slotted E-Graphs, Colored E-Graphs,
  Equivalence Hypergraphs, and RVSDG IR.

## Open Design Risks To Resolve Early

- DD can maintain arbitrary updates, but egglog mostly wants monotone growth plus
  representation-level churn from canonicalization. The cost of maintaining too
  much retraction history needs to be measured.
- A DD expression can encode union-find-like behavior, but it may not match the
  performance of direct mutable union-find unless the iteration and compaction
  strategy are chosen carefully.
- Egglog's proof / term encoding may be a useful relational lowering, but the
  current overhead and missing container support could make it a poor direct
  substrate without redesign.
- Egglog schedules are richer than "run all rules to a single fixed point". Any
  prototype should first show how arbitrary schedules map to DD loops.
- Relational e-matching and streaming WCOJ line up conceptually, but rebuilding
  and changing e-class labels create deltas that must be handled explicitly.
- Higher-level container operations are partly a way to avoid pairwise or
  structural explosion. A backend that lowers them into fully materialized
  first-order matches may preserve semantics while losing the point of the
  representation.
- Custom tables, containers, and sorts are central to egglog extensibility. A DD
  backend should prove that these extension points remain possible before
  chasing whole-program performance.
