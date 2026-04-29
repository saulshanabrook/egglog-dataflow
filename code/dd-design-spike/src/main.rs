use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use differential_dataflow::input::Input;
use differential_dataflow::trace::cursor::Cursor;
use differential_dataflow::trace::TraceReader;
use serde::{Deserialize, Serialize};
use timely::dataflow::operators::probe::Handle as ProbeHandle;
use timely::dataflow::operators::Probe;
use timely::progress::frontier::AntichainRef;

type Epoch = u64;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
struct Row {
    value: u64,
    row_ts: Epoch,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
struct Task {
    rule_id: u64,
    last_run: Epoch,
    output_ts: Epoch,
    add: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
struct Candidate {
    rule_id: u64,
    output: u64,
    source_row_ts: Epoch,
    output_ts: Epoch,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
struct Event {
    candidate: Candidate,
    timely_time: Epoch,
    diff: isize,
}

fn collect_positive(events: &[Event], rule_id: u64) -> Vec<Row> {
    events
        .iter()
        .filter(|event| event.diff > 0 && event.candidate.rule_id == rule_id)
        .map(|event| Row {
            value: event.candidate.output,
            row_ts: event.candidate.output_ts,
        })
        .collect()
}

fn main() {
    let summaries = timely::execute(timely::Config::process(1), |worker| {
        let ab_events = Arc::new(Mutex::new(Vec::<Event>::new()));
        let mut ab_probe: ProbeHandle<Epoch> = ProbeHandle::new();
        let ab_events_for_dataflow = Arc::clone(&ab_events);
        let ab_dataflow = worker.next_dataflow_index();

        let (mut a_input, mut a_trace, mut ab_task_input) =
            worker.dataflow::<Epoch, _, _>(|scope| {
                let (a_input, a_rows) = scope.new_collection::<Row, isize>();
                let (task_input, tasks) = scope.new_collection::<Task, isize>();

                let a_arranged = a_rows.clone().arrange_by_self();
                let a_trace = a_arranged.trace.clone();
                a_arranged.stream.probe_with(&mut ab_probe);

                tasks
                    .map(|task| ((), task))
                    .join_map(a_rows.map(|row| ((), row)), |_unit, task, row| Candidate {
                        rule_id: task.rule_id,
                        output: row.value + task.add,
                        source_row_ts: row.row_ts,
                        output_ts: task.output_ts,
                    })
                    .filter(|candidate| candidate.source_row_ts >= 1)
                    .inspect_batch(move |time, data| {
                        let mut events = ab_events_for_dataflow.lock().expect("event log poisoned");
                        for (candidate, event_time, diff) in data {
                            events.push(Event {
                                candidate: *candidate,
                                timely_time: (*event_time).max(*time),
                                diff: *diff,
                            });
                        }
                    })
                    .probe_with(&mut ab_probe);

                (a_input, a_trace, task_input)
            });

        if worker.index() == 0 {
            a_input.update_at(
                Row {
                    value: 1,
                    row_ts: 1,
                },
                1,
                1,
            );
            ab_task_input.update_at(
                Task {
                    rule_id: 1,
                    last_run: 1,
                    output_ts: 2,
                    add: 100,
                },
                2,
                1,
            );
        }
        a_input.advance_to(3);
        a_input.flush();
        ab_task_input.advance_to(3);
        ab_task_input.flush();
        worker.step_while(|| ab_probe.less_equal(&2));

        let b_rows = collect_positive(&ab_events.lock().expect("event log poisoned"), 1);
        assert_eq!(
            b_rows,
            vec![Row {
                value: 101,
                row_ts: 2
            }]
        );

        a_trace.set_logical_compaction(AntichainRef::new(&[3]));
        a_trace.set_physical_compaction(AntichainRef::new(&[3]));

        let bc_events = Arc::new(Mutex::new(Vec::<Event>::new()));
        let mut bc_probe: ProbeHandle<Epoch> = ProbeHandle::new();
        let bc_events_for_dataflow = Arc::clone(&bc_events);
        let bc_dataflow = worker.next_dataflow_index();

        let (mut b_input, mut c_input, mut c_trace, mut bc_task_input) = worker
            .dataflow::<Epoch, _, _>(|scope| {
                let (b_input, b_rows) = scope.new_collection::<Row, isize>();
                let (c_input, c_rows) = scope.new_collection::<Row, isize>();
                let (task_input, tasks) = scope.new_collection::<Task, isize>();

                let c_arranged = c_rows.arrange_by_self();
                let c_trace = c_arranged.trace.clone();
                c_arranged.stream.probe_with(&mut bc_probe);

                tasks
                    .map(|task| ((), task))
                    .join_map(b_rows.map(|row| ((), row)), |_unit, task, row| Candidate {
                        rule_id: task.rule_id,
                        output: row.value + task.add,
                        source_row_ts: row.row_ts,
                        output_ts: task.output_ts,
                    })
                    .filter(|candidate| candidate.source_row_ts >= 2)
                    .inspect_batch(move |time, data| {
                        let mut events = bc_events_for_dataflow.lock().expect("event log poisoned");
                        for (candidate, event_time, diff) in data {
                            events.push(Event {
                                candidate: *candidate,
                                timely_time: (*event_time).max(*time),
                                diff: *diff,
                            });
                        }
                    })
                    .probe_with(&mut bc_probe);

                (b_input, c_input, c_trace, task_input)
            });

        if worker.index() == 0 {
            for row in &b_rows {
                b_input.update_at(*row, 3, 1);
            }
            bc_task_input.update_at(
                Task {
                    rule_id: 2,
                    last_run: 2,
                    output_ts: 4,
                    add: 100,
                },
                4,
                1,
            );
        }
        b_input.advance_to(5);
        b_input.flush();
        c_input.advance_to(5);
        c_input.flush();
        bc_task_input.advance_to(5);
        bc_task_input.flush();
        worker.step_while(|| bc_probe.less_equal(&4));

        let c_rows = collect_positive(&bc_events.lock().expect("event log poisoned"), 2);
        assert_eq!(
            c_rows,
            vec![Row {
                value: 201,
                row_ts: 4
            }]
        );

        if worker.index() == 0 {
            for row in &c_rows {
                c_input.update_at(*row, 5, 1);
            }
        }
        c_input.advance_to(6);
        c_input.flush();
        b_input.advance_to(6);
        b_input.flush();
        bc_task_input.advance_to(6);
        bc_task_input.flush();
        worker.step_while(|| bc_probe.less_than(c_input.time()));

        c_trace.set_logical_compaction(AntichainRef::new(&[6]));
        c_trace.set_physical_compaction(AntichainRef::new(&[6]));

        let (mut cursor, storage) = c_trace.cursor();
        let mut c_contents = BTreeMap::new();
        for ((row, _unit), times) in cursor.to_vec(&storage, |row| *row, |_| ()) {
            let diff = times.into_iter().map(|(_time, diff)| diff).sum();
            if diff != 0 {
                c_contents.insert(row, diff);
            }
        }
        assert_eq!(
            c_contents,
            BTreeMap::from([(
                Row {
                    value: 201,
                    row_ts: 4
                },
                1
            )])
        );

        drop(a_input);
        drop(ab_task_input);
        drop(a_trace);
        drop(b_input);
        drop(c_input);
        drop(bc_task_input);
        drop(c_trace);
        worker.drop_dataflow(bc_dataflow);
        worker.drop_dataflow(ab_dataflow);
        let installed_after_drop = worker.installed_dataflows();
        assert!(!installed_after_drop.contains(&ab_dataflow));
        assert!(!installed_after_drop.contains(&bc_dataflow));

        (b_rows, c_contents, installed_after_drop)
    })
    .expect("timely execution failed")
    .join();

    for summary in summaries {
        let (b_rows, c_contents, installed_after_drop) = summary.expect("worker failed");
        println!("derived_b_rows={b_rows:?}");
        println!("c_trace_contents={c_contents:?}");
        println!("installed_after_drop={installed_after_drop:?}");
    }
}
