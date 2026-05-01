<!-- Generated from `src/lib.rs` by `tools/rust_literate.py`; do not edit by hand. -->

# Minimal relation-only Differential Dataflow trial against native egglog

## What we will build and check

In this walkthrough we run a tiny relation-only Differential Dataflow model
beside native egglog and check that both produce the same logical rows. Each
scenario is represented twice: native egglog runs a real `.egg` fixture and
exports lower function-table rows through `EGraph::function_for_each`, while
the trial side evaluates a small hand-written relation/rule model in
Differential Dataflow.

By the end, the acceptance report should say `all_match_oracle: true` for
three scenarios:

- reachability derives six `path` rows from three `edge` rows;
- repeated-variable matching keeps only `(1, 1)` and `(2, 2)`;
- a three-way join derives `out(1, 5)`, `out(1, 6)`, and `out(9, 5)`.

Run the visible check with:

```text
cargo run --manifest-path code/minimal-dd-trial/Cargo.toml
```

You may see compile warnings from the vendored `egglog` crate first. For
this tutorial, ignore those warnings unless the command exits with an error.
The command prints a large JSON report; do not read it top to bottom.

Check these four success markers:

```text
"scenario": "path-reachability"
"scenario": "repeated-variable"
"scenario": "three-way-join"
"all_match_oracle": true
```

Then check that each scenario block contains:

```text
"matches_oracle": true
```

That is the first lesson: the trial is not trying to replace all of egglog.
It runs three small relation programs twice, once in native egglog and once
in the DD model, and checks that the final logical row sets match.

The comparison deliberately projects logical `i64` input tuples and keeps
lower-row output ids, raw values, sorts, and `subsumed` flags as debug
evidence.
Here "lower rows" means egglog's function-table rows below the rendered
`print-function` / `TermDag` layer: the stored input values, output value,
and subsumption bit used by the database.

## Four DD/TD words used in this file

Keep these local meanings in mind on a first pass:

- A DD collection is a relation-like multiset of records.
- A signed diff is an update weight: `+1` adds a record and `-1` removes one.
- An arrangement is a maintained index over a collection, like indexing a
  relation by selected columns before a join.
- A Timely scope is the dataflow context where DD operators are connected.
  An iterative scope is the loop form used to keep deriving rows until the
  recursive relation stops changing.

These definitions are intentionally small. The walkthrough only needs them
to explain how the three acceptance scenarios move through the trial.

## What we will use

We only need a small slice of egglog and DD for this walkthrough: `i64`
relation facts, relation atoms, repeated-variable filters, natural joins,
one recursive reachability loop, and a final comparison against native
egglog's lower rows.

## What this walkthrough leaves aside

This first pass only follows relation facts and relation rules over `i64`.
Equality/rebuild, containers, custom schedulers, host callbacks, extraction,
proofs, direct `ResolvedCoreRule` export, and performance measurement are
later gates. Keep that boundary in mind, but do not follow those threads yet.

## The path through the code

We will follow one successful run in four steps.

1. [`acceptance_scenarios`] defines the three tiny programs.
2. [`run_scenario_trial`] runs native egglog and the DD model for one
   program.
3. [`dd_evaluate_scenario`] builds the DD collections and captures signed
   updates.
4. The host nets those signed updates into visible rows and compares them
   with the native lower-row oracle.

Notice the repeated pattern: each section turns one representation into the
next, then checks a concrete row set.

<details>
<summary>Imports and crate wiring</summary>

```rust
use differential_dataflow::input::Input;
use differential_dataflow::lattice::Lattice;
use differential_dataflow::operators::iterate::Variable;
use differential_dataflow::VecCollection;
use egglog::{EGraph, Value};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::Path;
use timely::dataflow::operators::capture::Extract;
use timely::dataflow::operators::Capture;
use timely::order::Product;
use timely::progress::Timestamp;

pub type TrialResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub const PATH_REACHABILITY_EGG: &str = include_str!("../fixtures/path-reachability.egg");
pub const REPEATED_VARIABLE_EGG: &str = include_str!("../fixtures/repeated-variable.egg");
pub const THREE_WAY_JOIN_EGG: &str = include_str!("../fixtures/three-way-join.egg");

```

</details>

## Acceptance scenario fixtures

All scenarios in the tutorial's success path.

Read these before the evaluator. They give us small expected results that
make the later DD operators easier to follow.

```rust
pub fn acceptance_scenarios() -> TrialResult<Vec<ScenarioSpec>> {
    Ok(vec![
        path_scenario()?,
        repeated_variable_scenario()?,
        three_way_join_scenario()?,
    ])
}

```

Recursive reachability scenario.

Start here. The facts are `edge(1,2)`, `edge(2,3)`, and `edge(3,4)`.
The expected final `path` relation is:
`(1,2)`, `(1,3)`, `(1,4)`, `(2,3)`, `(2,4)`, `(3,4)`.

Notice that `(1,4)` cannot appear from one rule firing; it confirms that
the DD loop reached the recursive fixed point.

```rust
pub fn path_scenario() -> TrialResult<ScenarioSpec> {
    scenario(
        "path-reachability",
        PATH_REACHABILITY_EGG,
        "(run 3)",
        3,
        &["edge", "path"],
        rows("edge", &[&[1, 2], &[2, 3], &[3, 4]]),
        vec![
            RuleSpec {
                name: "edge-to-path".to_string(),
                body: vec![atom("edge", &[var("x"), var("y")])],
                head: atom("path", &[var("x"), var("y")]),
            },
            RuleSpec {
                name: "path-step".to_string(),
                body: vec![
                    atom("path", &[var("x"), var("y")]),
                    atom("edge", &[var("y"), var("z")]),
                ],
                head: atom("path", &[var("x"), var("z")]),
            },
        ],
    )
}

```

Repeated-variable scenario.

The input contains `(1,1)`, `(1,2)`, and `(2,2)`. The atom `pair(x, x)`
should keep only the rows whose two columns are equal, so the expected
`same` rows are `(1)` and `(2)`.

```rust
pub fn repeated_variable_scenario() -> TrialResult<ScenarioSpec> {
    scenario(
        "repeated-variable",
        REPEATED_VARIABLE_EGG,
        "(run 1)",
        1,
        &["pair", "same"],
        rows("pair", &[&[1, 1], &[1, 2], &[2, 2]]),
        vec![RuleSpec {
            name: "diagonal".to_string(),
            body: vec![atom("pair", &[var("x"), var("x")])],
            head: atom("same", &[var("x")]),
        }],
    )
}

```

Non-recursive three-way join scenario.

Follow the shared variables from left to right: `a(x,y)` joins `b(y,z)`,
then `c(z,w)`, and the rule outputs `out(x,w)`. The expected outputs are
`(1,5)`, `(1,6)`, and `(9,5)`.

