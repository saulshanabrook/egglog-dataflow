# DD Design Spike Prototype

This throwaway prototype supports the archived
`findings/archive/2026-04-prior-backend-plans/dd-full-refactor-design-spike.md`.
It is not an egglog backend.

The prototype checks a narrow DD lifecycle question:

- install one compiled dataflow in a long-lived Timely worker;
- run an `A -> B` rule with `InputSession<u64, Row, isize>`, `row_ts`, a task
  stream, and a probe visibility gate;
- after that worker has already advanced, install a second compiled dataflow
  with new table inputs and run `B -> C`;
- feed derived rows host-side at the next logical epoch;
- read back a maintained arrangement with a cursor;
- set logical and physical compaction frontiers; and
- drop both installed dataflows with `Worker::drop_dataflow`.

Run:

```sh
cargo run --manifest-path code/dd-design-spike/Cargo.toml
```

Expected output:

```text
derived_b_rows=[Row { value: 101, row_ts: 2 }]
c_trace_contents={Row { value: 201, row_ts: 4 }: 1}
installed_after_drop=[]
```

This validates the minimal dynamic-install path. It does not validate rule
churn, multi-worker behavior, imported shared traces, recursive rules, or memory
retention under repeated install/drop.
