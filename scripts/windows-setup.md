# Windows setup (MSVC)

This repo uses Bash helper scripts and Rust native builds. The smoothest setup is WSL2, but Git Bash also works.

## Recommended tools

- Rust (MSVC toolchain)
- Visual Studio Build Tools (Desktop development with C++ + Windows SDK)
- Node.js (LTS) + npm
- Java 17+ (JDK)
- Git
- CMake + Ninja
- Git Bash or WSL2

Optional (depending on targets):

- Android Studio (SDK, NDK, CMake) for Android builds
- Kotlin compiler (`kotlinc`) and `ktlint` for Kotlin tests/formatting
- Python 3.x if you plan to use the Python UniFFI bindings

## Feature to dependency checklist

| Feature | Required dependencies |
| --- | --- |
| Rust core / C FFI | Rust (MSVC preferred), VS Build Tools (C++), CMake, Ninja |
| Node.js N-API | Rust (MSVC preferred), VS Build Tools (C++), Node.js (LTS) |
| UniFFI bindings (Python/Swift/Kotlin/TS) | Rust (MSVC preferred), `uniffi-bindgen` CLI |
| Python bindings | Python 3.x, UniFFI bindings |
| Kotlin bindings/tests | JDK 17+, Kotlin compiler, `ktlint` (optional) |
| React Native Android | Android Studio (SDK + NDK + CMake), `ANDROID_HOME`, JDK 17+ |
| React Native iOS | macOS + Xcode (build on macOS, not Windows) |

## Official installers

- Rust: https://www.rust-lang.org/tools/install
- Visual Studio Build Tools: https://visualstudio.microsoft.com/downloads/ (Build Tools for Visual Studio)
- Node.js (LTS): https://nodejs.org/en/download
- Git: https://git-scm.com/download/win
- CMake: https://cmake.org/download/
- Ninja: https://github.com/ninja-build/ninja/releases
- Java 17 (JDK): https://adoptium.net/temurin/releases/?version=17
- Android Studio: https://developer.android.com/studio
- Python: https://www.python.org/downloads/windows/
- Git Bash (bundled with Git): https://git-scm.com/download/win
- WSL2: https://learn.microsoft.com/windows/wsl/install

## Verify tools

```bash
rustc -V
cargo -V
cmake --version
ninja --version
node -v
npm -v
java -version
git --version
```

## PATH tips (Windows shells)

- PowerShell: make sure install directories are in the System PATH, then restart the shell.
- Git Bash: inherits PATH from Windows; if a tool isn't found, add it to the Windows PATH and reopen Git Bash.
- WSL2: uses its own PATH; Windows installs are not automatically visible unless you enable interop.

## Quickstart (WSL2)

```bash
sudo apt-get update
sudo apt-get install -y build-essential clang cmake ninja-build pkg-config
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup default stable-x86_64-unknown-linux-gnu

cd /mnt/c/Path/To/slhdsa-sdk
npm install
./scripts/build.sh
./scripts/gen-bindings.sh
```

## Quickstart (Git Bash + MSVC)

1) Install Visual Studio Build Tools with the C++ workload.
2) Open a "Developer Command Prompt for VS" so `cl.exe` and `link.exe` are on PATH.
3) Install Rust and the MSVC toolchain:
```bash
rustup toolchain install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc
```
4) From that prompt, launch Git Bash and run:

```bash
npm install
./scripts/build.sh
./scripts/gen-bindings.sh
```

## Environment variables

- `JAVA_HOME`: `C:\Program Files\Java\jdk-17`
- `ANDROID_HOME`: `C:\Users\you\AppData\Local\Android\Sdk`
- `CARGO_TARGET_DIR`: optional override for Rust build output

## Troubleshooting

- `link.exe` not found: install the VS Build Tools C++ workload and use the MSVC toolchain.
- MSVC vs GNU toolchains: prefer `stable-x86_64-pc-windows-msvc` for Windows builds to match the VS linker.
- `uniffi-bindgen` missing: `cargo install uniffi --features cli`.
- `.dll` not found: check `target/release` for `slh_dsa_uniffi.dll` or `libslh_dsa_uniffi.dll` and update `--library` paths.
- Node.js N-API output: `slh-dsa-napi/slh_dsa_napi.node` after `./scripts/build-napi.sh`.