```rust
pub fn three_way_join_scenario() -> TrialResult<ScenarioSpec> {
    let mut facts = rows("a", &[&[1, 2], &[9, 9]]);
    facts.extend(rows("b", &[&[2, 3], &[2, 4], &[9, 3]]));
    facts.extend(rows("c", &[&[3, 5], &[4, 6], &[8, 8]]));

    scenario(
        "three-way-join",
        THREE_WAY_JOIN_EGG,
        "(run 1)",
        1,
        &["a", "b", "c", "out"],
        facts,
        vec![RuleSpec {
            name: "abc-to-out".to_string(),
            body: vec![
                atom("a", &[var("x"), var("y")]),
                atom("b", &[var("y"), var("z")]),
                atom("c", &[var("z"), var("w")]),
            ],
            head: atom("out", &[var("x"), var("w")]),
        }],
    )
}

```

## Trial execution and oracle comparison

Run the full tutorial check.

This is the quickest confidence step: it runs all three scenarios and
returns one report. A successful run has `all_match_oracle: true`. If one
scenario fails, keep the report: it contains both the native oracle rows and
the DD rows needed to see where they diverged.

```rust
pub fn run_acceptance_trial() -> TrialResult<TrialReport> {
    let specs = acceptance_scenarios()?;
    let scenarios = specs
        .iter()
        .map(run_scenario_trial)
        .collect::<TrialResult<Vec<_>>>()?;
    let all_match_oracle = scenarios.iter().all(|scenario| scenario.matches_oracle);

    Ok(TrialReport {
        scenarios,
        all_match_oracle,
        observations: vec![
            "oracle rows were exported with EGraph::function_for_each, not print-function"
                .to_string(),
            "DD results are netted from signed diffs before logical-row comparison".to_string(),
            "relation identity projects i64 input columns and keeps raw lower-row output ids as debug evidence".to_string(),
        ],
        limitations: vec![
            "this acceptance gate is relation-only over i64 values".to_string(),
            "equality/rebuild, containers, custom schedulers, primitives, extraction, proofs, and direct ResolvedCoreRule export remain follow-up probes".to_string(),
            "performance measurement is intentionally deferred until semantic acceptance is stable".to_string(),
        ],
    })
}

```

Run one scenario through native egglog and the DD evaluator, then compare.

This is the main semantic gate: native egglog provides lower-row oracle
snapshots, DD computes relation rows, and both sides are projected to sorted
logical `i64` rows for exact equality.

```rust
pub fn run_scenario_trial(spec: &ScenarioSpec) -> TrialResult<ScenarioReport> {
```

Each scenario is run twice: native egglog exports lower table rows as the
oracle, while DD evaluates the small relation-only model.

```rust
    let oracle_snapshots = run_native_oracle(spec)?;
    let final_snapshot = oracle_snapshots
        .last()
        .ok_or_else(|| format!("scenario {} produced no final snapshot", spec.name))?;
    let oracle_final_rows = spec
        .observed_functions
        .iter()
        .map(|function| Ok((function.clone(), logical_rows(final_snapshot, function)?)))
        .collect::<TrialResult<BTreeMap<_, _>>>()?;
    let dd_rows = dd_evaluate_scenario(spec)?;
    let mut dd_grouped = BTreeMap::<String, Vec<LogicalRow>>::new();
    for row in dd_rows {
        dd_grouped
            .entry(row.relation.clone())
            .or_default()
            .push(LogicalRow {
                function: row.relation,
                values: row.values,
            });
    }
    for rows in dd_grouped.values_mut() {
        rows.sort();
    }
    let dd_logical_rows = spec
        .observed_functions
        .iter()
        .map(|function| {
            (
                function.clone(),
                dd_grouped.remove(function).unwrap_or_default(),
            )
        })
        .collect();
    let matches_oracle = dd_logical_rows == oracle_final_rows;

    Ok(ScenarioReport {
        scenario: spec.name.clone(),
        oracle_snapshots,
        dd_logical_rows,
        oracle_final_rows,
        matches_oracle,
    })
}

```

## Differential Dataflow evaluator

This is the main DD-side path. Read it with `path-reachability` in mind:
`edge` facts enter as base collections, `edge -> path` seeds direct paths,
`path(x,y), edge(y,z) -> path(x,z)` runs in the recursive loop, and captured
signed updates are netted into the six final `path` rows.

Evaluate one scenario with the DD model.

Watch for three transitions:

- facts become one DD collection per relation;
- rules add derived rows until recursive relations stop changing;
- captured signed updates are netted into the final visible row set.

The final `BTreeSet<RelationRow>` is what we compare with native egglog.

```rust
pub fn dd_evaluate_scenario(spec: &ScenarioSpec) -> TrialResult<BTreeSet<RelationRow>> {
```

Planning happens before the Timely dataflow is built. That keeps the
closure that constructs operators focused on data movement, not string
lookup or scenario validation.

```rust
    let planned_rules = compile_rules(&spec.rules)?;
    let relation_names = relation_names(spec);
    if relation_names.is_empty() {
        return Ok(BTreeSet::new());
    }

    let facts_by_relation = fact_values_by_relation(&spec.facts);

```

`timely::example` creates one worker, builds this dataflow, runs it to
completion, and returns the captured updates. For this tutorial, that
gives us one finished result to compare with the oracle.

```rust
    let captured = timely::example(move |scope| {
        let mut base_by_relation = BTreeMap::new();
        for relation in &relation_names {
            let facts = facts_by_relation.get(relation).cloned().unwrap_or_default();

```

`new_collection_from` converts an ordinary Rust iterator into an
initial DD collection at time zero with positive unit diffs. The
handle is ignored because these fixtures are static; dynamic
update experiments would keep it, advance time, and feed signed
changes across epochs.

```rust
            let (_input, collection) = scope.new_collection_from(facts);
            base_by_relation.insert(relation.clone(), collection);
        }

```

The evaluator keeps one collection per relation until the end. That
shape mirrors how DD joins want to see data: relation-local streams
can be arranged by relation-specific keys and reused.

```rust
        let relation_rows = relation_rows_from_collections(relation_fixpoint(
            base_by_relation,
            relation_names,
            planned_rules,
        ))
        .unwrap_or_else(|| {
            let (_input, empty) = scope.new_collection_from(Vec::<RelationRow>::new());
            empty
        });

```

`consolidate` combines equal `(row, time)` updates before capture.
It is not the semantic comparison by itself; it just reduces the
amount of update traffic that leaves the dataflow. `.inner` exposes
the underlying Timely stream of DD update batches. Capture gives the
host the raw signed update batches; the semantic row set is computed
only below by summing diffs per row.

```rust
        relation_rows.consolidate().inner.capture()
    });

```

Capture gives us signed DD updates, not final rows. Sum the diffs first.
A row is visible only when its net diff is positive. This is the key
check: the tutorial compares final relation contents, not raw update
events.

```rust
    let mut diffs = BTreeMap::<RelationRow, isize>::new();
    for (_capture_time, batch) in captured.extract() {
        for (row, _data_time, diff) in batch {
            *diffs.entry(row).or_insert(0) += diff;
        }
    }

    Ok(diffs
        .into_iter()
        .filter_map(|(row, diff)| (diff > 0).then_some(row))
        .collect())
}

```

## Timely iteration and relation-local fixpoints

This is the recursive part of the evaluator. Open the implementation when
you want to see exactly how relation-local collections enter Timely's loop,
how DD feeds the recursive relation back, and where `distinct` turns signed
updates into set semantics.

