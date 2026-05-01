---
title: "Why, How, and When To Use Materialized Views | Materialize"
url: "https://materialize.com/blog/why-use-a-materialized-view/"
date: "2020-08-11"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "e6b6a07cad0e90d5ee305593099a13a192f464f12f4fa9aa6f2c34aa2ba57ddd"
source_type: materialize_blog
---

# Notes

- Summary: Discover how to reduce database query costs with Materialized Views. This guide will walk you through the benefits, creation process, and impact on database efficiency.
- Topics inferred from section headings:
  - Why, How, and When To Use Materialized Views
  - The Cost of Querying Adds Up Fast
  - Example
  - Overview: Materialized Views
  - What is a view?
  - What is a materialized view?
  - The Implications of Views Being “Materialized”
  - Use Cases for Materialized Views
- Key passages captured for indexing (short excerpts):
  - This post will look at the cost of querying databases, use cases for Materialized Views, how they work in specific databases, and what it looks like to create Materialized Views.
  - Each time you query a database you incur some cost. Your database will parse, validate, plan, optimize, and execute your query, using up wall clock time, CPU time, memory, opportunity cost, and, potentially, actual dolla...
