---
title: "Generalizing linear operators in differential dataflow | Materialize"
url: "https://materialize.com/blog/generalizing-linear-operators/"
date: "2021-04-29"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "d76fba440f4783e2ba7c7f3d5ff60c334c17ba5878bd9898911356572c1444fa"
source_type: materialize_blog
---

# Notes

- Summary: Differential dataflow uses simple linear operators: `map`, `filter`, `flat_map` and complex: `explode` and temporal filter operators. But, with some thinking, we can generalize them all to a restricted form of join.
- Topics inferred from section headings:
  - Generalizing linear operators in differential dataflow
  - Differential dataflow background
  - Linear operators
  - Even more linear operators
  - All of the linear operators
  - An implementation
  - Fusing logic
  - Linear operators in Materialize
- Key passages captured for indexing (short excerpts):
  - Differential dataflows contain many operators, some of which are very complicated, but many of which are relatively simple.
  - The map operator applies a transformation to each record. The filter operator applies a predicate to each record, and drops records that do not pass it. The flat_map operator applies a function to each record that can re...
