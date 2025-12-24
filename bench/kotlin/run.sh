#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${ROOT_DIR}"

TARGET_DIR="${CARGO_TARGET_DIR:-${ROOT_DIR}/target}"
LIB_EXT="dylib"
case "$(uname -s)" in
  Linux*) LIB_EXT="so" ;;
  MINGW*|MSYS*|CYGWIN*) LIB_EXT="dll" ;;
esac

LIB_PATH="${TARGET_DIR}/release/libslh_dsa_uniffi.${LIB_EXT}"
if [[ ! -f "${LIB_PATH}" ]]; then
  echo "Missing ${LIB_PATH}. Run ./scripts/build.sh first." >&2
  exit 1
fi

mkdir -p bench/kotlin/lib bench/kotlin/lib/native bench/kotlin/build
JNA_JAR="bench/kotlin/lib/jna.jar"
if [[ ! -f "${JNA_JAR}" ]]; then
  curl -L "https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.14.0/jna-5.14.0.jar" -o "${JNA_JAR}"
fi

cp -f "${LIB_PATH}" "bench/kotlin/lib/native/"

kotlinc \
  bindings/kotlin/uniffi/slh_dsa_uniffi/slh_dsa_uniffi.kt \
  bench/kotlin/BenchUniffi.kt \
  -cp "${JNA_JAR}" \
  -nowarn \
  -include-runtime \
  -d bench/kotlin/build/bench.jar

java \
  --enable-native-access=ALL-UNNAMED \
  -Djava.library.path="bench/kotlin/lib/native" \
  -Djna.library.path="bench/kotlin/lib/native" \
  -cp "bench/kotlin/build/bench.jar:${JNA_JAR}" \
  bench.BenchUniffiKt
