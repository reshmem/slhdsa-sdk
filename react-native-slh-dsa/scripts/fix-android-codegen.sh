#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="${SCRIPT_DIR}/.."
SRC_JNI="${ROOT_DIR}/android/build/generated/source/codegen/jni"
DST_JNI="${ROOT_DIR}/android/generated/jni"
LEGACY_DST_JNI="${ROOT_DIR}/android/android/generated/jni"

if [[ -d "${SRC_JNI}" ]]; then
  mkdir -p "${ROOT_DIR}/android/generated"
  rm -rf "${DST_JNI}"
  ln -s "${SRC_JNI}" "${DST_JNI}"

  mkdir -p "${ROOT_DIR}/android/android/generated"
  rm -rf "${LEGACY_DST_JNI}"
  ln -s "${SRC_JNI}" "${LEGACY_DST_JNI}"
fi
