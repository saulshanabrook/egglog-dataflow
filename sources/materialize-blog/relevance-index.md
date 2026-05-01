# Materialize Blog Relevance Index

- Scope: `166` posts from `sources/materialize-blog/index.json` and the
  per-post markdown files under `sources/materialize-blog/posts/`.
- Counts: high `32`, medium `36`, low `98`.
- Validation: every index entry classified exactly once; no missing post files in this corpus.

## Classification Criteria

- High: direct engine/system detail relevant to external DD trial questions (Timely/Differential internals, arrangements/indexing tradeoffs, compaction/memory/frontier behavior, recursion execution, join maintenance algorithms, consistency semantics, query planner/runtime architecture, introspection tied to execution).
- Medium: adjacent operational architecture or performance interpretation that can constrain trial design, but with less algorithmic depth (freshness/cost framing, source/CDC behavior, deployment/runtime boundaries, benchmark framing).
- Low: announcements, partnerships, customer stories, product marketing, org/company news, and feature posts without durable engine evidence for DD/Timely/Differential questions.

## High Relevance (32)

| Date | Title | Local Path | URL |
|---|---|---|---|
| 2015-08-15 | Materialize and Memory \| Materialize | `sources/materialize-blog/posts/2015-08-15-materialize-and-memory.md` | https://materialize.com/blog/materialize-and-memory/ |
| 2019-12-29 | Slicing up Temporal Aggregates in Materialize \| Materialize | `sources/materialize-blog/posts/2019-12-29-slicing-up-temporal-aggregates-in-materialize.md` | https://materialize.com/blog/slicing-up-temporal-aggregates-in-materialize/ |
| 2020-02-24 | View Maintenance: A New Approach to Data Processing \| Materialize | `sources/materialize-blog/posts/2020-02-24-olvm.md` | https://materialize.com/blog/olvm/ |
| 2020-03-26 | Managing memory with differential dataflow \| Materialize | `sources/materialize-blog/posts/2020-03-26-managing-memory-with-differential-dataflow.md` | https://materialize.com/blog/managing-memory-with-differential-dataflow/ |
| 2020-03-27 | Upserts in Differential Dataflow \| Materialize | `sources/materialize-blog/posts/2020-03-27-upserts-in-differential-dataflow.md` | https://materialize.com/blog/upserts-in-differential-dataflow/ |
| 2020-03-31 | Consistency Guarantees in Data Streaming \| Materialize | `sources/materialize-blog/posts/2020-03-31-consistency.md` | https://materialize.com/blog/consistency/ |
| 2020-07-14 | Eventual Consistency isn't for Streaming \| Materialize | `sources/materialize-blog/posts/2020-07-14-eventual-consistency-isnt-for-streaming.md` | https://materialize.com/blog/eventual-consistency-isnt-for-streaming/ |
| 2020-08-04 | Robust Reductions in Materialize \| Materialize | `sources/materialize-blog/posts/2020-08-04-robust-reductions-in-materialize.md` | https://materialize.com/blog/robust-reductions-in-materialize/ |
| 2020-08-18 | Lateral Joins and Demand-Driven Queries \| Materialize | `sources/materialize-blog/posts/2020-08-18-lateral-joins-and-demand-driven-queries.md` | https://materialize.com/blog/lateral-joins-and-demand-driven-queries/ |
| 2020-09-30 | Materialize under the Hood \| Materialize | `sources/materialize-blog/posts/2020-09-30-materialize-under-the-hood.md` | https://materialize.com/blog/materialize-under-the-hood/ |
| 2020-11-18 | Joins in Materialize \| Materialize | `sources/materialize-blog/posts/2020-11-18-joins-in-materialize.md` | https://materialize.com/blog/joins-in-materialize/ |
| 2021-01-11 | Understanding Differential Dataflow \| Materialize | `sources/materialize-blog/posts/2021-01-11-life-in-differential-dataflow.md` | https://materialize.com/blog/life-in-differential-dataflow/ |
| 2021-02-16 | Temporal Filters: Enabling Windowed Queries in Materialize \| Materialize | `sources/materialize-blog/posts/2021-02-16-temporal-filters.md` | https://materialize.com/blog/temporal-filters/ |
| 2021-03-01 | How Materialize and other databases optimize SQL subqueries \| Materialize | `sources/materialize-blog/posts/2021-03-01-decorrelation-subquery-optimization.md` | https://materialize.com/blog/decorrelation-subquery-optimization/ |
| 2021-04-29 | Generalizing linear operators in differential dataflow \| Materialize | `sources/materialize-blog/posts/2021-04-29-generalizing-linear-operators.md` | https://materialize.com/blog/generalizing-linear-operators/ |
| 2021-06-02 | Maintaining Joins using Few Resources \| Materialize | `sources/materialize-blog/posts/2021-06-02-maintaining-joins-using-few-resources.md` | https://materialize.com/blog/maintaining-joins-using-few-resources/ |
| 2022-04-15 | How filter pushdown works \| Materialize | `sources/materialize-blog/posts/2022-04-15-how-filter-pushdown-works.md` | https://materialize.com/blog/how-filter-pushdown-works/ |
| 2022-06-14 | Virtual Time for Scalable Performance \| Materialize | `sources/materialize-blog/posts/2022-06-14-virtual-time-consistency-scalability.md` | https://materialize.com/blog/virtual-time-consistency-scalability/ |
| 2022-07-27 | Indexes: A Silent Frenemy \| Materialize | `sources/materialize-blog/posts/2022-07-27-indexes-a-silent-frenemy.md` | https://materialize.com/blog/indexes-a-silent-frenemy/ |
| 2022-12-25 | Recursion in Materialize \| Materialize | `sources/materialize-blog/posts/2022-12-25-recursion-in-materialize.md` | https://materialize.com/blog/recursion-in-materialize/ |
| 2023-01-18 | Delta Joins and Late Materialization \| Materialize | `sources/materialize-blog/posts/2023-01-18-delta-joins.md` | https://materialize.com/blog/delta-joins/ |
| 2023-02-09 | Building Differential Dataflow from Scratch \| Materialize | `sources/materialize-blog/posts/2023-02-09-differential-from-scratch.md` | https://materialize.com/blog/differential-from-scratch/ |
| 2023-02-23 | The Software Architecture of Materialize \| Materialize | `sources/materialize-blog/posts/2023-02-23-materialize-architecture.md` | https://materialize.com/blog/materialize-architecture/ |
| 2023-07-12 | Recursive SQL Queries in Materialize \| Materialize | `sources/materialize-blog/posts/2023-07-12-recursive-ctes-in-materialize.md` | https://materialize.com/blog/recursive-ctes-in-materialize/ |
| 2023-09-26 | Consistency and Operational Confidence \| Materialize | `sources/materialize-blog/posts/2023-09-26-operational-consistency.md` | https://materialize.com/blog/operational-consistency/ |
| 2024-08-14 | Incremental View Maintenance Replicas: Improve Database Stability and Accelerate Workloads \| Materialize | `sources/materialize-blog/posts/2024-08-14-ivm-database-replica.md` | https://materialize.com/blog/ivm-database-replica/ |
| 2025-01-13 | Replica expiration: Limiting temporal filters' resource requirements \| Materialize | `sources/materialize-blog/posts/2025-01-13-replica-expiration.md` | https://materialize.com/blog/replica-expiration/ |
| 2025-01-24 | Materialize's Strong Consistency Guarantees for Continually Changing Data \| Materialize | `sources/materialize-blog/posts/2025-01-24-strong-consistency-in-materialize.md` | https://materialize.com/blog/strong-consistency-in-materialize/ |
| 2025-01-30 | Source Mapping and Introspection: Debugging Materialize with Materialize \| Materialize | `sources/materialize-blog/posts/2025-01-30-debugging-query-performance.md` | https://materialize.com/blog/debugging-query-performance/ |
| 2025-09-18 | Scaling Beyond Memory: How Materialize Uses Swap for Larger Workloads \| Materialize | `sources/materialize-blog/posts/2025-09-18-scaling-beyond-memory.md` | https://materialize.com/blog/scaling-beyond-memory/ |
| 2026-02-26 | Self-Correcting Materialized Views \| Materialize | `sources/materialize-blog/posts/2026-02-26-self-correcting-materialized-views.md` | https://materialize.com/blog/self-correcting-materialized-views/ |
| 2026-03-19 | Speeding up Timely Dataflow by 100x \| Materialize | `sources/materialize-blog/posts/2026-03-19-speeding-up-timely-dataflow.md` | https://materialize.com/blog/speeding-up-timely-dataflow/ |

