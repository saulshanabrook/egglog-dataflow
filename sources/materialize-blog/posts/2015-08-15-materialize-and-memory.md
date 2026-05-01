---
title: "Materialize and Memory | Materialize"
url: "https://materialize.com/blog/materialize-and-memory/"
date: "2015-08-15"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "b8764a59d6965b18dec226f1de5edb0264c4324057e5e8a7eb49757bd76c789b"
source_type: materialize_blog
---

# Notes

- Summary: We reduced memory requirements for many users by nearly 2x, resulting in significant cost-savings.
- Topics inferred from section headings:
  - Materialize and Memory
  - The Fundemantals of Remembered Things
  - A First Evolution, circa many years ago
  - A More Thorough Accounting
  - Optimization
  - Further Optimization and Future Work
  - Wrapping Up
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - Materialize keeps your SQL views up to date as the underlying data change. The value Materialize provides comes from how promptly it reflects new data, but its cost comes from the computer resources needed to achieve thi...
  - Materialize maintains your source and derived data (e.g. any materialized view), durably in economical cloud storage. However, to promptly maintain views and serve results, we want to use much more immediately accessible...
