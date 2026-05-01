---
title: "Lateral Joins and Demand-Driven Queries | Materialize"
url: "https://materialize.com/blog/lateral-joins-and-demand-driven-queries/"
date: "2020-08-18"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "5bffaf45d0421aa31f88698d492de3ae1bf2e952690c2fa8f67080ae5fb6df62"
source_type: materialize_blog
---

# Notes

- Summary: Comprehensive guide to using Materialize's `LATERAL` join for efficient query patterns in incremental view maintenance engines.
- Topics inferred from section headings:
  - Lateral Joins and Demand-Driven Queries
  - What's a lateral join?
  - Lateral Joins in Materialize
  - But is it new?
  - In greater detail
  - A reactive microservice
  - Conclusions
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - In the streaming SQL setting, lateral joins automatically turn your SQL prepared statement queries into what is essentially a streaming, consistent, microservice (minus the hard work). You just put your parameter binding...
  - In SQL the LATERAL join modifier allows relations used in a join to "see" the bindings in relations earlier in the join. This allows us (you, especially) to write joins where the matched records can be restricted beyond...
