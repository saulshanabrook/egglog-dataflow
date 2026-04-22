**Objective**
Establish baseline evidence for the WCOJ examples: triangle, four-clique, q6, and q7.

**Commands**
`cargo check --manifest-path repos/dataflow-join/Cargo.toml --examples`
`cargo run --manifest-path repos/dataflow-join/Cargo.toml --example triangles-single -- --help`
`cargo run --manifest-path repos/dataflow-join/Cargo.toml --example motifs-single -- --help`

**Results**
The example suite compiled successfully. The runtime probes were blocked because the examples expect a graph file path and the checkout does not include a local dataset to feed them.

**Verdict**
Compile-only baseline. There is no measured end-to-end WCOJ runtime in this checkout yet.

**Option Implication**
Once a graph input is mounted, this lane can hold the concrete triangle/four-clique/q6/q7 timing baseline for the option-3 comparison.