<details>
<summary>Implementation: relation fixpoint and recursive feedback</summary>

Decide which rules need the DD feedback loop.

In the reachability scenario, `path` depends on itself through
`path(x,y), edge(y,z) -> path(x,z)`, so `path` goes into the loop.
Non-recursive relations stay outside the loop and are used as stable inputs.

```rust
fn relation_fixpoint<'scope, T>(
    base_by_relation: BTreeMap<String, ValueCollection<'scope, T>>,
    relation_names: Vec<String>,
    rules: Vec<PlannedRule>,
) -> BTreeMap<String, ValueCollection<'scope, T>>
where
    T: Timestamp + Lattice + Ord + 'static,
    T::Summary: Default + Clone,
{
```

For the reachability scenario, this finds that `path` is recursive. That
is the relation that must keep feeding newly discovered rows back through
the rules until no new `path` rows appear.

```rust
    let recursive_relations = recursive_relations(&rules);
    let mut pre_rules = Vec::new();
    let mut recursive_rules = Vec::new();
    let mut post_rules = Vec::new();

    for rule in rules.iter().cloned() {
        if recursive_relations.contains(&rule.head.relation) {
            recursive_rules.push(rule);
        } else if rule_uses_any_relation(&rule, &recursive_relations) {
            post_rules.push(rule);
        } else {
            pre_rules.push(rule);
        }
    }

```

Rules whose heads and bodies are outside the cyclic relation set run
first. Rules producing cyclic relations run inside the loop, including
nonrecursive seed rules such as `edge -> path`. Finally, rules that
depend on recursive outputs but do not themselves feed a cycle are
saturated once the recursive fixed point is available.

```rust
    let mut relations = apply_nonrecursive_rules(base_by_relation, &pre_rules);
    if !recursive_relations.is_empty() {
        let recursive_outputs = recursive_relation_fixpoint(
            relations.clone(),
            relation_names,
            recursive_relations,
            recursive_rules,
        );
        for (relation, collection) in recursive_outputs {
            relations.insert(relation, collection);
        }
    }

    apply_nonrecursive_rules(relations, &post_rules)
}

```

Run the recursive part of the rule set in one Timely iterative scope.

For the reachability scenario, this is the "keep going until no new paths
appear" step. Timely supplies the loop structure; DD's [`Variable`] is the
feedback point that carries newly discovered rows into the next loop round.
The product timestamp detail below is implementation plumbing for that
loop: the outer time is the input epoch, and the inner `u64` counts loop
rounds.

```rust
fn recursive_relation_fixpoint<'scope, T>(
    base_by_relation: BTreeMap<String, ValueCollection<'scope, T>>,
    relation_names: Vec<String>,
    recursive_relations: BTreeSet<String>,
    rules: Vec<PlannedRule>,
) -> BTreeMap<String, ValueCollection<'scope, T>>
where
    T: Timestamp + Lattice + Ord + 'static,
    T::Summary: Default + Clone,
{
    let Some(outer) = base_by_relation
        .values()
        .next()
        .map(|collection| collection.scope())
    else {
        return BTreeMap::new();
    };

    let mut rules_by_head = BTreeMap::<String, Vec<PlannedRule>>::new();
    for rule in rules.iter().cloned() {
        rules_by_head
            .entry(rule.head.relation.clone())
            .or_default()
            .push(rule);
    }

```

`iterative` opens a nested Timely scope. Collections from the outer scope
must be `enter`ed to participate in the loop, and loop results must
`leave` back to the outer scope. The type parameter `u64` is the loop
timestamp coordinate.

```rust
    outer.clone().iterative::<u64, _, _>(move |nested| {
```

The feedback summary advances only the inner loop coordinate by one;
the outer timestamp stays at the input epoch.

```rust
        let summary = Product::new(Default::default(), 1);
        let mut variables = Vec::new();
        let mut current_by_relation = BTreeMap::new();

        for relation in &relation_names {
            if let Some(base) = base_by_relation.get(relation) {
                if recursive_relations.contains(relation) {
```

`Variable::new_from` exposes a collection named
`current` whose logical contents are base rows plus the
current feedback rows. When we later call `set(next)`,
DD subtracts the original base before feeding rows back,
so the loop sends only the incremental correction. That
is why `next` is written as the whole desired relation,
not just the newly derived rows.

```rust
                    let (variable, current) =
                        Variable::new_from(base.clone().enter(nested), summary.clone());
                    variables.push((relation.clone(), variable));
                    current_by_relation.insert(relation.clone(), current);
                } else {
```

Nonrecursive relations are stable facts from the loop's
perspective. They can still be arranged and joined
inside the loop, but no feedback variable is needed.

```rust
                    current_by_relation.insert(relation.clone(), base.clone().enter(nested));
                }
            }
        }

```

Build arrangements over the current loop collections. For recursive
relations these arrangements are maintained as the loop discovers
new rows; for stable relations they are ordinary indexed inputs.

```rust
        let arrangements = relation_arrangements(&current_by_relation, &rules);
        let mut next_inner = BTreeMap::new();
        let mut outputs = BTreeMap::new();
        for relation in &recursive_relations {
            let Some(base_outer) = base_by_relation.get(relation) else {
                continue;
            };
            let base = base_outer.clone().enter(nested);
            let mut derived = base.clone().filter(|_| false);

            if let Some(rules) = rules_by_head.get(relation) {
                for rule in rules.iter().cloned() {
                    if let Some(rule_rows) =
                        apply_planned_rule(&current_by_relation, &arrangements, rule)
                    {
                        derived = derived.concat(rule_rows);
                    }
                }
            }

```

`concat` preserves signed multiplicities from base and derived
rows. `distinct` is the Datalog set-semantics boundary: DD treats
a tuple with non-zero accumulated weight as one occurrence. It
also provides the consolidation boundary required for a DD loop
to stop circulating cancelable differences.

```rust
            let next = base.concat(derived).distinct();
            outputs.insert(relation.clone(), next.clone().leave(outer.clone()));
            next_inner.insert(relation.clone(), next);
        }

        for (relation, variable) in variables {
            if let Some(next) = next_inner.remove(&relation) {
```

Binding the variable connects the loop feedback edge. The
variable is consumed here so each recursive relation has
exactly one definition inside this iterative scope.

```rust
                variable.set(next);
            }
        }

        outputs
    })
}

```

Saturate acyclic rules in the current scope.

These rules do not need DD feedback. Repeating the small rule set is enough
for this tutorial's acyclic chains, then the result can feed the recursive
or final comparison steps.

```rust
fn apply_nonrecursive_rules<'scope, T>(
    mut relations: BTreeMap<String, ValueCollection<'scope, T>>,
    rules: &[PlannedRule],
) -> BTreeMap<String, ValueCollection<'scope, T>>
where
    T: Timestamp + Lattice + Ord + 'static,
{
    for _ in 0..rules.len() {
        let arrangements = relation_arrangements(&relations, rules);
        for rule in rules.iter().cloned() {
            let head = rule.head.relation.clone();
            let Some(current) = relations.get(&head).cloned() else {
                continue;
            };
            if let Some(rule_rows) = apply_planned_rule(&relations, &arrangements, rule) {
                relations.insert(head, current.concat(rule_rows).distinct());
            }
        }
    }
    relations
}

```

