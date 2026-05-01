---
title: "Delta Joins and Late Materialization | Materialize"
url: "https://materialize.com/blog/delta-joins/"
date: "2023-01-18"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "bbcf941f45adb8d3f783b2dcfec525a4230e0dff3ae02c764b30a8c4378dedc9"
source_type: materialize_blog
---

# Notes

- Summary: Understand how to optimize joins with indexes and late materialization.
- Topics inferred from section headings:
  - Delta Joins and Late Materialization
  - Introducing Joins
  - Binary Joins in Materialize
  - Optimizing A Query from the TPC-H Benchmark
  - A First Implementation
  - Primary Indexes
  - Secondary Indexes
  - Delta Query
- Key passages captured for indexing (short excerpts):
  - This article has been updated from the original to reflect the latest version of Materialize. The original post is available here.
  - Materialize allows you to maintain declarative, relational SQL queries over continually changing data. One of the most powerful features of SQL queries are joins: the ability to correlate records from multiple collection...
