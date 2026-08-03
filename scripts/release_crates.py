#!/usr/bin/env python3
"""Publish mynd workspace crates in crates.io dependency order."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import time
from pathlib import Path

from release_policy import parse_version, validated_plan


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_PLAN = ROOT / "release-crates.toml"
PUBLISH_ORDER = (
    "mynd-math",
    "mynd-core",
    "mynd",
)


def run(
    command: list[str],
    *,
    dry_run: bool,
    extra_env: dict[str, str] | None = None,
) -> None:
    print(f"+ {' '.join(command)}", flush=True)
    if dry_run:
        return
    environment = None
    if extra_env is not None:
        environment = os.environ.copy()
        environment.update(extra_env)
    subprocess.run(command, cwd=ROOT, check=True, env=environment)


def capture(command: list[str]) -> str:
    return subprocess.check_output(command, cwd=ROOT, text=True).strip()


def try_capture(command: list[str]) -> str | None:
    result = subprocess.run(
        command,
        cwd=ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True,
    )
    if result.returncode != 0:
        return None
    return result.stdout.strip()


def release_version(plan_path: Path = DEFAULT_PLAN) -> str:
    plan = validated_plan(plan_path)
    return plan["release"]["version"]


def cargo_metadata() -> dict:
    raw = capture(["cargo", "metadata", "--format-version", "1", "--no-deps"])
    return json.loads(raw)


def workspace_packages(metadata: dict) -> dict[str, dict]:
    workspace_ids = set(metadata["workspace_members"])
    return {
        package["name"]: package
        for package in metadata["packages"]
        if package["id"] in workspace_ids
    }


def release_plan(plan_path: Path) -> dict:
    plan = validated_plan(plan_path)
    release = plan["release"]
    crates = plan["crates"]
    if set(crates) != set(PUBLISH_ORDER):
        raise RuntimeError(
            "release-crates.toml crates are not in sync with PUBLISH_ORDER: "
            f"expected {tuple(sorted(PUBLISH_ORDER))}, actual {tuple(sorted(crates))}"
        )
    return {
        "version": release["version"],
        "kind": release["kind"],
        "checkpoint": release["checkpoint"],
        "crates": crates,
    }


def require_clean_tree(*, allow_dirty: bool) -> None:
    if allow_dirty:
        return

    status = capture(["git", "status", "--porcelain"])
    if status:
        print("Refusing to publish from a dirty worktree:", file=sys.stderr)
        print(status, file=sys.stderr)
        print("Commit or stash changes, or pass --allow-dirty.", file=sys.stderr)
        sys.exit(1)


def verify_publish_order(packages: dict[str, dict], plan: dict) -> None:
    package_names = tuple(packages)
    expected_names = tuple(sorted(PUBLISH_ORDER))
    actual_names = tuple(sorted(package_names))
    if actual_names != expected_names:
        raise RuntimeError(
            "release_crates.py PUBLISH_ORDER is not in sync with workspace "
            f"packages: expected {expected_names}, actual {actual_names}"
        )

    reachable = {"mynd"}
    pending = ["mynd"]
    while pending:
        package_name = pending.pop()
        for dependency in packages[package_name]["dependencies"]:
            dependency_name = dependency["name"]
            if dependency_name in packages and dependency_name not in reachable:
                reachable.add(dependency_name)
                pending.append(dependency_name)
    unexpected_publish = sorted(
        package_name
        for package_name in PUBLISH_ORDER
        if plan["crates"][package_name]["publish"] and package_name not in reachable
    )
    if unexpected_publish:
        raise RuntimeError(
            "release plan selects crates not required by the facade: "
            f"{unexpected_publish}"
        )

    seen: set[str] = set()
    for package_name in PUBLISH_ORDER:
        package = packages[package_name]
        planned_version = plan["crates"][package_name]["version"]
        if package["version"] != planned_version:
            raise RuntimeError(
                f"{package_name} is version {package['version']}, "
                f"expected {planned_version}"
            )

        for dependency in package["dependencies"]:
            dependency_name = dependency["name"]
            if dependency_name in packages and dependency_name not in seen:
                raise RuntimeError(
                    f"{package_name} depends on {dependency_name}, but "
                    f"{dependency_name} appears later in PUBLISH_ORDER"
                )
            if dependency_name not in packages or not plan["crates"][package_name]["publish"]:
                continue
            dependency_entry = plan["crates"][dependency_name]
            expected_requirement = f"={dependency_entry['version']}"
            if dependency["req"] != expected_requirement:
                raise RuntimeError(
                    f"{package_name} must depend exactly on {dependency_name} "
                    f"{dependency_entry['version']}, found {dependency['req']}"
                )
            dependency_is_available = (
                dependency_entry["publish"]
                or dependency_entry["published_version"] == dependency_entry["version"]
            )
            if not dependency_is_available:
                raise RuntimeError(
                    f"{package_name} is selected for publication but dependency "
                    f"{dependency_name} {dependency_entry['version']} is neither "
                    "published nor selected for publication"
                )
        seen.add(package_name)


def check_release_tag(version: str, *, require_tag: bool) -> bool:
    tag = f"v{version}"
    head = try_capture(["git", "rev-parse", "HEAD"])
    tagged_commit = try_capture(["git", "rev-list", "-n", "1", tag])
    if head is None or tagged_commit is None:
        message = f"release tag {tag!r} was not found"
        if require_tag:
            print(f"Refusing to publish: {message}.", file=sys.stderr)
            sys.exit(1)
        print(f"Warning: {message}.", file=sys.stderr)
        return False

    if head != tagged_commit:
        message = f"HEAD is not tagged as {tag} (HEAD {head}, {tag} {tagged_commit})"
        if require_tag:
            print(f"Refusing to publish: {message}.", file=sys.stderr)
            sys.exit(1)
        print(f"Warning: {message}.", file=sys.stderr)
        return False

    print(f"Release tag {tag} points at HEAD.")
    return True


def confirm_no_verify(args: argparse.Namespace) -> int:
    if not args.no_verify or args.dry_run:
        return 0

    print(
        "\nWARNING: --no-verify bypasses cargo package verification.\n"
        "Use it only with a documented release incident or crates.io issue.\n"
        "Type 'no-verify confirmed' to continue:",
        file=sys.stderr,
    )
    response = input().strip()
    if response != "no-verify confirmed":
        print("Aborted.", file=sys.stderr)
        return 1
    return 0


def run_preflight(args: argparse.Namespace, *, release_tag_at_head: bool) -> None:
    if args.skip_checks:
        print("Skipping preflight checks by request.")
        return

    gate_env = None
    if release_tag_at_head:
        gate_env = {"MYND_RELEASE_PUBLISH_TAG": f"v{args.version}"}
    run(
        ["scripts/release-gate.sh", args.version],
        dry_run=args.dry_run,
        extra_env=gate_env,
    )


def publish_plan(plan: dict) -> tuple[str, ...]:
    return tuple(
        package
        for package in PUBLISH_ORDER
        if plan["crates"][package]["publish"]
    )


def selected_steps(start_at: str, steps: tuple[str, ...]) -> tuple[str, ...]:
    if not steps:
        return ()
    try:
        index = steps.index(start_at)
    except ValueError as exc:
        raise RuntimeError(f"unknown package for --start-at: {start_at}") from exc
    return steps[index:]


def verify_registry_version(package: str, version: str, *, dry_run: bool) -> None:
    if version == "0.0.0":
        return
    run(
        ["cargo", "info", "--registry", "crates-io", f"{package}@{version}"],
        dry_run=dry_run,
    )


def verify_registry_state(
    plan: dict,
    skipped_publish: tuple[str, ...],
    *,
    dry_run: bool,
) -> None:
    for package in PUBLISH_ORDER:
        published = plan["crates"][package]["published_version"]
        verify_registry_version(package, published, dry_run=dry_run)
    for package in skipped_publish:
        version = plan["crates"][package]["version"]
        verify_registry_version(package, version, dry_run=dry_run)


def wait_for_index(package: str, version: str, *, dry_run: bool) -> None:
    print()
    print(f"Published {package} {version}.")
    print(f"Wait until crates.io shows: https://crates.io/crates/{package}/{version}")
    print("Then press Enter to continue with dependent crates.")
    if dry_run:
        print("[dry-run] skipping wait")
        return
    input()
    time.sleep(5)


def publish(package: str, args: argparse.Namespace) -> None:
    command = ["cargo", "publish", "-p", package]
    if args.allow_dirty:
        command.append("--allow-dirty")
    if args.no_verify:
        command.append("--no-verify")
    run(command, dry_run=args.dry_run)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Publish mynd workspace crates in crates.io order."
    )
    parser.add_argument(
        "--version",
        default=None,
        help="Expected release version. Defaults to release-crates.toml.",
    )
    parser.add_argument(
        "--plan",
        default=str(DEFAULT_PLAN),
        help="Path to the per-crate release plan.",
    )
    parser.add_argument(
        "--start-at",
        default=None,
        choices=PUBLISH_ORDER,
        help="Resume publishing at a package if an earlier step already succeeded.",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Validate publish order and versions, then exit.",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print publish commands without running them or waiting.",
    )
    parser.add_argument(
        "--allow-dirty",
        action="store_true",
        help="Allow publishing from a dirty worktree and pass --allow-dirty to cargo.",
    )
    parser.add_argument(
        "--skip-checks",
        action="store_true",
        help="Skip local checks before publishing.",
    )
    parser.add_argument(
        "--no-verify",
        action="store_true",
        help="Pass --no-verify to cargo publish. Use only with a documented reason.",
    )
    parser.add_argument(
        "--require-tag",
        action="store_true",
        help="Refuse to publish unless HEAD matches the v<version> release tag.",
    )
    parser.add_argument(
        "--yes",
        action="store_true",
        help="Do not ask for the initial confirmation.",
    )
    args = parser.parse_args()
    raw_plan_path = Path(args.plan)
    plan_path = (
        raw_plan_path
        if raw_plan_path.is_absolute()
        else (ROOT / raw_plan_path).resolve()
    )
    plan = release_plan(plan_path)
    if args.version is None:
        args.version = plan["version"]
    elif args.version != plan["version"]:
        print(
            f"Refusing to publish: --version {args.version} does not match "
            f"{plan_path.name} release {plan['version']}.",
            file=sys.stderr,
        )
        return 1

    metadata = cargo_metadata()
    packages = workspace_packages(metadata)
    verify_publish_order(packages, plan)

    if args.check:
        print("release_crates.py publish order is up to date.")
        print(f"release_crates.py release plan is {args.version}.")
        print(f"release_crates.py release kind is {plan['kind']}.")
        return 0

    if plan["kind"] == "engineering":
        print(
            f"Refusing to publish engineering checkpoint {args.version}; "
            f"next publication checkpoint is {plan['checkpoint']}.",
            file=sys.stderr,
        )
        return 1
    if not args.dry_run and not args.require_tag:
        print(
            "Refusing to publish without --require-tag; publication must use the "
            "final green GitHub tag.",
            file=sys.stderr,
        )
        return 1

    require_clean_tree(allow_dirty=args.allow_dirty or args.dry_run)
    release_tag_at_head = check_release_tag(
        args.version, require_tag=args.require_tag
    )

    planned_publish = publish_plan(plan)
    start_at = args.start_at or (planned_publish[0] if planned_publish else "")
    steps = selected_steps(start_at, planned_publish)
    skipped_publish = planned_publish[: len(planned_publish) - len(steps)]

    print(f"Workspace root: {ROOT}")
    print(f"Release version: {args.version}")
    print("Publish sequence:")
    if steps:
        for package in steps:
            version = plan["crates"][package]["version"]
            change = plan["crates"][package]["change"]
            print(f"  - {package} {version} ({change})")
    else:
        print("  - no crates selected for publishing")
    print()

    if not args.yes:
        answer = input("Type the release version to start publishing: ").strip()
        if answer != args.version:
            print("Version confirmation did not match; aborting.", file=sys.stderr)
            return 1

    no_verify_result = confirm_no_verify(args)
    if no_verify_result != 0:
        return no_verify_result

    verify_registry_state(plan, skipped_publish, dry_run=args.dry_run)
    run_preflight(args, release_tag_at_head=release_tag_at_head)

    for index, package in enumerate(steps):
        publish(package, args)
        version = plan["crates"][package]["version"]
        if index != len(steps) - 1:
            wait_for_index(package, version, dry_run=args.dry_run)

    print()
    print("Release publish sequence completed.")
    print(f"Recommended follow-up: cargo info mynd@{args.version}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
