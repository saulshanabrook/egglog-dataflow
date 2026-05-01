---
title: "Performance Benchmark: Aurora PostgreSQL vs. Materialize | Materialize"
url: "https://materialize.com/blog/performance-benchmark-aurora-postgresql-materialize/"
date: "2024-08-12"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "23a746ca88779214ce5403fdfa505419dcc941c6492e25057bda2f3187a87043"
source_type: materialize_blog
---

# Notes

- Summary: Materialize outperforms Aurora for complex queries over relatively small data volumes. Here are the benchmarks.
- Topics inferred from section headings:
  - Performance Benchmark: Aurora PostgreSQL vs. Materialize
  - Aurora PostgreSQL: Not Designed for Complex, Read-Intensive Queries
  - Benchmarking Use Case: Dynamic Pricing for an Online Retailer
  - Configurations for Benchmark Testing
  - Aurora PostgreSQL Configuration
  - Materialize Configuration
  - Overview of Test Scenarios
  - Scenario 1: Single Database Connection with Continuous Writes
- Key passages captured for indexing (short excerpts):
  - This blog examines the performance of Materialize vs. Aurora PostgreSQL read replicas for computationally intensive workloads. We demonstrate that Materialize outperforms Aurora for complex queries over relatively small...
  - Specifically, for the same on-demand cost, Materialize delivers 100x greater throughput with 1000x lower latency. And unlike other solutions that offload computation from OLTP databases, Materialize does so without sacri...
