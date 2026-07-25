#!/usr/bin/env python3
"""Validate the dependency-free contract between SOURCES.json and its schema."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
SCHEMA_FILE = ROOT / "specs" / "SOURCES.schema.json"

COMMON_FIELDS = {
    "id",
    "title",
    "publisher",
    "edition",
    "role",
    "disposition",
    "filename",
    "terms_url",
    "license",
    "max_bytes",
}
CONDITIONAL_FIELDS = {"download_url", "acquisition_url"}
ALL_FIELDS = COMMON_FIELDS | CONDITIONAL_FIELDS


class SchemaError(RuntimeError):
    """The checked schema contract or a source record is invalid."""


def load_schema() -> dict[str, Any]:
    try:
        schema = json.loads(SCHEMA_FILE.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise SchemaError(f"cannot read {SCHEMA_FILE}: {error}") from error
    if not isinstance(schema, dict):
        raise SchemaError("SOURCES.schema.json must contain an object")
    return schema


def validate_schema_contract(schema: dict[str, Any]) -> None:
    if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        raise SchemaError("source schema must use JSON Schema draft 2020-12")
    if schema.get("additionalProperties") is not False:
        raise SchemaError("source schema root must reject unknown properties")
    properties = schema.get("properties")
    if not isinstance(properties, dict) or set(properties) != {
        "schema_version",
        "sources",
    }:
        raise SchemaError("source schema root properties changed without validator review")
    version = properties["schema_version"]
    if not isinstance(version, dict) or version.get("const") != 1:
        raise SchemaError("source schema version must remain 1")
    definitions = schema.get("$defs")
    source = definitions.get("source") if isinstance(definitions, dict) else None
    if not isinstance(source, dict) or source.get("additionalProperties") is not False:
        raise SchemaError("source entry schema must reject unknown properties")
    source_properties = source.get("properties")
    if not isinstance(source_properties, dict) or set(source_properties) != ALL_FIELDS:
        raise SchemaError("source entry properties changed without validator review")
    required = source.get("required")
    if not isinstance(required, list) or set(required) != COMMON_FIELDS:
        raise SchemaError("source entry required fields changed without validator review")


def validate_manifest_schema(document: Any) -> None:
    schema = load_schema()
    validate_schema_contract(schema)
    if not isinstance(document, dict) or set(document) != {"schema_version", "sources"}:
        raise SchemaError("SOURCES.json has unknown or missing root properties")
    if document["schema_version"] != 1 or not isinstance(document["sources"], list):
        raise SchemaError("SOURCES.json must use schema_version 1 and a source list")
    if not document["sources"]:
        raise SchemaError("SOURCES.json source list cannot be empty")
    for index, source in enumerate(document["sources"]):
        if not isinstance(source, dict):
            raise SchemaError(f"source entry {index} must be an object")
        unknown = set(source) - ALL_FIELDS
        missing = COMMON_FIELDS - set(source)
        if unknown or missing:
            raise SchemaError(
                f"source entry {index} schema mismatch; "
                f"missing={sorted(missing)}, unknown={sorted(unknown)}"
            )
        disposition = source.get("disposition")
        if disposition in {"public", "offline"}:
            required_url, forbidden_url = "download_url", "acquisition_url"
        elif disposition == "manual":
            required_url, forbidden_url = "acquisition_url", "download_url"
        else:
            raise SchemaError(f"source entry {index} has invalid disposition")
        if required_url not in source or forbidden_url in source:
            raise SchemaError(
                f"source entry {index} requires {required_url} and forbids {forbidden_url}"
            )
