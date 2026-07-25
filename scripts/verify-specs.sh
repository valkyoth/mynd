#!/usr/bin/env sh
set -eu

python3 scripts/spec-sources.py lock
python3 scripts/spec-sources.py verify "$@"
