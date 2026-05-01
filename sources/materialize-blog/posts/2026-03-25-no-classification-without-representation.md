---
title: "No Classification without Represention | Materialize"
url: "https://materialize.com/blog/no-classification-without-representation/"
date: "2026-03-25"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "8e718162d3071523b1b9b2e6cc62b3ba024fa64bf1cb49243ce6bea1625d0269"
source_type: materialize_blog
---

# Notes

- Summary: Learn how Materialize improves query performance by compiling SQL's type system to a simpler system of "representation types"—reducing casts, enabling better optimizations, and increasing efficiency.
- Topics inferred from section headings:
  - No Classification without Represention
  - A string by any other name
  - Casts: correct bookkeeping comes at a cost
  - Representation types: distinctions mean differences
  - Less busywork, more sharing
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - It’s well known that type systems are an avenue to better performance in conventional programming languages. Recently, an overhaul of how Materialize’s optimizer treats types led to better performance in our dataflows co...
  - Materialize presents itself using Postgres’s type system. Much of this type system is standard SQL stuff: exact numeric types (SMALLINT, BIGINT, etc.); datetime types (DATE, TIMESTAMP, etc.); string types (CHARACTER, CHA...
