#!/usr/bin/env bash
set -euo pipefail

BENCH_NAME="${1:-btree_bench}"
BASELINE_NAME="${2:-main}"

printf "==> Saving baseline '%s' for bench '%s'\n" "$BASELINE_NAME" "$BENCH_NAME"
cargo bench --bench "$BENCH_NAME" -- --save-baseline "$BASELINE_NAME"

printf "\n==> Running comparison against baseline '%s'\n" "$BASELINE_NAME"
cargo bench --bench "$BENCH_NAME" -- --baseline "$BASELINE_NAME"
