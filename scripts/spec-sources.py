#!/usr/bin/env python3
"""Fetch and verify Mynd's public, offline, and manual specification sources."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import stat
import sys
import tempfile
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from spec_fs import directory_entry_names
from spec_schema import SchemaError, validate_manifest_schema

ROOT = Path(__file__).resolve().parent.parent
SPEC_DIR = ROOT / "specs"
SOURCE_FILE = SPEC_DIR / "SOURCES.json"
PUBLIC_DIR = SPEC_DIR / "public"
OFFLINE_DIR = SPEC_DIR / "offline" / "files"
LOCK_FILES = {
    "public": SPEC_DIR / "SHA256SUMS",
    "offline": SPEC_DIR / "OFFLINE_SHA256SUMS",
    "manual": SPEC_DIR / "MANUAL_SHA256SUMS",
}
ALLOWED_HOSTS = {
    "archive.color.org",
    "creativecommons.org",
    "cie.co.at",
    "developers.google.com",
    "dl.acm.org",
    "git.suckless.org",
    "learn.microsoft.com",
    "netpbm.sourceforge.net",
    "opensource.adobe.com",
    "opensource.org",
    "phoboslab.org",
    "raw.githubusercontent.com",
    "trustee.ietf.org",
    "webstore.iec.ch",
    "www.acm.org",
    "www.adobe.com",
    "www.cie.co.at",
    "www.cipa.jp",
    "www.color.org",
    "www.ibm.com",
    "www.iec.ch",
    "www.iso.org",
    "www.itu.int",
    "www.loc.gov",
    "www.rfc-editor.org",
    "www.w3.org",
    "winprotocoldocs-bhdugrdyduf5h2e4.b02.azurefd.net",
    "winprotocoldoc.z19.web.core.windows.net",
}
ID_RE = re.compile(r"[a-z0-9][a-z0-9.-]*\Z")
HASH_RE = re.compile(r"([0-9a-f]{64})  ([A-Za-z0-9][A-Za-z0-9._+-]*)\Z")
DOWNLOAD_HEADERS = {
    "Accept-Encoding": "identity",
    "Cache-Control": "no-cache",
    "User-Agent": "mynd-spec-source-fetcher/1.0",
}

class SourceError(RuntimeError):
    """A source manifest, integrity, or download failure."""


@dataclass(frozen=True)
class Source:
    identifier: str
    title: str
    publisher: str
    edition: str
    role: str
    disposition: str
    filename: str
    download_url: str | None
    acquisition_url: str | None
    terms_url: str
    license: str
    max_bytes: int

    @property
    def destination(self) -> Path:
        base = PUBLIC_DIR if self.disposition == "public" else OFFLINE_DIR
        return base / self.filename


def checked_https_url(value: Any, field: str, identifier: str) -> str:
    if not isinstance(value, str) or not value:
        raise SourceError(f"{identifier}: {field} must be a non-empty string")
    parsed = urllib.parse.urlsplit(value)
    host = parsed.hostname
    try:
        port = parsed.port
    except ValueError as error:
        raise SourceError(f"{identifier}: malformed {field}: {value}") from error
    if (
        parsed.scheme != "https"
        or not host
        or host.lower() not in ALLOWED_HOSTS
        or port not in (None, 443)
        or parsed.username
        or parsed.password
        or parsed.fragment
        or any(ord(character) < 0x21 for character in value)
    ):
        raise SourceError(f"{identifier}: unsafe or unapproved {field}: {value}")
    return value


def load_sources() -> list[Source]:
    try:
        document = json.loads(SOURCE_FILE.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise SourceError(f"cannot read {SOURCE_FILE}: {error}") from error
    try:
        validate_manifest_schema(document)
    except SchemaError as error:
        raise SourceError(str(error)) from error
    sources: list[Source] = []
    identifiers: set[str] = set()
    destinations: set[tuple[str, str]] = set()
    required_text = ("title", "publisher", "edition", "role", "license")
    for raw in document["sources"]:
        if not isinstance(raw, dict):
            raise SourceError("every source entry must be an object")
        identifier = raw.get("id")
        if not isinstance(identifier, str) or not ID_RE.fullmatch(identifier):
            raise SourceError(f"invalid source id: {identifier!r}")
        if identifier in identifiers:
            raise SourceError(f"duplicate source id: {identifier}")
        identifiers.add(identifier)
        for field in required_text:
            if not isinstance(raw.get(field), str) or not raw[field].strip():
                raise SourceError(f"{identifier}: {field} must be a non-empty string")
        disposition = raw.get("disposition")
        if disposition not in LOCK_FILES:
            raise SourceError(f"{identifier}: invalid disposition {disposition!r}")
        filename = raw.get("filename")
        if (
            not isinstance(filename, str)
            or Path(filename).name != filename
            or filename in {".", ".."}
            or "\x00" in filename
        ):
            raise SourceError(f"{identifier}: filename must be one safe path component")
        destination_key = (disposition, filename)
        if destination_key in destinations:
            raise SourceError(f"{identifier}: duplicate destination {filename}")
        destinations.add(destination_key)
        max_bytes = raw.get("max_bytes")
        if not isinstance(max_bytes, int) or not 0 < max_bytes <= 100_000_000:
            raise SourceError(f"{identifier}: max_bytes must be between 1 and 100000000")
        terms_url = checked_https_url(raw.get("terms_url"), "terms_url", identifier)
        if disposition == "manual":
            if "download_url" in raw:
                raise SourceError(f"{identifier}: manual sources cannot have download_url")
            acquisition_url = checked_https_url(
                raw.get("acquisition_url"), "acquisition_url", identifier
            )
            download_url = None
        else:
            download_url = checked_https_url(
                raw.get("download_url"), "download_url", identifier
            )
            acquisition_url = None
        sources.append(
            Source(
                identifier=identifier,
                title=raw["title"],
                publisher=raw["publisher"],
                edition=raw["edition"],
                role=raw["role"],
                disposition=disposition,
                filename=filename,
                download_url=download_url,
                acquisition_url=acquisition_url,
                terms_url=terms_url,
                license=raw["license"],
                max_bytes=max_bytes,
            )
        )
    if not sources:
        raise SourceError("source manifest cannot be empty")
    return sources


def load_checksums(disposition: str) -> dict[str, str]:
    path = LOCK_FILES[disposition]
    try:
        lines = path.read_text(encoding="ascii").splitlines()
    except OSError as error:
        raise SourceError(f"cannot read {path}: {error}") from error
    checksums: dict[str, str] = {}
    for number, line in enumerate(lines, start=1):
        if not line or line.startswith("#"):
            continue
        match = HASH_RE.fullmatch(line)
        if not match:
            raise SourceError(f"{path}:{number}: malformed checksum entry")
        digest, filename = match.groups()
        if filename in checksums:
            raise SourceError(f"{path}:{number}: duplicate checksum for {filename}")
        checksums[filename] = digest
    return checksums


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    try:
        with path.open("rb") as handle:
            for chunk in iter(lambda: handle.read(1024 * 1024), b""):
                digest.update(chunk)
    except OSError as error:
        raise SourceError(f"cannot hash {path}: {error}") from error
    return digest.hexdigest()


def assert_file_type(path: Path) -> None:
    try:
        with path.open("rb") as handle:
            header = handle.read(4096)
    except OSError as error:
        raise SourceError(f"cannot inspect {path}: {error}") from error
    if path.suffix.lower() == ".pdf" and not header.startswith(b"%PDF-"):
        raise SourceError(f"expected a PDF document: {path}")
    if path.suffix.lower() == ".html" and b"<html" not in header.lower():
        raise SourceError(f"expected an HTML document: {path}")


def expected_by_disposition(sources: list[Source]) -> dict[str, dict[str, Source]]:
    result = {key: {} for key in LOCK_FILES}
    for source in sources:
        result[source.disposition][source.filename] = source
    return result


def validate_locks(sources: list[Source]) -> dict[str, dict[str, str]]:
    expected = expected_by_disposition(sources)
    locks = {key: load_checksums(key) for key in LOCK_FILES}
    for disposition in ("public", "offline"):
        wanted = set(expected[disposition])
        locked = set(locks[disposition])
        if wanted != locked:
            missing = sorted(wanted - locked)
            extra = sorted(locked - wanted)
            raise SourceError(
                f"{disposition} checksum set differs; missing={missing}, extra={extra}"
            )
    unknown_manual = set(locks["manual"]) - set(expected["manual"])
    if unknown_manual:
        raise SourceError(f"manual checksums have unknown files: {sorted(unknown_manual)}")
    return locks


def assert_integrity(path: Path, expected: str) -> None:
    if not path.is_file() or path.is_symlink():
        raise SourceError(f"missing or unsafe source file: {path}")
    if path.stat().st_mode & (stat.S_IWUSR | stat.S_IWGRP | stat.S_IWOTH):
        raise SourceError(f"source file is writable; run scripts/lock-specs.sh: {path}")
    assert_file_type(path)
    actual = sha256_file(path)
    if actual != expected:
        raise SourceError(f"checksum mismatch for {path}: expected {expected}, got {actual}")


def verify(
    sources: list[Source], require_offline: bool, require_manual: bool
) -> None:
    locks = validate_locks(sources)
    expected = expected_by_disposition(sources)
    public_actual = directory_entry_names(PUBLIC_DIR)
    public_expected = set(expected["public"])
    if public_actual != public_expected:
        raise SourceError(
            "public source set differs; "
            f"missing={sorted(public_expected - public_actual)}, "
            f"extra={sorted(public_actual - public_expected)}"
        )
    for filename, digest in locks["public"].items():
        assert_integrity(PUBLIC_DIR / filename, digest)

    private_actual = directory_entry_names(OFFLINE_DIR)
    private_expected = set(expected["offline"]) | set(expected["manual"])
    private_extra = private_actual - private_expected
    if private_extra:
        raise SourceError(f"private source set has unknown entries: {sorted(private_extra)}")

    for disposition, required in (
        ("offline", require_offline),
        ("manual", require_manual),
    ):
        for filename, source in expected[disposition].items():
            path = source.destination
            if not path.exists() and not path.is_symlink():
                if required:
                    raise SourceError(f"required {disposition} source is missing: {filename}")
                continue
            if filename not in locks[disposition]:
                raise SourceError(
                    f"{disposition} source exists without a checksum lock: {filename}"
                )
            assert_integrity(path, locks[disposition][filename])


class SafeRedirectHandler(urllib.request.HTTPRedirectHandler):
    def redirect_request(
        self,
        req: urllib.request.Request,
        fp: Any,
        code: int,
        msg: str,
        headers: Any,
        newurl: str,
    ) -> urllib.request.Request | None:
        checked_https_url(newurl, "redirect", "download")
        return super().redirect_request(req, fp, code, msg, headers, newurl)


def download(source: Source, expected: str) -> None:
    destination = source.destination
    if destination.exists():
        assert_integrity(destination, expected)
        print(f"verified {source.identifier}: {destination.relative_to(ROOT)}")
        return
    destination.parent.mkdir(parents=True, exist_ok=True)
    opener = urllib.request.build_opener(SafeRedirectHandler())
    request = urllib.request.Request(
        source.download_url or "",
        headers=DOWNLOAD_HEADERS,
    )
    temporary_name: str | None = None
    try:
        with opener.open(request, timeout=30) as response:
            checked_https_url(response.geturl(), "final_url", source.identifier)
            content_length = response.headers.get("Content-Length")
            if content_length and int(content_length) > source.max_bytes:
                raise SourceError(f"{source.identifier}: response exceeds max_bytes")
            with tempfile.NamedTemporaryFile(
                mode="wb", dir=destination.parent, prefix=f".{source.filename}.", delete=False
            ) as temporary:
                temporary_name = temporary.name
                digest = hashlib.sha256()
                total = 0
                while True:
                    chunk = response.read(1024 * 1024)
                    if not chunk:
                        break
                    total += len(chunk)
                    if total > source.max_bytes:
                        raise SourceError(f"{source.identifier}: response exceeds max_bytes")
                    digest.update(chunk)
                    temporary.write(chunk)
                temporary.flush()
                os.fsync(temporary.fileno())
        if total == 0:
            raise SourceError(f"{source.identifier}: empty response")
        actual = digest.hexdigest()
        if actual != expected:
            raise SourceError(
                f"{source.identifier}: upstream bytes changed; "
                f"expected {expected}, got {actual}"
            )
        os.chmod(temporary_name, stat.S_IRUSR | stat.S_IRGRP | stat.S_IROTH)
        os.replace(temporary_name, destination)
        temporary_name = None
        print(f"fetched {source.identifier}: {destination.relative_to(ROOT)}")
    except (OSError, ValueError, urllib.error.URLError) as error:
        if isinstance(error, SourceError):
            raise
        raise SourceError(f"{source.identifier}: download failed: {error}") from error
    finally:
        if temporary_name:
            try:
                os.unlink(temporary_name)
            except FileNotFoundError:
                pass


def fetch(sources: list[Source], scope: str) -> None:
    locks = validate_locks(sources)
    dispositions = {"public", "offline"} if scope == "all" else {scope}
    lock_local_files(sources)
    for source in sources:
        if source.disposition in dispositions:
            download(source, locks[source.disposition][source.filename])
    lock_local_files(sources)


def remote_digest(source: Source) -> str:
    opener = urllib.request.build_opener(SafeRedirectHandler())
    request = urllib.request.Request(
        source.download_url or "",
        headers=DOWNLOAD_HEADERS,
    )
    digest = hashlib.sha256()
    total = 0
    try:
        with opener.open(request, timeout=30) as response:
            checked_https_url(response.geturl(), "final_url", source.identifier)
            content_length = response.headers.get("Content-Length")
            if content_length and int(content_length) > source.max_bytes:
                raise SourceError(f"{source.identifier}: response exceeds max_bytes")
            while True:
                chunk = response.read(1024 * 1024)
                if not chunk:
                    break
                total += len(chunk)
                if total > source.max_bytes:
                    raise SourceError(f"{source.identifier}: response exceeds max_bytes")
                digest.update(chunk)
    except (OSError, ValueError, urllib.error.URLError) as error:
        if isinstance(error, SourceError):
            raise
        raise SourceError(f"{source.identifier}: download failed: {error}") from error
    if total == 0:
        raise SourceError(f"{source.identifier}: empty response")
    return digest.hexdigest()


def candidate_locks(sources: list[Source], scope: str) -> None:
    dispositions = {"public", "offline"} if scope == "all" else {scope}
    for source in sources:
        if source.disposition in dispositions:
            print(f"{remote_digest(source)}  {source.filename}")


def lock_local_files(sources: list[Source]) -> None:
    for source in sources:
        path = source.destination
        if path.is_file() and not path.is_symlink():
            path.chmod(stat.S_IRUSR | stat.S_IRGRP | stat.S_IROTH)


def status(sources: list[Source]) -> None:
    locks = validate_locks(sources)
    for disposition in LOCK_FILES:
        members = [source for source in sources if source.disposition == disposition]
        present = sum(source.destination.is_file() for source in members)
        locked = sum(source.filename in locks[disposition] for source in members)
        print(f"{disposition}: {present}/{len(members)} present, "
              f"{locked}/{len(members)} checksum-locked")
    missing_manual = [
        source
        for source in sources
        if source.disposition == "manual" and not source.destination.exists()
    ]
    if missing_manual:
        print("\nManual acquisition required:")
        for source in missing_manual:
            print(f"- {source.filename}: {source.acquisition_url}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)
    fetch_parser = subparsers.add_parser("fetch", help="fetch checksum-locked sources")
    fetch_parser.add_argument("scope", choices=("public", "offline", "all"))
    candidate_parser = subparsers.add_parser("candidate-locks", help="print current remote hashes for maintainer review")
    candidate_parser.add_argument("scope", choices=("public", "offline", "all"))
    verify_parser = subparsers.add_parser("verify", help="verify manifests and local files")
    verify_parser.add_argument("--require-offline", action="store_true")
    verify_parser.add_argument("--require-manual", action="store_true")
    subparsers.add_parser("lock", help="make present source copies read-only")
    subparsers.add_parser("status", help="show corpus and manual acquisition status")
    return parser.parse_args()


def main() -> int:
    arguments = parse_args()
    try:
        sources = load_sources()
        if arguments.command == "fetch":
            fetch(sources, arguments.scope)
        elif arguments.command == "candidate-locks":
            candidate_locks(sources, arguments.scope)
        elif arguments.command == "verify":
            verify(sources, arguments.require_offline, arguments.require_manual)
        elif arguments.command == "lock":
            validate_locks(sources)
            lock_local_files(sources)
        else:
            status(sources)
    except SourceError as error:
        print(f"spec source error: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
