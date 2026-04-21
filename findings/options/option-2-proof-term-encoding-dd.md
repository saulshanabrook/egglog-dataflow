# Option 2: Proof/Term Encoding To DD

## Viability
- Low / Medium: usable as an executable specification for a relational lowering, but risky as the main DD architecture. The proof/term encoding already rewrites egglog into generated UF, view, term, proof, rebuild, cleanup, and delete/subsume relations, so it shows that equality maintenance can be expressed relationally for a supported subset. The problem is that this is an intentionally heavy encoding: it runs explicit saturating rebuild schedules, duplicates function state into term/view/proof tables, rejects several current egglog features, and does not support presort/custom container sorts. DD can maintain the joins and arrangements, but the encoding is only a partial oracle for validation. It does not cover the full Python/container/scheduler/presort/custom frontend surface, so a native escape hatch would still be required for the behavior that users actually exercise.

## General Approach
- Treat `repos/egglog/src/proofs/proof_encoding.rs` as the reference lowering: for each egg sort, generate a constructor UF table plus function-backed UF index; for each function, generate immutable term rows and mutable view rows keyed by canonical children plus representative output; generate rebuild rules that join view rows with UF leaders and emit delete/insert view deltas; generate congruence or merge-function rules from duplicate canonical views; keep proof tables optional. In DD, map these generated relations to collections/arrangements, run the proof encoding's rebuild schedule as a DD/Timely fixed-point epoch, and use FlowLog/datatoad-style lowering only for the rule/query layer around these collections.

## What Would Move
- DD would own the generated relation storage, joins over view tables, UF-index lookups, canonical-view rebuild joins, duplicate-view congruence detection, merge cleanup joins, delete/subsume helper tables, seminaive delta propagation, shared arrangements, and trace/progress scheduling. FlowLog/datatoad ideas would mainly help compile the generated Datalog-like rules and arrange stable indexes such as `(child_id -> parent views)` and `(op, canonical_children -> representative)`.

## What Stays Native
- Egglog should keep parsing, typechecking, desugaring, primitive execution/validation, extraction, proof extraction/checking, command semantics, and the policy decisions around equality ordering. Native code likely also keeps a specialized union/congruence component or at least validates the proof-encoding UF rules, because the current DD prototype uses reduce-heavy transitive closure and explicitly is not evidence of an efficient connected-components implementation. Containers, presort/custom sort implementations, Python-facing command behavior, and custom scheduler behavior stay native unless redesigned, because proof encoding rejects presorts and current container rebuild tracks same-id semantic changes through `dirty_ids`, not ordinary relational value changes.

## Required Interfaces
- A stable generated-relation schema for sort UF tables, UF function indexes, term tables, view tables, proof tables, cleanup tables, and delete/subsume request tables.
- A command boundary that batches original egglog commands into DD epochs and runs the generated rebuild schedule to a fixed point before checks, extraction, and subsequent commands.
- A primitive/merge-function callback interface from DD actions into egglog for evaluated expressions and user merge functions.
- A delta API for equality changes and canonical leader changes, including deletes from old view rows and inserts for rebuilt view rows.
- A native escape hatch for containers: container value rebuild must emit both changed-id deltas and same-id dirty-id refresh/invalidation events.

## Main Risks
- Overhead: every function becomes at least a term table plus view table, every sort gets UF and UF-index tables, proof mode adds term/view/UF proof functions, and rebuild runs after commands via nested saturating rules (`proof_encoding.md`, `proof_encoding.rs`).
- Equality churn: a single union can force canonicalization across every view column that mentions affected ids; DD arrangements help but still process wide delete/insert waves rather than a compact union-find mutation.
- Unsupported features: proof encoding rejects primitives without validators, function lookups in actions/extracts, presort/custom container sorts, input commands, user-defined commands, non-global `:no-merge` functions, and some non-eq let/set cases (`proof_encoding_helpers.rs`). That makes it a partial oracle, not a complete validation path for the current frontend.
- Merge semantics: custom functions need generated merge rules during rebuilding plus cleanup of old values, so DD must run egglog merge expressions inside the rebuild loop, not just in user actions.
- Container incompatibility: presort containers are rejected by proof encoding, while current native containers can change semantics without changing outer ids; that needs parent-row refresh and is not captured by ordinary term/view delete-insert logic (`findings/source-notes/egglog-core-proof.md`).
- Timestamp/trace cost: DD supports nested iteration and arrangements, but Timely progress cost grows with timestamp granularity and DD traces retain history until compaction; per-union or per-rebuild micro-times could dominate.

## Evidence To Gather
- Run the proof/term encoding on small constructor-heavy, merge-heavy, and rebuild-heavy egglog programs and compare row counts against the native backend.
- Use proof/term encoding only as a partial relational reference, then separately test Python/container/scheduler/presort/custom frontend behavior natively where the encoding cannot reach.
- Port only the generated UF/view/rebuild subset to a tiny DD prototype: constructors, UF parent relation, UF function index, view rebuild, congruence, and merge cleanup.
- Measure DD records emitted for one large class merge as parent fanout grows, especially view rebuild deletes/inserts and congruence-key duplicate detection.
- Test against `repos/egglog/tests/container-rebuild.egg` and `repos/egglog/tests/merge-during-rebuild.egg`; expect the former to require a native dirty-id invalidation stream.
- Decide and measure timestamp granularity: per command, per rebuild phase, per rule batch, or per union.

## Current Assessment
- The proof/term encoding is valuable as a partial specification and prototype oracle because it names the relational state DD would need and provides a concrete correctness target for supported UF/view/rebuild behavior. It is weak as a main production lowering until measurements show that its generated state is affordable and until unsupported Python/container/scheduler/presort/custom frontend behavior has a separate validation path or native side channel.
