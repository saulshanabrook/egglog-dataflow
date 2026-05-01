---
title: "Making Iceberg Work for Operational Data | Materialize"
url: "https://materialize.com/blog/making-iceberg-work-for-operational-data/"
date: "2026-02-27"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "45739debcaf0c3c77aaa45e5d536d13a1ffb0fb966539dafb3c7c154df6bfac2"
source_type: materialize_blog
---

# Notes

- Summary: Apache Iceberg was built for batch analytics — but operational data changes continuously. Learn how Materialize streams live, transactionally consistent data into Iceberg without the memory and latency costs of batching.
- Topics inferred from section headings:
  - Making Iceberg Work for Operational Data
  - How Materialize Thinks About Consistency
  - The Naive Approach
  - Minting Batch Descriptions Ahead of Time
  - The Delete Problem
  - Recovery Without External State
  - The Empty Snapshot Problem
  - Multi-Table Transactions
- Key passages captured for indexing (short excerpts):
  - Apache Iceberg has become the de facto open table format for analytics — it's what Snowflake, Databricks, and AWS S3 Tables all converged on. Write Parquet files to object storage, track them with some JSON metadata, and...
  - But Iceberg was designed for batch ETL jobs that run periodically and write big, consolidated files. Iceberg wants big, infrequent commits. Operational data changes continuously.