## Medium Relevance (36)

| Date | Title | Local Path | URL |
|---|---|---|---|
| 2020-02-18 | Introducing Materialize: the Streaming Data Warehouse \| Materialize | `sources/materialize-blog/posts/2020-02-18-introduction.md` | https://materialize.com/blog/introduction/ |
| 2020-06-22 | Rust for Data-Intensive Computation \| Materialize | `sources/materialize-blog/posts/2020-06-22-rust-for-data-intensive-computation.md` | https://materialize.com/blog/rust-for-data-intensive-computation/ |
| 2020-08-11 | Why, How, and When To Use Materialized Views \| Materialize | `sources/materialize-blog/posts/2020-08-11-why-use-a-materialized-view.md` | https://materialize.com/blog/why-use-a-materialized-view/ |
| 2020-08-13 | Change Data Capture (part 1) \| Materialize | `sources/materialize-blog/posts/2020-08-13-change-data-capture-part-1.md` | https://materialize.com/blog/change-data-capture-part-1/ |
| 2020-11-17 | Live Maintained Views on Boston Transit to Run at Home \| Materialize | `sources/materialize-blog/posts/2020-11-17-live-maintained-views-on-boston-transit-to-run-at-home.md` | https://materialize.com/blog/live-maintained-views-on-boston-transit-to-run-at-home/ |
| 2021-01-20 | Efficient Real-Time App with TAIL \| Materialize | `sources/materialize-blog/posts/2021-01-20-a-simple-and-efficient-real-time-application-powered-by-materializes-tail-command.md` | https://materialize.com/blog/a-simple-and-efficient-real-time-application-powered-by-materializes-tail-command/ |
| 2021-09-21 | Change Data Capture is having a moment. Why? \| Materialize | `sources/materialize-blog/posts/2021-09-21-change-data-capture-is-having-a-moment-why.md` | https://materialize.com/blog/change-data-capture-is-having-a-moment-why/ |
| 2022-02-16 | Direct PostgreSQL Replication Stream Setup \| Materialize | `sources/materialize-blog/posts/2022-02-16-connecting-materialize-directly-to-postgresql-via-the-replication-stream.md` | https://materialize.com/blog/connecting-materialize-directly-to-postgresql-via-the-replication-stream/ |
| 2022-03-03 | Subscribe to changes in a view with Materialize \| Materialize | `sources/materialize-blog/posts/2022-03-03-subscribe-to-changes-in-a-view-with-tail-in-materialize.md` | https://materialize.com/blog/subscribe-to-changes-in-a-view-with-tail-in-materialize/ |
| 2022-05-06 | Materialize's unbundled cloud architecture \| Materialize | `sources/materialize-blog/posts/2022-05-06-materialize-unbundled.md` | https://materialize.com/blog/materialize-unbundled/ |
| 2022-10-18 | How and why is Materialize compatible with PostgreSQL? \| Materialize | `sources/materialize-blog/posts/2022-10-18-postgres-compatibility.md` | https://materialize.com/blog/postgres-compatibility/ |
| 2023-02-16 | When to Use Indexes and Materialized Views \| Materialize | `sources/materialize-blog/posts/2023-02-16-views-indexes.md` | https://materialize.com/blog/views-indexes/ |
| 2023-05-18 | Real-Time Postgres Views Updates \| Materialize | `sources/materialize-blog/posts/2023-05-18-postgres-source-updates.md` | https://materialize.com/blog/postgres-source-updates/ |
| 2023-07-27 | Cloud Data Warehouse Uses & Misuses \| Materialize | `sources/materialize-blog/posts/2023-07-27-warehouse-abuse.md` | https://materialize.com/blog/warehouse-abuse/ |
| 2023-08-01 | Capturing Change Data Capture (CDC) Data \| Materialize | `sources/materialize-blog/posts/2023-08-01-capturing-cdc-data.md` | https://materialize.com/blog/capturing-cdc-data/ |
| 2023-08-29 | Lower Data Freshness Costs for Teams \| Materialize | `sources/materialize-blog/posts/2023-08-29-decouple-cost-and-freshness.md` | https://materialize.com/blog/decouple-cost-and-freshness/ |
| 2023-09-22 | A guided tour through Materialize's product principles \| Materialize | `sources/materialize-blog/posts/2023-09-22-operational-attributes.md` | https://materialize.com/blog/operational-attributes/ |
| 2023-10-12 | Freshness and Operational Autonomy \| Materialize | `sources/materialize-blog/posts/2023-10-12-freshness.md` | https://materialize.com/blog/freshness/ |
| 2024-01-11 | Responsiveness and Operational Agility \| Materialize | `sources/materialize-blog/posts/2024-01-11-responsiveness.md` | https://materialize.com/blog/responsiveness/ |
| 2024-02-02 | What is an operational data warehouse? \| Materialize | `sources/materialize-blog/posts/2024-02-02-what-is-an-operational-data-warehouse.md` | https://materialize.com/blog/what-is-an-operational-data-warehouse/ |
| 2024-02-23 | Data Freshness: Why It Matters and How to Deliver It \| Materialize | `sources/materialize-blog/posts/2024-02-23-what-is-data-freshness.md` | https://materialize.com/blog/what-is-data-freshness/ |
| 2024-03-21 | Building a MySQL source for Materialize \| Materialize | `sources/materialize-blog/posts/2024-03-21-building-a-mysql-source.md` | https://materialize.com/blog/building-a-mysql-source/ |
| 2024-06-26 | The Missing Element in Your Data Architecture \| Materialize | `sources/materialize-blog/posts/2024-06-26-missing-element-data-architecture.md` | https://materialize.com/blog/missing-element-data-architecture/ |
| 2024-08-01 | OLTP Queries: Transfer Expensive Workloads to Materialize \| Materialize | `sources/materialize-blog/posts/2024-08-01-oltp-queries.md` | https://materialize.com/blog/oltp-queries/ |
| 2024-08-12 | Performance Benchmark: Aurora PostgreSQL vs. Materialize \| Materialize | `sources/materialize-blog/posts/2024-08-12-performance-benchmark-aurora-postgresql-materialize.md` | https://materialize.com/blog/performance-benchmark-aurora-postgresql-materialize/ |
| 2024-09-13 | Zero-Staleness: Like using your primary, but faster \| Materialize | `sources/materialize-blog/posts/2024-09-13-zero-staleness-faster-primary.md` | https://materialize.com/blog/zero-staleness-faster-primary/ |
| 2024-10-04 | Fresh Data, Complex Queries: A Guide for PostgreSQL Users \| Materialize | `sources/materialize-blog/posts/2024-10-04-data-queries-postgres.md` | https://materialize.com/blog/data-queries-postgres/ |
| 2024-12-11 | The Challenges With Microservices (and how Materialize can help) \| Materialize | `sources/materialize-blog/posts/2024-12-11-challenges-with-microservices.md` | https://materialize.com/blog/challenges-with-microservices/ |
| 2025-01-14 | How to Simplify Microservices with a Shared Database and Materialized Views \| Materialize | `sources/materialize-blog/posts/2025-01-14-simplify-microservices-shared-database-materialized-views.md` | https://materialize.com/blog/simplify-microservices-shared-database-materialized-views/ |
| 2025-06-11 | Introducing Materialize v25.2: Enhanced Performance, Security, and Observability \| Materialize | `sources/materialize-blog/posts/2025-06-11-materialize-v25-2-release-performance-security-integrations.md` | https://materialize.com/blog/materialize-v25-2-release-performance-security-integrations/ |
| 2025-07-03 | Decoupling Control and Data: Better Architecture Through Larger Results \| Materialize | `sources/materialize-blog/posts/2025-07-03-decoupling-control-data-large-results.md` | https://materialize.com/blog/decoupling-control-data-large-results/ |
| 2026-01-09 | When Is Kappa Architecture Most Effective? Real-Time Analytics Explained \| Materialize | `sources/materialize-blog/posts/2026-01-09-when-is-kappa-architecture-most-effective.md` | https://materialize.com/blog/when-is-kappa-architecture-most-effective/ |
| 2026-02-09 | Does Kappa architecture improve on Lambda architecture? \| Materialize | `sources/materialize-blog/posts/2026-02-09-does-kappa-architecture-improve-on-lambda.md` | https://materialize.com/blog/does-kappa-architecture-improve-on-lambda/ |
| 2026-02-10 | Workload Capture & Replay in Materialize \| Materialize | `sources/materialize-blog/posts/2026-02-10-materialize-workload-capture-replay.md` | https://materialize.com/blog/materialize-workload-capture-replay/ |
| 2026-02-27 | Making Iceberg Work for Operational Data \| Materialize | `sources/materialize-blog/posts/2026-02-27-making-iceberg-work-for-operational-data.md` | https://materialize.com/blog/making-iceberg-work-for-operational-data/ |
| 2099-01-01 | Demonstrating Operational Data with SQL \| Materialize | `sources/materialize-blog/posts/2099-01-01-operational-data-sql.md` | https://materialize.com/blog/operational-data-sql/ |

