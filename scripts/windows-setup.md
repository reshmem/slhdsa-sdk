# Windows setup (MSVC)

This repo uses Bash helper scripts and Rust native builds. The smoothest setup is WSL2, but Git Bash also works.

## Recommended tools

- Rust (MSVC toolchain)
- Visual Studio Build Tools (Desktop development with C++)
- Node.js (LTS)
- Java 17+ (JDK)
- CMake + Ninja
- Git Bash or WSL2

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
2) Open a "Developer Command Prompt for VS".
3) From that prompt, launch Git Bash and run:

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
- `uniffi-bindgen` missing: `cargo install uniffi --features cli`.
- `.dll` not found: check `target/release` for `slh_dsa_uniffi.dll` or `libslh_dsa_uniffi.dll` and update `--library` paths.
