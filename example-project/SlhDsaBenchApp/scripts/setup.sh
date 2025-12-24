#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
LIB_DIR="${SCRIPT_DIR}/../../react-native-slh-dsa"

if [[ -d "${LIB_DIR}" ]]; then
  (cd "${LIB_DIR}" && npm install)
fi

npm install