</details>

## Arranged rule evaluation

This section evaluates one rule body as relation-local bindings and arranged
joins. Open it when you want the column-level mechanics behind repeated
variables, shared-variable join keys, and head projection.

<details>
<summary>Implementation: arranged rule evaluation and atom matching</summary>

Evaluate one rule body as a sequence of joins.

Follow the `path-step` rule as the main example: first bind `path(x, y)`,
then join `edge(y, z)` on the shared `y`, then project the completed binding
into `path(x, z)`.

```rust
fn apply_planned_rule<'scope, T>(
    relations: &BTreeMap<String, ValueCollection<'scope, T>>,
    arrangements: &BTreeMap<ArrangementKey, RelationArrangement<'scope, T>>,
    rule: PlannedRule,
) -> Option<ValueCollection<'scope, T>>
where
    T: Timestamp + Lattice + Ord + 'static,
{
    let (first, rest) = rule.body.split_first()?;

```

The first atom seeds the binding stream. At this point there is no join:
matching one relation tuple either fails atom-local filters
(constants/repeated variables) or produces one partial assignment.

```rust
    let mut bindings = relation_bindings(relations, first, rule.var_count)?;
    let mut known_vars = first.vars.clone();

    for (index, atom) in rest.iter().cloned().enumerate() {
```

A natural join is an equality join over variables that appear on
both sides. Because variables have already been lowered to `VarId`,
the join key is just a vector of bound values in a stable order.

```rust
        let shared = known_vars
            .intersection(&atom.vars)
            .copied()
            .collect::<Vec<_>>();
```

If `shared` is empty, both sides use the empty vector key. That is
the deliberate Cartesian-product case for atoms with no variables in
common.

```rust
        let shared_for_left = shared.clone();

```

The right side is a relation-local arrangement keyed by the columns
where those shared variables occur. If the rule fragment says
`path(x, y), edge(y, z)`, then the `edge` arrangement is keyed by its
first column because that is where `y` appears.

```rust
        let key_columns = atom_key_columns(&atom, &shared)?;
        let relation_arrangement = arrangements
            .get(&(atom.relation.clone(), key_columns))
            .cloned()?;
        let atom_for_join = atom.clone();

```

The left side is rule-local state: partial bindings from all prior
atoms. It is arranged on the same shared-variable values so
`join_core` can line up matching batches by key.

```rust
        let left_arrangement = bindings
            .flat_map(move |binding| Some((binding.key(&shared_for_left)?, binding)))
            .arrange_by_key_named(&format!("Arrange {} left {}", rule.name, index));

```

`join_core` is the low-level arranged join hook. For every matching
key, DD multiplies the signed differences from left and right. The
closure only describes the output record: merge the right relation
row into the left partial binding, dropping the pair if it violates a
constant or repeated-variable equality.

```rust
        bindings = left_arrangement.join_core(relation_arrangement, move |_key, left, row| {
            left.merge_atom_row(row, &atom_for_join)
        });
        known_vars.extend(atom.vars);
    }

```

Once all body atoms have joined, projecting the head turns bindings back
into relation tuples. The caller attaches the tuple to the head relation
and applies `distinct` at the relation boundary.

```rust
    let head = rule.head;
    Some(bindings.flat_map(move |binding| binding.project(&head)))
}

```

Build reusable relation/key arrangements required by the planned joins.

The registry is keyed by `(relation, key_columns)`, so two rules that probe
the same relation on the same column set share one maintained DD
arrangement. Intermediate binding streams are still arranged at each join
point because their schema is rule-local.

```rust
fn relation_arrangements<'scope, T>(
    relations: &BTreeMap<String, ValueCollection<'scope, T>>,
    rules: &[PlannedRule],
) -> BTreeMap<ArrangementKey, RelationArrangement<'scope, T>>
where
    T: Timestamp + Lattice + Ord + 'static,
{
    let mut required = BTreeSet::<ArrangementKey>::new();
    for rule in rules {
        let Some((first, rest)) = rule.body.split_first() else {
            continue;
        };
        let mut known_vars = first.vars.clone();
        for atom in rest {
```

Only atoms after the first need pre-built right-side
arrangements. The first atom creates bindings directly from its
relation; every later atom joins against variables already known
from the prefix of the body.

```rust
            let shared = known_vars
                .intersection(&atom.vars)
                .copied()
                .collect::<Vec<_>>();
            if let Some(columns) = atom_key_columns(atom, &shared) {
                required.insert((atom.relation.clone(), columns));
            }
            known_vars.extend(atom.vars.iter().copied());
        }
    }

    let mut arrangements = BTreeMap::new();
    for (relation, columns) in required {
        if let Some(collection) = relations.get(&relation) {
            arrangements.insert(
                (relation.clone(), columns.clone()),
                arrange_relation_by_columns(collection.clone(), &relation, columns),
            );
        }
    }
    arrangements
}

```

Arrange one relation collection by a selected list of tuple columns.

The arranged key is a `Vec<i64>` containing the selected columns; the value
is the whole tuple, because later projection may need columns not present in
the key. For well-formed scenario rows, the arrangement preserves the
relation tuples while adding a maintained key. This is the first
performance-aware shape in the trial: repeated joins can reuse the same
maintained index instead of scanning a mixed row stream. Atom-local filters
still happen when tuples are converted into bindings or merged into existing
bindings.

```rust
fn arrange_relation_by_columns<'scope, T>(
    collection: ValueCollection<'scope, T>,
    relation: &str,
    columns: Vec<usize>,
) -> RelationArrangement<'scope, T>
where
    T: Timestamp + Lattice + Ord + 'static,
{
    let columns_for_rows = columns.clone();
    collection
        .flat_map(move |values| {
            let mut key = Vec::with_capacity(columns_for_rows.len());
            for column in &columns_for_rows {
                key.push(*values.get(*column)?);
            }
            Some((key, values))
        })
        .arrange_by_key_named(&format!("Arrange relation {relation} by {columns:?}"))
}

```

Locate the tuple columns that should form an atom's join key.

The caller supplies shared variables in a stable `VarId` order. Returning
columns in that same order makes left binding keys and right relation keys
comparable even when a relation stores variables in a different column
order.

```rust
fn atom_key_columns(atom: &PlannedAtom, shared: &[VarId]) -> Option<Vec<usize>> {
    shared
        .iter()
        .map(|var| {
            atom.terms
                .iter()
                .position(|term| matches!(term, PlannedTerm::Var(candidate) if candidate == var))
        })
        .collect()
}

```

Convert rows from one relation into compiled variable bindings.

This is the relation-local equivalent of selecting rows for one atom. It
does not scan a mixed collection; the caller already chose the relation's
collection, so this operator only applies column-local constraints.

```rust
fn relation_bindings<'scope, T>(
    relations: &BTreeMap<String, ValueCollection<'scope, T>>,
    atom: &PlannedAtom,
    var_count: usize,
) -> Option<BindingCollection<'scope, T>>
where
    T: Timestamp + Lattice + Ord + 'static,
{
    let collection = relations.get(&atom.relation)?.clone();
    let atom = atom.clone();
    Some(collection.flat_map(move |values| match_planned_atom(values, &atom, var_count)))
}

```

