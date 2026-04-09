#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${PROJECT_DIR}"

read -r -p "This will uninstall SuDel and remove its installed app bundle. Continue? [y/N] " answer
case "${answer}" in
  y|Y|yes|YES)
    ;;
  *)
    echo "Uninstall canceled."
    exit 0
    ;;
esac

if [[ -x "${PROJECT_DIR}/target/release/SuDel" ]]; then
  "${PROJECT_DIR}/target/release/SuDel" --uninstall
else
  echo "Building SuDel release binary..."
  cargo build --release
  "${PROJECT_DIR}/target/release/SuDel" --uninstall
fi

echo "Uninstall complete."
