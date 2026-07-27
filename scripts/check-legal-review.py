#!/usr/bin/env python3
"""Validate the exact legal disposition review for tracked specifications."""

from __future__ import annotations

import datetime
import importlib.util
import json
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
SOURCE_FILE = ROOT / "specs" / "SOURCES.json"
REVIEW = ROOT / "specs" / "LEGAL_REVIEW.json"

TOP_LEVEL_KEYS = {
    "schema_version",
    "reviewed_on",
    "reviewed_by",
    "scope",
    "decision",
    "limitations",
    "groups",
}
GROUP_KEYS = {
    "id",
    "license",
    "terms_url",
    "decision",
    "attribution",
    "source_ids",
}


class ReviewError(RuntimeError):
    """The legal disposition evidence is incomplete or inconsistent."""


def load_json(path: Path) -> dict[str, Any]:
    try:
        document = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ReviewError(f"cannot read {path}: {error}") from error
    if not isinstance(document, dict):
        raise ReviewError(f"{path} must contain one object")
    return document


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ReviewError(message)


def load_validated_sources() -> list[Any]:
    module_path = ROOT / "scripts" / "spec-sources.py"
    spec = importlib.util.spec_from_file_location("mynd_legal_spec_sources", module_path)
    if spec is None or spec.loader is None:
        raise ReviewError("cannot load the validated specification source model")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    module.SOURCE_FILE = SOURCE_FILE
    try:
        return module.load_sources()
    except module.SourceError as error:
        raise ReviewError(f"source manifest validation failed: {error}") from error


def git_lines(*arguments: str) -> list[str]:
    result = subprocess.run(
        ["git", *arguments],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return [line for line in result.stdout.splitlines() if line]


def validate_review(sources: list[Any], review: dict[str, Any]) -> None:
    require(set(review) == TOP_LEVEL_KEYS, "legal review top-level fields differ")
    require(review["schema_version"] == 1, "unsupported legal review schema")
    try:
        datetime.date.fromisoformat(review["reviewed_on"])
    except (TypeError, ValueError) as error:
        raise ReviewError("reviewed_on must be an ISO calendar date") from error
    require(review["reviewed_by"] == "Mynd maintainers", "reviewer identity drift")
    require(review["decision"] == "approved", "tracked corpus is not approved")
    require(
        isinstance(review["scope"], str) and review["scope"].strip(),
        "review scope is missing",
    )
    require(
        isinstance(review["limitations"], list)
        and len(review["limitations"]) >= 3
        and all(isinstance(item, str) and item.strip() for item in review["limitations"]),
        "legal review limitations are incomplete",
    )

    entries = {source.identifier: source for source in sources}
    public_ids = {
        identifier
        for identifier, source in entries.items()
        if source.disposition == "public"
    }
    reviewed: list[str] = []
    group_ids: set[str] = set()
    require(isinstance(review["groups"], list), "legal review groups must be a list")
    for group in review["groups"]:
        require(isinstance(group, dict), "every legal review group must be an object")
        require(set(group) == GROUP_KEYS, "legal review group fields differ")
        require(
            isinstance(group["id"], str)
            and group["id"]
            and group["id"] not in group_ids,
            "legal review group IDs must be unique",
        )
        group_ids.add(group["id"])
        require(
            group["decision"] == "track-unmodified-with-notices",
            f"{group['id']}: unsupported disposition decision",
        )
        require(
            isinstance(group["attribution"], str) and group["attribution"].strip(),
            f"{group['id']}: attribution is missing",
        )
        require(
            isinstance(group["source_ids"], list) and group["source_ids"],
            f"{group['id']}: source coverage is empty",
        )
        for identifier in group["source_ids"]:
            require(identifier in public_ids, f"{group['id']}: non-public source {identifier}")
            entry = entries[identifier]
            require(
                entry.license == group["license"],
                f"{identifier}: reviewed license differs from source manifest",
            )
            require(
                entry.terms_url == group["terms_url"],
                f"{identifier}: reviewed terms URL differs from source manifest",
            )
            reviewed.append(identifier)

    duplicates = sorted({item for item in reviewed if reviewed.count(item) > 1})
    require(not duplicates, f"public sources reviewed more than once: {duplicates}")
    require(
        set(reviewed) == public_ids,
        "legal review coverage differs; "
        f"missing={sorted(public_ids - set(reviewed))}, "
        f"extra={sorted(set(reviewed) - public_ids)}",
    )


def validate_private_exclusion(sources: list[Any]) -> None:
    tracked = git_lines("ls-files", "specs/offline/files")
    require(not tracked, f"private specification files are tracked: {tracked}")
    private_paths = [
        f"specs/offline/files/{source.filename}"
        for source in sources
        if source.disposition in {"offline", "manual"}
    ]
    result = subprocess.run(
        ["git", "check-ignore", "--stdin"],
        cwd=ROOT,
        input="\n".join(private_paths) + "\n",
        check=False,
        capture_output=True,
        text=True,
    )
    ignored = set(result.stdout.splitlines())
    require(
        result.returncode == 0 and ignored == set(private_paths),
        "one or more offline/manual destinations are not ignored",
    )


def main() -> int:
    try:
        sources = load_validated_sources()
        review = load_json(REVIEW)
        validate_review(sources, review)
        validate_private_exclusion(sources)
    except (KeyError, ReviewError, subprocess.SubprocessError) as error:
        print(f"legal review error: {error}", file=sys.stderr)
        return 1
    print("tracked specification legal review and private exclusions agree")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
