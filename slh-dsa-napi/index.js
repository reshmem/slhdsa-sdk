"use strict";

const fs = require("fs");
const path = require("path");

function loadNative() {
  const override = process.env.SLH_DSA_NAPI_PATH;
  const nativePath = override
    ? path.resolve(override)
    : path.join(__dirname, "slh_dsa_napi.node");

  if (!fs.existsSync(nativePath)) {
    throw new Error(
      `Missing native module at ${nativePath}. Build it with ./scripts/build-napi.sh`
    );
  }

  return require(nativePath);
}

const native = loadNative();

module.exports = native;
