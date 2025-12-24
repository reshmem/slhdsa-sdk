import {
  ParameterSetId,
  slhDsaKeypairGenerate,
  slhDsaSignDeterministic,
  slhDsaVerify,
} from "react-native-slh-dsa";

export type BenchRow = {
  size: number;
  signMs: number;
  verifyMs: number;
  totalMs: number;
};

const sizes = [32, 64, 256, 1024, 4096, 8128, 32768];

const nowMs = (): number => {
  const perf = (globalThis as any).performance;
  if (perf && typeof perf.now === "function") {
    return perf.now();
  }
  return Date.now();
};

export function runBench(iters = 5, warmup = 1): BenchRow[] {
  const param = ParameterSetId.Shake256f;
  const rows: BenchRow[] = [];

  for (const size of sizes) {
    const msg = new Uint8Array(size);
    for (let i = 0; i < size; i += 1) {
      msg[i] = (i * 31) & 0xff;
    }
    const ctx = new Uint8Array(0);

    const keypair = slhDsaKeypairGenerate(param);

    for (let i = 0; i < warmup; i += 1) {
      const sig = slhDsaSignDeterministic(
        param,
        keypair.signingKey,
        msg.buffer,
        ctx.buffer,
      );
      slhDsaVerify(
        param,
        keypair.verifyingKey,
        msg.buffer,
        ctx.buffer,
        sig.signature,
      );
    }

    let signTotal = 0;
    let verifyTotal = 0;

    for (let i = 0; i < iters; i += 1) {
      const startSign = nowMs();
      const sig = slhDsaSignDeterministic(
        param,
        keypair.signingKey,
        msg.buffer,
        ctx.buffer,
      );
      signTotal += nowMs() - startSign;

      const startVerify = nowMs();
      const verify = slhDsaVerify(
        param,
        keypair.verifyingKey,
        msg.buffer,
        ctx.buffer,
        sig.signature,
      );
      verifyTotal += nowMs() - startVerify;

      if (!verify.valid) {
        throw new Error("Verify failed");
      }
    }

    const signAvg = signTotal / iters;
    const verifyAvg = verifyTotal / iters;
    rows.push({
      size,
      signMs: signAvg,
      verifyMs: verifyAvg,
      totalMs: signAvg + verifyAvg,
    });
  }

  return rows;
}
