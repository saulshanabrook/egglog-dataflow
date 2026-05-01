---
title: "Building Differential Dataflow from Scratch | Materialize"
url: "https://materialize.com/blog/differential-from-scratch/"
date: "2023-02-09"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "e110fe754ee050682124ca3a8ccb0a138b33e3a95c99d756d6b2f9110a668dee"
source_type: materialize_blog
---

# Notes

- Summary: Let's build (in Python) the Differential Dataflow framework at the heart of Materialize, and explain what it's doing along the way.
- Topics inferred from section headings:
  - Building Differential Dataflow from Scratch
  - Structure of this Post
  - v0: Intro / What Are We Trying to Compute?
  - concat
  - negate
  - map / filter
  - reduce
  - join
- Key passages captured for indexing (short excerpts):
  - Materialize is an operational data store that delivers sub-second results on the same complex queries that would take down your transactional DB or run overnight in your warehouse. It works by using Differential Dataflow...
  - This post will explain Differential Dataflow by starting from scratch and reimplementing it in Python. Differential Dataflow is carefully engineered to run efficiently across multiple threads, processes, and/or machines,...
