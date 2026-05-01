---
title: "Real-Time Postgres Views Updates | Materialize"
url: "https://materialize.com/blog/postgres-source-updates/"
date: "2023-05-18"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "a366a94cdc55a83afa263fff820d61bdbced2dad404dfc0828b17ef3f3921404"
source_type: materialize_blog
---

# Notes

- Summary: Major updates to PostgreSQL streaming replication allow for real-time & incrementally updated materialized views with Materialize.
- Topics inferred from section headings:
  - Real-Time Postgres Views Updates | Materialize
  - How Postgres sources work in Materialize
  - First Implementation
  - Why use Postgres sources?
  - Supporting schema changes
  - Not quite that simple, though....
  - Improved error handling
  - Adding and removing tables
- Key passages captured for indexing (short excerpts):
  - Materialize is a cloud-native database built on streaming internals. Our core feature: incrementally updated materialized views, is based on PostgreSQL materialized views––and aims to supplant them entirely, even for Pos...
  - PostgreSQL offers a replication stream of changes to your tables, and Materialize can act as a read replica of that stream. Once we get the data into Materialize, though, you can build complex, incrementally maintained m...
