# Materialize Blog Source Snapshot

- Crawl date (UTC): 2026-05-01T18:28:54+00:00
- Source boundary: `https://materialize.com/blog/` posts only (`/blog/<slug>/`)
- Discovery method:
  1. Crawl blog landing page (`/blog/`) for visible post links.
  2. Fallback/coverage via `https://materialize.com/sitemap.xml` filtered to `/blog/<slug>/` URLs.
- Capture method:
  - Per post markdown file with YAML frontmatter metadata.
  - Structured notes emphasize metadata, headings, and brief excerpts for indexing.
  - Content hashes computed from parsed headings plus short paragraph samples.
- Limitations:
  - Dynamic page rendering may hide some content from static parsing.
  - Dates/authors/tags depend on page metadata/JSON-LD availability.
  - To reduce copyright risk, files avoid full verbatim article reproduction.
