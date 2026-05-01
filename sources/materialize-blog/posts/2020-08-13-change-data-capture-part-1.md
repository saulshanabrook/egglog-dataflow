---
title: "Change Data Capture (part 1) | Materialize"
url: "https://materialize.com/blog/change-data-capture-part-1/"
date: "2020-08-13"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "8a9c7bdad0c6a0d017db17de7229de516554f533352abe5455a0b3792a479480"
source_type: materialize_blog
---

# Notes

- Summary: Here we set the context for and propose a change data capture protocol: a means of writing down and reading back changes to data.
- Topics inferred from section headings:
  - Change Data Capture (part 1)
  - Data that Change
  - Anomalies and Opportunities
  - Materialize CDCv2
  - A Sketch
  - An Implementation
  - An Iterator
  - Conclusions
- Key passages captured for indexing (short excerpts):
  - At Materialize we traffic in computation over data that change. As a consequence, it is important to have a way to write down and read back changes to data. An unambiguous, robust, and performant way to write down and re...
  - What makes this challenging? Why not just write out a log of whatever happens to the data?
