#!/usr/bin/env python3
"""Filesystem helpers for the specification-corpus boundary."""

from pathlib import Path


def directory_entry_names(path: Path) -> set[str]:
    """Return every direct entry name, or an empty set for a missing directory."""
    try:
        return {entry.name for entry in path.iterdir()}
    except FileNotFoundError:
        return set()
