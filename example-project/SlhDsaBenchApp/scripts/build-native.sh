#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
LIB_DIR="${SCRIPT_DIR}/../../react-native-slh-dsa"

if [[ ! -d "${LIB_DIR}" ]]; then
  echo "Expected React Native library at ${LIB_DIR}"
  exit 1
fi

(cd "${LIB_DIR}" && npm run ubrn:ios)
(cd "${LIB_DIR}" && npm run ubrn:android)