## Low Relevance (98)

| Date | Title | Local Path | URL |
|---|---|---|---|
| 2018-01-01 | dbt & Materialize: Streamline Jaffle Shop Demo \| Materialize | `sources/materialize-blog/posts/2018-01-01-dbt-materialize-jaffle-shop-demo.md` | https://materialize.com/blog/dbt-materialize-jaffle-shop-demo/ |
| 2020-02-15 | Materialize Beta: The Details \| Materialize | `sources/materialize-blog/posts/2020-02-15-materialize-beta-the-details.md` | https://materialize.com/blog/materialize-beta-the-details/ |
| 2020-06-01 | Release: Materialize 0.3 \| Materialize | `sources/materialize-blog/posts/2020-06-01-release-materialize-0-3.md` | https://materialize.com/blog/release-materialize-0-3/ |
| 2020-06-08 | CMU DB Talk: Building Materialize \| Materialize | `sources/materialize-blog/posts/2020-06-08-cmudb.md` | https://materialize.com/blog/cmudb/ |
| 2020-07-24 | Streaming TAIL to the Browser - A One Day Project \| Materialize | `sources/materialize-blog/posts/2020-07-24-streaming-tail-to-the-browser-a-one-day-project.md` | https://materialize.com/blog/streaming-tail-to-the-browser-a-one-day-project/ |
| 2020-07-28 | Release: Materialize 0.4 \| Materialize | `sources/materialize-blog/posts/2020-07-28-release-materialize-0-4.md` | https://materialize.com/blog/release-materialize-0-4/ |
| 2020-11-24 | Release: Materialize 0.5 \| Materialize | `sources/materialize-blog/posts/2020-11-24-release-materialize-0-5.md` | https://materialize.com/blog/release-materialize-0-5/ |
| 2020-11-30 | Materialize Raises a Series B \| Materialize | `sources/materialize-blog/posts/2020-11-30-materialize-series-b.md` | https://materialize.com/blog/materialize-series-b/ |
| 2021-01-07 | Release: 0.6 \| Materialize | `sources/materialize-blog/posts/2021-01-07-release-0-6.md` | https://materialize.com/blog/release-0-6/ |
| 2021-03-01 | Introducing: dbt + Materialize \| Materialize | `sources/materialize-blog/posts/2021-03-01-introducing-dbt-materialize.md` | https://materialize.com/blog/introducing-dbt-materialize/ |
| 2021-03-09 | Release: 0.7 \| Materialize | `sources/materialize-blog/posts/2021-03-09-release-0-7.md` | https://materialize.com/blog/release-0-7/ |
| 2021-04-27 | Join Kafka with a Database using Debezium and Materialize \| Materialize | `sources/materialize-blog/posts/2021-04-27-join-kafka-with-database-debezium-materialize.md` | https://materialize.com/blog/join-kafka-with-database-debezium-materialize/ |
| 2021-06-14 | Release: 0.8 \| Materialize | `sources/materialize-blog/posts/2021-06-14-release-0-8.md` | https://materialize.com/blog/release-0-8/ |
| 2021-08-05 | Materialize & Datalot: Real-time Application Development \| Materialize | `sources/materialize-blog/posts/2021-08-05-materialize-datalot-real-time-application-development.md` | https://materialize.com/blog/materialize-datalot-real-time-application-development/ |
| 2021-08-27 | Release: 0.9 \| Materialize | `sources/materialize-blog/posts/2021-08-27-release-0-9.md` | https://materialize.com/blog/release-0-9/ |
| 2021-09-13 | Materialize Cloud Enters Open Beta \| Materialize | `sources/materialize-blog/posts/2021-09-13-materialize-cloud-open-beta.md` | https://materialize.com/blog/materialize-cloud-open-beta/ |
| 2021-09-30 | Materialize Secures $60M Series C Funding \| Materialize | `sources/materialize-blog/posts/2021-09-30-materialize-raises-a-series-c.md` | https://materialize.com/blog/materialize-raises-a-series-c/ |
| 2021-10-19 | Stream Analytics with Redpanda & Materialize \| Materialize | `sources/materialize-blog/posts/2021-10-19-taking-streaming-analytics-further-faster-with-redpanda-materialize.md` | https://materialize.com/blog/taking-streaming-analytics-further-faster-with-redpanda-materialize/ |
| 2021-12-01 | What's new in Materialize? Volume 1 \| Materialize | `sources/materialize-blog/posts/2021-12-01-whats-new-in-materialize-vol1.md` | https://materialize.com/blog/whats-new-in-materialize-vol1/ |
| 2022-01-19 | Introducing: Tailscale + Materialize \| Materialize | `sources/materialize-blog/posts/2022-01-19-introducing-tailscale-materialize.md` | https://materialize.com/blog/introducing-tailscale-materialize/ |
| 2022-03-01 | What's new in Materialize? Vol. 2 \| Materialize | `sources/materialize-blog/posts/2022-03-01-whats-new-in-materialize-vol-2.md` | https://materialize.com/blog/whats-new-in-materialize-vol-2/ |
| 2022-04-22 | Creating a Real-Time Feature Store with Materialize \| Materialize | `sources/materialize-blog/posts/2022-04-22-real-time-feature-store-with-materialize.md` | https://materialize.com/blog/real-time-feature-store-with-materialize/ |
| 2022-06-07 | Let’s talk about Data Apps \| Materialize | `sources/materialize-blog/posts/2022-06-07-lets-talk-about-data-apps.md` | https://materialize.com/blog/lets-talk-about-data-apps/ |
| 2022-06-15 | Managing streaming analytics pipelines with dbt \| Materialize | `sources/materialize-blog/posts/2022-06-15-managing-streaming-analytics-pipelines-with-dbt.md` | https://materialize.com/blog/managing-streaming-analytics-pipelines-with-dbt/ |
| 2022-07-14 | Real-time data quality tests using dbt and Materialize \| Materialize | `sources/materialize-blog/posts/2022-07-14-real-time-data-quality-tests-using-dbt-and-materialize.md` | https://materialize.com/blog/real-time-data-quality-tests-using-dbt-and-materialize/ |
| 2022-10-03 | Announcing the next generation of Materialize \| Materialize | `sources/materialize-blog/posts/2022-10-03-next-generation.md` | https://materialize.com/blog/next-generation/ |
| 2022-10-19 | Real-Time Customer Data Platform Views on Materialize \| Materialize | `sources/materialize-blog/posts/2022-10-19-customer-data-platforms.md` | https://materialize.com/blog/customer-data-platforms/ |
| 2022-12-06 | Rust for high-performance concurrency and network services \| Materialize | `sources/materialize-blog/posts/2022-12-06-our-experience-with-rust.md` | https://materialize.com/blog/our-experience-with-rust/ |
| 2023-03-09 | Towards Real-Time dbt \| Materialize | `sources/materialize-blog/posts/2023-03-09-real-time-dbt.md` | https://materialize.com/blog/real-time-dbt/ |
| 2023-04-25 | A Terraform Provider for Materialize \| Materialize | `sources/materialize-blog/posts/2023-04-25-terraform-provider.md` | https://materialize.com/blog/terraform-provider/ |
| 2023-07-18 | Confluent & Materialize Expand Streaming \| Materialize | `sources/materialize-blog/posts/2023-07-18-confluent-partnership.md` | https://materialize.com/blog/confluent-partnership/ |
| 2023-08-31 | RBAC now available for all customers \| Materialize | `sources/materialize-blog/posts/2023-08-31-rbac.md` | https://materialize.com/blog/rbac/ |
| 2023-10-16 | VS Code Integration Guide \| Materialize | `sources/materialize-blog/posts/2023-10-16-vs-code-integration.md` | https://materialize.com/blog/vs-code-integration/ |
| 2023-10-27 | Compile Times and Code Graphs \| Materialize | `sources/materialize-blog/posts/2023-10-27-compile-times-and-code-graphs.md` | https://materialize.com/blog/compile-times-and-code-graphs/ |
| 2023-12-21 | How we built the SQL Shell \| Materialize | `sources/materialize-blog/posts/2023-12-21-building-sql-shell.md` | https://materialize.com/blog/building-sql-shell/ |
| 2024-01-19 | Materialize and Advent of Code: Using SQL to solve your puzzles! \| Materialize | `sources/materialize-blog/posts/2024-01-19-advent-of-code-2023.md` | https://materialize.com/blog/advent-of-code-2023/ |
| 2024-02-12 | Doing business with recursive SQL \| Materialize | `sources/materialize-blog/posts/2024-02-12-doing-business-with-recursive-sql.md` | https://materialize.com/blog/doing-business-with-recursive-sql/ |
| 2024-02-26 | Introducing Query History \| Materialize | `sources/materialize-blog/posts/2024-02-26-query-history-private-preview.md` | https://materialize.com/blog/query-history-private-preview/ |
| 2024-03-04 | View your usage and billing history \| Materialize | `sources/materialize-blog/posts/2024-03-04-usage-and-billing.md` | https://materialize.com/blog/usage-and-billing/ |
| 2024-03-07 | Real-Time Fraud Detection: Analytical vs. Operational Data Warehouses \| Materialize | `sources/materialize-blog/posts/2024-03-07-fraud-detection-latency-accuracy.md` | https://materialize.com/blog/fraud-detection-latency-accuracy/ |
| 2024-03-15 | Native MySQL Source, now in Private Preview \| Materialize | `sources/materialize-blog/posts/2024-03-15-mysql-source-private-preview.md` | https://materialize.com/blog/mysql-source-private-preview/ |
| 2024-03-19 | Materialize + Redpanda Serverless: Simplified developer experience for real-time apps \| Materialize | `sources/materialize-blog/posts/2024-03-19-redpanda-serverless.md` | https://materialize.com/blog/redpanda-serverless/ |
| 2024-04-08 | Announcing our new CEO: Nate Stewart \| Materialize | `sources/materialize-blog/posts/2024-04-08-new-ceo-nate-stewart.md` | https://materialize.com/blog/new-ceo-nate-stewart/ |
| 2024-05-01 | Now Generally Available: New Cluster Sizes \| Materialize | `sources/materialize-blog/posts/2024-05-01-new-cluster-sizes.md` | https://materialize.com/blog/new-cluster-sizes/ |
| 2024-05-07 | Loan Underwriting Process: The Move to Big Data & SQL \| Materialize | `sources/materialize-blog/posts/2024-05-07-loan-underwriting-big-data-sql.md` | https://materialize.com/blog/loan-underwriting-big-data-sql/ |
| 2024-05-08 | Loan Underwriting: Real-Time Data Architectures \| Materialize | `sources/materialize-blog/posts/2024-05-08-loan-underwriting-real-time-data-architectures.md` | https://materialize.com/blog/loan-underwriting-real-time-data-architectures/ |
| 2024-05-13 | Testing Materialize: Our QA Process \| Materialize | `sources/materialize-blog/posts/2024-05-13-qa-process-overview.md` | https://materialize.com/blog/qa-process-overview/ |
| 2024-05-30 | Celebrating our newest partnership at Data Cloud Summit \| Materialize | `sources/materialize-blog/posts/2024-05-30-materialize-at-snowflake-summit.md` | https://materialize.com/blog/materialize-at-snowflake-summit/ |
| 2024-06-03 | Bulk exports to S3, now in Private Preview! \| Materialize | `sources/materialize-blog/posts/2024-06-03-bulk-exports-s3.md` | https://materialize.com/blog/bulk-exports-s3/ |
| 2024-06-05 | The Problem with Lying is Keeping Track of All the Lies \| Materialize | `sources/materialize-blog/posts/2024-06-05-keeping-track-lies.md` | https://materialize.com/blog/keeping-track-lies/ |
| 2024-06-10 | How Materialize Unlocks Private Kafka Connectivity via PrivateLink and SSH \| Materialize | `sources/materialize-blog/posts/2024-06-10-private-kafka-connectivity.md` | https://materialize.com/blog/private-kafka-connectivity/ |
| 2024-07-02 | Real-Time Data Architectures: Why Small Data Teams Can't Wait \| Materialize | `sources/materialize-blog/posts/2024-07-02-real-time-small-data-teams.md` | https://materialize.com/blog/real-time-small-data-teams/ |
| 2024-07-10 | Operational Data Warehouse: Streaming Solution for Small Data Teams \| Materialize | `sources/materialize-blog/posts/2024-07-10-operational-data-warehouse-small-team.md` | https://materialize.com/blog/operational-data-warehouse-small-team/ |
| 2024-07-22 | Sync your data into Materialize with Fivetran \| Materialize | `sources/materialize-blog/posts/2024-07-22-fivetran-and-materialize.md` | https://materialize.com/blog/fivetran-and-materialize/ |
| 2024-09-12 | Materialize + Novu: Real-Time Alerting Powered by a Cloud Operational Data Store \| Materialize | `sources/materialize-blog/posts/2024-09-12-novu-materialize-real-time-alerting.md` | https://materialize.com/blog/novu-materialize-real-time-alerting/ |
| 2024-09-24 | Real-Time CDC from Oracle to Materialize Using Estuary Flow \| Materialize | `sources/materialize-blog/posts/2024-09-24-oracle-materialize-estuary-flow.md` | https://materialize.com/blog/oracle-materialize-estuary-flow/ |
| 2024-09-25 | Supporting Open Source: Materialize’s Community Sponsorship Program \| Materialize | `sources/materialize-blog/posts/2024-09-25-materialize-community-sponsorship-program.md` | https://materialize.com/blog/materialize-community-sponsorship-program/ |
| 2024-10-02 | Migrating from dbt-postgres to dbt-materialize \| Materialize | `sources/materialize-blog/posts/2024-10-02-migrating-postgres-materialize.md` | https://materialize.com/blog/migrating-postgres-materialize/ |
| 2024-10-10 | How to Use the Materialize Emulator \| Materialize | `sources/materialize-blog/posts/2024-10-10-materialize-emulator.md` | https://materialize.com/blog/materialize-emulator/ |
| 2024-10-23 | Transforming Real-Time Data with Operational Data Stores: A Dynamic Pricing Use Case \| Materialize | `sources/materialize-blog/posts/2024-10-23-ods-ecommerce-demo.md` | https://materialize.com/blog/ods-ecommerce-demo/ |
| 2024-11-25 | It’s (almost) here: Materialize Self-Managed \| Materialize | `sources/materialize-blog/posts/2024-11-25-self-managed.md` | https://materialize.com/blog/self-managed/ |
| 2024-12-09 | Re:Inventing Real-Time Data Integration \| Materialize | `sources/materialize-blog/posts/2024-12-09-reinvent-real-time-data-integration-takeaways.md` | https://materialize.com/blog/reinvent-real-time-data-integration-takeaways/ |
| 2024-12-13 | Reimagining Agentic Orchestration: Materialize and the Future of Autonomous Systems \| Materialize | `sources/materialize-blog/posts/2024-12-13-reimagining-agentic-orchestration-materialize.md` | https://materialize.com/blog/reimagining-agentic-orchestration-materialize/ |
| 2024-12-16 | Materialize Self-Managed: Early Access Now Available \| Materialize | `sources/materialize-blog/posts/2024-12-16-self-managed-materialize-early-access.md` | https://materialize.com/blog/self-managed-materialize-early-access/ |
| 2024-12-17 | The Making of Materialize Self-Managed: Flexible Deployments Explained \| Materialize | `sources/materialize-blog/posts/2024-12-17-making-self-managed-materialize-flexible-deployments.md` | https://materialize.com/blog/making-self-managed-materialize-flexible-deployments/ |
| 2025-01-16 | Real-Time Structured Data for RAG: Enrich prompts with live context \| Materialize | `sources/materialize-blog/posts/2025-01-16-realtime-structured-data-for-rag.md` | https://materialize.com/blog/realtime-structured-data-for-rag/ |
| 2025-02-21 | Why AI Systems Fail—And How Real-Time Data Fixes Them \| Materialize | `sources/materialize-blog/posts/2025-02-21-whitepaper-ai.md` | https://materialize.com/blog/whitepaper-ai/ |
| 2025-03-11 | Materialize For Everyone: Introducing Self-Managed and our Free Community Edition \| Materialize | `sources/materialize-blog/posts/2025-03-11-materialize-for-everyone.md` | https://materialize.com/blog/materialize-for-everyone/ |
| 2025-05-07 | Materialize Turns Views into Tools for Agents \| Materialize | `sources/materialize-blog/posts/2025-05-07-materialize-turns-views-into-tools-for-agents.md` | https://materialize.com/blog/materialize-turns-views-into-tools-for-agents/ |
| 2025-05-08 | AI Data Products: Best Practices for Scaling Your AI Data Initiatives \| Materialize | `sources/materialize-blog/posts/2025-05-08-ai-data-products-best-practices.md` | https://materialize.com/blog/ai-data-products-best-practices/ |
| 2025-05-15 | Diagnosing a Double-Free Concurrency Bug in Rust's Unbounded Channels \| Materialize | `sources/materialize-blog/posts/2025-05-15-rust-concurrency-bug-unbounded-channels.md` | https://materialize.com/blog/rust-concurrency-bug-unbounded-channels/ |
| 2025-06-02 | Materialize Now Ingests SQL Server Natively \| Materialize | `sources/materialize-blog/posts/2025-06-02-materialize-sql-server-native-ingestion.md` | https://materialize.com/blog/materialize-sql-server-native-ingestion/ |
| 2025-06-30 | AI Agents Need Digital Twin \| Materialize | `sources/materialize-blog/posts/2025-06-30-ai-agents-need-digital-twins.md` | https://materialize.com/blog/ai-agents-need-digital-twins/ |
| 2025-07-16 | Analyzing Live Social Data: Exploring Social Trends on Bluesky \| Materialize | `sources/materialize-blog/posts/2025-07-16-analyzing-social-data.md` | https://materialize.com/blog/analyzing-social-data/ |
| 2025-07-17 | Materialize Self-Managed v26.0.0: Schema Change Support, Cost Savings & Security Upgrades \| Materialize | `sources/materialize-blog/posts/2025-07-17-materialize-self-managed-v26-0-0-release.md` | https://materialize.com/blog/materialize-self-managed-v26-0-0-release/ |
| 2025-07-30 | Building Digital Twins for AI Agents \| Materialize | `sources/materialize-blog/posts/2025-07-30-building-digital-twins-for-ai-agents.md` | https://materialize.com/blog/building-digital-twins-for-ai-agents/ |
| 2025-08-08 | Speeding up Materialize CI \| Materialize | `sources/materialize-blog/posts/2025-08-08-speeding-up-materialize-ci.md` | https://materialize.com/blog/speeding-up-materialize-ci/ |
| 2025-08-13 | Materialize's Spring Hackathon: A Report \| Materialize | `sources/materialize-blog/posts/2025-08-13-spring-hackathon-report.md` | https://materialize.com/blog/spring_hackathon_report/ |
| 2025-09-04 | Digital Twins in Manufacturing: A Practical Guide to Getting Started \| Materialize | `sources/materialize-blog/posts/2025-09-04-digital-twins-in-manufacturing.md` | https://materialize.com/blog/digital-twins-in-manufacturing/ |
| 2025-09-10 | Digital Twins in Construction: A Practical Guide to Getting Started \| Materialize | `sources/materialize-blog/posts/2025-09-10-digital-twins-in-construction.md` | https://materialize.com/blog/digital-twins-in-construction/ |
| 2025-09-12 | Digital Twins in Logistics: A Practical Guide to Getting Started \| Materialize | `sources/materialize-blog/posts/2025-09-12-digital-twins-in-logistics.md` | https://materialize.com/blog/digital-twins-in-logistics/ |
| 2025-09-15 | Digital Twins for Supply Chains: A Practical Guide to Getting Started \| Materialize | `sources/materialize-blog/posts/2025-09-15-digital-twins-in-supply-chains.md` | https://materialize.com/blog/digital-twins-in-supply-chains/ |
| 2025-09-22 | Vector database pipelines made easy \| Materialize | `sources/materialize-blog/posts/2025-09-22-vector-database-pipelines-made-easy.md` | https://materialize.com/blog/vector-database-pipelines-made-easy/ |
| 2025-10-08 | Introducing New Materialize Cloud M.1 Clusters \| Materialize | `sources/materialize-blog/posts/2025-10-08-introducing-new-materialize-cloud-m-1-clusters.md` | https://materialize.com/blog/introducing-new-materialize-cloud-m.1-clusters/ |
| 2025-11-03 | Low-latency Context Engineering for Production AI \| Materialize | `sources/materialize-blog/posts/2025-11-03-low-latency-context-engineering-for-production-ai.md` | https://materialize.com/blog/low-latency-context-engineering-for-production-ai/ |
| 2025-11-25 | Your Vector Search is (Probably) Broken: Here's Why \| Materialize | `sources/materialize-blog/posts/2025-11-25-your-vector-search-is-probably-broken.md` | https://materialize.com/blog/your-vector-search-is-probably-broken/ |
| 2026-01-08 | What is a Digital Twin for AI Agents? \| Materialize | `sources/materialize-blog/posts/2026-01-08-what-is-a-digital-twin-for-ai-agents.md` | https://materialize.com/blog/what-is-a-digital-twin-for-ai-agents/ |
| 2026-01-23 | What challenges are involved in integrating AI with operational data? \| Materialize | `sources/materialize-blog/posts/2026-01-23-integrating-ai-with-operational-data.md` | https://materialize.com/blog/integrating-ai-with-operational-data/ |
| 2026-02-11 | What's a live data product? \| Materialize | `sources/materialize-blog/posts/2026-02-11-live-data-product.md` | https://materialize.com/blog/live-data-product/ |
| 2026-02-16 | What does it cost to run Flink? \| Materialize | `sources/materialize-blog/posts/2026-02-16-apache-flink-cost-tco.md` | https://materialize.com/blog/apache-flink-cost-tco/ |
| 2026-02-24 | Four Thoughts from Four Years at Materialize \| Materialize | `sources/materialize-blog/posts/2026-02-24-four-thoughts-four-years-materialize.md` | https://materialize.com/blog/four-thoughts-four-years-materialize/ |
| 2026-02-26 | AI Context Engines: The Next Evolution of Context Engineering \| Materialize | `sources/materialize-blog/posts/2026-02-26-ai-context-engines-context-engineering-evolution.md` | https://materialize.com/blog/ai-context-engines-context-engineering-evolution/ |
| 2026-03-02 | The New Agentic Data Architecture: A Live Operational Data Mesh \| Materialize | `sources/materialize-blog/posts/2026-03-02-agentic-data-architecture-live-operational-data-mesh.md` | https://materialize.com/blog/agentic-data-architecture-live-operational-data-mesh/ |
| 2026-03-02 | Why You're Doing Context Engineering Wrong \| Materialize | `sources/materialize-blog/posts/2026-03-02-why-youre-doing-context-engineering-wrong-live-data-architecture.md` | https://materialize.com/blog/why-youre-doing-context-engineering-wrong-live-data-architecture/ |
| 2026-03-09 | How Does AI Change Digit Twins? \| Materialize | `sources/materialize-blog/posts/2026-03-09-how-ai-agents-are-redefining-digital-twins.md` | https://materialize.com/blog/how-ai-agents-are-redefining-digital-twins/ |
| 2026-03-25 | No Classification without Represention \| Materialize | `sources/materialize-blog/posts/2026-03-25-no-classification-without-representation.md` | https://materialize.com/blog/no-classification-without-representation/ |
| 2026-03-26 | Enterprise Context Engineering \| Materialize | `sources/materialize-blog/posts/2026-03-26-enterprise-context-engineering.md` | https://materialize.com/blog/enterprise-context-engineering/ |
| 2026-04-09 | What Chipotle Can Teach Us About Real-Time Data Products \| Materialize | `sources/materialize-blog/posts/2026-04-09-what-chipotle-can-teach-us-about-real-time-data-products.md` | https://materialize.com/blog/what-chipotle-can-teach-us-about-real-time-data-products/ |
