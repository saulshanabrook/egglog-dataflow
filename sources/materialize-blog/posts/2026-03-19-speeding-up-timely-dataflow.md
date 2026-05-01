---
title: "Speeding up Timely Dataflow by 100x | Materialize"
url: "https://materialize.com/blog/speeding-up-timely-dataflow/"
date: "2026-03-19"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "0570ebb20b9f903644ba715ac62d473ddd436984e52eb879cd4526ab14798f19"
source_type: materialize_blog
---

# Notes

- Summary: A great example of why timely dataflow's approach to progress tracking can be orders of magnitude more efficient than other stream processors.
- Topics inferred from section headings:
  - Speeding up Timely Dataflow by 100x
  - The set-up: big dataflows
  - A reality check
  - Smartness: tracking progress in timely dataflow
  - Opting out of timestamp progress
  - Conclusions
  - Technical details and sneaky caveats
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - A bit of a bait title, but by the end of the post we will have sped up a timely dataflow computation by 100x. We won't have changed the computation, just a flag that it uses. We will also see a great example of why timel...
  - We're going to start with the problem, which is a real enough problem we see at Materialize.
