# Comparative Extension Models

## Sources Read
- `README.md`: project objective and source inventory for extension, container, DD, and benchmark questions.
- `papers/Seamless DeductiveInference via Macros.pdf`: Ascent paper; extracted locally with `uv run --with pypdf`.
- `repos/ascent/README.MD`: Ascent feature overview, including Rust embedding, lattices, aggregation, parallelism, and BYODS.
- `repos/ascent/BYODS.MD`: custom relation-provider contract for Ascent BYODS.
- `repos/ascent/ascent/src/internal.rs`: trait surface custom relations must satisfy (`RelIndexRead`, `RelIndexWrite`, `RelIndexMerge`, etc.).
- `repos/ascent/ascent/src/rel.rs`: default relation provider macros and index types.
- `repos/ascent/byods/ascent-byods-rels/src/adaptor/bin_rel.rs`: smaller helper trait for binary custom relations.
- `repos/ascent/byods/ascent-byods-rels/src/eqrel.rs`: equivalence-relation provider facade.
- `repos/ascent/byods/ascent-byods-rels/src/trrel_uf.rs`: union-find-backed reflexive-transitive relation provider.
- `repos/ascent/byods/ascent-byods-rels/examples/steensgaard/main.rs`: concrete `#[ds(eqrel)]` analysis compared with explicit closure rules.
- `repos/columnar/README.md`: columnar layout benefits, costs, and benchmark shape.
- `repos/columnar/src/lib.rs`: `Columnar`, `Container`, `Borrow`, `Push`, `Index`, and byte-conversion trait design.
- `repos/blog/posts/2024-10-11.md`: Timely custom containers plus `columnar` as an operator-boundary extension model.
- `repos/blog/posts/2025-11-21.md`: datatoad WCOJ planning and delta/base rule evaluation model.
- `repos/blog/posts/2025-12-03.md`: columnar WCOJ direction and GALEN-style cyclic join motivation.
- `repos/blog/posts/2016-06-21.md`: Datalog-on-DD model: joins, explicit iteration, arbitrary input updates, and magic-set style querying.
- `repos/dynamic-datalog/README.md`: benchmark corpus, published timings, and caveats about hand-written DD implementations.
- `repos/dynamic-datalog/differential-dataflow/src/bin/crdt.rs`: hand-written DD CRDT implementation using `iterate`, antijoins, reductions, and updates.
- `repos/dynamic-datalog/differential-dataflow/src/bin/doop.rs`: hand-written DD DOOP implementation with large generated relation/index surface.
- `repos/dynamic-datalog/differential-dataflow/src/bin/galen.rs`: hand-written DD GALEN implementation with explicit arrangements and recursive variables.

## Key Findings
- Ascent's BYODS is the clearest local example of a maintainable extension boundary for non-standard relations: a relation can be tagged `#[ds(provider)]`, while provider macros return relation storage plus index adapters (`repos/ascent/BYODS.MD`, `repos/ascent/ascent/src/rel.rs`). This makes provider-style relation boundaries a first-class architecture axis, not just a local optimization.
- The BYODS contract is powerful but not small. Custom providers must line up macro expansion, shared per-relation state, readable indexes, writable indexes, merge behavior, and sometimes parallel variants (`repos/ascent/BYODS.MD`, `repos/ascent/ascent/src/internal.rs`). That implies egglog-on-DD cannot treat "custom table" support as a late wrapper; it needs an explicit provider ABI and a comparison against the ordinary-relation path.
- The Ascent BYODS examples map directly to egglog concerns: `eqrel` and `trrel_uf` provide equivalence/transitive relation semantics through specialized data structures (`repos/ascent/byods/ascent-byods-rels/src/eqrel.rs`, `repos/ascent/byods/ascent-byods-rels/src/trrel_uf.rs`). The Steensgaard example replaces explicit equivalence-closure rules with `#[ds(eqrel)]`, which is analogous to avoiding relationalizing all equality maintenance in egglog while still leaving ordinary relations available for DD/FlowLog-backed storage (`repos/ascent/byods/ascent-byods-rels/examples/steensgaard/main.rs`).
- The Ascent paper strengthens the extensibility argument: Ascent's design goal is embedding logic rules in Rust with user-defined components, user-defined data types, lattices, generated indexes, and semi-naive evaluation. It reports comparable performance to Datafrog/Souffle on borrow-checker benchmarks, but this is from compiled Rust kernels, not DD traces (`papers/Seamless DeductiveInference via Macros.pdf`).
- Columnar storage is a plausible performance lever, not a semantic abstraction. `columnar` gives `#[derive(Columnar)]`, struct-of-arrays containers, few large primitive allocations, zero-copy byte conversion, and efficient field projection (`repos/columnar/README.md`, `repos/columnar/src/lib.rs`). The blog connects this to Timely's ability to ship non-`Vec<T>` containers (`repos/blog/posts/2024-10-11.md`).
- Columnar storage is type-invasive: inserted values come back as reference-shaped values like `(&str, &i64)` rather than `&(String, i64)`, and the README explicitly warns that this may make values unusable as map keys, remove locality for coupled fields, and limit in-place mutation (`repos/columnar/README.md`). Egglog terms, e-class ids, and containers would need careful representation design before adopting it.
- Dynamic Datalog is useful as an evaluation warning, not a reusable framework. The README says DD results are hand-written Rust and can include optimizations unavailable to Datalog systems; CRDT, DOOP, and GALEN each stress different features: negation/reduce idioms, scale, and skewed/cyclic joins (`repos/dynamic-datalog/README.md`).
- The hand-written DD benchmark code shows maintainability risk. `doop.rs` defines many domain aliases, loaders, arrangements, and a custom recursive `Relation` wrapper; `galen.rs` manually constructs arrangements like `p_by0`, `p_by1`, `q_by01`, and joins for each IR rule. This is evidence that a naive egglog lowering to DD may create substantial generated code and planning complexity (`repos/dynamic-datalog/differential-dataflow/src/bin/doop.rs`, `repos/dynamic-datalog/differential-dataflow/src/bin/galen.rs`).
- The blog posts point toward a hybrid design: Datalog-on-DD handles explicit iteration and arbitrary updates, while datatoad/WCOJ-style planning handles cyclic joins and delta/base staging. That fits relational e-matching better than plain binary joins, but it also means the backend is more than "compile rules to DD joins" (`repos/blog/posts/2016-06-21.md`, `repos/blog/posts/2025-11-21.md`).

