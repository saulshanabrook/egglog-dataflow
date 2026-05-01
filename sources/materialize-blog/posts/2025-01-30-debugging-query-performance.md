---
title: "Source Mapping and Introspection: Debugging Materialize with Materialize | Materialize"
url: "https://materialize.com/blog/debugging-query-performance/"
date: "2025-01-30"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "6ed723834a820cafbe89972de923e89d426091cb72e414cab4526428506f77cf"
source_type: materialize_blog
---

# Notes

- Summary: Materialize now exposes source maps in its catalog, so you can build your own debugging queries that attribute performance characteristics to high-level operators.
- Topics inferred from section headings:
  - Source Mapping and Introspection: Debugging Materialize with Materialize
  - Databases have a lot of EXPLAINing to do
  - Materialize's compilation pipeline
  - Mapping dataflow metrics up to LIR
  - Attributing memory usage
  - Setting hints for TopK queries
  - What's next?
  - Footnotes
- Key passages captured for indexing (short excerpts):
  - We have a new way to understand the performance of views, indexes, and materialized views in Materialize. By mapping runtime data about low-level dataflow operators up to a sensible intermediate language, you'll be bette...
  - Databases typically offer some way to understand how queries run. Postgres, for example, has the EXPLAIN statement; running EXPLAIN ... query ... presents the user with a summary of how the plan will be run (what kind of...
