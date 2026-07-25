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
    "v0.94.8",
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
    "v0.98.4",
    "v0.98.5",
    "v0.98.6",
    "v0.98.7",
    "v0.98.8",
    "v0.99.0",
    "v0.99.1",
    "v0.99.2",
    "v0.99.3",
    "v0.99.4",
    "v0.99.5",
    "v0.99.6",
    "v0.99.7",
    "v0.99.8",
    "v0.99.9",
    "v0.99.10",
    "v0.99.11",
    "v0.99.12",
    "v0.99.13",
    "v0.99.14",
    "v0.99.15",
    "v0.99.16",
    "v0.99.17",
    "v0.99.18",
)
FACADE_MARKERS = {
    "v0.94.8": "audited candidate baseline",
    "v0.98.8": "Zero unresolved implementation or public-API issues",
    "v0.99.18": "verification-only final public API freeze",
}
ASSURANCE_ONLY = tuple(f"v0.99.{minor}" for minor in range(18))
PRE_1_FARBFELD_MARKERS = {
    "v0.2.0": "reconciled image list and pre-1.0 implementation plan must name farbfeld",
    "v0.34.0": "farbfeld decode and encode",
    "v1.0.0-rc.1": "production image-format matrix with farbfeld decode and encode",
    "v1.0.0": "promoted support matrix still includes the exact audited farbfeld",
}
BMP_ORDER = (
    "v0.20.0",
    "v0.20.1",
    "v0.20.2",
    "v0.20.3",
    "v0.21.0",
    "v0.22.0",
    "v0.23.0",
    "v0.24.0",
    "v0.25.0",
    "v0.25.1",
    "v0.25.2",
)
BMP_MARKERS = {
    "v0.20.0": "machine-readable cross-product",
    "v0.20.1": "RGBTRIPLE",
    "v0.20.2": "40/108/124-byte dispatch",
    "v0.20.3": "BA/IC/CI/PT/CP",
    "v0.21.0": "Per-header depth/palette/stride/orientation matrix",
    "v0.22.0": "BI_ALPHABITFIELDS decision",
    "v0.23.0": "compressed top-down rejection",
    "v0.24.0": "Calibrated/sRGB/profile/intent revision matrix",
    "v0.25.0": "Per-dialect encoder capability matrix",
    "v0.25.1": "Header/depth encoder matrix",
    "v0.25.2": "no wildcard, nearest-version, or fallback claims",
}
SIZING_ORDERS = {
    "source corpus": ("v0.2.0", "v0.2.1", "v0.3.0"),
    "ICC and shared metadata syntax": tuple(f"v0.15.{minor}" for minor in range(10)),
    "PNG framing": ("v0.36.0", "v0.36.1", "v0.36.2", "v0.37.0"),
    "Deflate": ("v0.38.0", "v0.38.1", "v0.38.2", "v0.39.0"),
    "PNG color metadata": ("v0.44.0", "v0.44.1", "v0.44.2"),
    "APNG": ("v0.46.0", "v0.46.1", "v0.46.2", "v0.46.3", "v0.46.4"),
    "JPEG declarations": (
        "v0.52.0",
        "v0.52.1",
        "v0.52.2",
        "v0.52.3",
        "v0.53.0",
        "v0.53.1",
    ),
    "TIFF compression": (
        "v0.71.0",
        "v0.71.1",
        "v0.72.0",
        "v0.72.1",
        "v0.72.2",
        "v0.72.3",
    ),
    "TIFF layout": ("v0.74.0", "v0.74.1", "v0.74.2"),
    "TIFF color": tuple(f"v0.75.{minor}" for minor in range(6)),
    "JPEG metadata": tuple(f"v0.60.{minor}" for minor in range(4)),
    "VP8L transforms": (
        "v0.68.0",
        "v0.68.1",
        "v0.68.2",
        "v0.68.3",
        "v0.68.4",
        "v0.68.5",
    ),
    "TIFF encoders": tuple(f"v0.77.{minor}" for minor in range(15)),
    "drawing": tuple(f"v0.92.{minor}" for minor in range(1, 6)),
    "selective decode": (
        "v0.94.1",
        "v0.94.2",
        "v0.94.3",
        "v0.94.4",
        "v0.94.5",
    ),
    "Exif and XMP": (
        "v0.80.0",
        "v0.80.1",
        "v0.80.2",
        "v0.80.3",
        "v0.81.0",
        "v0.81.1",
        "v0.81.2",
        "v0.81.3",
        "v0.81.4",
    ),
}
SIZING_MARKERS = {
    "v0.2.1": "public, offline, or manual",
    "v0.15.6": "ISO 21496-1:2025",
    "v0.15.7": "patent/licensing review",
    "v0.15.9": "registry-independent",
    "v0.30.0": "explicit linear/sRGB variant policy",
    "v0.32.0": "linear opacity",
    "v0.60.3": "SPIFF structural support",
    "v0.75.4": "all eight defined values",
    "v0.75.5": "PrimaryChromaticities",
    "v0.80.3": "DC-008/DC-010",
    "v0.81.1": "Reject DTDs",
    "v0.81.2": "no URI dereference",
    "v0.33.0": "PFM has incompatible",
    "v0.72.3": "separately sourced Adobe extension",
    "v0.68.2": "distinct from the entropy color cache",
    "v0.92.5": "fonts",
}
RETIRED_OVERSIZED_TITLES = (
    "PNG source map, signature, chunk state machine, CRC, ordering",
    "PNG cHRM/gAMA/sRGB/iCCP/cICP/mDCV/cLLI and precedence",
    "JPEG source map, marker/segment parser, frame/scan/table declarations",
    "TIFF LZW, Deflate, and horizontal predictors",
    "Tiles, planar layouts, multipage/SubIFD traversal",
    "CLI decode/encode/convert/frame operations",
    "Kani Deflate/LZW/Huffman/JPEG/WebP/TIFF state proofs",
    "VP8L predictor, subtract-green, color-index, and cache transforms",
    "TIFF tile, planar, and multipage encoders",
    "TIFF extended color and JPEG encoders",
    "Miri, sanitizers, target, feature, and stack audit",
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
    if len(versions) != 269:
        errors.append(f"expected 269 release handoffs, found {len(versions)}")
    if len(set(versions)) != len(versions):
        errors.append("release headings contain duplicate versions")

    summary_start = lines.index("## Milestone summary")
    summary_end = next(
        index
        for index, line in enumerate(lines[summary_start + 1:], summary_start + 1)
        if line.startswith("## Phase:")
    )
    summary_version_list = [
        match.group(1)
        for line in lines[summary_start:summary_end]
        if (match := SUMMARY.match(line)) is not None
    ]
    summary_versions = set(summary_version_list)
    detailed_0x = {version[1:] for version in versions if version.startswith("v0.")}
    if summary_versions != detailed_0x:
        errors.append("0.x summary and detailed milestone versions differ")
    detailed_0x_list = [
        version[1:] for version in versions if version.startswith("v0.")
    ]
    if summary_version_list != detailed_0x_list:
        errors.append("0.x summary and detailed milestone order differs")
    numeric_versions = [
        tuple(int(component) for component in version.split("."))
        for version in detailed_0x_list
    ]
    if numeric_versions != sorted(numeric_versions):
        errors.append("0.x handoffs are not in monotonically increasing SemVer order")

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
        if "exact v0.98.8 input" not in normalized_body:
            errors.append(f"{version} is not bound to the v0.98.8 assurance input")
        if "product implementation and public API changes are prohibited" not in normalized_body:
            errors.append(f"{version} does not prohibit post-reconciliation changes")

    for version, marker in PRE_1_FARBFELD_MARKERS.items():
        normalized_body = " ".join(release_bodies.get(version, "").split())
        if marker not in normalized_body:
            errors.append(f"{version} is missing pre-1.0 farbfeld marker: {marker}")

    try:
        bmp_positions = [versions.index(version) for version in BMP_ORDER]
    except ValueError as error:
        errors.append(f"BMP dialect handoff is missing: {error}")
    else:
        if bmp_positions != sorted(bmp_positions):
            errors.append("BMP dialect handoffs are out of dependency order")

    for version, marker in BMP_MARKERS.items():
        normalized_body = " ".join(release_bodies.get(version, "").split())
        if marker not in normalized_body:
            errors.append(f"{version} is missing BMP dialect marker: {marker}")

    for label, order in SIZING_ORDERS.items():
        try:
            positions = [versions.index(version) for version in order]
        except ValueError as error:
            errors.append(f"{label} sizing handoff is missing: {error}")
        else:
            if positions != sorted(positions):
                errors.append(f"{label} sizing handoffs are out of order")

    for version, marker in SIZING_MARKERS.items():
        normalized_body = " ".join(release_bodies.get(version, "").split())
        if marker not in normalized_body:
            errors.append(f"{version} is missing sizing marker: {marker}")

    if "## Release sizing invariant" not in text:
        errors.append("release sizing invariant is missing")
    for retired_title in RETIRED_OVERSIZED_TITLES:
        if retired_title in text:
            errors.append(f"retired oversized milestone returned: {retired_title}")
    if "full public facade freezes only after v0.94.8" in text:
        errors.append("v0.94.8 must remain a facade candidate, not the final freeze")
    if "release blocker no later than\nv0.99.18" in text:
        errors.append("adapter corrections must close at v0.98.8 before assurance")

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