Match one relation-local tuple against one compiled atom.

Constants are filters. Repeated variables are equality filters: binding the
same `VarId` twice succeeds only if the observed values agree.

```rust
fn match_planned_atom(
    values: Vec<i64>,
    atom: &PlannedAtom,
    var_count: usize,
) -> Option<BindingRow> {
    if values.len() != atom.terms.len() {
        return None;
    }

    let mut binding = BindingRow::empty(var_count);
    for (term, value) in atom.terms.iter().zip(values) {
        match term {
            PlannedTerm::Var(var) => binding.bind(*var, value)?,
            PlannedTerm::Const(expected) => {
                if *expected != value {
                    return None;
                }
            }
        }
    }

    Some(binding)
}

```

</details>

## Regression tests

End the tutorial here. These tests are the compact executable checkpoints for
the row sets named above. If the JSON report was too large to inspect by
hand, run:

```text
cargo test --manifest-path code/minimal-dd-trial/Cargo.toml --lib
```

The important checks are the same as the walkthrough: the path scenario
derives six rows, repeated-variable matching keeps `same(1)` and `same(2)`,
the three-way join derives three `out` rows, and the aggregate report says
all scenarios match the native oracle. Reference appendices follow the
tests for readers who want implementation details.

<details>
<summary>Regression test code</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn expected_rows(function: &str, values: &[&[i64]]) -> Vec<LogicalRow> {
        values
            .iter()
            .map(|values| LogicalRow {
                function: function.to_string(),
                values: values.to_vec(),
            })
            .collect()
    }

    fn scenario_report(name: &str) -> TrialResult<ScenarioReport> {
        acceptance_scenarios()?
            .iter()
            .find(|scenario| scenario.name == name)
            .ok_or_else(|| format!("missing scenario {name}").into())
            .and_then(run_scenario_trial)
    }

    #[test]
    fn oracle_lower_rows_path() -> TrialResult<()> {
        let snapshot = run_fixture_program(PATH_REACHABILITY_EGG, &names(&["edge", "path"]))?;

        assert_eq!(
            logical_rows(&snapshot, "path")?,
            expected_rows(
                "path",
                &[&[1, 2], &[1, 3], &[1, 4], &[2, 3], &[2, 4], &[3, 4]]
            )
        );
        let path_rows = snapshot.rows.get("path").expect("path rows");
        assert!(path_rows
            .iter()
            .all(|row| row.input_sorts == ["i64", "i64"]));
        assert!(path_rows
            .iter()
            .all(|row| row.raw_values.len() == row.input_sorts.len() + 1));

        Ok(())
    }

    #[test]
    fn staged_oracle_snapshots() -> TrialResult<()> {
        let snapshots = run_native_oracle(&path_scenario()?)?;
        let counts = snapshots
            .iter()
            .map(|snapshot| logical_rows(snapshot, "path").map(|rows| rows.len()))
            .collect::<TrialResult<Vec<_>>>()?;

        assert_eq!(counts.first().copied(), Some(0));
        assert_eq!(counts.last().copied(), Some(6));
        assert!(counts.windows(2).all(|window| window[0] <= window[1]));

        Ok(())
    }

    #[test]
    fn generic_dd_path_reachability_matches_oracle() -> TrialResult<()> {
        let report = scenario_report("path-reachability")?;

        assert!(report.matches_oracle);
        assert_eq!(
            report
                .dd_logical_rows
                .get("path")
                .cloned()
                .unwrap_or_default(),
            expected_rows(
                "path",
                &[&[1, 2], &[1, 3], &[1, 4], &[2, 3], &[2, 4], &[3, 4]]
            )
        );

        Ok(())
    }

    #[test]
    fn generic_dd_repeated_variable_matches_oracle() -> TrialResult<()> {
        let report = scenario_report("repeated-variable")?;

        assert!(report.matches_oracle);
        assert_eq!(
            report
                .dd_logical_rows
                .get("same")
                .cloned()
                .unwrap_or_default(),
            expected_rows("same", &[&[1], &[2]])
        );

        Ok(())
    }

    #[test]
    fn generic_dd_three_way_join_matches_oracle() -> TrialResult<()> {
        let report = scenario_report("three-way-join")?;

        assert!(report.matches_oracle);
        assert_eq!(
            report
                .dd_logical_rows
                .get("out")
                .cloned()
                .unwrap_or_default(),
            expected_rows("out", &[&[1, 5], &[1, 6], &[9, 5]])
        );

        Ok(())
    }

    #[test]
    fn all_acceptance_scenarios_match_oracle() -> TrialResult<()> {
        let report = run_acceptance_trial()?;

        assert!(report.all_match_oracle);
        assert_eq!(report.scenarios.len(), 3);
        assert!(report
            .scenarios
            .iter()
            .all(|scenario| scenario.matches_oracle));

        Ok(())
    }
}

```

</details>

## Reference appendices

The tutorial path above is enough for a first pass. The remaining sections are
expandable reference material for the exact data model, planning metadata,
oracle export, compilation helpers, binding operations, and construction
helpers.

## Public scenario and report data model

These types name the JSON report fields from the tutorial path: scenario
inputs, native egglog snapshots, DD rows, and the final match flag. Keep
this appendix collapsed unless you need the exact report schema.

<details>
<summary>Appendix: scenario and report data types</summary>

Logical relation tuple used by the DD model.

This is the DD-side row identity for the first gate: a relation name plus
the logical `i64` input tuple. It intentionally does not include egglog's
lower-row output/eclass id, because those ids are oracle debug evidence
rather than the relation payload being compared.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct RelationRow {
    pub relation: String,
    pub values: Vec<i64>,
}

```

Term in a relation atom.

Constants are supported by the matcher because they naturally fall out of
atom filtering, although the first acceptance scenarios only need variables.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum AtomTerm {
    Var(String),
    Const(i64),
}

```

A body or head atom in the small relation-rule language.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Atom {
    pub relation: String,
    pub terms: Vec<AtomTerm>,
}

```

One Datalog-shaped rule for the DD trial evaluator.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct RuleSpec {
    pub name: String,
    pub body: Vec<Atom>,
    pub head: Atom,
}

```

Paired native-oracle and DD-model description for one scenario.

`stages` are fed to native egglog for oracle snapshots. `facts` and `rules`
are the manually inspected DD model of the same relation-only program.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScenarioSpec {
    pub name: String,
    pub observed_functions: Vec<String>,
```

Native egglog stages are the oracle program. The DD side intentionally
uses the hand-written facts/rules below so the mapping stays inspectable.

```rust
    pub stages: Vec<ScenarioStage>,
    pub facts: Vec<RelationRow>,
    pub rules: Vec<RuleSpec>,
}

```

One native egglog execution step.

Staging lets the oracle expose intermediate lower-row snapshots while the DD
side currently computes the final relation closure in one dataflow.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScenarioStage {
    pub label: String,
    pub program: String,
}

