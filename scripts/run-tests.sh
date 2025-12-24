#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"

./scripts/build.sh
./scripts/gen-bindings.sh

TARGET_DIR="${CARGO_TARGET_DIR:-${ROOT_DIR}/target}"
LIB_EXT="dylib"
case "$(uname -s)" in
  Linux*) LIB_EXT="so" ;;
  MINGW*|MSYS*|CYGWIN*) LIB_EXT="dll" ;;
esac

LIB_PATH="${TARGET_DIR}/release/libslh_dsa_uniffi.${LIB_EXT}"

if [[ ! -f "${LIB_PATH}" ]]; then
  echo "Missing ${LIB_PATH}." >&2
  exit 1
fi

# Python
python3 tests/python/test_uniffi.py

# Swift
SWIFT_BIN="tests/swift/test_uniffi"
rm -f "${SWIFT_BIN}"

swiftc \
  tests/swift/TestUniffi.swift \
  bindings/swift/slh_dsa_uniffi.swift \
  -I bindings/swift \
  -Xcc -fmodule-map-file=bindings/swift/slh_dsa_uniffiFFI.modulemap \
  -L "${TARGET_DIR}/release" \
  -lslh_dsa_uniffi \
  -Xlinker -rpath -Xlinker "${TARGET_DIR}/release" \
  -o "${SWIFT_BIN}"

"${SWIFT_BIN}"

# Kotlin
if command -v kotlinc >/dev/null 2>&1; then
  mkdir -p tests/kotlin/lib tests/kotlin/build
  JNA_JAR="tests/kotlin/lib/jna.jar"
  if [[ ! -f "${JNA_JAR}" ]]; then
    curl -L "https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.14.0/jna-5.14.0.jar" -o "${JNA_JAR}"
  fi
  KOTLIN_NATIVE_DIR="tests/kotlin/lib/native"
  mkdir -p "${KOTLIN_NATIVE_DIR}"
  cp -f "${LIB_PATH}" "${KOTLIN_NATIVE_DIR}/"

  kotlinc \
    bindings/kotlin/uniffi/slh_dsa_uniffi/slh_dsa_uniffi.kt \
    tests/kotlin/TestUniffi.kt \
    -cp "${JNA_JAR}" \
    -include-runtime \
    -d tests/kotlin/build/test.jar

  java \
    -Djava.library.path="${KOTLIN_NATIVE_DIR}" \
    -Djna.library.path="${KOTLIN_NATIVE_DIR}" \
    -cp "tests/kotlin/build/test.jar:${JNA_JAR}" \
    test.TestUniffiKt
else
  echo "kotlinc not found; skipping Kotlin test." >&2
fi

# TypeScript
if ! command -v npx >/dev/null 2>&1; then
  echo "npx not found; skipping TypeScript test." >&2
else
  npx tsc -p tests/ts/tsconfig.json
  npx tsx tests/ts/test.ts
  npx tsx tests/ts/test_napi.ts
fi
