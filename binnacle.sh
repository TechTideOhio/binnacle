#!/usr/bin/env bash
# binnacle: TideOS Evaluator Agent runner wrapper

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_RELEASE="${SCRIPT_DIR}/target/release/binnacle"
BIN_DEBUG="${SCRIPT_DIR}/target/debug/binnacle"

if [ ! -f "$BIN_RELEASE" ] && [ ! -f "$BIN_DEBUG" ]; then
    echo "[binnacle] Compiling release binary..."
    (cd "$SCRIPT_DIR" && cargo build --release)
fi

if [ -f "$BIN_RELEASE" ]; then
    exec "$BIN_RELEASE" "$@"
else
    exec "$BIN_DEBUG" "$@"
fi