```

Raw lower function-table row exported from native egglog.

These rows preserve the details needed to debug the oracle boundary: schema
sort names, raw lower values, decoded inputs/output where possible, and the
`subsumed` bit. Logical comparison is derived from these rows but does not
discard them from the report.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct LowerRow {
    pub function: String,
    pub input_sorts: Vec<String>,
    pub output_sort: String,
    pub raw_values: Vec<String>,
    pub input_values: Vec<DecodedValue>,
    pub output_value: DecodedValue,
    pub subsumed: bool,
}

```

Decoded lower-row value.

The first gate only projects `i64` input columns into `LogicalRow`; other
values remain raw so non-`i64` evidence is still visible in reports.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DecodedValue {
    I64 { value: i64 },
    Raw { sort: String, value: String },
}

```

Comparable logical row projected from native lower rows or DD rows.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct LogicalRow {
    pub function: String,
    pub values: Vec<i64>,
}

```

Native egglog oracle snapshot after one staged execution step.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OracleSnapshot {
    pub stage: String,
    pub rows: BTreeMap<String, Vec<LowerRow>>,
    pub logical_rows: BTreeMap<String, Vec<LogicalRow>>,
}

```

Per-scenario result comparing native lower rows with DD logical rows.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScenarioReport {
    pub scenario: String,
    pub oracle_snapshots: Vec<OracleSnapshot>,
    pub dd_logical_rows: BTreeMap<String, Vec<LogicalRow>>,
    pub oracle_final_rows: BTreeMap<String, Vec<LogicalRow>>,
    pub matches_oracle: bool,
}

```

Aggregate acceptance report emitted by the CLI and tests.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrialReport {
    pub scenarios: Vec<ScenarioReport>,
    pub all_match_oracle: bool,
    pub observations: Vec<String>,
    pub limitations: Vec<String>,
}

```

</details>

## DD names used by the evaluator

This is a glossary-in-code, not the main path. Read the short comments here
only to translate later code: rows become DD collections, joins use
arrangements, and rule matches are carried as compact variable bindings.

<details>
<summary>Appendix: DD evaluator aliases and planning types</summary>

Relation-local DD collection used by the evaluator.

A `VecCollection<'scope, T, D, R>` is Differential Dataflow's multiset-like
collection abstraction in a Timely scope. Here `D = Vec<i64>` is one logical
tuple for a single relation and `R = isize` is the signed difference type:
`+1` means an occurrence arrives, `-1` means it is retracted, and larger
magnitudes are possible after aggregation.

```rust
type ValueCollection<'scope, T> = VecCollection<'scope, T, Vec<i64>, isize>;

```

DD collection after relation-local values are reattached to relation names.

Internally we avoid a mixed row collection while planning rules, because
scanning all rows for every atom hides the relation/key structure that DD is
good at exploiting. We only rebuild mixed `RelationRow`s at the output
boundary so reporting and oracle comparison stay simple.

```rust
type RowCollection<'scope, T> = VecCollection<'scope, T, RelationRow, isize>;

```

DD collection of partial rule bindings.

During a rule join, each record is a compact vector indexed by compiled
variable id. This replaces string-keyed maps in the hot path and makes join
keys explicit column projections.

```rust
type BindingCollection<'scope, T> = VecCollection<'scope, T, BindingRow, isize>;

```

Registry key for a maintained arrangement: relation name plus key columns.

Arrangements are DD's reusable indexed representation of a collection. The
same relation can be arranged several ways if different rules probe it by
different shared-variable columns.

```rust
type ArrangementKey = (String, Vec<usize>);

```

Concrete arrangement type for `Vec<i64>` relation tuples.

This alias is noisy because it exposes DD's lower trace implementation. The
useful idea is simpler: an `Arranged` collection is logically the same data
as the input relation, but maintained as keyed batches/traces so joins can
reuse indexed state instead of rebuilding an index at every probe.

```rust
type RelationArrangement<'scope, T> = differential_dataflow::operators::arrange::Arranged<
    'scope,
    differential_dataflow::operators::arrange::TraceAgent<
        differential_dataflow::trace::implementations::ValSpine<Vec<i64>, Vec<i64>, T, isize>,
    >,
>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct VarId(usize);

```

Compiled atom term.

Public scenarios use variable names for readability. The DD operators use
`VarId`s so a binding can be a vector lookup instead of a map lookup.

```rust
#[derive(Clone, Debug)]
enum PlannedTerm {
    Var(VarId),
    Const(i64),
}

```

Body or head atom after lowering names to relation metadata.

`vars` is cached because each join step needs to know which variables are
already bound on the left and which variables an atom can contribute.

```rust
#[derive(Clone, Debug)]
struct PlannedAtom {
    relation: String,
    terms: Vec<PlannedTerm>,
    vars: BTreeSet<VarId>,
}

```

Rule after the once-per-scenario compile pass.

`var_count` fixes the width of each [`BindingRow`]. Body atoms can introduce
variables; head atoms may only project variables already bound by the body.

```rust
#[derive(Clone, Debug)]
struct PlannedRule {
    name: String,
    body: Vec<PlannedAtom>,
    head: PlannedAtom,
    var_count: usize,
}

```

Compact partial assignment carried through DD joins.

The vector position is a [`VarId`]. `None` means this variable has not been
bound by the atoms processed so far; `Some(value)` means every occurrence of
that variable must agree with `value`.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
struct BindingRow {
    values: Vec<Option<i64>>,
}

```

</details>

## Native oracle and report helpers

<details>
<summary>Appendix: native oracle and report helpers</summary>

Execute the staged native egglog oracle for a scenario.

All stages run on one `EGraph`, so later snapshots include effects from
earlier declarations, facts, and runs. This matches the explicit staged
boundary the MVP intends to compare against.

```rust
pub fn run_native_oracle(spec: &ScenarioSpec) -> TrialResult<Vec<OracleSnapshot>> {
    let mut egraph = EGraph::default();
    let mut snapshots = Vec::new();

    for stage in &spec.stages {
        egraph.parse_and_run_program(Some(stage.label.clone()), &stage.program)?;
        snapshots.push(snapshot_functions(
            &egraph,
            &stage.label,
            &spec.observed_functions,
        )?);
    }

    Ok(snapshots)
}

```

Run a complete `.egg` fixture and return one final lower-row snapshot.

Tests use this for the direct oracle preflight where no staged replay is
needed.

```rust
pub fn run_fixture_program(
    program: &str,
    observed_functions: &[String],
) -> TrialResult<OracleSnapshot> {
    let mut egraph = EGraph::default();
    egraph.parse_and_run_program(Some("fixture".to_string()), program)?;
    snapshot_functions(&egraph, "fixture-final", observed_functions)
}

```

Write a JSON acceptance report, creating the parent directory if needed.

```rust
pub fn write_report(path: impl AsRef<Path>, report: &TrialReport) -> TrialResult<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let body = serde_json::to_string_pretty(report)?;
    fs::write(path, format!("{body}\n"))?;
    Ok(())
}

