---
title: "Upserts in Differential Dataflow | Materialize"
url: "https://materialize.com/blog/upserts-in-differential-dataflow/"
date: "2020-03-27"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "e41ba3ebeb12296fbbc5ac57e83fea27f065a860f07a3ab58926b7946cc80feb"
source_type: materialize_blog
---

# Notes

- Summary: Comprehensive guide to implementing upserts in differential dataflow with Materialize for real-time data warehouse optimization & efficiency.
- Topics inferred from section headings:
  - Upserts in Differential Dataflow
  - Upserts vs differential updates
  - From upserts to differential updates
  - State machines and beyond
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - "Upserts" are a common way to express streams of changing data, especially in relational settings with primary keys. However, they aren't the best format for working with incremental computation. We're about to learn why...
  - "Upsert" is a portmanteau of "update" and "insert", and they are (I believe) used primarily in reconciling merges of databases: they allow you to think about inserting and updating data in a consistent framework: each ne...
