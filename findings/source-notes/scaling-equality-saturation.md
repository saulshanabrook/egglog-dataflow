# Scaling Equality Saturation

## Sources Read
- `repos/scaling-equality-saturation/egglog-new-backend.md`: Eli Rosenthal's June 2025 draft on egglog's backend design, with emphasis on scheduling, seminaive evaluation, parallelism, table layout, BYODS, Free Join, and benchmark caveats.
- `messages/eli-scheduling-seminaive.md`: local preservation of Eli's note that supporting both seminaive evaluation and arbitrary schedules is a major egglog design constraint.
- `messages/eli-dd-overlapped-scheduling.md`: local preservation of Eli's clarification that DD may overlap physical iteration work without changing egglog's logical schedule semantics.
- `https://gist.github.com/ezrosent/80190c70245632388f536fa259ec54b8`: public gist copy of the same draft; the public gist comments endpoint returned an empty list when checked, so no separate public gist comments were integrated.

## Key Findings
- Arbitrary schedules are a semantic and runtime constraint, not just frontend syntax. The draft says egglog imports egg-style explicit scheduling into a Datalog-like system: users can partition rules into rulesets and explicitly compose `repeat`, `saturate`, and bounded `run` commands to control blowup, handle non-saturating rules, and manually stratify reasoning that depends on a canonical e-graph shape (`repos/scaling-equality-saturation/egglog-new-backend.md`).
- Standard Datafrog-style seminaive evaluation is not correct under arbitrary egglog schedules. The reachability example in the draft shows that if one ruleset is saturated before another, tuples that are "stable" globally may still be new to a rule that has not run recently. A backend must therefore track freshness relative to each rule's last execution, not only global recent/new/stable sets.
- Egglog's current solution is per-rule logical timestamping: each row is tagged with a logical timestamp advanced when any ruleset finishes an iteration, and each rule records the last timestamp at which it ran. Seminaive queries then use timestamp windows such as `TLast..` so a rule sees facts that are new since that rule last executed.
- This timestamp solution forces a storage/index requirement: tables must support efficient timestamp-range slicing in addition to value-based lookup and join access. The draft describes timestamp-ordered hash tables, tombstoning for stale rows, and deterministic parallel compaction that preserves timestamp order.
- The backend is intentionally two-phase for parallelism: queries read immutable table state and stage mutations, then merge applies staged updates in bulk. The constructor/function split exists partly to keep reads low-coordination while still supporting nested constructor insertion and function merge semantics.
- Rebuild remains native and specialized. The draft describes incremental and nonincremental congruence-closure table rebuild, fixed-point rebuild because merge can trigger more unions, and the performance importance of doing canonicalization in bulk.
- Egglog already has a BYODS-like table boundary. The `Table` trait supports scans, timestamp updates, constraint refinement, optional fast subsets, key lookup, staged mutation buffers, merge, and rebuild hooks. The union-find is implemented behind this API, but the draft says implementing congruence closure as a database query had too much overhead compared with the direct custom algorithm.
- Egglog's rule matcher is closer to Free Join than plain binary joins. The draft describes lazy subsets, cached hash indexes, fused scans, batching, vectorized actions, morsel-driven parallelism, and dynamic variable ordering. It also says binary and bushy plans are future work because many queries would be faster with traditional planning.
- The benchmark section is useful but explicitly provisional. It reports good but non-linear scaling to 16 cores, identifies serial union-find insertion as a bottleneck in the math benchmark, and warns that some numbers use unsubmitted low-level optimizations.
- The draft explicitly leaves Timely/Differential rehosting as future work: those systems appear to generalize what egglog can do, but that claim still needs confirmation against egglog's scheduling, timestamp, equality, and table-interface constraints.
- Eli's later clarification changes the interpretation of "small-iteration" scheduling. The interesting DD hypothesis is not primarily semantic relaxation; it is that DD's multidimensional time/frontier tracking may let physical work for iteration `N+1` start before iteration `N` is fully complete, while still withholding or correcting later results so egglog's logical schedule semantics are preserved (`messages/eli-dd-overlapped-scheduling.md`, `findings/source-notes/differential-timely.md`).
- The first `code/option3-overlap` prototype reproduced the corrected scheduled
  reachability witness as an oracle/DD test. A deliberately broken global
  seminaive baseline missed `reachable(1, 3)`, while `dd-barrier` and
  `dd-overlap` preserved per-rule freshness with zero early visibility
  violations (`findings/option-3-experiments.md`).

