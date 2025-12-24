#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"

./scripts/build.sh
./scripts/gen-bindings.sh

echo "=== Bench: Rust ==="
./bench/rust/run.sh
echo "=== Bench: Python ==="
./bench/python/run.sh
echo "=== Bench: Kotlin ==="
./bench/kotlin/run.sh

if [[ "${RUN_TS_BENCH:-}" == "1" ]]; then
  echo "=== Bench: TypeScript (React Native JSI) ==="
  ./bench/ts/run.sh
else
  echo "Skipping TypeScript bench (set RUN_TS_BENCH=1 to run in a React Native environment)."
fi
