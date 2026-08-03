#!/usr/bin/env sh
set -eu

rustc --version | grep -q '^rustc 1\.97\.1 '
scripts/checks.sh
scripts/check-kani.sh
scripts/check-spec-recreation.py
scripts/check-rust-version-matrix.sh
scripts/check-platform-targets.sh
scripts/check_latest_tools.sh
cargo deny check
cargo audit
scripts/generate-sbom.sh --check
MYND_RELEASE_REQUIRE_PASS=1 scripts/validate-release-metadata.sh 0.5.0
