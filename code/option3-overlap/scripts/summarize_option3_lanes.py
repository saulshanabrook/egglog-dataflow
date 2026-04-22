#!/usr/bin/env python3
"""Build the ordered Option 3 experiment summary from lane artifacts."""

from __future__ import annotations

import json
import sys
from pathlib import Path


def load_lanes(artifact_dir: Path) -> list[dict]:
    lanes = []
    for path in sorted(artifact_dir.glob("*.json")):
        if path.name == "schema.json" or not path.name[:2].isdigit():
            continue
        with path.open() as f:
            value = json.load(f)
        value["_artifact_name"] = path.name
        lanes.append(value)
    return lanes


def yes_no(value: object) -> str:
    if value is True:
        return "pass"
    if value is False:
        return "fail"
    return "n/a"


def perf_summary(lane: dict) -> str:
    metrics = lane.get("metrics", {})
    for key in [
        "performance_summary",
        "runtime_summary",
        "throughput_summary",
        "limitation_summary",
    ]:
        value = metrics.get(key)
        if isinstance(value, str) and value:
            return value
    runtime = lane.get("runtime_us")
    if isinstance(runtime, int):
        return f"{runtime} us"
    return "not measured"


def option_implication(lane: dict) -> str:
    metrics = lane.get("metrics", {})
    value = metrics.get("option_implication")
    if isinstance(value, str) and value:
        return value
    return lane.get("verdict", "")


def render(lanes: list[dict]) -> str:
    lines = [
        "# Option 3 Experiment Index",
        "",
        "This index is generated from the numbered lane artifacts under",
        "`findings/artifacts/option-3`. The lane ordering is dependency order:",
        "schedule baseline, independent gates, dependent planner/scheduler/container",
        "lanes, then integration verdicts.",
        "",
        "| Branch | Key question | Artifact | Semantic | Freshness | Visibility | Barrier | Performance | Option implication |",
        "| --- | --- | --- | --- | --- | --- | --- | --- | --- |",
    ]

    for lane in lanes:
        metrics = lane.get("metrics", {})
        question = metrics.get("key_question", lane.get("workload", ""))
        visibility = lane.get("early_visibility_violations")
        visibility_text = "n/a" if visibility is None else str(visibility)
        barrier = lane.get("barrier_count")
        barrier_text = "n/a" if barrier is None else str(barrier)
        lines.append(
            "| {branch} | {question} | `{artifact}` | {semantic} | {freshness} | {visibility} | {barrier} | {performance} | {implication} |".format(
                branch=lane.get("lane", ""),
                question=str(question).replace("|", "\\|"),
                artifact=lane.get("_artifact_name", ""),
                semantic=yes_no(lane.get("semantic_ok")),
                freshness=yes_no(lane.get("freshness_ok")),
                visibility=visibility_text,
                barrier=barrier_text,
                performance=perf_summary(lane).replace("|", "\\|"),
                implication=option_implication(lane).replace("|", "\\|"),
            )
        )

    lines.extend(
        [
            "",
            "## Verdict Rules",
            "",
            "- Exact overlap strengthens the schedule subclaim only when semantic equivalence, per-rule freshness, and visibility pass.",
            "- Passing semantics with high native-authoritative barrier counts downgrades a permanent adapter, not a single-owner replacement backend.",
            "- Any freshness or early-visibility failure rejects the exact-backend claim unless the failing case is explicitly out of scope.",
            "- WCOJ/planner wins count as component evidence until they integrate with rebuild, scheduler, and container semantics.",
            "",
        ]
    )
    return "\n".join(lines)


def main(argv: list[str]) -> int:
    artifact_dir = Path(argv[1]) if len(argv) > 1 else Path("findings/artifacts/option-3")
    output = (
        Path(argv[2])
        if len(argv) > 2
        else Path("findings/experiments/option-3/README.md")
    )
    lanes = load_lanes(artifact_dir)
    if not lanes:
        raise SystemExit(f"{artifact_dir}: no lane artifacts found")
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(render(lanes), encoding="utf-8")
    print(f"wrote {output} from {len(lanes)} lane artifact(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
