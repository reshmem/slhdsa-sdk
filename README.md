# slhdsa-sdk

Rust SLH-DSA core library with C FFI + UniFFI bindings (Python/Swift/Kotlin), React Native TypeScript/JSI bindings, and a Node.js N-API TypeScript module.

## Prerequisites

- Rust toolchain (stable)
- `npm` or `yarn`
- Optional: `ktlint` for Kotlin formatting (if desired)
- iOS builds: Xcode + CocoaPods
- Android builds: Android SDK/NDK + CMake + Java 17+
- Linux builds (Ubuntu): build-essential, clang, cmake, ninja, pkg-config, JDK 17+
- Windows builds: Visual Studio Build Tools (Desktop development with C++ + Windows SDK), CMake, Ninja, Java 17+, Git, Node.js (LTS)

## Windows notes

- Use WSL2 (Ubuntu) or Git Bash to run the `./scripts/*.sh` helpers.
- `./scripts/install_deps.sh` is Linux/macOS only; install deps manually on Windows.
- Rust MSVC toolchain recommended: `rustup toolchain install stable-x86_64-pc-windows-msvc` then `rustup default stable-x86_64-pc-windows-msvc`.
- Windows shared libs are `.dll` and may not have the `lib` prefix; check `target/release` when wiring `--library` paths.
- For Android builds on Windows: install Android Studio (SDK + NDK + CMake) and set `ANDROID_HOME`.
- For Kotlin tests: install `kotlinc` and `ktlint` (optional).
- Prefer the MSVC toolchain over GNU for compatibility with VS Build Tools (see `scripts/windows-setup.md`).
- If you hit path/space issues, set `CARGO_TARGET_DIR` to a short path (e.g. `C:\slh-target`).
- iOS builds require macOS + Xcode; on Windows, run iOS builds on a Mac CI or dev machine.

## Windows quickstart (MSVC)

1) Install Rust (MSVC), Node.js (LTS), Java 17+, Git, CMake, Ninja, and Visual Studio Build Tools (C++ workload).
2) Open Git Bash or WSL2 in the repo root.
3) Install JS deps:
```bash
npm install
```
4) Build native libs:
```bash
./scripts/build.sh
```
5) Generate bindings:
```bash
./scripts/gen-bindings.sh
```

PowerShell variant (uses Git Bash for scripts):

```powershell
npm install
bash ./scripts/build.sh
bash ./scripts/gen-bindings.sh
```

More details: `scripts/windows-setup.md`.

## Install Cargo tools

