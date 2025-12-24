export enum ParameterSetId {
  Shake128s = "Shake128s",
  Shake128f = "Shake128f",
  Shake192s = "Shake192s",
  Shake192f = "Shake192f",
  Shake256s = "Shake256s",
  Shake256f = "Shake256f",
  Sha2_128s = "Sha2_128s",
  Sha2_128f = "Sha2_128f",
  Sha2_192s = "Sha2_192s",
  Sha2_192f = "Sha2_192f",
  Sha2_256s = "Sha2_256s",
  Sha2_256f = "Sha2_256f",
}

export enum SlhDsaStatus {
  Ok = "Ok",
  InvalidParam = "InvalidParam",
  InvalidLength = "InvalidLength",
  DecodeError = "DecodeError",
  VerifyFailed = "VerifyFailed",
}

export type Keypair = {
  signingKey: Buffer;
  verifyingKey: Buffer;
};

export type SignResult = {
  status: SlhDsaStatus;
  signature: Buffer;
};

export type VerifyResult = {
  status: SlhDsaStatus;
  valid: boolean;
};

export type KeyResult = {
  status: SlhDsaStatus;
  verifyingKey: Buffer;
};

export function slhDsaParameterName(param: ParameterSetId): string;
export function slhDsaSigningKeyLen(param: ParameterSetId): number;
export function slhDsaVerifyingKeyLen(param: ParameterSetId): number;
export function slhDsaSignatureLen(param: ParameterSetId): number;
export function slhDsaKeypairGenerate(param: ParameterSetId): Keypair;
export function slhDsaSign(
  param: ParameterSetId,
  signingKey: Buffer,
  msg: Buffer,
  ctx: Buffer
): SignResult;
export function slhDsaSignDeterministic(
  param: ParameterSetId,
  signingKey: Buffer,
  msg: Buffer,
  ctx: Buffer
): SignResult;
export function slhDsaVerifyingKeyFromSigningKey(
  param: ParameterSetId,
  signingKey: Buffer
): KeyResult;
export function slhDsaVerify(
  param: ParameterSetId,
  verifyingKey: Buffer,
  msg: Buffer,
  ctx: Buffer,
  signature: Buffer
): VerifyResult;
