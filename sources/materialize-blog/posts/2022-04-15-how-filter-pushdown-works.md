---
title: "How filter pushdown works | Materialize"
url: "https://materialize.com/blog/how-filter-pushdown-works/"
date: "2022-04-15"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "c34117b327f20ed576d74c095ea744742d0e6f028889a2458618a57556fc65d1"
source_type: materialize_blog
---

# Notes

- Summary: Using part statistics and abstract interpretation to push complex filters all the way down to the storage layer.
- Topics inferred from section headings:
  - How filter pushdown works
  - A toy example
  - Nullability analysis
  - Range analysis
  - Abstract interpretation
  - Abstract values
  - Abstract functions
  - Putting it all together
- Key passages captured for indexing (short excerpts):
  - Let’s imagine I have a database table — maybe a large collection of events, the sort of thing with a created_at timestamp and a few other columns. We’ll also imagine that I want fast, consistent queries as my data change...
  - Materialize splits the data in a durable collection like this into multiple bounded-size parts, and stores each of those parts in an object store like S3. It stores the metadata separately, in a serializable store like C...
