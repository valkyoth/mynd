#!/usr/bin/env python3
"""Shared release-cadence and crate-version policy for Mynd tooling."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover - release host guard.
    print("Python 3.11+ is required because this script uses tomllib.", file=sys.stderr)
    raise


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_PLAN = ROOT / "release-crates.toml"
CHANGE_KINDS = ("code", "bugfix", "dependency", "metadata", "unchanged")
RELEASE_KINDS = ("engineering", "publication", "emergency")


def parse_version(version: str) -> tuple[int, int, int]:
    """Parse the numeric pre-1.0 versions used by the current release train."""
    parts = version.split(".")
    if len(parts) != 3:
        raise RuntimeError(f"version must be MAJOR.MINOR.PATCH: {version}")
    try:
        parsed = tuple(int(part) for part in parts)
    except ValueError as exc:
        raise RuntimeError(f"version must be numeric: {version}") from exc
    if any(part < 0 for part in parsed):
        raise RuntimeError(f"version components must be non-negative: {version}")
    return parsed  # type: ignore[return-value]


def normal_release_kind(version: str) -> str:
    major, minor, patch = parse_version(version)
    if major != 0:
        raise RuntimeError("numeric cadence policy currently covers the pre-1.0 train")
    if minor < 5:
        return "publication"
    if minor <= 95 and patch == 0 and minor % 5 == 0:
        return "publication"
    return "engineering"


def next_publication_checkpoint(version: str) -> str:
    major, minor, _patch = parse_version(version)
    if major != 0:
        raise RuntimeError("numeric cadence policy currently covers the pre-1.0 train")
    if normal_release_kind(version) == "publication":
        return version
    next_minor = ((minor // 5) + 1) * 5
    if next_minor <= 95:
        return f"0.{next_minor}.0"
    return "1.0.0-rc.1"


def load_plan(path: Path = DEFAULT_PLAN) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def validate_release(release: dict) -> None:
    version = release.get("version")
    kind = release.get("kind")
    checkpoint = release.get("checkpoint")
    previous_publication = release.get("previous_publication")
    if not all(
        isinstance(value, str)
        for value in (version, kind, checkpoint, previous_publication)
    ):
        raise RuntimeError("release plan has incomplete cadence metadata")
    if kind not in RELEASE_KINDS:
        raise RuntimeError(f"invalid release kind {kind!r}")
    parse_version(version)
    parse_version(previous_publication)
    if parse_version(previous_publication) >= parse_version(version):
        raise RuntimeError("previous publication must precede the planned release")

    expected_kind = normal_release_kind(version)
    expected_checkpoint = next_publication_checkpoint(version)
    if kind == "emergency":
        reason = release.get("emergency_reason")
        if not isinstance(reason, str) or not reason.strip():
            raise RuntimeError("emergency publication requires emergency_reason")
        if checkpoint != version:
            raise RuntimeError("emergency publication checkpoint must equal its version")
    else:
        if kind != expected_kind:
            raise RuntimeError(
                f"{version} must be a {expected_kind} release, not {kind}"
            )
        if checkpoint != expected_checkpoint:
            raise RuntimeError(
                f"{version} must use publication checkpoint {expected_checkpoint}"
            )


def validate_source_change(package_name: str, entry: dict, release: str) -> None:
    previous = entry.get("previous_version")
    published = entry.get("published_version")
    version = entry.get("version")
    change = entry.get("change")
    publish = entry.get("publish")
    reason = entry.get("reason")
    if not all(
        isinstance(value, str)
        for value in (previous, published, version, change, reason)
    ):
        raise RuntimeError(f"{package_name} has incomplete release plan metadata")
    if change not in CHANGE_KINDS:
        raise RuntimeError(f"{package_name} has invalid change kind {change!r}")
    if not isinstance(publish, bool):
        raise RuntimeError(f"{package_name} publish must be true or false")

    previous_parts = parse_version(previous)
    published_parts = parse_version(published)
    planned_parts = parse_version(version)
    release_parts = parse_version(release)
    if published_parts > previous_parts:
        raise RuntimeError(
            f"{package_name} published_version cannot exceed previous_version"
        )

    if change == "code":
        expected = (
            release_parts
            if package_name == "mynd"
            else (previous_parts[0], previous_parts[1] + 1, 0)
        )
    elif change in ("bugfix", "dependency", "metadata"):
        if package_name == "mynd":
            expected = release_parts
        else:
            expected = (previous_parts[0], previous_parts[1], previous_parts[2] + 1)
    else:
        expected = previous_parts

    if planned_parts != expected:
        expected_text = ".".join(str(part) for part in expected)
        raise RuntimeError(
            f"{package_name} {change} change requires version {expected_text}"
        )


def validate_publication_state(package_name: str, entry: dict, kind: str) -> None:
    published = parse_version(entry["published_version"])
    planned = parse_version(entry["version"])
    publish = entry["publish"]
    if planned < published:
        raise RuntimeError(f"{package_name} cannot move behind its published version")
    if kind == "engineering" and publish:
        raise RuntimeError(f"{package_name} cannot publish at an engineering checkpoint")
    if publish and planned == published:
        raise RuntimeError(f"{package_name} version is already published")


def validated_plan(path: Path = DEFAULT_PLAN) -> dict:
    plan = load_plan(path)
    release = plan.get("release", {})
    crates = plan.get("crates", {})
    validate_release(release)
    if not isinstance(crates, dict) or not crates:
        raise RuntimeError("release plan must contain crate entries")
    for package_name, entry in crates.items():
        if not isinstance(entry, dict):
            raise RuntimeError(f"{package_name} release entry must be a table")
        validate_source_change(package_name, entry, release["version"])
        validate_publication_state(package_name, entry, release["kind"])
    if crates.get("mynd", {}).get("published_version") != release["previous_publication"]:
        raise RuntimeError(
            "previous_publication must match the facade's published_version"
        )
    if release["kind"] != "engineering" and not crates.get("mynd", {}).get("publish"):
        raise RuntimeError("publication checkpoints must publish the mynd facade")
    return plan


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--plan", default=str(DEFAULT_PLAN))
    subparsers = parser.add_subparsers(dest="command", required=True)
    plan_field = subparsers.add_parser("plan-field")
    plan_field.add_argument(
        "field", choices=("version", "kind", "checkpoint", "previous_publication")
    )
    crate_field = subparsers.add_parser("crate-field")
    crate_field.add_argument("crate")
    crate_field.add_argument(
        "field", choices=("version", "previous_version", "published_version", "change")
    )
    subparsers.add_parser("check")
    args = parser.parse_args()
    try:
        plan = validated_plan(Path(args.plan).resolve())
        if args.command == "plan-field":
            print(plan["release"][args.field])
        elif args.command == "crate-field":
            print(plan["crates"][args.crate][args.field])
        else:
            print("release cadence and crate publication state are valid")
    except (KeyError, OSError, RuntimeError) as error:
        print(f"release policy error: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
