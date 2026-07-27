#!/usr/bin/env python3
"""Recreate every automatic specification source in isolated empty directories."""

from __future__ import annotations

import importlib.util
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MODULE_PATH = ROOT / "scripts" / "spec-sources.py"


def load_module():
    spec = importlib.util.spec_from_file_location("mynd_spec_recreation", MODULE_PATH)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def main() -> int:
    module = load_module()
    sources = module.load_sources()
    with tempfile.TemporaryDirectory(prefix="mynd-spec-recreation-") as directory:
        destination = Path(directory)
        module.ROOT = destination
        module.PUBLIC_DIR = destination / "public"
        module.OFFLINE_DIR = destination / "private"
        module.fetch(sources, "all")
        module.verify(sources, require_offline=True, require_manual=False)

        expected_public = {
            source.filename for source in sources if source.disposition == "public"
        }
        expected_offline = {
            source.filename for source in sources if source.disposition == "offline"
        }
        present_public = {path.name for path in module.PUBLIC_DIR.iterdir()}
        present_private = {path.name for path in module.OFFLINE_DIR.iterdir()}
        if present_public != expected_public:
            raise module.SourceError("isolated public recreation set differs")
        if present_private != expected_offline:
            raise module.SourceError(
                "isolated private recreation fetched a manual or unknown source"
            )
    print("automatic specification corpus recreated from empty destinations")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
