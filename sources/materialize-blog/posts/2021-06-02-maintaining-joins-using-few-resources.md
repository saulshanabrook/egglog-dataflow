---
title: "Maintaining Joins using Few Resources | Materialize"
url: "https://materialize.com/blog/maintaining-joins-using-few-resources/"
date: "2021-06-02"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "30fbe8e51a7332674fa31910897988b935c3fb580acd064358783bdba774a606"
source_type: materialize_blog
---

# Notes

- Summary: Efficiently maintain joins with shared arrangements & reduce resource usage with Materialize's innovative approach.
- Topics inferred from section headings:
  - Maintaining Joins using Few Resources
  - Materialize
  - Relational Joins
  - Relational Joins on Update Streams
  - Update Streams?
  - Shared Arrangements
  - Connecting the dots
  - Conclusions
- Key passages captured for indexing (short excerpts):
  - Today's post is on a topic that a lot of folks have asked for, once they dive a bit into Materialize.
  - One of our join implementation strategies uses a surprisingly small amount of additional memory: none. "None" is a surprising amount of memory because streaming joins normally need to maintain their inputs indexed in mem...
