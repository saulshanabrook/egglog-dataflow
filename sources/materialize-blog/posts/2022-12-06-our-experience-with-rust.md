---
title: "Rust for high-performance concurrency and network services | Materialize"
url: "https://materialize.com/blog/our-experience-with-rust/"
date: "2022-12-06"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "5981ba159ccfd622f6a7193407a9cce3816c5859933ad7cc50977561cc59bd55"
source_type: materialize_blog
---

# Notes

- Summary: Materialize is written in Rust. Why did we make that decision and how has it turned out for the project?
- Topics inferred from section headings:
  - Rust for high-performance concurrency and network services
  - Why Rust and Materialize are a Good Match
  - Guaranteeing Correctness
  - How Using Rust for Materialize Gave Us Actually Fearless Concurrency
  - The Community and the Ecosystem
  - Problems
  - Why We’re Happy Using Rust for Materialize
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - The core execution engine of Materialize is built with Timely Dataflow and Differential Dataflow, both of which are written in Rust (more about that choice here). So it was only natural to build the rest of our services...
  - Rust is designed to be a good choice for many niches. It’s particularly well-suited for the kinds of programs we are writing at Materialize: high-performance concurrency and network services. These are niches that are co...
