---
title: "Subscribe to changes in a view with Materialize | Materialize"
url: "https://materialize.com/blog/subscribe-to-changes-in-a-view-with-tail-in-materialize/"
date: "2022-03-03"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "e8970a3829d2174ded954051fb215f81deeef7a514ae364fbafd64b2d50691f9"
source_type: materialize_blog
---

# Notes

- Summary: Real-time SQL query & view update subscriptions are made simple with Materialize's SUBSCRIBE feature.
- Topics inferred from section headings:
  - Subscribe to changes in a view with Materialize
  - Subscription Example
  - Subscriptions with Snapshots
  - Subscriptions with custom Compaction Windows
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - Note: When this article was published, Materialize used TAIL syntax for the query subscription primitive. Code snippets and references have been updated to reflect the current SUBSCRIBE syntax. For more info, see SUBSCRI...
  - Most of the internet is built on a "pull" or "request" paradigm: A user loads a page or takes an action, a backend does some work and sends a response. Job done.
