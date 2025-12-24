# SLH-DSA Bench App

This React Native app runs the Shake-256f bench via the UniFFI JSI bindings.

## Setup

From this directory:

```bash
npm install
```

Generate native bindings and build Rust for the platform (from the library):

```bash
(cd ../../react-native-slh-dsa && npm run ubrn:ios)
(cd ../../react-native-slh-dsa && npm run ubrn:android)
```

Then run the app:

```bash
npm run ios
npm run android
```

Scripts:

```bash
./scripts/setup.sh
./scripts/build-native.sh
./scripts/run-ios.sh
./scripts/run-android.sh
```

If you see "Make sure the packager is running", use the run scripts above or start Metro manually:

```bash
PORT=8082 npm run start -- --port 8082
RCT_METRO_PORT=8082 npm run ios -- --no-packager --port 8082
```

## Bench

Open the app and tap:

- **Install JSI**
- **Run Bench**

Output is shown in-app as CSV rows (size, sign_ms, verify_ms, total_ms).

## Notes

- `ubrn.yml` points at the root Rust workspace (`../..`) and the `slh-dsa-uniffi` crate.
- The JSI bindings are generated into `src/uniffi` and `cpp/bindings`.
