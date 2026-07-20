#!/usr/bin/env python3
"""Focused tests for the adapted independent-crate release planner."""

from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).with_name("release_crates.py")
SPEC = importlib.util.spec_from_file_location("mynd_release_crates", SCRIPT)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("could not load release_crates.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class ReleasePlannerTests(unittest.TestCase):
    def test_version_requires_three_numeric_parts(self) -> None:
        self.assertEqual(MODULE.parse_version("1.2.3"), (1, 2, 3))
        with self.assertRaises(RuntimeError):
            MODULE.parse_version("1.2")
        with self.assertRaises(RuntimeError):
            MODULE.parse_version("1.two.3")

    def test_initial_plan_loads(self) -> None:
        plan = MODULE.release_plan(MODULE.DEFAULT_PLAN)
        self.assertEqual(plan["version"], "0.1.0")
        self.assertEqual(tuple(plan["crates"]), MODULE.PUBLISH_ORDER)

    def test_unchanged_crate_cannot_publish(self) -> None:
        entry = {
            "previous_version": "0.1.0",
            "version": "0.1.0",
            "change": "unchanged",
            "publish": True,
            "reason": "test",
        }
        with self.assertRaises(RuntimeError):
            MODULE.validate_plan_entry("mynd-core", entry, "0.2.0")

    def test_support_code_change_requires_next_minor(self) -> None:
        entry = {
            "previous_version": "0.1.0",
            "version": "0.1.1",
            "change": "code",
            "publish": True,
            "reason": "test",
        }
        with self.assertRaises(RuntimeError):
            MODULE.validate_plan_entry("mynd-core", entry, "0.2.0")

    def test_plan_requires_every_workspace_crate(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "plan.toml"
            path.write_text('[release]\nversion = "0.1.0"\n', encoding="utf-8")
            with self.assertRaises(RuntimeError):
                MODULE.release_plan(path)


if __name__ == "__main__":
    unittest.main()
