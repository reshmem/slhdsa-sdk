#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET_DIR="${CARGO_TARGET_DIR:-${ROOT_DIR}/target}"

cargo build -p slh-dsa-napi --release

LIB_BASENAME="libslh_dsa_napi"
LIB_EXT="dylib"
case "$(uname -s)" in
  Linux*) LIB_EXT="so" ;;
  MINGW*|MSYS*|CYGWIN*) LIB_EXT="dll" ;;
esac

SRC_PATH="${TARGET_DIR}/release/${LIB_BASENAME}.${LIB_EXT}"
if [[ "${LIB_EXT}" == "dll" && ! -f "${SRC_PATH}" ]]; then
  SRC_PATH="${TARGET_DIR}/release/slh_dsa_napi.dll"
fi

if [[ ! -f "${SRC_PATH}" ]]; then
  echo "Missing ${SRC_PATH}." >&2
  exit 1
fi

DEST_PATH="${ROOT_DIR}/slh-dsa-napi/slh_dsa_napi.node"
cp -f "${SRC_PATH}" "${DEST_PATH}"
