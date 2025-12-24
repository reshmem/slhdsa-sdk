# Benchmarks

Benchmarks target the SLH-DSA SHAKE-256f parameter set and measure sign + verify latency
for payload sizes: 32, 64, 256, 1024, 4096, 8128, 32768 bytes.

Common env vars:

- `BENCH_ITERS` (default 5)
- `BENCH_WARMUP` (default 1)

## Rust

```bash
./bench/rust/run.sh
```

## Python

```bash
./bench/python/run.sh
```

## Kotlin (JNA)

```bash
./bench/kotlin/run.sh
```

## TypeScript (React Native JSI)

This requires a React Native environment with the JSI module registered as
`globalThis.NativeSlhDsaUniffi`.

```bash
./bench/ts/run.sh
```
