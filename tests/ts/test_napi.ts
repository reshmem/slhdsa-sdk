import {
  ParameterSetId,
  slhDsaKeypairGenerate,
  slhDsaSign,
  slhDsaVerify,
} from "../../slh-dsa-napi";

const param = ParameterSetId.Shake256f;
const keypair = slhDsaKeypairGenerate(param);
const msg = Buffer.from("napi smoke test");
const ctx = Buffer.alloc(0);
const sig = slhDsaSign(param, keypair.signingKey, msg, ctx);
const verify = slhDsaVerify(
  param,
  keypair.verifyingKey,
  msg,
  ctx,
  sig.signature
);

if (!verify.valid) {
  throw new Error(`N-API verify failed: ${verify.status}`);
}

console.log("napi ts test ok");