```

Fetch projected logical rows for one function from an oracle snapshot.

```rust
pub fn logical_rows(snapshot: &OracleSnapshot, function: &str) -> TrialResult<Vec<LogicalRow>> {
    snapshot
        .logical_rows
        .get(function)
        .cloned()
        .ok_or_else(|| format!("missing logical rows for function {function}").into())
}

```

</details>

## Native egglog lower-row oracle

<details>
<summary>Appendix: lower-row oracle export</summary>

Export lower rows and projected logical rows for all observed functions.

```rust
fn snapshot_functions(
    egraph: &EGraph,
    stage: &str,
    functions: &[String],
) -> TrialResult<OracleSnapshot> {
    let mut rows = BTreeMap::new();
    let mut logical = BTreeMap::new();

    for function in functions {
```

This is the oracle boundary: read the public lower function table
rows, not rendered `print-function` / TermDag output. The schema tells
us how many leading `vals` entries are logical inputs; the next value
is the lower output/eclass id.

```rust
        let function_info = egraph
            .get_function(function)
            .ok_or_else(|| format!("missing function {function}"))?;
        let schema = function_info.schema();
        let input_sorts = schema
            .input
            .iter()
            .map(|sort| sort.name().to_string())
            .collect::<Vec<_>>();
        let output_sort = schema.output.name().to_string();
        let mut function_rows = Vec::new();

        egraph.function_for_each(function, |row| {
            let input_values = row
                .vals
                .iter()
                .take(input_sorts.len())
                .zip(input_sorts.iter())
                .map(|(value, sort)| decode_value(egraph, sort, *value))
                .collect::<Vec<_>>();
            let output_value = row
                .vals
                .get(input_sorts.len())
                .map(|value| decode_value(egraph, &output_sort, *value))
                .unwrap_or_else(|| DecodedValue::Raw {
                    sort: output_sort.clone(),
                    value: "<missing-output>".to_string(),
                });

            function_rows.push(LowerRow {
                function: function.to_string(),
                input_sorts: input_sorts.clone(),
                output_sort: output_sort.clone(),
                raw_values: row.vals.iter().map(raw_value).collect(),
                input_values,
                output_value,
                subsumed: row.subsumed,
            });
        })?;

        function_rows.sort();
        let mut logical_rows = function_rows
            .iter()
            .filter(|row| !row.subsumed)
            .filter_map(|row| {
                let mut values = Vec::with_capacity(row.input_values.len());
                for value in &row.input_values {
                    match value {
                        DecodedValue::I64 { value } => values.push(*value),
                        DecodedValue::Raw { .. } => return None,
                    }
                }
                Some(LogicalRow {
                    function: row.function.clone(),
                    values,
                })
            })
            .collect::<Vec<_>>();
        logical_rows.sort();
        rows.insert(function.clone(), function_rows);
        logical.insert(function.clone(), logical_rows);
    }

    Ok(OracleSnapshot {
        stage: stage.to_string(),
        rows,
        logical_rows: logical,
    })
}

```

Decode a lower egglog value using the sort name when this trial understands it.

```rust
fn decode_value(egraph: &EGraph, sort: &str, value: Value) -> DecodedValue {
    if sort == "i64" {
        DecodedValue::I64 {
            value: egraph.value_to_base::<i64>(value),
        }
    } else {
        DecodedValue::Raw {
            sort: sort.to_string(),
            value: raw_value(&value),
        }
    }
}

```

Stable debug rendering for lower values this trial does not decode.

```rust
fn raw_value(value: &Value) -> String {
    format!("{value:?}")
}

```

</details>

## Rule compilation

<details>
<summary>Appendix: rule compilation and dependency analysis</summary>

Compile string variables into stable numeric ids and validate head variables.

This compile step is tiny, but it is the line between readable scenario
specs and efficient DD operators. Anything that can be decided once per
scenario belongs here rather than inside `map`/`flat_map` closures.

```rust
fn compile_rules(rules: &[RuleSpec]) -> TrialResult<Vec<PlannedRule>> {
    rules.iter().map(compile_rule).collect()
}

```

Compile one rule from public scenario syntax into a DD execution plan.

```rust
fn compile_rule(rule: &RuleSpec) -> TrialResult<PlannedRule> {
    if rule.body.is_empty() {
        return Err(format!("rule {} has an empty body", rule.name).into());
    }

    let mut vars = BTreeMap::<String, VarId>::new();
    let mut body = Vec::with_capacity(rule.body.len());
    for atom in &rule.body {
        body.push(compile_body_atom(atom, &mut vars));
    }
    let head = compile_head_atom(&rule.head, &vars, &rule.name)?;

    Ok(PlannedRule {
        name: rule.name.clone(),
        body,
        head,
        var_count: vars.len(),
    })
}

```

Compile a body atom, assigning new variable ids as names are first seen.

```rust
fn compile_body_atom(atom: &Atom, vars: &mut BTreeMap<String, VarId>) -> PlannedAtom {
    let mut planned_terms = Vec::with_capacity(atom.terms.len());
    let mut atom_vars = BTreeSet::new();

    for term in &atom.terms {
        match term {
            AtomTerm::Var(name) => {
                let next = VarId(vars.len());
                let var = *vars.entry(name.clone()).or_insert(next);
                atom_vars.insert(var);
                planned_terms.push(PlannedTerm::Var(var));
            }
            AtomTerm::Const(value) => planned_terms.push(PlannedTerm::Const(*value)),
        }
    }

    PlannedAtom {
        relation: atom.relation.clone(),
        terms: planned_terms,
        vars: atom_vars,
    }
}

```

Compile a head atom and reject variables not bound by the body.

```rust
fn compile_head_atom(
    atom: &Atom,
    vars: &BTreeMap<String, VarId>,
    rule_name: &str,
) -> TrialResult<PlannedAtom> {
    let mut planned_terms = Vec::with_capacity(atom.terms.len());
    let mut atom_vars = BTreeSet::new();

    for term in &atom.terms {
        match term {
            AtomTerm::Var(name) => {
                let var = *vars
                    .get(name)
                    .ok_or_else(|| format!("rule {rule_name} head uses unbound variable {name}"))?;
                atom_vars.insert(var);
                planned_terms.push(PlannedTerm::Var(var));
            }
            AtomTerm::Const(value) => planned_terms.push(PlannedTerm::Const(*value)),
        }
    }

    Ok(PlannedAtom {
        relation: atom.relation.clone(),
        terms: planned_terms,
        vars: atom_vars,
    })
}

```

Return relation names whose rule dependencies reach themselves.

These are the relations that need DD feedback variables. A full planner
would compute strongly connected components; the trial only needs the set of
cyclic relation names so nonrecursive relations can remain outside the loop.

```rust
fn recursive_relations(rules: &[PlannedRule]) -> BTreeSet<String> {
    let mut graph = BTreeMap::<String, BTreeSet<String>>::new();
    for rule in rules {
        graph
            .entry(rule.head.relation.clone())
            .or_default()
            .extend(rule.body.iter().map(|atom| atom.relation.clone()));
    }

    graph
        .keys()
        .filter(|relation| relation_reaches_itself(relation, &graph))
        .cloned()
        .collect()
}

