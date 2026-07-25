#!/usr/bin/env sh
set -eu

scope="${1:-all}"
if [ "$#" -gt 1 ]; then
    echo "usage: scripts/fetch-specs.sh [public|offline|all]" >&2
    exit 2
fi

python3 scripts/spec-sources.py fetch "$scope"
python3 scripts/spec-sources.py verify
