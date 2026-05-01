---
title: "Speeding up Materialize CI | Materialize"
url: "https://materialize.com/blog/speeding-up-materialize-ci/"
date: "2025-08-08"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "d1534d6efe1696c06edd4ed9e24dd9efebebe545ea4c0a053d8c94829d8c65e3"
source_type: materialize_blog
---

# Notes

- Summary: How we slashed CI runtime for Materialize by up to 86% through smarter builds, caching, parallelization, and clever tooling.
- Topics inferred from section headings:
  - Speeding up Materialize CI
  - Pipeline creation
  - Builds
  - Lints & cargo test
  - SQL Logic Tests
  - Other Tests
  - Hetzner Agent Provisioning
  - Eat my Data
- Key passages captured for indexing (short excerpts):
  - In the previous post I talked about how we test Materialize. This time I’ll describe how I significantly sped up our Continuous Integration (CI) Test pipeline in July, especially for pull requests that require a build an...
  - We always kept CI runtime in mind, but it still slowly crept up over the years through adding tests, the code itself growing larger, as well as hundreds of minor cuts adding up.
