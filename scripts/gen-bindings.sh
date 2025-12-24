#!/usr/bin/env bash
set -euo pipefail

TARGET_DIR="${CARGO_TARGET_DIR:-$(pwd)/target}"
LIB_EXT="dylib"
case "$(uname -s)" in
  Linux*) LIB_EXT="so" ;;
  MINGW*|MSYS*|CYGWIN*) LIB_EXT="dll" ;;
esac

LIB_PATH="${TARGET_DIR}/release/libslh_dsa_uniffi.${LIB_EXT}"
LIB_PATH_NO_SPACE="${LIB_PATH}"

if [[ ! -f "${LIB_PATH}" ]]; then
  echo "Missing ${LIB_PATH}. Run scripts/build.sh first." >&2
  exit 1
fi

if ! command -v uniffi-bindgen >/dev/null 2>&1; then
  echo "uniffi-bindgen not found. Install with: cargo install uniffi --features cli" >&2
  exit 1
fi

uniffi-bindgen generate --library "${LIB_PATH}" --language python --out-dir bindings/python
uniffi-bindgen generate --library "${LIB_PATH}" --language swift --out-dir bindings/swift
uniffi-bindgen generate --library "${LIB_PATH}" --language kotlin --out-dir bindings/kotlin

if [[ "${LIB_PATH}" == *" "* ]]; then
  mkdir -p .tmp
  LIB_PATH_NO_SPACE="$(pwd)/.tmp/libslh_dsa_uniffi.${LIB_EXT}"
  ln -sf "${LIB_PATH}" "${LIB_PATH_NO_SPACE}"
fi

RUSTFLAGS="-Awarnings" npx uniffi-bindgen-react-native generate jsi bindings \
  --library \
  --ts-dir bindings/ts \
  --cpp-dir bindings/cpp \
  "${LIB_PATH_NO_SPACE}"
