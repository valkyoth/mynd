#!/usr/bin/env python3
"""Reject third-party runtime dependencies in published workspace crates."""

from __future__ import annotations

import json
import subprocess
import sys


def main() -> int:
    raw = subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"], text=True
    )
    metadata = json.loads(raw)
    workspace_ids = set(metadata["workspace_members"])
    failed = False

    for package in metadata["packages"]:
        if package["id"] not in workspace_ids:
            continue
        for dependency in package["dependencies"]:
            if dependency["kind"] == "dev":
                continue
            if not dependency["name"].startswith("mynd"):
                print(
                    f"{package['name']} has non-Mynd runtime dependency "
                    f"{dependency['name']}",
                    file=sys.stderr,
                )
                failed = True

    if failed:
        return 1
    print("published runtime dependency boundary is first-party only")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
