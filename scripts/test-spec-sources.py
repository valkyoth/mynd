#!/usr/bin/env python3
"""Self-tests for the specification source corpus and security boundary."""

from __future__ import annotations

import dataclasses
import hashlib
import importlib.util
import json
import stat
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MODULE_PATH = ROOT / "scripts" / "spec-sources.py"
LEGAL_MODULE_PATH = ROOT / "scripts" / "check-legal-review.py"
RECREATION_MODULE_PATH = ROOT / "scripts" / "check-spec-recreation.py"


def load_module():
    spec = importlib.util.spec_from_file_location("mynd_spec_sources", MODULE_PATH)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def load_legal_module():
    spec = importlib.util.spec_from_file_location("mynd_legal_review", LEGAL_MODULE_PATH)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def load_recreation_module():
    spec = importlib.util.spec_from_file_location(
        "mynd_spec_recreation_test", RECREATION_MODULE_PATH
    )
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


def test_legal_review_reuses_validated_filename_boundary() -> None:
    module = load_legal_module()
    document = json.loads((ROOT / "specs" / "SOURCES.json").read_text())
    document["sources"][0]["filename"] = "../../../etc/passwd"
    original = module.SOURCE_FILE
    with tempfile.TemporaryDirectory() as directory:
        malicious = Path(directory) / "SOURCES.json"
        malicious.write_text(json.dumps(document), encoding="utf-8")
        module.SOURCE_FILE = malicious
        try:
            module.load_validated_corpus()
        except module.ReviewError:
            pass
        else:
            raise AssertionError("legal review accepted an unsafe source filename")
        finally:
            module.SOURCE_FILE = original


def test_legal_review_binds_exact_public_corpus() -> None:
    module = load_legal_module()
    sources, locks = module.load_validated_corpus()
    review = module.load_json(module.REVIEW)
    module.validate_review(sources, locks, review)
    source = next(source for source in sources if source.disposition == "public")
    changed_sources = [
        dataclasses.replace(item, title=f"{item.title} changed")
        if item.identifier == source.identifier
        else item
        for item in sources
    ]
    changed_locks = dict(locks)
    changed_locks[source.filename] = "0" * 64
    try:
        module.validate_review(changed_sources, changed_locks, review)
    except module.ReviewError:
        pass
    else:
        raise AssertionError("legal review accepted changed provenance and source bytes")


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


def test_byte_mutation_is_rejected(module) -> None:
    source = next(
        source for source in module.load_sources() if source.disposition == "public"
    )
    original = source.destination.read_bytes()
    expected = hashlib.sha256(original).hexdigest()
    lock_before = module.LOCK_FILES["public"].read_bytes()
    with tempfile.TemporaryDirectory() as directory:
        candidate = Path(directory) / source.filename
        candidate.write_bytes(original)
        candidate.chmod(stat.S_IRUSR | stat.S_IRGRP | stat.S_IROTH)
        module.assert_integrity(candidate, expected)
        candidate.chmod(stat.S_IRUSR | stat.S_IWUSR)
        changed = bytearray(original)
        changed[0] ^= 1
        candidate.write_bytes(changed)
        candidate.chmod(stat.S_IRUSR | stat.S_IRGRP | stat.S_IROTH)
        try:
            module.assert_integrity(candidate, expected)
        except module.SourceError:
            pass
        else:
            raise AssertionError("mutated source bytes were accepted")
    assert module.LOCK_FILES["public"].read_bytes() == lock_before


def test_unknown_private_entry_is_rejected(module) -> None:
    original = module.OFFLINE_DIR
    with tempfile.TemporaryDirectory() as directory:
        module.OFFLINE_DIR = Path(directory)
        (module.OFFLINE_DIR / "unreviewed-source.pdf").write_bytes(b"%PDF-1.7\n")
        try:
            module.verify(
                module.load_sources(), require_offline=False, require_manual=False
            )
        except module.SourceError:
            pass
        else:
            raise AssertionError("unknown private source entry was accepted")
        finally:
            module.OFFLINE_DIR = original


def test_private_dangling_symlink_is_rejected(module) -> None:
    original = module.OFFLINE_DIR
    source = next(
        source for source in module.load_sources() if source.disposition == "offline"
    )
    with tempfile.TemporaryDirectory() as directory:
        module.OFFLINE_DIR = Path(directory)
        (module.OFFLINE_DIR / source.filename).symlink_to("missing-source")
        try:
            module.verify(
                module.load_sources(), require_offline=False, require_manual=False
            )
        except module.SourceError:
            pass
        else:
            raise AssertionError("private dangling symlink was accepted")
        finally:
            module.OFFLINE_DIR = original


