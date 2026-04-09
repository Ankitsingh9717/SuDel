#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${PROJECT_DIR}"
echo "Building SuDel release binary..."
cargo build --release
echo "Installing SuDel..."
"${PROJECT_DIR}/target/release/SuDel" --install "$@"
echo
echo "Install complete."
echo "On macOS, add ~/Applications/SuDel.app to Full Disk Access and Accessibility."
