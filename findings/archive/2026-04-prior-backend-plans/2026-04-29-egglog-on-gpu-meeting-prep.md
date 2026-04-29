# Egglog On GPU Meeting Prep - April 29, 2026

Meeting: Wednesday, April 29, 2026, 11:00 AM Pacific / 2:00 PM Eastern. The email thread uses PST/EST shorthand; this note treats the scheduled time as Pacific/Eastern local time.

## Source Basis

- Current repo findings: `findings/synthesis.md`, `findings/options/option-3-new-backend.md`, `findings/options/option-2-proof-term-encoding-dd.md`, `findings/source-notes/datalog-wcoj-planning.md`, `findings/source-notes/egglog-core-proof.md`, `findings/source-notes/containers-frontends.md`, and `findings/source-notes/scaling-equality-saturation.md`.
- Fresh Slack blocker list from `#parallel-egglog`, April 23, 2026: WCOJ was named as one of four blockers for egglog on the GPU; the remaining blockers were UF, containers/primitives, and scheduling.
- Fresh email thread `Re: egglog on the GPU`, March 16-April 27, 2026: Oliver pointed Yihao at the proof/term encoding and the `--term-encoding` flag; Yihao described a GPU Datalog engine with stratified aggregation/negation, `let`/`if` binding, instrumentation hooks, and a pluggable data-structure API, but no GPU union-find yet; Oliver noted proof encoding avoids built-in union-find/rebuild for a subset but is still much slower on large benchmarks.
- New paper: `papers/Scaling Worst-Case Optimal Datalog to GPUs.pdf`.

## One-Screen Context

The repo's current conclusion is not "move egglog to DD" or "move egglog to GPU." It is a tradeoff map. Differential Dataflow/Timely look useful as substrate ideas for maintained views, arrangements, fixed points, product timestamps, and overlapped physical execution, but DD does not automatically solve proof-query optimization, equality maintenance, rebuild invalidation, containers, primitives, or arbitrary schedules.

Proof/term encoding is a real alternate path because it relationalizes UF/view/rebuild/proof state and can remove explicit `union` operators for a supported subset. The caveats are central to this meeting: it is currently high-overhead, it does not cover presort/custom containers, and it is only a partial validation path for Python, containers, schedulers, primitives, and arbitrary per-rule freshness.

Option 3 is the relevant repo path for a serious rehost: a FlowLog/datatoad/DD-inspired single-owner new backend. The current experiments downgrade a permanent adapter over native state, because mirroring native rebuild, equality, containers, and scheduler state duplicates too much backend logic. They leave open a replacement-backend vertical slice that owns the moved responsibilities directly and compares step-visible state against native egglog.

## Paper Summary

`Scaling Worst-Case Optimal Datalog to GPUs` presents SRDatalog, a GPU Datalog engine built around recursive semi-naive evaluation plus WCOJ. The paper's core claim is that binary-join GPU Datalog engines run out of memory on deep cyclic program-analysis rules, while static GPU WCOJ engines are poorly suited to iterative Datalog because trie-like indexes are expensive to rebuild after every delta.

SRDatalog's design response is specific: flat sorted Structure-of-Arrays relation storage; a deterministic two-phase `Count` then `Materialize` pipeline for output sizing and coalesced writes; root-level histogram-guided load balancing for skew; targeted helper-relation splitting when skew is buried inside deep joins; and phase-aligned CUDA stream scheduling to overlap independent rule phases without fully asynchronous tuple-at-a-time execution.

For this project, the paper substantially strengthens the WCOJ/GPU evidence. It means WCOJ should not be treated as a purely speculative blocker anymore. But the paper also sharpens the remaining gap: SRDatalog is recursive Datalog, not egglog. It does not show how to preserve union-find locality, rebuild retimestamping, same-id container refresh, primitive callbacks, proof/term encoding, or custom scheduler admission.

## Blocker Map

