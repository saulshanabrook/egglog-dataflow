---
title: "How and why is Materialize compatible with PostgreSQL? | Materialize"
url: "https://materialize.com/blog/postgres-compatibility/"
date: "2022-10-18"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "c666919c113b368de1613aacac77dcfa9b309f3850a93fe64f2cee9f9fc2d1bb"
source_type: materialize_blog
---

# Notes

- Summary: As an operational data store, Materialize is fundamentally different on the inside, but it's compatible with PostgreSQL in a few important ways.
- Topics inferred from section headings:
  - How and why is Materialize compatible with PostgreSQL?
  - 1. PostgreSQL wire-compatibility
  - Benefits
  - Limitations
  - 2. PostgreSQL syntax consistency
  - Common Syntax between Materialize and PostgreSQL
  - PostgreSQL syntax not implemented in Materialize
  - New Syntax in Materialize
- Key passages captured for indexing (short excerpts):
  - We often say Materialize "presents as Postgres" or that it's "Postgres wire-compatible" and (understandably) sometimes we get asked what that means.
  - For context, Materialize is a streaming-first data store used by data teams to serve operational workloads like business automation, customer-facing features and AI and ML serving. While we present as Postgres externally...