## Relevance To The Main Objective
- This source strengthens the case that a DD/FlowLog backend must model logical scheduling as a first-class interface. It is not enough to preserve final saturated results for all-rules-at-once runs; exact backend modes must preserve per-rule freshness under arbitrary user schedules. DD-overlapped physical execution may still be possible if timestamp/frontier tracking keeps that logical contract intact.
- It weakens any design that treats seminaive evaluation as a generic Datalog feature already solved by DD/FlowLog. Egglog's seminaive semantics depend on per-rule last-run timestamps and efficient timestamp-window scans.
- It strengthens the native-improvement and provider-boundary arguments: the current backend already embodies several substrate-like ideas, including incremental timestamps, maintained indexes, staged mutation, dynamic table providers, rebuild hooks, and parallel bulk execution.

## Likely Blockers
- DD timestamp/progress design must be reconciled with egglog's per-rule logical timestamps. A backend cannot collapse many logical rule executions into one coarse time if that loses rule-local freshness, but one dataflow time per tiny physical task may create too much progress and trace overhead.
- A DD/FlowLog rule evaluator needs efficient timestamp-window access. If DD arrangements are keyed only by value or join attributes, preserving egglog seminaive behavior may require additional time-keyed arrangements or an auxiliary freshness index.
- Small-iteration physical scheduling could conflict with logical seminaive windows. The current Option 3 hypothesis is that DD frontiers can make this exact rather than relaxed: splitting a ruleset into many DD tasks is valid only if later-iteration work remains observationally hidden or correctable until each rule sees exactly the facts it would see under the user-visible schedule.
- Rehosting too much of the current backend risks duplicating existing table-provider, timestamp, rebuild, and mutation-buffer machinery rather than replacing it with a simpler shared substrate.
- Current benchmark evidence is not enough to compare against DD/FlowLog. It is provisional, limited to available workloads, and includes unsubmitted low-level optimizations.

## Promising Connections
- Treat per-rule last-run timestamps as an explicit compatibility test for Options 1 and 3: a DD-backed matcher must reproduce the reachability schedule counterexample from the draft.
- Use egglog's timestamp-ordered hash table as the native baseline when evaluating DD arrangements, FlowLog plans, or datatoad-style indexes.
- Use the existing `Table` trait as the concrete local analogue for provider-style relations: ordinary tables, union-find, containers, and rebuild-aware tables can be compared against DD-backed provider designs.
- Use Free Join, dynamic variable ordering, and future binary/bushy planning as Option 4 native-improvement targets and as Option 3 planner requirements.
- Use two-phase query/merge execution as the semantic boundary for rule evaluation prototypes: read-only matching first, staged native action/merge/rebuild second.

## Evidence Needed Next
- Reproduce the scheduled reachability example from the draft in native egglog and include it as a regression for any DD/FlowLog rule-evaluation prototype.
- Measure the cost of preserving per-rule timestamp windows in a DD design: number of arrangements, trace times, progress messages, and retained records.
- Compare DD value-keyed arrangements against native timestamp-ordered hash tables on one rule that needs both keyed lookup and timestamp-window slicing.
- Extend the Option 3 freshness test to cases with native actions, rebuild
  invalidation, custom scheduler match selection, and equality/container refresh
  barriers. The first reachability-only DD-overlap run preserved freshness, but
  synthetic native barriers reduced the scheduling benefit.
- Compare native Free Join with binary/bushy planning on a few real egglog rule bodies before assuming DD binary joins or WCOJ kernels are the right default.

## Local Experiment Update
- The Option 3 schedule lane now provides the requested per-rule freshness
  regression: the broken global-seminaive baseline misses the witness, while
  `dd-barrier` and `dd-overlap` match the oracle with zero early visibility
  violations (`findings/option-3-experiments.md`).
- Follow-up rebuild, container, and scheduler lanes passed their targeted
  native regressions, but they reinforce the need for explicit logical
  boundaries. Exactness is preserved by respecting rebuild/action/scheduler
  phases, not by proving those phases can be freely overlapped.
- The next useful scheduling measurement is direct instrumentation: per-rule
  match/admission counters, row refresh counters, and DD trace memory/compaction
  metrics.

## Confidence
- Medium / High. The source is a detailed design draft from an egglog maintainer and directly addresses the scheduling/seminaive issue, but the performance numbers are explicitly provisional and this pass did not run the described benchmarks.
