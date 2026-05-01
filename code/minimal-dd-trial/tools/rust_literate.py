#!/usr/bin/env python3
"""Render a Rust source file as a small literate-programming Markdown file.

The renderer intentionally keeps the format simple:

- full-line `//!`, `///`, and `//` comments become Markdown prose;
- `docs:` directives in those comments control rendering and are omitted;
- all other source lines become fenced Rust code blocks;
- blank lines are preserved inside code blocks and coalesced in prose.

This is not a Rust parser. It is a deterministic documentation extractor for
the tutorial comments in `src/lib.rs`.
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path


COMMENT_PATTERNS = (
    re.compile(r"^\s*//!\s?(.*)$"),
    re.compile(r"^\s*///\s?(.*)$"),
    re.compile(r"^\s*//\s?(.*)$"),
)


DIRECTIVE_PREFIX = "docs:"


def comment_text(line: str) -> str | None:
    """Return Markdown text for a full-line Rust comment, if this line is one."""
    for pattern in COMMENT_PATTERNS:
        match = pattern.match(line)
        if match:
            return match.group(1).rstrip()
    return None


def append_blank(lines: list[str]) -> None:
    """Append one blank line unless the output is already separated."""
    if lines and lines[-1] != "":
        lines.append("")


def directive(comment: str) -> tuple[str, str | None] | None:
    """Parse a renderer directive from comment text.

    Supported forms:

- `docs:show-code`
- `docs:collapse-code title="Summary"`
- `docs:collapse-next-code title="Summary"`
- `docs:collapse-section title="Summary"`
- `docs:end-collapse-section`
    """
    if not comment.startswith(DIRECTIVE_PREFIX):
        return None

    body = comment[len(DIRECTIVE_PREFIX) :].strip()
    if not body:
        return ("", None)

    parts = body.split(maxsplit=1)
    name = parts[0]
    rest = parts[1] if len(parts) > 1 else ""
    title = None
    match = re.search(r'title="([^"]+)"', rest)
    if match:
        title = match.group(1)
    return name, title


def render(source: str, source_path: Path) -> str:
    """Convert Rust comments and code into Markdown prose and Rust code fences."""
    output = [
        f"<!-- Generated from `{source_path.as_posix()}` by `tools/rust_literate.py`; do not edit by hand. -->",
        "",
    ]
    code_open = False
    code_details_open = False
    section_details_open = False
    collapse_code = False
    collapse_title = "Show code"
    collapse_next_title: str | None = None

    def close_code() -> None:
        nonlocal code_open, code_details_open
        if code_open:
            output.append("```")
            output.append("")
            if code_details_open:
                output.append("</details>")
                output.append("")
            code_open = False
            code_details_open = False

    def close_section_details() -> None:
        nonlocal section_details_open
        if section_details_open:
            close_code()
            output.append("</details>")
            output.append("")
            section_details_open = False

    def open_code() -> None:
        nonlocal code_open, code_details_open, collapse_next_title
        title = collapse_next_title
        if title is None and collapse_code:
            title = collapse_title
        collapse_next_title = None

        append_blank(output)
        if title:
            output.append("<details>")
            output.append(f"<summary>{title}</summary>")
            output.append("")
            code_details_open = True
        output.append("```rust")
        code_open = True

    for raw_line in source.splitlines():
        comment = comment_text(raw_line)
        if comment is not None:
            close_code()
            parsed = directive(comment)
            if parsed is not None:
                name, title = parsed
                if name == "show-code":
                    collapse_code = False
                elif name == "collapse-code":
                    collapse_code = True
                    collapse_title = title or "Show code"
                elif name == "collapse-next-code":
                    collapse_next_title = title or "Show code"
                elif name == "collapse-section":
                    close_section_details()
                    append_blank(output)
                    output.append("<details>")
                    output.append(f"<summary>{title or 'Show code'}</summary>")
                    output.append("")
                    section_details_open = True
                    collapse_code = False
                elif name == "end-collapse-section":
                    close_section_details()
                else:
                    raise ValueError(f"unknown docs directive: {name}")
                continue
            if comment:
                output.append(comment)
            else:
                append_blank(output)
            continue

        if raw_line.strip() == "":
            if code_open:
                output.append("")
            else:
                append_blank(output)
            continue

        if not code_open:
            open_code()
        output.append(raw_line.rstrip())

    close_code()
    close_section_details()
    while output and output[-1] == "":
        output.pop()
    return "\n".join(output) + "\n"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()

    source = args.source.read_text()
    rendered = render(source, args.source)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(rendered)


if __name__ == "__main__":
    main()
