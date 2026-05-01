---
title: "How Materialize and other databases optimize SQL subqueries | Materialize"
url: "https://materialize.com/blog/decorrelation-subquery-optimization/"
date: "2021-03-01"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "70a6d9f90861c5437bac7762f5aca69e148db8fd3b834bd9755bd42a04dc0ef7"
source_type: materialize_blog
---

# Notes

- Summary: Insight into SQL subquery optimization & how Materialize's approach differs from other databases, enhancing query performance.
- Topics inferred from section headings:
  - How Materialize and other databases optimize SQL subqueries
  - The problem
  - Existing approaches
  - The algebraic approach
  - Closing the gaps
  - Future work
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - Recursive CTEs are now production-ready, available to all Materialize users, and battle-tested at scale—learn more here.
  - Subqueries are a SQL feature that allow writing queries nested inside a scalar expression in an outer query. Using subqueries is often the most natural way to express a given problem, but their use is discouraged because...
