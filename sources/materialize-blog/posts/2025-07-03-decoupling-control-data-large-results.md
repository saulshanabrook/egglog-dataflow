---
title: "Decoupling Control and Data: Better Architecture Through Larger Results | Materialize"
url: "https://materialize.com/blog/decoupling-control-data-large-results/"
date: "2025-07-03"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "ee664e36754e1f78d0a076ec5157018775041afa92afae466f59c63a0de300f3"
source_type: materialize_blog
---

# Notes

- Topics inferred from section headings:
  - Decoupling Control and Data: Better Architecture Through Larger Results
  - The Problem
  - The Solution: Out-of-Band Data Transfer
  - Implementation Details
  - Architecture Benefits
  - Broader Implications
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - For data processing systems, there is an ongoing tension between control complexity and data throughput. Control paths need coordination and correctness guarantees, while data paths need bandwidth and efficiency. When th...
  - I recently worked on a feature that illustrates this tension nicely: lifting the result size limitation for SELECT queries in Materialize. On the surface, this is about allowing users to retrieve larger result sets. But...