def test_fetch_all_never_selects_manual_sources(module) -> None:
    original_download = module.download
    original_lock = module.lock_local_files
    selected: list[str] = []
    module.download = lambda source, _expected: selected.append(source.disposition)
    module.lock_local_files = lambda _sources: None
    try:
        module.fetch(module.load_sources(), "all")
    finally:
        module.download = original_download
        module.lock_local_files = original_lock
    assert selected
    assert set(selected) == {"public", "offline"}


def test_changed_upstream_bytes_are_not_installed(module) -> None:
    class Response:
        headers: dict[str, str] = {}

        def __init__(self) -> None:
            self.remaining = b"unexpected upstream bytes"

        def __enter__(self):
            return self

        def __exit__(self, *_args) -> None:
            return None

        def geturl(self) -> str:
            return "https://www.w3.org/TR/png-3/"

        def read(self, _size: int) -> bytes:
            chunk, self.remaining = self.remaining, b""
            return chunk

    class Opener:
        def open(self, request, timeout: int):
            assert timeout == 30
            assert request.get_header("Accept-encoding") == "identity"
            assert request.get_header("Cache-control") == "no-cache"
            assert request.get_header("User-agent") == "mynd-spec-source-fetcher/1.0"
            return Response()

    original_public = module.PUBLIC_DIR
    original_builder = module.urllib.request.build_opener
    with tempfile.TemporaryDirectory() as directory:
        module.PUBLIC_DIR = Path(directory)
        module.urllib.request.build_opener = lambda *_handlers: Opener()
        source = module.Source(
            identifier="changed-upstream",
            title="Changed upstream",
            publisher="W3C",
            edition="Test",
            role="Test fixture",
            disposition="public",
            filename="changed.txt",
            download_url="https://www.w3.org/TR/png-3/",
            acquisition_url=None,
            terms_url="https://www.w3.org/copyright/document-license-2023/",
            license="W3C Document License 2023",
            max_bytes=1024,
        )
        try:
            module.download(source, "0" * 64)
        except module.SourceError:
            pass
        else:
            raise AssertionError("changed upstream bytes were installed")
        finally:
            module.PUBLIC_DIR = original_public
            module.urllib.request.build_opener = original_builder
        assert not list(Path(directory).iterdir())


def test_recreation_integrity_retry_is_bounded() -> None:
    recreation = load_recreation_module()

    class _SourceError(RuntimeError):
        pass

    class TransientModule:
        SourceError = _SourceError

        def __init__(self) -> None:
            self.calls = 0

        def fetch(self, _sources, scope: str) -> None:
            assert scope == "all"
            self.calls += 1
            if self.calls == 1:
                raise self.SourceError("upstream bytes changed")

    transient = TransientModule()
    recreation.fetch_with_integrity_retry(transient, [])
    assert transient.calls == 2

    class PersistentModule(TransientModule):
        def fetch(self, _sources, scope: str) -> None:
            assert scope == "all"
            self.calls += 1
            raise self.SourceError("upstream bytes changed")

    persistent = PersistentModule()
    try:
        recreation.fetch_with_integrity_retry(persistent, [])
    except _SourceError:
        pass
    else:
        raise AssertionError("persistent source drift was accepted")
    assert persistent.calls == recreation.INTEGRITY_ATTEMPTS

    class NetworkFailureModule(TransientModule):
        def fetch(self, _sources, scope: str) -> None:
            assert scope == "all"
            self.calls += 1
            raise self.SourceError("download failed")

    network_failure = NetworkFailureModule()
    try:
        recreation.fetch_with_integrity_retry(network_failure, [])
    except _SourceError:
        pass
    else:
        raise AssertionError("non-integrity failure was retried or accepted")
    assert network_failure.calls == 1


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
    test_legal_review_reuses_validated_filename_boundary()
    test_legal_review_binds_exact_public_corpus()
    test_malformed_lock_rejected(module)
    test_byte_mutation_is_rejected(module)
    test_unknown_private_entry_is_rejected(module)
    test_private_dangling_symlink_is_rejected(module)
    test_fetch_all_never_selects_manual_sources(module)
    test_changed_upstream_bytes_are_not_installed(module)
    test_recreation_integrity_retry_is_bounded()
    test_offline_tree_is_ignored()
    test_manifest_is_plain_metadata()
    print("specification source tests passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
