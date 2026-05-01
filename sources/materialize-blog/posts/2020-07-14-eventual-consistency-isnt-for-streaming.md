---
title: "Eventual Consistency isn't for Streaming | Materialize"
url: "https://materialize.com/blog/eventual-consistency-isnt-for-streaming/"
date: "2020-07-14"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "9e1598162b7365c025c5011ad876e658fc252131f627b887504831c9d0e30aad"
source_type: materialize_blog
---

# Notes

- Summary: Understand why eventual consistency isn't suitable for streaming systems & the systematic errors it can cause with Materialize's insights.
- Topics inferred from section headings:
  - Eventual Consistency isn't for Streaming
  - Background on Eventual Consistency
  - Streaming computations
  - Eventual consistency in streaming: example 1
  - Eventual consistency in streaming: example 2
  - Eventual consistency in streaming: example 3
  - Testing for consistency errors
  - Conclusions
- Key passages captured for indexing (short excerpts):
  - Streaming systems consume inputs and produce outputs asyncronously: the output of a system at any moment may not reflect all of the inputs seen so far. These systems provide various guarantees about how their outputs rel...
  - In this post we'll see that for as long as its input streams haven't been stopped, natural eventually consistent computations can produce unboundedly large and systematic errors. If you are doing even slightly non-trivia...
