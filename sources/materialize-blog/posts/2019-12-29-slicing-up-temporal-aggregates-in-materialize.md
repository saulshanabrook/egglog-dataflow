---
title: "Slicing up Temporal Aggregates in Materialize | Materialize"
url: "https://materialize.com/blog/slicing-up-temporal-aggregates-in-materialize/"
date: "2019-12-29"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "f74a07d15bfecca83286f5665f6c8e39c716ef7c18e3cfc54d7e0705b29a3d7e"
source_type: materialize_blog
---

# Notes

- Summary: Comprehensive guide on slicing temporal aggregates with Materialize for real-time data analysis & actionable insights.
- Topics inferred from section headings:
  - Slicing up Temporal Aggregates in Materialize
  - Temporal data, and queries
  - A first approach: That Query
  - A second approach: (Lateral) Joins
  - A third approach: Time Slicing
  - Trying it out
  - Thoughtful comments
  - Appendix: All the views you need to write
- Key passages captured for indexing (short excerpts):
  - Materialize computes and maintains SQL queries as your underlying data change. This makes it especially well-suited to tracking the current state of various SQL queries and aggregates!
  - But, what if you want to root around in the past? Maybe you want to compare today's numbers to yesterday's numbers. Maybe you want to scrub through the past, moving windows around looking for the most interesting moments...