## Relevance To The Main Objective
- These sources support moving some egglog rule evaluation onto a maintained dataflow substrate only if the design includes first-class extension points for custom relations, indexes, and storage layouts.
- They weaken a full "DD owns everything" approach: the most relevant custom-equality example uses a provider (`eqrel`/`trrel_uf`) to avoid expressing closure purely as ordinary rules, suggesting egglog equality maintenance may also need specialized state.
- Ordinary DD/FlowLog-backed relations could coexist with specialized equality, container, or rebuild providers, but that is a concrete architecture comparison to make, not an assumption to bake in.
- Columnar and WCOJ sources support optimizing relational e-matching inputs and joins, but not necessarily congruence closure, rebuilding, analyses, extraction, or container rebuilding.

## Likely Blockers
- Provider ABI complexity: Ascent's BYODS surface is macro-heavy and trait-heavy; reproducing this around DD traces, updates, and arrangements could become another engine-specific abstraction layer.
- Equality churn: if egglog equality is modeled as ordinary DD collections, representative changes and cleanup may cause high retained-history or retraction costs; if modeled as custom providers, DD may not see enough structure to optimize.
- Type mismatch from columnar: borrowed/reference-shaped access may not work directly for keys, hash maps, canonical e-class ids, or mutable rebuild paths.
- Planning burden: Dynamic Datalog's DD examples are hand-optimized; an egglog backend would need an automatic planner for arrangements, delta rules, WCOJ stages, and possibly custom table operators.
- Extensibility vs. performance tension: the more custom providers are allowed, the harder it is to preserve DD's generic incremental-maintenance story.
- Architecture split: without a prototype, it is not clear whether the ordinary/provider boundary should be one backend with multiple relation kinds or a more explicit cross-cutting model that every option has to account for.

## Promising Connections
- Treat Ascent BYODS as a design pattern for an egglog backend interface: relation declarations can choose default DD-backed storage, equality-specific providers, container index providers, or columnar providers.
- Use `ByodsBinRel` as a minimal inspiration for provider ergonomics: require `contains`, `iter_all`, keyed iterators, estimates, and insertion before exposing the full lower-level index API.
- Use `columnar` for high-volume immutable or append-heavy fact batches crossing Timely/DD operator boundaries, especially relation rows with strings, enum terms, or nested AST-like payloads.
- Use Dynamic Datalog's CRDT/DOOP/GALEN as non-egglog stress tests for any generated backend before claiming DD maintainability benefits.
- Use datatoad's delta/base WCOJ staging as the join-planning model for e-matching-heavy rules, then compare against DD binary arrangements.
- Compare a mixed model explicitly: ordinary DD/FlowLog relations plus specialized providers for equality, containers, or rebuild-sensitive state.

## Evidence Needed Next
- A tiny DD-backed relation-provider prototype with one ordinary relation, one `eqrel`-like custom provider, and one columnar-backed relation.
- Measurements comparing explicit relational equality closure against a custom union-find/equality provider on an egglog-shaped workload.
- A generated-code review for one realistic egglog program lowered to DD: number of arrangements, variables, nested scopes, and custom operators.
- Columnar microbenchmarks for actual egglog row shapes: constructor rows, e-class ids, small vectors/maps/multisets, and string/symbol-heavy terms.
- A maintainability audit of whether custom providers can support retractions, rebuild/canonicalization, provenance/debugging, and Python-facing container behavior.
- A direct comparison of ordinary DD/FlowLog-backed relations against provider-based equality/container/rebuild relations, to see whether the split is worth making explicit.

## Confidence
- Medium: the local source evidence strongly identifies the extension and storage tradeoffs, but no egglog-specific DD/provider prototype or measurements were run in this pass.
