---
title: "Capturing Change Data Capture (CDC) Data | Materialize"
url: "https://materialize.com/blog/capturing-cdc-data/"
date: "2023-08-01"
authors:
  - "Unknown"
categories:
  - "Unspecified"
tags:
  - "Unspecified"
retrieved_at: "2026-05-01T18:28:54+00:00"
content_hash: "daa291c83c1e635cdd7a4d5416d08785875f98436ea8ccecfc0c95085dcd5b45"
source_type: materialize_blog
---

# Notes

- Summary: An illustration of the unexpectedly high downstream cost of clever optimizations to change data capture.
- Topics inferred from section headings:
  - Capturing Change Data Capture (CDC) Data
  - CDC representations
  - Downstream uses, and burden
  - Maintaining SELECT COUNT(*)
  - Maintaining SELECT age, COUNT(*)
  - Maintaining SELECT age, zip, COUNT(*)
  - Looping back around
  - Related Resources
- Key passages captured for indexing (short excerpts):
  - Change Data Capture (CDC) describes the process of recording and communicating how a collection of data changes. There are several ways to do this, ranging from the rather simple to the seemingly quite clever. However, i...
  - The cost of cleverness is often invisible to the CDC provider, and is borne instead by the recipient. It is not necessarily a bad call to move cost from the CDC provider to the recipient, but it's worth knowing the cost....
