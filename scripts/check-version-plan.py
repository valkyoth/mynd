#!/usr/bin/env python3
"""Validate that every release-plan milestone is a complete handoff."""

from __future__ import annotations

import re
import sys
from pathlib import Path


PLAN = Path("docs/VERSION_PLAN.md")
HEADING = re.compile(r"^#{2,3} (v[0-9][^ ]*) - ")
SUMMARY = re.compile(r"^\| (0\.\d+\.\d+) \|")
REQUIRED = (
    "Status:",
    "Context:",
    "Goal:",
    "Deliverables:",
    "Verification:",
    "Exit criteria:",
)


def main() -> int:
    text = PLAN.read_text(encoding="utf-8")
    lines = text.splitlines()
    releases: list[tuple[str, list[str]]] = []
    current_version: str | None = None
    current_lines: list[str] = []

    for line in lines:
        match = HEADING.match(line)
        if match:
            if current_version is not None:
                releases.append((current_version, current_lines))
            current_version = match.group(1)
            current_lines = []
        elif current_version is not None:
            current_lines.append(line)

    if current_version is not None:
        releases.append((current_version, current_lines))

    errors: list[str] = []
    versions = [version for version, _ in releases]
    if len(versions) != 169:
        errors.append(f"expected 169 release handoffs, found {len(versions)}")
    if len(set(versions)) != len(versions):
        errors.append("release headings contain duplicate versions")

    summary_versions = {
        match.group(1)
        for line in lines
        if (match := SUMMARY.match(line)) is not None
    }
    detailed_0x = {version[1:] for version in versions if version.startswith("v0.")}
    if summary_versions != detailed_0x:
        errors.append("0.x summary and detailed milestone versions differ")

    for version, body_lines in releases:
        positions: list[int] = []
        for label in REQUIRED:
            try:
                positions.append(
                    next(
                        index
                        for index, line in enumerate(body_lines)
                        if line.startswith(label)
                    )
                )
            except StopIteration:
                errors.append(f"{version} is missing {label}")
        if len(positions) == len(REQUIRED) and positions != sorted(positions):
            errors.append(f"{version} milestone sections are out of order")

        stop = (
            f"`{version} implementation stop reached. "
            "Run pentest for this exact commit.`"
        )
        if not any(stop in line for line in body_lines):
            errors.append(f"{version} is missing the exact pentest stop line")

    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1

    print(f"validated {len(releases)} release handoffs in {PLAN}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
