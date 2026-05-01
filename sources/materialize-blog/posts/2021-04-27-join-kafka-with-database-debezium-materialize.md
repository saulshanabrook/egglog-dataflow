---
title: "Join Kafka with a Database using Debezium and Materialize | Materialize"
url: "https://materialize.com/blog/join-kafka-with-database-debezium-materialize/"
date: "2021-04-27"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "74d073c0b2d43dea9c4becbe338102ff0b77a58d2ee66cfe4374e5b6983a734c"
source_type: materialize_blog
---

# Notes

- Summary: Debezium and Materialize can be used as powerful tools for joining high-volume streams of data from Kafka and tables from databases.
- Topics inferred from section headings:
  - Join Kafka with a Database using Debezium and Materialize
  - The Problem
  - Solution: Stream the database to Kafka, materialize a view
  - Table of Contents
  - Learn about the components
  - Debezium
  - Materialize
  - Build the solution
- Key passages captured for indexing (short excerpts):
  - We need to provide (internal or end-user) access to a view of data that combines a fast-changing stream of events from Kafka with a table from a database (which is also changing).
  - Here are a few real-world examples where this problem comes up:
