---
title: "Scaling Beyond Memory: How Materialize Uses Swap for Larger Workloads | Materialize"
url: "https://materialize.com/blog/scaling-beyond-memory/"
date: "2025-09-18"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "443ac9942e99e5e09f5f69b7e6c26e02ab8b827a58edea870b6744c97dbaf70f"
source_type: materialize_blog
---

# Notes

- Summary: Materialize now uses swap to scale maintained SQL objects beyond RAM, delivering faster hydration, more efficient memory use, and support for larger datasets.
- Topics inferred from section headings:
  - Scaling Beyond Memory: How Materialize Uses Swap for Larger Workloads
  - Phase 1: manually manage data that can spill to disk
  - Phase 2: let the operating system page memory
  - Swap in production
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - At Materialize, we often ask ourselves which parts of our system we could fundamentally change to enable new workloads. How we manage memory for maintained SQL objects is one such area. In this post, I'll explain our pre...
  - Users value Materialize for its data freshness. Results are always up-to-date, and we precisely report how quickly we respond to upstream changes. Materialize transforms SQL into differential dataflow programs that incre...