```bash
cargo install uniffi --features cli
cargo install cargo-ndk
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

Note: these targets are only needed when building iOS/Android from this machine.

## Install JS tooling

```bash
npm install
```

This installs `uniffi-bindgen-react-native` and `prettier` (used for TS formatting).
It also installs `typescript` and `tsx` for the TypeScript test runner.

Windows note: use `npm` if your environment blocks Yarn config from your home directory.

You can install all dependencies at once with:

```bash
./scripts/install_deps.sh
```

On Ubuntu, `./scripts/install_deps.sh` will use `apt-get` to install common
build tools and `openjdk-17-jdk`.

## Run tests

```bash
./scripts/run-tests.sh
```

Notes:

- Kotlin tests download `jna-5.14.0.jar` on first run.
- Kotlin requires `kotlinc` and `ktlint` (installed by `./scripts/install_deps.sh` when `brew` is available).
- Gradle is installed by `./scripts/install_deps.sh` when `brew` is available.
- Windows: run tests from WSL2 or Git Bash; Android/iOS-specific tests require their respective SDKs and platforms.

## Benchmarks

See `bench/README.md` for per-language benchmarks (Rust, Python, Kotlin, Node.js N-API, TypeScript/React Native).
Run them all with:

```bash
./scripts/bench.sh
```

## Example React Native app

See `example-project/SlhDsaBenchApp/README.md` for a React Native app that runs the Shake-256f bench on-device.
Make sure `ANDROID_HOME` points to your Android SDK and `JAVA_HOME` points to a Java 17+ JDK.
Windows example:
- `ANDROID_HOME=C:\Users\you\AppData\Local\Android\Sdk`
- `JAVA_HOME=C:\Program Files\Java\jdk-17`

## Architecture

See `Architecture.md` for a guided tour of the layers and integration patterns.

## Third-party integration (git submodule)

If you consume this repo from another React Native project:

1) Add the submodule:
```bash
git submodule add <repo-url> vendor/slhdsa-sdk
git submodule update --init --recursive
```
2) Build the library:
```bash
cd vendor/slhdsa-sdk
npm install
./scripts/build.sh
./scripts/gen-bindings.sh
cd react-native-slh-dsa
npm run ubrn:ios
npm run ubrn:android
```
3) Install into your app:
```bash
npm install ./vendor/slhdsa-sdk/react-native-slh-dsa
```
4) iOS:
```bash
cd ios && pod install
```
5) Android:
- Set `ANDROID_HOME` and `JAVA_HOME`
- Ensure NDK + CMake installed

If you use symlinks/monorepo layout, update Metro’s `watchFolders` and `extraNodeModules`
to include `vendor/slhdsa-sdk/react-native-slh-dsa`.

## Updating API (Rust upstream -> bindings)

When you add or change a public API in the Rust crates, regenerate the UniFFI bindings and rebuild the RN library:

1) Update the UniFFI surface in `slh-dsa-uniffi/src/lib.rs` (export the new API).
2) Rebuild the Rust libs and N-API addon:
```bash
./scripts/build.sh
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
./scripts/build.sh
```

Artifacts are built in the cargo target directory:

```bash
${CARGO_TARGET_DIR:-$PWD/target}
```

Key artifacts live in `$(CARGO_TARGET_DIR:-$PWD/target)/release/`:

- `libslh_dsa_ffi` (C ABI)
- `libslh_dsa_uniffi` (UniFFI)
- `slh-dsa-napi/slh_dsa_napi.node` (Node.js N-API)

## Generate bindings

You can use the helper scripts:

```bash
./scripts/build.sh
./scripts/gen-bindings.sh
```

Order matters: build first, then generate bindings.

`scripts/gen-bindings.sh` creates a temporary symlink if your target dir contains spaces (required by the React Native bindgen).

Platform library names:

- macOS: `libslh_dsa_uniffi.dylib`
- Linux: `libslh_dsa_uniffi.so`
- Windows (MSVC): `slh_dsa_uniffi.dll` (GNU toolchains may produce `libslh_dsa_uniffi.dll`)

## Windows environment variables

- `JAVA_HOME`: point at a JDK 17+ install, e.g. `C:\Program Files\Java\jdk-17`.
- `ANDROID_HOME`: point at the Android SDK root, e.g. `C:\Users\you\AppData\Local\Android\Sdk`.
- `CARGO_TARGET_DIR`: optional override for Rust build output; avoid spaces in the path if possible.

## WSL2 vs Git Bash

- WSL2: best compatibility for `./scripts/*.sh` and Linux-y toolchains; keep the repo inside the WSL filesystem for best file permission behavior.
- Git Bash: works well for the bash scripts, but path/line-ending quirks can pop up if files get checked out with CRLF.

### Python

```bash
uniffi-bindgen generate \
  --library "${CARGO_TARGET_DIR:-$PWD/target}/release/libslh_dsa_uniffi.dylib" \
  --language python \
  --out-dir bindings/python
```

Windows (MSVC) example:

```bash
uniffi-bindgen generate \
  --library "${CARGO_TARGET_DIR:-$PWD/target}/release/slh_dsa_uniffi.dll" \
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

Windows (MSVC) example:

```bash
uniffi-bindgen generate \
  --library "${CARGO_TARGET_DIR:-$PWD/target}/release/slh_dsa_uniffi.dll" \
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

Windows (MSVC) example:

```bash
npx uniffi-bindgen-react-native generate jsi bindings \
  --library \
  --ts-dir bindings/ts \
  --cpp-dir bindings/cpp \
  "${CARGO_TARGET_DIR:-$PWD/target}/release/slh_dsa_uniffi.dll"
```

Outputs:

- TypeScript: `bindings/ts/slh_dsa_uniffi.ts`, `bindings/ts/slh_dsa_uniffi-ffi.ts`
- C++ JSI glue: `bindings/cpp/slh_dsa_uniffi.cpp`, `bindings/cpp/slh_dsa_uniffi.hpp`

### Node.js N-API (TypeScript)

Build the native addon for your host platform:

```bash
./scripts/build-napi.sh
```

You can override the native path with `SLH_DSA_NAPI_PATH=/path/to/slh_dsa_napi.node`.

Windows example:

```bash
./scripts/build-napi.sh
# Output: slh-dsa-napi/slh_dsa_napi.node
```

Use it from Node.js:

```js
const slh = require("./slh-dsa-napi");

const param = slh.ParameterSetId.Shake256f;
const keypair = slh.slhDsaKeypairGenerate(param);
const msg = Buffer.from("hello");
const ctx = Buffer.alloc(0);
const sig = slh.slhDsaSign(param, keypair.signingKey, msg, ctx);
const ok = slh.slhDsaVerify(
  param,
  keypair.verifyingKey,
  msg,
  ctx,
  sig.signature
);
console.log(ok);
```

## C header

The C header is generated via cbindgen during build:

```
slh-dsa-ffi/include/slh_dsa.h
```

## Notes

- Install the UniFFI CLI once: `cargo install uniffi --features cli`
- If your cargo target directory changes, update the `--library` path in commands above.

## Windows troubleshooting

- `error: linker 'link.exe' not found`: install Visual Studio Build Tools with the C++ workload and use the MSVC toolchain.
- `cl.exe`/`link.exe` found but build fails: run from a Developer Command Prompt or set up the MSVC environment in your shell.
- `uniffi-bindgen` can’t find the `.dll`: check `target/release` for `slh_dsa_uniffi.dll` or `libslh_dsa_uniffi.dll` and update `--library`.
- `CMake`/`ninja` not found: install both and ensure they are on `PATH`.
