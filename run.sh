#!/usr/bin/env bash
# High-Performance Launcher for UOFiddler (100% Rust)
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_PATH="${SCRIPT_DIR}/target/release/uo_gui"

if [ ! -f "${BIN_PATH}" ]; then
    echo "Compilando UOFiddler em modo Release..."
    cargo build --release -p uo_gui
fi

exec "${BIN_PATH}" "$@"
