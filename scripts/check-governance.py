#!/usr/bin/env python3
"""Fail when Mynd's normative scope, claims, sources, or crate graph drift."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

import tomllib

from spec_schema import SchemaError, validate_manifest_schema

ROOT = Path(__file__).resolve().parent.parent

CLAIMS = (
    "not-implemented",
    "probe-only",
    "structural-parse",
    "defensive-decode",
    "conformant-decode",
    "defensive-encode",
    "conformant-encode",
    "stable",
)

FAMILIES = (
    ("BMP/DIB versioned dialects", "mynd-bmp", "0.20.0", "0.25.2"),
    ("QOI", "mynd-qoi", "0.26.0", "0.27.0"),
    ("Netpbm PNM/PAM", "mynd-netpbm", "0.28.0", "0.33.0"),
    ("farbfeld", "mynd-farbfeld", "0.34.0", "0.34.0"),
    ("PNG Third Edition and APNG", "mynd-png", "0.36.0", "0.46.4"),
    ("GIF87a/89a", "mynd-gif", "0.47.0", "0.51.8"),
    ("Classic JPEG", "mynd-jpeg", "0.52.0", "0.62.0"),
    ("WebP", "mynd-webp", "0.63.0", "0.69.4"),
    ("TIFF 6.0 profiles", "mynd-tiff", "0.70.0", "0.77.14"),
)

EXCLUSIONS = (
    "TGA",
    "PFM",
    "BigTIFF",
    "AVIF/HEIF",
    "JPEG XL",
    "JPEG 2000",
    "JPEG-LS",
    "JPEG XR",
    "JPEG XS",
)


def text(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def toml(path: str) -> dict:
    with (ROOT / path).open("rb") as handle:
        return tomllib.load(handle)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise RuntimeError(message)


def train(first: str, last: str) -> str:
    return f"`v{first}`" if first == last else f"`v{first}-v{last}`"


def check_scope_table() -> None:
    scope = text("docs/SCOPE_AND_CLAIMS.md")
    version_plan = text("docs/VERSION_PLAN.md")
    for family, crate, first, last in FAMILIES:
        row_prefix = f"| {family} | `{crate}` | {train(first, last)} |"
        require(row_prefix in scope, f"scope table drift for {family}")
        require(
            f"### v{first} -" in version_plan,
            f"version plan is missing first {family} handoff v{first}",
        )
        require(
            f"### v{last} -" in version_plan,
            f"version plan is missing final {family} handoff v{last}",
        )


def check_document_scope() -> None:
    readme = text("README.md")
    facade_readme = text("crates/mynd/README.md")
    support = text("FORMAT_SUPPORT.md")
    core_readme = text("crates/mynd-core/README.md")
    implementation = text("docs/IMPLEMENTATION_PLAN.md")
    post_1 = text("docs/POST_1_0_CODEC_PLAN.md")
    sources = text("SPEC_SOURCES.md")
    modularity = text("docs/modularity-policy.md")
    combined_active = "\n".join(
        (readme, facade_readme, support, implementation, sources, modularity)
    )
    require(readme == facade_readme, "facade README differs from repository README")
    require("current 0.2.0 governance phase" in readme, "README phase is stale")
    require('version = "0.2.0"' in readme, "README install version is stale")
    require("0.13.x-0.19.x" in readme, "README shared-contract train is stale")
    require(
        "Version 0.1.0 is a repository foundation only." in core_readme
        and 'version = "0.1.0"' in core_readme,
        "unchanged mynd-core 0.1.0 README drifted",
    )
    require("`v0.20.0-v0.25.2`" in support, "BMP support train is stale")
    require("`v0.34.0`" in support, "farbfeld support handoff is missing")
    for family, crate, _first, _last in FAMILIES:
        family_token = family.split()[0].split("/")[0]
        require(
            family_token.casefold() in combined_active.casefold(),
            f"active documents omit {family}",
        )
        require(crate in text("docs/SCOPE_AND_CLAIMS.md"), f"missing crate {crate}")
    admitted = post_1.split("## Already-admitted 1.0 families", 1)[1].split(
        "## mynd-jxl internal structure", 1
    )[0]
    for token in ("BMP", "QOI", "PNM/PAM", "farbfeld", "PNG/APNG", "GIF"):
        require(token in admitted, f"post-1.0 boundary omits admitted family {token}")
    for excluded in EXCLUSIONS:
        require(excluded in text("docs/SCOPE_AND_CLAIMS.md"), f"scope omits {excluded}")
        require(excluded in support, f"support matrix omits exclusion {excluded}")


def check_claims() -> None:
    scope = text("docs/SCOPE_AND_CLAIMS.md")
    support = text("FORMAT_SUPPORT.md")
    for claim in CLAIMS:
        require(f"`{claim}`" in scope, f"claim taxonomy omits {claim}")
        require(f"`{claim}`" in support, f"support matrix vocabulary omits {claim}")
    support_rows = [
        line
        for line in support.splitlines()
        if line.startswith("| ") and "Not implemented" in line
    ]
    require(len(support_rows) == 10, "format support row count changed without review")
    require(
        all(row.count("Not implemented") == 5 for row in support_rows),
        "an unreviewed format capability is advertised",
    )


def check_source_coverage() -> None:
    document = json.loads(text("specs/SOURCES.json"))
    try:
        validate_manifest_schema(document)
    except SchemaError as error:
        raise RuntimeError(f"source schema validation failed: {error}") from error
    manifest_ids = [source["id"] for source in document["sources"]]
    ledger_ids = re.findall(r"`source:([a-z0-9][a-z0-9.-]*)`", text("docs/STANDARDS_LEDGER.md"))
    duplicates = sorted({item for item in ledger_ids if ledger_ids.count(item) > 1})
    require(not duplicates, f"standards ledger repeats source IDs: {duplicates}")
    require(
        set(ledger_ids) == set(manifest_ids),
        "standards ledger source coverage differs; "
        f"missing={sorted(set(manifest_ids) - set(ledger_ids))}, "
        f"extra={sorted(set(ledger_ids) - set(manifest_ids))}",
    )


def check_release_graph() -> None:
    workspace = toml("Cargo.toml")
    facade = toml("crates/mynd/Cargo.toml")
    core = toml("crates/mynd-core/Cargo.toml")
    release = toml("release-crates.toml")
    require(workspace["workspace"]["package"]["version"] == "0.1.0", "support baseline version")
    require(facade["package"]["version"] == "0.2.0", "mynd facade train version")
    require(
        core["package"]["version"] == {"workspace": True},
        "mynd-core must remain at the 0.1.0 workspace baseline",
    )
    dependency = workspace["workspace"]["dependencies"]["mynd-core"]
    require(dependency["version"] == "=0.1.0", "mynd-core dependency must stay exact")
    require(release["release"]["version"] == "0.2.0", "release plan version drift")
    require(
        release["crates"]["mynd-core"]["change"] == "unchanged"
        and release["crates"]["mynd-core"]["publish"] is False,
        "unchanged mynd-core must not publish",
    )
    require(
        release["crates"]["mynd"]["version"] == "0.2.0"
        and release["crates"]["mynd"]["publish"] is True,
        "mynd 0.2.0 must be the only published package",
    )


def main() -> int:
    try:
        check_scope_table()
        check_document_scope()
        check_claims()
        check_source_coverage()
        check_release_graph()
    except (OSError, KeyError, RuntimeError, ValueError) as error:
        print(f"governance consistency error: {error}", file=sys.stderr)
        return 1
    print("scope, claim, standards, provenance, and crate-version ledgers agree")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
