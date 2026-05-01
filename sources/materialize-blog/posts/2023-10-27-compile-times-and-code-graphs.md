---
title: "Compile Times and Code Graphs | Materialize"
url: "https://materialize.com/blog/compile-times-and-code-graphs/"
date: "2023-10-27"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "ad0b3c5de3f6d75c990ca1c5b68bc72bdfc5fab3f3ac614a2617dd94b9a68bc5"
source_type: materialize_blog
---

# Notes

- Summary: Recently, I've felt the pain of long Rust compile times at Materialize, and so was motived to improve them a bit. Here's how I did it.
- Topics inferred from section headings:
  - Compile Times and Code Graphs
  - Ideal Code Dependency Structure
  - A Pattern Emerges
  - Case Study: Materialize Storage
  - Case Study: Materialize Stash
  - Difficulties
  - Footnotes
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - This is a Materialize engineering post originally published on Dan's blog at https://blog.danhhz.com/compile-times-and-code-graphs
  - At Materialize, Rust compile times are a frequent complaint. On one hand, I'm forever anchored by the Scala compile times from my days at Foursquare; a clean build without cache hits took over an hour. On the other, Go a...
