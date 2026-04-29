notes from eli

## feedback on design spike
might keep egglog-bridge API mostly
nice to have crate divisions


## current issues with crates
- frontend API is bad, python is reasonable
- egglog crate code is horrible. Ways that commands and rules are elaborated in confusing. same types for unnested and nested rules, different phases operate on different ways. Params are confusing with type params, NCommands.
  - Proofs make this harder, because they speak about surface syntax. Proof must be about surface syntax
    - unlike smtlib no standards for proof format. So need own version of proof format
    - for proofs, about core thing, not about elaboraiton. Issue with Python frontend doing syntax transformations.
- if we want egglog to be sophisticated language and have type checking, we need to think hard about it
  - have awesome surface API not required if you build on top of it, most of the time we write rust programs that generate egglog, happy to specify types.
- really complicated parser and type checker (egg also has this problem, but its simpler).
- as implemented today, its ok how the separation works in terms of backend and
problems with containers:
- containers resolved eagerly, have to grab a lock every time we access containers, a lot of work to resolve this with parallel
- matching hard


## Should we try DD?
- get simple subset in new crate that models egglog, so I know how it uses DD to model core things. Top down patterns, arbitrary joins, multi-patterns, schedules (Find something in test suite for this). working with agent on how to model them in DD. Mock up basic benchmarks. Have stripped back chunk of code so that it's easier to reason about how it behaves to look at performance profiles. Then turn that into artifact. Use egglog as oracle, use egglog to generate test databases, and verify it behaves the same. Use that to see in performance. Priority is simplicity to understand how things map to DD land so we can understand it and to compare performance.
- two types of benchmark, few rules that process a lot data. New backend much faster there. Benchmark suite however isn't like this, lots of tiny iterations, fixed cost of building rule becomes more important, harder to beat. High throughput, like Math, and tiny iterations like eggcc and python array optimize. Then hard joins, hardboiled benchmark.

- WCOJ did have paper on "Distributed evaluation of subgraph queries using worst case optimal low memory dataflows" but just look at dataflow join repo

Should read DBSP paper, simpler model, also supports nested and recursive. Frank was last author, authors founded competing startup.

Benefit of DBSP: automatically differentiates code instead of DD, where you do it manually
