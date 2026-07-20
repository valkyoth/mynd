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
INCREMENTAL_ORDER = ("v0.12.3", "v0.14.0", "v0.14.1", "v0.14.2")
INCREMENTAL_MARKERS = {
    "v0.12.3": "ordinary hashes are insufficient",
    "v0.14.0": "DecodeStepReport<'a>",
    "v0.14.1": "Yielded",
    "v0.14.2": "generation-bound tokens",
}
STEP_REPORT_MARKERS = (
    "EncodeStepReport<'a>",
    "all eventual Progress",
    "LimitExceeded, Error",
    "explicit lifetime",
    "caller-provided and plan-sized",
    "committing unreportable pixels",
)
FACADE_ORDER = (
    "v0.94.6",
    "v0.95.0",
    "v0.95.1",
    "v0.96.0",
    "v0.96.1",
    "v0.97.0",
    "v0.97.1",
    "v0.98.0",
    "v0.98.1",
    "v0.98.2",
    "v0.98.3",
    "v0.99.0",
    "v0.99.1",
    "v0.99.2",
    "v0.99.3",
    "v0.99.4",
    "v0.99.5",
    "v0.99.6",
)
FACADE_MARKERS = {
    "v0.94.6": "audited candidate baseline",
    "v0.98.3": "Zero unresolved implementation or public-API issues",
    "v0.99.6": "verification-only final public API freeze",
}
ASSURANCE_ONLY = tuple(f"v0.99.{minor}" for minor in range(6))


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
    if len(versions) != 173:
        errors.append(f"expected 173 release handoffs, found {len(versions)}")
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

    if "v0.12.4" in versions:
        errors.append("v0.12.4 must not own incremental decode commit modes")

    try:
        incremental_positions = [versions.index(version) for version in INCREMENTAL_ORDER]
    except ValueError as error:
        errors.append(f"incremental contract handoff is missing: {error}")
    else:
        if incremental_positions != sorted(incremental_positions):
            errors.append("incremental contract handoffs are out of dependency order")

    release_bodies = {version: "\n".join(body) for version, body in releases}
    for version, marker in INCREMENTAL_MARKERS.items():
        if marker not in release_bodies.get(version, ""):
            errors.append(f"{version} is missing incremental marker: {marker}")

    for marker in STEP_REPORT_MARKERS:
        if marker not in release_bodies.get("v0.14.0", ""):
            errors.append(f"v0.14.0 is missing concrete report marker: {marker}")

    try:
        facade_positions = [versions.index(version) for version in FACADE_ORDER]
    except ValueError as error:
        errors.append(f"facade stabilization handoff is missing: {error}")
    else:
        if facade_positions != sorted(facade_positions):
            errors.append("facade stabilization handoffs are out of dependency order")

    for version, marker in FACADE_MARKERS.items():
        if marker not in release_bodies.get(version, ""):
            errors.append(f"{version} is missing facade marker: {marker}")

    for version in ASSURANCE_ONLY:
        body = release_bodies.get(version, "")
        normalized_body = " ".join(body.split())
        if "exact v0.98.3 input" not in normalized_body:
            errors.append(f"{version} is not bound to the v0.98.3 assurance input")
        if "product implementation and public API changes are prohibited" not in normalized_body:
            errors.append(f"{version} does not prohibit post-reconciliation changes")

    if "full public facade freezes only after v0.94.6" in text:
        errors.append("v0.94.6 must remain a facade candidate, not the final freeze")
    if "release blocker no later than\nv0.99.6" in text:
        errors.append("adapter corrections must close at v0.98.3 before assurance")

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
