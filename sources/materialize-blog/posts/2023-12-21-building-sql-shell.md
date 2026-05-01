---
title: "How we built the SQL Shell | Materialize"
url: "https://materialize.com/blog/building-sql-shell/"
date: "2023-12-21"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "c8001457aa837035938802780c8aac2bd70f59838c9df262f5fe46ae54d08b5f"
source_type: materialize_blog
---

# Notes

- Summary: Learn how we built an in-browser SQL shell that empowers Materialize users to interact with their databases
- Topics inferred from section headings:
  - How we built the SQL Shell
  - How the Shell Works
  - The SQL Editor
  - Rendering results
  - The WebSocket API
  - Quickstart Tutorial
  - Wrapping Up
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - At Materialize, we strive to meet customers where they are. While we provide our users with an operational data warehouse that presents as PostgreSQL, getting access to a Postgres client (such as psql) and accompanying c...
  - There are some nearly magical technologies that allow developers to run a full x86 virtual machine in WebAssembly and render the framebuffer to a canvas. Using that, one can actually embed real psql in a browser. An earl...
