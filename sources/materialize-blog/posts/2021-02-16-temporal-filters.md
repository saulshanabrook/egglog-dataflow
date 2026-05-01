---
title: "Temporal Filters: Enabling Windowed Queries in Materialize | Materialize"
url: "https://materialize.com/blog/temporal-filters/"
date: "2021-02-16"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "22c4d6817275739d50f9dcfce2feb14fa96abb291d5e213ac4f69069f3bc2e6c"
source_type: materialize_blog
---

# Notes

- Summary: Temporal filters give you a powerful SQL primitive for defining time-windowed computations over temporal data.
- Topics inferred from section headings:
  - Temporal Filters: Enabling Windowed Queries in Materialize
  - Temporal Data
  - Time-Windowed Queries
  - Time-Windowed Computation
  - A Brief Example
  - Windows: Sliding and Tumbling
  - Going Beyond Count
  - Conclusions
- Key passages captured for indexing (short excerpts):
  - Materialize provides a SQL interface to work with continually changing data. You write SQL queries as if against static data, and then as your data change we keep the results of your queries automatically up to date, in...
  - Materialize leans hard into the ideal that SQL is what you know best, and what you want to use to look at streaming data. At the same time, there are several tantalizing concepts that native stream processors provide tha...