- WCOJ: partly de-risked by SRDatalog at the Datalog/GPU level. Still not de-risked for egglog because canonical-id rewrites, rebuild invalidation, timestamp-window freshness, and scheduler materialization may invalidate the storage/scheduling assumptions that make SRDatalog fast.
- Union-find: still central. Options are GPU-native UF/pluggable data structure, proof/term encoding without built-in UF, or keeping UF native/specialized behind a provider boundary. The repo evidence says representative choice and rebuild locality matter, not just equivalence closure.
- Containers/primitives: still a compatibility blocker. Containers can change semantics while outer ids remain stable; primitive and Python-facing operations are not ordinary relational joins. The meeting should separate "primitive callbacks in Datalog" from "egglog container rebuild semantics."
- Scheduling: still semantic, not just performance. Egglog needs per-rule last-run timestamps and arbitrary user schedules; SRDatalog's stream scheduling relies on monotone Datalog rule parallelism, while egglog has bounded schedules, staged actions, custom scheduler admission, and rebuild/action visibility.

## Proposed Agenda

1. Align on the target: are we discussing a quick proof-encoding demo on Yihao's engine, a native egglog/GPU integration, or an Option 3 replacement-backend research slice?
2. Let Yihao/Kris summarize the SRDatalog execution model and what parts of their GPU engine are already reusable: WCOJ kernel, column storage, seminaive loop, stream scheduling, pluggable data structures, primitive hooks.
3. Let Oliver summarize proof/term encoding as the minimal relational egglog path: what it covers, what it rejects, expected overhead, and whether it can emit a small benchmark for Yihao's engine.
4. Let Eli summarize the native backend constraints that must be preserved: per-rule seminaive timestamps, timestamp-ordered tables, Free Join baseline, two-phase query/merge, rebuild, and scheduler behavior.
5. Decide the first concrete artifact: proof-encoding export on a small program, a WCOJ-only egglog rule benchmark, a UF/rebuild microbenchmark, or an Option 3 vertical-slice design note.

## Decision Points

- Is WCOJ now "solved enough" for the first conversation, so the near-term focus should move to UF, containers/primitives, and scheduling?
- Should the first experiment use proof/term encoding because it avoids native UF/rebuild, or should it use a native-backed/provider split because proof encoding is too slow and feature-incomplete?
- Is the desired collaboration an external GPU Datalog backend for a supported egglog subset, or a longer-term single-owner egglog backend path?
- What is the smallest benchmark everyone accepts as meaningful: constructor-only equality, proof-encoded arithmetic, a cyclic multiway join, a container dirty-refresh case, or a scheduled reachability/per-rule freshness witness?
- What needs to be exposed from Yihao's engine to make this real: pluggable UF, delete/retract support, primitive callbacks, stable sorted column updates, stream-schedule controls, or profiling counters?

## Questions For The Meeting

- Yihao: In SRDatalog, how tightly coupled are WCOJ performance and the flat sorted column layout? Could canonical-id rewrites or container dirty refresh be represented as cheap deltas, or would they force broad resort/rebuild?
- Yihao: How mature is the pluggable data-structure API, and what would a GPU union-find need to expose to the Datalog rule engine?
- Yihao: Does the engine support retractions/deletes strongly enough for proof/term encoding's generated rebuild/delete/subsume relations?
- Kris: Which application should drive the first benchmark: decompilation, DOOP-like pointer analysis, proof-encoded egglog examples, or a small equality-saturation kernel?
- Kris: For collaboration scope, is the first useful outcome a demo, a joint design doc, or a measurable prototype against the SRDatalog paper workloads?
- Oliver: What is the smallest proof/term-encoded egglog program that exercises equality/rebuild but avoids unsupported containers/presorts/primitives?
- Oliver: Is there a clean way to emit the proof/term encoding as Datalog plus expected facts so Yihao can run it independently?
- Eli: Which native backend witness best protects scheduling correctness: scheduled reachability, a custom scheduler materialization case, or rebuild/container freshness?
- Eli: What native counters would most quickly tell us whether a GPU/WCOJ path is dominated by joins or by rebuild/container/scheduler churn?

## Suggested Outcome

Leave the meeting with one agreed first artifact and one explicit non-goal. The cleanest first artifact is probably a two-track microbenchmark: one SRDatalog-shaped WCOJ-heavy Datalog rule body to test the GPU upside, and one egglog-shaped equality/rebuild or scheduling witness to test the semantic blocker. The non-goal should be a broad egglog backend port before the team knows whether UF/rebuild, containers/primitives, and scheduling can cross the GPU boundary without becoming a second native backend.
