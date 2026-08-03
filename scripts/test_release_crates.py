#!/usr/bin/env python3
"""Focused tests for release cadence and independent crate publication."""

from __future__ import annotations

import copy
import importlib.util
import tempfile
import unittest
from pathlib import Path


def load_script(name: str, filename: str):
    script = Path(__file__).with_name(filename)
    spec = importlib.util.spec_from_file_location(name, script)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"could not load {filename}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


POLICY = load_script("mynd_release_policy", "release_policy.py")
RELEASE = load_script("mynd_release_crates", "release_crates.py")


def entry(
    previous: str,
    published: str,
    version: str,
    change: str,
    publish: bool,
) -> dict:
    return {
        "previous_version": previous,
        "published_version": published,
        "version": version,
        "change": change,
        "publish": publish,
        "reason": "test fixture",
    }


class ReleasePolicyTests(unittest.TestCase):
    def test_cadence_is_exact(self) -> None:
        expected = {
            "0.5.0": ("publication", "0.5.0"),
            "0.5.1": ("engineering", "0.10.0"),
            "0.9.0": ("engineering", "0.10.0"),
            "0.10.0": ("publication", "0.10.0"),
            "0.10.1": ("engineering", "0.15.0"),
            "0.95.0": ("publication", "0.95.0"),
            "0.95.1": ("engineering", "1.0.0-rc.1"),
            "0.99.18": ("engineering", "1.0.0-rc.1"),
        }
        for version, (kind, checkpoint) in expected.items():
            with self.subTest(version=version):
                self.assertEqual(POLICY.normal_release_kind(version), kind)
                self.assertEqual(
                    POLICY.next_publication_checkpoint(version), checkpoint
                )

    def test_current_plan_loads(self) -> None:
        plan = RELEASE.release_plan(RELEASE.DEFAULT_PLAN)
        self.assertEqual(plan["version"], "0.5.0")
        self.assertEqual(plan["kind"], "publication")
        self.assertEqual(tuple(plan["crates"]), RELEASE.PUBLISH_ORDER)

    def test_engineering_checkpoint_accepts_unpublished_changes(self) -> None:
        core = entry("0.3.0", "0.3.0", "0.4.0", "code", False)
        facade = entry("0.5.0", "0.5.0", "0.5.1", "code", False)
        POLICY.validate_source_change("mynd-core", core, "0.5.1")
        POLICY.validate_publication_state("mynd-core", core, "engineering")
        POLICY.validate_source_change("mynd", facade, "0.5.1")
        POLICY.validate_publication_state("mynd", facade, "engineering")

    def test_engineering_checkpoint_rejects_publication(self) -> None:
        core = entry("0.3.0", "0.3.0", "0.4.0", "code", True)
        with self.assertRaisesRegex(RuntimeError, "engineering checkpoint"):
            POLICY.validate_publication_state("mynd-core", core, "engineering")

    def test_publication_checkpoint_accepts_carried_version(self) -> None:
        core = entry("0.4.0", "0.3.0", "0.4.0", "unchanged", True)
        POLICY.validate_source_change("mynd-core", core, "0.10.0")
        POLICY.validate_publication_state("mynd-core", core, "publication")

    def test_already_published_version_cannot_republish(self) -> None:
        core = entry("0.4.0", "0.4.0", "0.4.0", "unchanged", True)
        with self.assertRaisesRegex(RuntimeError, "already published"):
            POLICY.validate_publication_state("mynd-core", core, "publication")

    def test_emergency_release_requires_reason(self) -> None:
        release = {
            "version": "0.5.1",
            "kind": "emergency",
            "checkpoint": "0.5.1",
            "previous_publication": "0.5.0",
        }
        with self.assertRaisesRegex(RuntimeError, "emergency_reason"):
            POLICY.validate_release(release)
        release["emergency_reason"] = "critical published vulnerability"
        POLICY.validate_release(release)

    def test_previous_publication_matches_facade_registry_state(self) -> None:
        plan = POLICY.load_plan(POLICY.DEFAULT_PLAN)
        plan["release"]["previous_publication"] = "0.3.0"
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "plan.toml"
            lines = [
                "[release]",
                'version = "0.5.0"',
                'kind = "publication"',
                'checkpoint = "0.5.0"',
                'previous_publication = "0.3.0"',
                "",
            ]
            for package_name, package in plan["crates"].items():
                lines.append(f"[crates.{package_name}]")
                for key, value in package.items():
                    rendered = str(value).lower() if isinstance(value, bool) else f'"{value}"'
                    lines.append(f"{key} = {rendered}")
                lines.append("")
            path.write_text("\n".join(lines), encoding="utf-8")
            with self.assertRaisesRegex(RuntimeError, "facade's published_version"):
                POLICY.validated_plan(path)

    def test_published_facade_requires_available_internal_dependencies(self) -> None:
        plan = RELEASE.release_plan(RELEASE.DEFAULT_PLAN)
        packages = RELEASE.workspace_packages(RELEASE.cargo_metadata())
        broken = copy.deepcopy(plan)
        broken["crates"]["mynd-core"]["publish"] = False
        with self.assertRaisesRegex(RuntimeError, "neither published nor selected"):
            RELEASE.verify_publish_order(packages, broken)

    def test_resume_steps_preserve_dependency_order(self) -> None:
        planned = ("mynd-math", "mynd-core", "mynd")
        self.assertEqual(
            RELEASE.selected_steps("mynd-core", planned),
            ("mynd-core", "mynd"),
        )
        with self.assertRaisesRegex(RuntimeError, "unknown package"):
            RELEASE.selected_steps("unknown", planned)

    def test_plan_requires_every_workspace_crate(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "plan.toml"
            path.write_text(
                "[release]\n"
                'version = "0.5.1"\nkind = "engineering"\n'
                'checkpoint = "0.10.0"\nprevious_publication = "0.5.0"\n'
                "[crates.mynd]\n"
                'previous_version = "0.5.0"\npublished_version = "0.5.0"\n'
                'version = "0.5.1"\nchange = "code"\npublish = false\n'
                'reason = "fixture"\n',
                encoding="utf-8",
            )
            with self.assertRaisesRegex(RuntimeError, "PUBLISH_ORDER"):
                RELEASE.release_plan(path)


if __name__ == "__main__":
    unittest.main()
