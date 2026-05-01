---
title: "Towards Real-Time dbt | Materialize"
url: "https://materialize.com/blog/real-time-dbt/"
date: "2023-03-09"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "3544776ff2997d32eeff22536ef45d0b54ab1c5d3d9db01c5289b1233ef51218"
source_type: materialize_blog
---

# Notes

- Summary: Explore strategies for unleashing real-time dbt, from materializing views to leveraging micro-batches and incrementally maintained views.
- Topics inferred from section headings:
  - Towards Real-Time dbt
  - Views
  - Transforming your data on read with Views
  - Scalability issues with Views
  - Microbatches
  - Transforming your data on write with Microbatches
  - Microbatches = Frequent batch jobs
  - Materialized views, incremental models, and lambda views
- Key passages captured for indexing (short excerpts):
  - Transforming your data with dbt solves a bunch of important problems for you. dbt version controls your transformations, allows your team to collaborate easily, encourages documentation, and unlocks easy testing of your...
  - However, there’s one thing it doesn’t help you with: reducing the time between data originating somewhere in your business to when you transform that input data into useful results for your business.
