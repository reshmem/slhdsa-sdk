# react-native-slh-dsa

React Native TurboModule wrapper for the SLH-DSA UniFFI bindings.

## Build

From this directory:

```bash
npm install
npm run ubrn:ios
npm run ubrn:android
```

These commands generate TurboModule glue code and build the Rust library.

## App usage

In the app, add a file dependency:

```json
"react-native-slh-dsa": "file:../../react-native-slh-dsa"
```

Then use it in JS:

```ts
import NativeSlhDsa from 'react-native-slh-dsa/NativeSlhDsa';
import { slhDsaKeypairGenerate } from 'react-native-slh-dsa';

NativeSlhDsa.installRustCrate();
```
