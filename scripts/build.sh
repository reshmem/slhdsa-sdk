#!/usr/bin/env bash
set -euo pipefail

cargo build -p slh-dsa-ffi -p slh-dsa-uniffi --release