```

Depth-first reachability test in the relation-dependency graph.

```rust
fn relation_reaches_itself(relation: &str, graph: &BTreeMap<String, BTreeSet<String>>) -> bool {
    let mut stack = graph
        .get(relation)
        .into_iter()
        .flatten()
        .cloned()
        .collect::<Vec<_>>();
    let mut visited = BTreeSet::new();

    while let Some(next) = stack.pop() {
        if next == relation {
            return true;
        }
        if visited.insert(next.clone()) {
            if let Some(dependencies) = graph.get(&next) {
                stack.extend(dependencies.iter().cloned());
            }
        }
    }

    false
}

```

True when any body atom depends on one of the supplied relations.

```rust
fn rule_uses_any_relation(rule: &PlannedRule, relations: &BTreeSet<String>) -> bool {
    rule.body
        .iter()
        .any(|atom| relations.contains(&atom.relation))
}

```

</details>

## Binding operations

<details>
<summary>Appendix: binding row operations</summary>

```rust
impl BindingRow {
```

Create an all-unbound row with one slot per rule variable.

```rust
    fn empty(var_count: usize) -> Self {
        Self {
            values: vec![None; var_count],
        }
    }

```

Bind a variable or check that an existing binding agrees.

Returning `None` drops the DD record from the stream. In relational
terms, that is a failed selection predicate, not an exceptional case.

```rust
    fn bind(&mut self, var: VarId, value: i64) -> Option<()> {
        match self.values.get_mut(var.0)? {
            Some(existing) if *existing != value => None,
            Some(_) => Some(()),
            slot @ None => {
                *slot = Some(value);
                Some(())
            }
        }
    }

```

Project this binding into a join key for the requested variables.

```rust
    fn key(&self, vars: &[VarId]) -> Option<Vec<i64>> {
        vars.iter()
            .map(|var| self.values.get(var.0).copied().flatten())
            .collect()
    }

```

Merge one right-side atom tuple into an existing partial binding.

This is the post-key-check part of a natural join. The arrangement key
already guarantees the shared variables agree; this method also binds
newly introduced variables and checks constants/repeated variables that
were not fully represented in the join key.

```rust
    fn merge_atom_row(&self, row: &[i64], atom: &PlannedAtom) -> Option<Self> {
        if row.len() != atom.terms.len() {
            return None;
        }

        let mut merged = self.clone();
        for (term, value) in atom.terms.iter().zip(row.iter().copied()) {
            match term {
                PlannedTerm::Var(var) => merged.bind(*var, value)?,
                PlannedTerm::Const(expected) if *expected != value => return None,
                PlannedTerm::Const(_) => {}
            }
        }
        Some(merged)
    }

```

Project a complete binding into the rule head tuple.

A missing variable means the rule was ill-planned or the head referenced
a variable not produced by the body. The compile pass prevents that for
normal scenarios, so `None` here is a controlled dataflow drop.

```rust
    fn project(&self, head: &PlannedAtom) -> Option<Vec<i64>> {
        let mut values = Vec::with_capacity(head.terms.len());
        for term in &head.terms {
            match term {
                PlannedTerm::Var(var) => values.push(self.values.get(var.0).copied().flatten()?),
                PlannedTerm::Const(value) => values.push(*value),
            }
        }
        Some(values)
    }
}

```

</details>

## Relation inventory and report conversion

<details>
<summary>Appendix: relation inventory and report conversion</summary>

Collect every relation name that might need a DD collection.

```rust
fn relation_names(spec: &ScenarioSpec) -> Vec<String> {
    let mut names = BTreeSet::new();
    names.extend(spec.observed_functions.iter().cloned());
    names.extend(spec.facts.iter().map(|row| row.relation.clone()));
    for rule in &spec.rules {
        names.extend(rule.body.iter().map(|atom| atom.relation.clone()));
        names.insert(rule.head.relation.clone());
    }
    names.into_iter().collect()
}

```

Group initial facts by relation and drop the relation name from hot rows.

```rust
fn fact_values_by_relation(facts: &[RelationRow]) -> BTreeMap<String, Vec<Vec<i64>>> {
    let mut by_relation = BTreeMap::<String, Vec<Vec<i64>>>::new();
    for fact in facts {
        by_relation
            .entry(fact.relation.clone())
            .or_default()
            .push(fact.values.clone());
    }
    by_relation
}

```

Concatenate relation-local value collections into report-shaped rows.

This is intentionally at the edge of the DD program. Before this point, the
relation name is metadata that selects a collection/arrangement; after this
point, it is part of the report key for oracle comparison.

```rust
fn relation_rows_from_collections<'scope, T>(
    collections: BTreeMap<String, ValueCollection<'scope, T>>,
) -> Option<RowCollection<'scope, T>>
where
    T: Timestamp + Lattice + Ord + 'static,
{
    let mut iter = collections.into_iter();
    let (relation, collection) = iter.next()?;
    let mut rows = relation_values_to_rows(relation, collection);
    for (relation, collection) in iter {
        rows = rows.concat(relation_values_to_rows(relation, collection));
    }
    Some(rows)
}

```

Attach a relation name to every tuple in one value collection.

```rust
fn relation_values_to_rows<'scope, T>(
    relation: String,
    collection: ValueCollection<'scope, T>,
) -> RowCollection<'scope, T>
where
    T: Timestamp + Lattice + Ord + 'static,
{
    collection.map(move |values| RelationRow {
        relation: relation.clone(),
        values,
    })
}

```

</details>

## Scenario-construction helpers

<details>
<summary>Appendix: scenario-construction helpers</summary>

Construct a scenario from a native fixture and an explicit DD model.

The fixture remains the source of truth for native egglog execution. The
DD facts/rules are intentionally written next to the fixture reference so a
reader can audit the mapping instead of trusting a parser or bridge.

```rust
fn scenario(
    name: &str,
    fixture: &str,
    final_run_command: &str,
    run_steps: usize,
    observed_functions: &[&str],
    facts: Vec<RelationRow>,
    rules: Vec<RuleSpec>,
) -> TrialResult<ScenarioSpec> {
    let setup = fixture
        .trim_end()
        .strip_suffix(final_run_command)
        .ok_or_else(|| format!("fixture must end with {final_run_command:?}"))?
        .trim_end()
        .to_string();
    let mut stages = vec![ScenarioStage {
        label: "setup".to_string(),
        program: setup,
    }];

    for index in 1..=run_steps {
        stages.push(ScenarioStage {
            label: format!("run-{index}"),
            program: "(run 1)".to_string(),
        });
    }

    Ok(ScenarioSpec {
        name: name.to_string(),
        observed_functions: names(observed_functions),
        stages,
        facts,
        rules,
    })
}

fn rows(relation: &str, values: &[&[i64]]) -> Vec<RelationRow> {
    values
        .iter()
        .map(|values| RelationRow {
            relation: relation.to_string(),
            values: values.to_vec(),
        })
        .collect()
}

fn atom(relation: &str, terms: &[AtomTerm]) -> Atom {
    Atom {
        relation: relation.to_string(),
        terms: terms.to_vec(),
    }
}

fn var(name: &str) -> AtomTerm {
    AtomTerm::Var(name.to_string())
}

fn names(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

```

</details>
