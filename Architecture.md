# Architecture

This document explains how the SLH-DSA SDK is structured and how to integrate it
into projects. It is written for junior engineers who are new to Rust + native
bindings.

## High-level layout

- `slh-dsa-core/`: pure Rust implementation (no_std). This is the source of truth.
- `slh-dsa-ffi/`: C ABI wrapper around `slh-dsa-core` for native consumers.
- `slh-dsa-uniffi/`: UniFFI wrapper that generates Python/Swift/Kotlin/TS bindings.
- `slh-dsa-napi/`: Node.js N-API bindings for TypeScript/Node.
- `react-native-slh-dsa/`: React Native TurboModule + JSI glue.
- `bindings/`: generated UniFFI bindings (Python/Swift/Kotlin/TS).
- `bench/`: benchmark runners for each language.
- `tests/`: smoke tests for each language.
- `example-project/SlhDsaBenchApp/`: React Native demo app.

## Architecture diagram

```
slh-dsa-core
   |
   +--> slh-dsa-ffi (C ABI)
   |
   +--> slh-dsa-uniffi (UniFFI API)
   |        |
   |        +--> bindings/python|swift|kotlin|ts
   |        |
   |        +--> react-native-slh-dsa (TurboModule/JSI)
   |
   +--> slh-dsa-napi (Node.js N-API)
```

## Why the layers exist

1) `slh-dsa-core` is the cryptographic implementation. Keep it small and stable.
2) `slh-dsa-ffi` exposes a minimal C ABI for anything that can call C.
3) `slh-dsa-uniffi` exposes a higher-level API that UniFFI can bind to multiple languages.
4) `react-native-slh-dsa` packages UniFFI bindings as a TurboModule/JSI library.
5) `slh-dsa-napi` exposes the same API via Node-API for Node.js/TypeScript.

Each layer depends only on the one below it.

## Data flow

- Rust code runs in `slh-dsa-core`.
- FFI layers serialize inputs/outputs into byte buffers.
- Bindings convert those buffers into language-native types.

## Adding or changing APIs

When you add a new function or type:

1) Add it to `slh-dsa-core` first.
2) If C users need it, add a C ABI wrapper in `slh-dsa-ffi`.
3) Add the public UniFFI surface in `slh-dsa-uniffi/src/lib.rs`.
4) Add the N-API export in `slh-dsa-napi/src/lib.rs` if Node users need it.
5) Regenerate and rebuild:
   - `./scripts/gen-bindings.sh`
   - `./scripts/build-napi.sh`
   - `npm run ubrn:ios` / `npm run ubrn:android` in `react-native-slh-dsa/`

## Integration guide

### Rust

Use `slh-dsa-core` directly:

- Import the crate in your Cargo project.
- Call `generate_keypair`, `sign`, and `verify` using your chosen parameter set.

### C/C++

Use `slh-dsa-ffi`:

- Include `slh-dsa-ffi/include/slh_dsa.h`.
- Link against `libslh_dsa_ffi` from `${CARGO_TARGET_DIR:-$PWD/target}/release`.
- Call the `slh_dsa_*` C functions.

### Python/Swift/Kotlin

Use UniFFI bindings:

- Run `./scripts/gen-bindings.sh` to generate language bindings.
- Load `libslh_dsa_uniffi` and call the generated APIs.

### Node.js / TypeScript

Use `slh-dsa-napi`:

- Build with `./scripts/build-napi.sh`.
- Import from `slh-dsa-napi/` and call the camelCase APIs.

### React Native (iOS/Android)

Use `react-native-slh-dsa`:

- Build the RN library (`npm run ubrn:ios` and `npm run ubrn:android`).
- Install it in your RN app and call the exported functions.
- For JSI benchmarks, make sure the native module is registered.

## Third-party integration (submodule)

If an external React Native project wants to use this repo as a git submodule,
use the steps below.

### 1) Add the submodule

```bash
git submodule add <repo-url> vendor/slhdsa-sdk
git submodule update --init --recursive
```

### 2) Build the RN library inside the submodule

```bash
cd vendor/slhdsa-sdk
npm install
cargo build -p slh-dsa-uniffi --release
./scripts/gen-bindings.sh
cd react-native-slh-dsa
npm run ubrn:ios
npm run ubrn:android
```

### 3) Link the RN library into your app

From your app root:

```bash
npm install ./vendor/slhdsa-sdk/react-native-slh-dsa
```

### 4) iOS setup

```bash
cd ios
pod install
```

### 5) Android setup

- Set `ANDROID_HOME` and `JAVA_HOME`.
- Ensure NDK + CMake are installed.

### 6) Metro config (if you use a monorepo or symlink)

Add the submodule to `watchFolders` and `extraNodeModules`:

```js
// metro.config.js
const path = require("path");

module.exports = {
  watchFolders: [path.resolve(__dirname, "vendor/slhdsa-sdk/react-native-slh-dsa")],
  resolver: {
    extraNodeModules: {
      "react-native-slh-dsa": path.resolve(
        __dirname,
        "vendor/slhdsa-sdk/react-native-slh-dsa"
      ),
    },
  },
};
```

### 7) Use it in JS/TS

```ts
import {
  ParameterSetId,
  slhDsaKeypairGenerate,
  slhDsaSignDeterministic,
  slhDsaVerify,
} from "react-native-slh-dsa";
```

## Common pitfalls

- Android builds need `ANDROID_HOME`, `JAVA_HOME`, and a valid NDK/CMake install.
- React Native JSI benchmarks only work inside an RN runtime.
- Rebuild bindings after any public API changes.
- Native artifacts are in `${CARGO_TARGET_DIR:-$PWD/target}` by default.
- On Windows, the helper scripts are Bash; run them from WSL2 or Git Bash.
- Windows shared libs are `.dll` and may be named `slh_dsa_uniffi.dll` without the `lib` prefix.
- WSL2 generally has fewer path/line-ending issues than Git Bash, especially for build scripts.

## Scripts to know

- `./scripts/build.sh`: builds Rust libs and N-API.
- `./scripts/gen-bindings.sh`: regenerates UniFFI bindings.
- `./scripts/run-tests.sh`: runs multi-language tests.
- `./scripts/bench.sh`: runs multi-language benchmarks.

## Ubuntu notes

On Ubuntu, run `./scripts/install_deps.sh` to install common build tools
(clang, cmake, ninja, pkg-config) and `openjdk-17-jdk` via `apt-get`.
