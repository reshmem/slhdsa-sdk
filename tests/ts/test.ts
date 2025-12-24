import {
  ParameterSetId,
  slhDsaParameterName,
  slhDsaSigningKeyLen,
} from "../../bindings/ts/slh_dsa_uniffi";

// Smoke-check exports. Do not call into native code here.
const param = ParameterSetId.Shake128f;
const name = slhDsaParameterName;
const skLen = slhDsaSigningKeyLen;

if (typeof name !== "function" || typeof skLen !== "function") {
  throw new Error("Bindings exports are missing");
}

console.log("ts test ok", param);
