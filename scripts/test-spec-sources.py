#!/usr/bin/env python3
"""Self-tests for the specification source corpus and security boundary."""

from __future__ import annotations

import importlib.util
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MODULE_PATH = ROOT / "scripts" / "spec-sources.py"


def load_module():
    spec = importlib.util.spec_from_file_location("mynd_spec_sources", MODULE_PATH)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def expect_source_error(module, value: str) -> None:
    try:
        module.checked_https_url(value, "test_url", "test")
    except module.SourceError:
        return
    raise AssertionError(f"unsafe URL was accepted: {value}")


def test_url_security(module) -> None:
    for value in (
        "http://www.rfc-editor.org/rfc/rfc2119.txt",
        "https://attacker.invalid/spec",
        "https://www.w3.org:8443/TR/png-3/",
        "https://user@www.w3.org/TR/png-3/",
        "https://www.w3.org/TR/png-3/#changed",
        "file:///tmp/source",
    ):
        expect_source_error(module, value)
    assert (
        module.checked_https_url(
            "https://www.rfc-editor.org/rfc/rfc2119.txt", "test_url", "test"
        )
        == "https://www.rfc-editor.org/rfc/rfc2119.txt"
    )


def test_manifest_and_locks(module) -> None:
    sources = module.load_sources()
    locks = module.validate_locks(sources)
    assert len(sources) >= 60
    assert {source.disposition for source in sources} == {
        "public",
        "offline",
        "manual",
    }
    assert len(locks["public"]) >= 25
    assert len(locks["offline"]) >= 21
    assert not locks["manual"]
    public = [source for source in sources if source.disposition == "public"]
    assert all(
        "not established" not in source.terms_url
        and source.download_url is not None
        for source in public
    )
    # A clean checkout intentionally lacks redistribution-restricted offline
    # files. Validate every present file, while the checksum manifest proves
    # the complete set remains reproducible for maintainers.
    module.verify(sources, require_offline=False, require_manual=False)


def test_manifest_schema_rejects_drift(module) -> None:
    document = json.loads((ROOT / "specs" / "SOURCES.json").read_text())
    module.validate_manifest_schema(document)

    unknown = json.loads(json.dumps(document))
    unknown["sources"][0]["unreviewed_field"] = "not allowed"
    try:
        module.validate_manifest_schema(unknown)
    except module.SchemaError:
        pass
    else:
        raise AssertionError("unknown provenance field was accepted")

    wrong_url_kind = json.loads(json.dumps(document))
    wrong_url_kind["sources"][0]["acquisition_url"] = wrong_url_kind["sources"][0].pop(
        "download_url"
    )
    try:
        module.validate_manifest_schema(wrong_url_kind)
    except module.SchemaError:
        pass
    else:
        raise AssertionError("public source with manual acquisition URL was accepted")


def test_malformed_lock_rejected(module) -> None:
    original = module.LOCK_FILES["manual"]
    with tempfile.TemporaryDirectory() as directory:
        malformed = Path(directory) / "MANUAL_SHA256SUMS"
        malformed.write_text("not-a-checksum\n", encoding="ascii")
        module.LOCK_FILES["manual"] = malformed
        try:
            try:
                module.load_checksums("manual")
            except module.SourceError:
                pass
            else:
                raise AssertionError("malformed checksum was accepted")
        finally:
            module.LOCK_FILES["manual"] = original


def test_offline_tree_is_ignored() -> None:
    result = subprocess.run(
        ["git", "check-ignore", "specs/offline/files/probe.pdf"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    assert result.stdout.strip() == "specs/offline/files/probe.pdf"
    tracked = subprocess.run(
        ["git", "ls-files", "specs/offline/files"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    assert not tracked.stdout


def test_manifest_is_plain_metadata() -> None:
    document = json.loads((ROOT / "specs" / "SOURCES.json").read_text())
    for source in document["sources"]:
        assert "/" not in source["filename"]
        assert ".." not in source["filename"]
        assert source["max_bytes"] <= 100_000_000
    for manifest in (
        ROOT / "crates" / "mynd" / "Cargo.toml",
        ROOT / "crates" / "mynd-core" / "Cargo.toml",
    ):
        assert "specs/" not in manifest.read_text(encoding="utf-8")


def main() -> int:
    module = load_module()
    test_url_security(module)
    test_manifest_and_locks(module)
    test_manifest_schema_rejects_drift(module)
    test_malformed_lock_rejected(module)
    test_offline_tree_is_ignored()
    test_manifest_is_plain_metadata()
    print("specification source tests passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
