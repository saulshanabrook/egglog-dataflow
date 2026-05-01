---
title: "Consistency Guarantees in Data Streaming | Materialize"
url: "https://materialize.com/blog/consistency/"
date: "2020-03-31"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "2cbfebd03b0b2505459ff75680b31b197b3389400cb9aae82f7405312d36ac8a"
source_type: materialize_blog
---

# Notes

- Summary: Understand the necessary consistency guarantees for a streaming data platform & how they ensure accurate data views.
- Topics inferred from section headings:
  - Consistency Guarantees in Data Streaming | Materialize
  - Definitions
  - Defining consistency
  - Internal consistency
  - Query consistency
  - What about multiple users?
  - What about failures?
  - Enforcing consistency
- Key passages captured for indexing (short excerpts):
  - Editor’s note: Natacha Crooks is currently a visiting researcher at Materialize. Starting in Fall 2020, Natacha will be an assistant professor of computer science at UC Berkeley.
  - The quality of a result for streaming systems is traditionally measured along three axes: latency, how long did it take the system to compute the query, freshness, how up-to-date is the result, and correctness, does the...
