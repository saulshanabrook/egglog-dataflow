---
title: "Building a MySQL source for Materialize | Materialize"
url: "https://materialize.com/blog/building-a-mysql-source/"
date: "2024-03-21"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "7817bcec81800b9fb4c7008d4a7385f6b12b5796741ac8bf4611298ab1fbde9d"
source_type: materialize_blog
---

# Notes

- Summary: An in-depth breakdown of how we architected and built a native MySQL CDC source
- Topics inferred from section headings:
  - Building a MySQL source for Materialize
  - Starting the project
  - MySQL replication
  - Source architecture
  - Snapshotting and replication
  - Progress tracking
  - Data types
  - Validating our work
- Key passages captured for indexing (short excerpts):
  - Our new native MySQL source enables real-time replication from MySQL into Materialize, enabling users to power their operational workloads with a fresh and consistent view of their MySQL data.
  - The MySQL source is the second native “change data capture” (CDC) source we’ve built (the first is our PostgreSQL source). While it was already possible to ingest MySQL data into Materialize using Debezium as a CDC servi...
