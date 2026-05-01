---
title: "Virtual Time for Scalable Performance | Materialize"
url: "https://materialize.com/blog/virtual-time-consistency-scalability/"
date: "2022-06-14"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "af91d43c1a90bd5e23c465f3d70340160dbd018e3f7a807df13f8f8217b64b12"
source_type: materialize_blog
---

# Notes

- Summary: The key to Materialize's ability to separate compute from storage and scale horizontally without sacrificing consistency is a concept called virtual time.
- Topics inferred from section headings:
  - Virtual Time for Scalable Performance | Materialize
  - Some context
  - Materialize's consistency mechanism
  - Materialize's Unbundled Architecture
  - Storage: Writing things down
  - Compute: Transforming data
  - Adapter: Serving results
  - Putting the pieces back together
- Key passages captured for indexing (short excerpts):
  - Materialize allows you to frame SQL queries against continually changing data, and it will compute, maintain, and serve the answers even as the underlying data change.
  - Consistency is a watchword at Materialize. We are able to maintain query outputs that at all times correspond exactly to their inputs. This is a solution to the cache invalidation problem, one of the core hard problems i...
