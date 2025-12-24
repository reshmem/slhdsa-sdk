import { performance } from "perf_hooks";

import {
  ParameterSetId,
  slhDsaKeypairGenerate,
  slhDsaSignDeterministic,
  slhDsaVerify,
} from "../../slh-dsa-napi";

const sizes = [32, 64, 256, 1024, 4096, 8128, 32768];
const iters = Number.parseInt(process.env.BENCH_ITERS ?? "5", 10);
const warmup = Number.parseInt(process.env.BENCH_WARMUP ?? "1", 10);

const param = ParameterSetId.Shake256f;

console.log("param_set,SLH-DSA-SHAKE-256f");
console.log("size_bytes,sign_ms,verify_ms,total_ms");

for (const size of sizes) {
  const msg = Buffer.alloc(size);
  for (let i = 0; i < size; i += 1) {
    msg[i] = (i * 31) & 0xff;
  }
  const ctx = Buffer.alloc(0);

  const keypair = slhDsaKeypairGenerate(param);

  for (let i = 0; i < warmup; i += 1) {
    const sig = slhDsaSignDeterministic(param, keypair.signingKey, msg, ctx);
    slhDsaVerify(param, keypair.verifyingKey, msg, ctx, sig.signature);
  }

  let signTotal = 0;
  let verifyTotal = 0;

  for (let i = 0; i < iters; i += 1) {
    const startSign = performance.now();
    const sig = slhDsaSignDeterministic(param, keypair.signingKey, msg, ctx);
    signTotal += performance.now() - startSign;

    const startVerify = performance.now();
    const verify = slhDsaVerify(param, keypair.verifyingKey, msg, ctx, sig.signature);
    verifyTotal += performance.now() - startVerify;
    if (!verify.valid) {
      throw new Error("Verify failed");
    }
  }

  const signAvg = signTotal / iters;
  const verifyAvg = verifyTotal / iters;
  const total = signAvg + verifyAvg;
  console.log(`${size},${signAvg.toFixed(6)},${verifyAvg.toFixed(6)},${total.toFixed(6)}`);
}
