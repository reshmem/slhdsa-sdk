"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const slh_dsa_uniffi_1 = require("../../bindings/ts/slh_dsa_uniffi");
// Smoke-check exports. Do not call into native code here.
const param = slh_dsa_uniffi_1.ParameterSetId.Shake128f;
const name = slh_dsa_uniffi_1.slhDsaParameterName;
const skLen = slh_dsa_uniffi_1.slhDsaSigningKeyLen;
if (typeof name !== "function" || typeof skLen !== "function") {
    throw new Error("Bindings exports are missing");
}
console.log("ts test ok", param);
