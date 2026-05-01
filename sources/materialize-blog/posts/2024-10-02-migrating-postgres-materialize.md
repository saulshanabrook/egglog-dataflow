---
title: "Migrating from dbt-postgres to dbt-materialize | Materialize"
url: "https://materialize.com/blog/migrating-postgres-materialize/"
date: "2024-10-02"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "005740ca9a27ecc1ed94bcaf7e03cf8768800fe264066675e03fd6d7ce096895"
source_type: materialize_blog
---

# Notes

- Summary: In this guide, we’ll show you how to migrate your existing PostgreSQL dbt project to Materialize with minimal SQL tweaks.
- Topics inferred from section headings:
  - Migrating from dbt-postgres to dbt-materialize
  - Materialize’s dbt Adapter: Standard dbt + New Streaming Functionality
  - Step-by-Step Walkthrough: How to Install dbt-materialize
  - Migrating Model Types: What You Need to Know
  - Change #1 - Tables Become Views with Indexes
  - Change #2 - Incremental Models Become Views with Indexes
  - Change #3 - Materialized Views: Data Sharing, Complex Logic
  - Change #4 - Temporal Filters
- Key passages captured for indexing (short excerpts):
  - We'll be in attendance at dbt's upcoming Coalesce 2024 conference next week, and we look forward to seeing you there! Our very own Steffen Hausmann — Field Engineer at Materialize — will speak with Wolf Rendall — Directo...
  - In the Vontive use case, the team needed to port over data models from PostgreSQL into Materialize in order to power real-time loan underwriting. With the spotlight on this use case, we wanted to highlight how to perform...
