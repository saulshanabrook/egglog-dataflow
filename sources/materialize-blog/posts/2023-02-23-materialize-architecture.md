---
title: "The Software Architecture of Materialize | Materialize"
url: "https://materialize.com/blog/materialize-architecture/"
date: "2023-02-23"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "153f3ade61370210699a3f65f26c8d698e3a62c9e4091c4571b7866260b5e987"
source_type: materialize_blog
---

# Notes

- Summary: Materialize aims to be usable by anyone who knows SQL, but for those interested in going deeper and understanding the architecture powering Materialize, this post is for you!
- Topics inferred from section headings:
  - The Software Architecture of Materialize
  - Introduction
  - Logical Structure
  - Key Abstraction: Persist and pTVCs
  - Storage
  - Adapter
  - Compute
  - Physical Structure
- Key passages captured for indexing (short excerpts):
  - Materialize is a fast, distributed SQL database built on streaming internals. Data and software engineering teams use it to build apps and services where data must be processed and served at speeds and scales not possibl...
  - Materialize is divided into three logical components: Storage (including Persist), Adapter, and Compute. These are hosted by two physical components: environmentd and clusterd. Broadly speaking, clusterd handles data pla...
