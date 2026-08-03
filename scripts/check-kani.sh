#!/usr/bin/env sh
set -eu

# Harnesses use no assumptions, so disabling assertion-reachability probes
# cannot make their properties vacuous. It avoids a second SAT query per
# assertion while retaining Kani's safety, overflow, and explicit assertions.
cargo kani -p mynd-math -p mynd-core --no-assertion-reach-checks
