#!/usr/bin/env bash
# Fast local dev: compile only the route crates you need.
# Usage: ./scripts/dev-fast.sh [feature...]
# Examples:
#   ./scripts/dev-fast.sh              # lab only (default)
#   ./scripts/dev-fast.sh malware-traffic
#   ./scripts/dev-fast.sh lab malware-traffic
#   ./scripts/dev-fast.sh full           # everything (same as production)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

cargo run -p prebuild --release --quiet

if [[ "${1:-}" == "full" ]]; then
  echo "dev-fast: trunk serve (all features)"
  exec trunk serve
fi

FEATURES="${*:-lab}"
echo "dev-fast: trunk serve --no-default-features --features $FEATURES"
exec trunk serve -- --no-default-features --features "$FEATURES"
