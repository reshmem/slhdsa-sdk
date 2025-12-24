#!/usr/bin/env bash
set -euo pipefail

cargo install uniffi --features cli
cargo install cargo-ndk

if command -v brew >/dev/null 2>&1; then
  brew install kotlin
  brew install ktlint
  brew install gradle
elif command -v apt-get >/dev/null 2>&1; then
  sudo apt-get update
  sudo apt-get install -y \
    build-essential \
    curl \
    cmake \
    ninja-build \
    pkg-config \
    clang \
    openjdk-17-jdk
fi

rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android

npm install
