#!/usr/bin/env bash
set -euo pipefail

cargo install uniffi --features cli

if command -v brew >/dev/null 2>&1; then
  brew install kotlin
  brew install ktlint
  brew install gradle
fi

rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android

npm install
