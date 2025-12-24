# slhdsa-sdk

Rust SLH-DSA core library with C FFI + UniFFI bindings (Python/Swift/Kotlin) and React Native TypeScript/JSI bindings.

## Prerequisites

- Rust toolchain (stable)
- `npm` or `yarn`
- Optional: `ktlint` for Kotlin formatting (if desired)

## Install Cargo tools

```bash
cargo install uniffi --features cli
```

## Install Rust targets (iOS/Android)

```bash
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
```

## Install JS tooling

```bash
npm install
```

This installs `uniffi-bindgen-react-native` and `prettier` (used for TS formatting).
It also installs `typescript` and `tsx` for the TypeScript test runner.

You can install all dependencies at once with:

```bash
./scripts/install_deps.sh
```

## Run tests

```bash
./scripts/run-tests.sh
```

Notes:

- Kotlin tests download `jna-5.14.0.jar` on first run.
- Kotlin requires `kotlinc` and `ktlint` (installed by `./scripts/install_deps.sh` when `brew` is available).
- Gradle is installed by `./scripts/install_deps.sh` when `brew` is available.

## Benchmarks

See `bench/README.md` for per-language benchmarks (Rust, Python, Kotlin, TypeScript/React Native).

## Example React Native app

See `example-project/SlhDsaBenchApp/README.md` for a React Native app that runs the Shake-256f bench on-device.

## Updating API (Rust upstream -> bindings)

When you add or change a public API in the Rust crates, regenerate the UniFFI bindings and rebuild the RN library:

1) Update the UniFFI surface in `slh-dsa-uniffi/src/lib.rs` (export the new API).
2) Rebuild the Rust libs:
```bash
cargo build -p slh-dsa-uniffi --release
```
3) Regenerate all bindings (Python/Swift/Kotlin/TS):
```bash
./scripts/gen-bindings.sh
```
4) Rebuild the React Native TurboModule library:
```bash
cd react-native-slh-dsa
npm run ubrn:ios
npm run ubrn:android
```
5) Reinstall the example app and refresh Metro:
```bash
cd example-project/SlhDsaBenchApp
npm install
cd ios && pod install
PORT=8082 npm run start -- --port 8082 --reset-cache
```
6) Rebuild and run Android (device/emulator):
```bash
cd example-project/SlhDsaBenchApp
PORT=8082 npm run android -- --no-packager --port 8082
```

## Build native libraries

```bash
cargo build -p slh-dsa-ffi -p slh-dsa-uniffi --release
```

Artifacts are built in the cargo target directory:

```bash
${CARGO_TARGET_DIR:-$PWD/target}
```

Key artifacts live in `$(CARGO_TARGET_DIR:-$PWD/target)/release/`:

- `libslh_dsa_ffi` (C ABI)
- `libslh_dsa_uniffi` (UniFFI)

## Generate bindings

You can use the helper scripts:

```bash
./scripts/build.sh
./scripts/gen-bindings.sh
```

`scripts/gen-bindings.sh` creates a temporary symlink if your target dir contains spaces (required by the React Native bindgen).

### Python

```bash
uniffi-bindgen generate \
  --library "${CARGO_TARGET_DIR:-$PWD/target}/release/libslh_dsa_uniffi.dylib" \
  --language python \
  --out-dir bindings/python
```

### Swift

```bash
uniffi-bindgen generate \
  --library "${CARGO_TARGET_DIR:-$PWD/target}/release/libslh_dsa_uniffi.dylib" \
  --language swift \
  --out-dir bindings/swift
```

### Kotlin

```bash
uniffi-bindgen generate \
  --library "${CARGO_TARGET_DIR:-$PWD/target}/release/libslh_dsa_uniffi.dylib" \
  --language kotlin \
  --out-dir bindings/kotlin
```

Optional formatting:

```bash
ktlint -F bindings/kotlin/slh_dsa_uniffi.kt
```

### React Native TypeScript/JSI

```bash
npx uniffi-bindgen-react-native generate jsi bindings \
  --library \
  --ts-dir bindings/ts \
  --cpp-dir bindings/cpp \
  "${CARGO_TARGET_DIR:-$PWD/target}/release/libslh_dsa_uniffi.dylib"
```

Outputs:

- TypeScript: `bindings/ts/slh_dsa_uniffi.ts`, `bindings/ts/slh_dsa_uniffi-ffi.ts`
- C++ JSI glue: `bindings/cpp/slh_dsa_uniffi.cpp`, `bindings/cpp/slh_dsa_uniffi.hpp`

## C header

The C header is generated via cbindgen during build:

```
slh-dsa-ffi/include/slh_dsa.h
```

## Notes

- Install the UniFFI CLI once: `cargo install uniffi --features cli`
- If your cargo target directory changes, update the `--library` path in commands above.
