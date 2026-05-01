---
title: "Direct PostgreSQL Replication Stream Setup | Materialize"
url: "https://materialize.com/blog/connecting-materialize-directly-to-postgresql-via-the-replication-stream/"
date: "2022-02-16"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "22a51d4f35784380c123fb167f3d4975a2ad2822a9de29282b4b9bae57d043b1"
source_type: materialize_blog
---

# Notes

- Summary: Comprehensive guide on using PostgreSQL's write-ahead log as a data source for Materialize, with technical insights & benefits.
- Topics inferred from section headings:
  - Direct PostgreSQL Replication Stream Setup | Materialize
  - Sourcing data directly from PostgreSQL
  - Enter the PostgreSQL logical replication protocol
  - Interpreting logical replication messages in order
  - Constructing a logical log
  - Ordering gotchas
  - Cold Starts / Resuming
  - Putting it all together
- Key passages captured for indexing (short excerpts):
  - When someone says "event driven", most of us immediately think about consuming events from a message broker, like Kafka. That might be essential for LinkedIn-scale, but it's not necessary for all event-driven architectur...
  - When Petros Angelatos joined the Materialize engineering team, he proposed a feature that would allow Materialize users to build event-driven architectures without requiring the complexities of message brokers. How? By c...
