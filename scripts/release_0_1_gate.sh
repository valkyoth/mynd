#!/usr/bin/env sh
set -eu

scripts/checks.sh
scripts/check-rust-version-matrix.sh
scripts/check-platform-targets.sh
cargo deny check
cargo audit
scripts/generate-sbom.sh --check
MYND_RELEASE_REQUIRE_PASS=1 scripts/validate-release-metadata.sh 0.1.0
