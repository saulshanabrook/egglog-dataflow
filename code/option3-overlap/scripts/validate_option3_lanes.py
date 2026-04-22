#!/usr/bin/env python3
"""Validate Option 3 experiment lane JSON artifacts.

This intentionally uses only the Python standard library so the experiment
artifacts can be checked in a fresh checkout without installing jsonschema.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


REQUIRED = {
    "experiment_id",
    "lane",
    "workload",
    "command",
    "repo_state",
    "semantic_ok",
    "freshness_ok",
    "early_visibility_violations",
    "barrier_count",
    "barrier_reasons",
    "runtime_us",
    "metrics",
    "verdict",
}


def fail(path: Path, message: str) -> None:
    raise SystemExit(f"{path}: {message}")


def validate_lane(path: Path) -> None:
    with path.open() as f:
        value = json.load(f)

    if not isinstance(value, dict):
        fail(path, "lane artifact must be a JSON object")

    missing = sorted(REQUIRED - value.keys())
    if missing:
        fail(path, f"missing required fields: {', '.join(missing)}")

    for field in ["experiment_id", "lane", "workload", "verdict"]:
        if not isinstance(value[field], str) or not value[field]:
            fail(path, f"{field} must be a non-empty string")

    command = value["command"]
    if not isinstance(command, str) and not (
        isinstance(command, list) and all(isinstance(item, str) for item in command)
    ):
        fail(path, "command must be a string or list of strings")

    repo_state = value["repo_state"]
    if not isinstance(repo_state, dict):
        fail(path, "repo_state must be an object")
    for field in ["root", "git_head", "git_dirty"]:
        if field not in repo_state:
            fail(path, f"repo_state.{field} is required")
    if not isinstance(repo_state["root"], str):
        fail(path, "repo_state.root must be a string")
    if not isinstance(repo_state["git_head"], str):
        fail(path, "repo_state.git_head must be a string")
    if not isinstance(repo_state["git_dirty"], bool):
        fail(path, "repo_state.git_dirty must be a boolean")

    for field in ["semantic_ok", "freshness_ok"]:
        if value[field] is not None and not isinstance(value[field], bool):
            fail(path, f"{field} must be a boolean or null")

    for field in ["early_visibility_violations", "barrier_count", "runtime_us"]:
        if value[field] is not None:
            if not isinstance(value[field], int) or value[field] < 0:
                fail(path, f"{field} must be a non-negative integer or null")

    if not isinstance(value["barrier_reasons"], list) or not all(
        isinstance(item, str) for item in value["barrier_reasons"]
    ):
        fail(path, "barrier_reasons must be a list of strings")

    if not isinstance(value["metrics"], dict):
        fail(path, "metrics must be an object")


def main(argv: list[str]) -> int:
    root = Path(argv[1]) if len(argv) > 1 else Path("findings/artifacts/option-3")
    if root.is_file():
        paths = [root]
    else:
        paths = sorted(
            path
            for path in root.glob("*.json")
            if path.name != "schema.json" and path.name[:2].isdigit()
        )

    if not paths:
        raise SystemExit(f"{root}: no lane artifacts found")

    for path in paths:
        validate_lane(path)

    print(f"validated {len(paths)} Option 3 lane artifact(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
