use getrandom::fill;
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use rand_core::{CryptoRng, RngCore};
use slh_dsa_core::{
    ParameterSet, Sha2_128f, Sha2_128s, Sha2_192f, Sha2_192s, Sha2_256f, Sha2_256s, Shake128f,
    Shake128s, Shake192f, Shake192s, Shake256f, Shake256s,
};

#[napi]
pub enum ParameterSetId {
    Shake128s,
    Shake128f,
    Shake192s,
    Shake192f,
    Shake256s,
    Shake256f,
    Sha2_128s,
    Sha2_128f,
    Sha2_192s,
    Sha2_192f,
    Sha2_256s,
    Sha2_256f,
}

#[napi]
pub enum SlhDsaStatus {
    Ok,
    InvalidParam,
    InvalidLength,
    DecodeError,
    VerifyFailed,
}

#[napi(object)]
pub struct Keypair {
    pub signing_key: Buffer,
    pub verifying_key: Buffer,
}

#[napi(object)]
pub struct SignResult {
    pub status: SlhDsaStatus,
    pub signature: Buffer,
}

#[napi(object)]
pub struct VerifyResult {
    pub status: SlhDsaStatus,
    pub valid: bool,
}

#[napi(object)]
pub struct KeyResult {
    pub status: SlhDsaStatus,
    pub verifying_key: Buffer,
}

struct OsRng;

impl RngCore for OsRng {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        fill(&mut bytes).expect("OsRng failure");
        u32::from_le_bytes(bytes)
    }

    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0u8; 8];
        fill(&mut bytes).expect("OsRng failure");
        u64::from_le_bytes(bytes)
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        fill(dst).expect("OsRng failure");
    }
}

impl CryptoRng for OsRng {}

fn param_name(param: ParameterSetId) -> &'static str {
    match param {
        ParameterSetId::Shake128s => slh_dsa_core::ParameterSetId::Shake128s.name(),
        ParameterSetId::Shake128f => slh_dsa_core::ParameterSetId::Shake128f.name(),
        ParameterSetId::Shake192s => slh_dsa_core::ParameterSetId::Shake192s.name(),
        ParameterSetId::Shake192f => slh_dsa_core::ParameterSetId::Shake192f.name(),
        ParameterSetId::Shake256s => slh_dsa_core::ParameterSetId::Shake256s.name(),
        ParameterSetId::Shake256f => slh_dsa_core::ParameterSetId::Shake256f.name(),
        ParameterSetId::Sha2_128s => slh_dsa_core::ParameterSetId::Sha2_128s.name(),
        ParameterSetId::Sha2_128f => slh_dsa_core::ParameterSetId::Sha2_128f.name(),
        ParameterSetId::Sha2_192s => slh_dsa_core::ParameterSetId::Sha2_192s.name(),
        ParameterSetId::Sha2_192f => slh_dsa_core::ParameterSetId::Sha2_192f.name(),
        ParameterSetId::Sha2_256s => slh_dsa_core::ParameterSetId::Sha2_256s.name(),
        ParameterSetId::Sha2_256f => slh_dsa_core::ParameterSetId::Sha2_256f.name(),
    }
}

fn sk_len(param: ParameterSetId) -> usize {
    let core_id = match param {
        ParameterSetId::Shake128s => slh_dsa_core::ParameterSetId::Shake128s,
        ParameterSetId::Shake128f => slh_dsa_core::ParameterSetId::Shake128f,
        ParameterSetId::Shake192s => slh_dsa_core::ParameterSetId::Shake192s,
        ParameterSetId::Shake192f => slh_dsa_core::ParameterSetId::Shake192f,
        ParameterSetId::Shake256s => slh_dsa_core::ParameterSetId::Shake256s,
        ParameterSetId::Shake256f => slh_dsa_core::ParameterSetId::Shake256f,
        ParameterSetId::Sha2_128s => slh_dsa_core::ParameterSetId::Sha2_128s,
        ParameterSetId::Sha2_128f => slh_dsa_core::ParameterSetId::Sha2_128f,
        ParameterSetId::Sha2_192s => slh_dsa_core::ParameterSetId::Sha2_192s,
        ParameterSetId::Sha2_192f => slh_dsa_core::ParameterSetId::Sha2_192f,
        ParameterSetId::Sha2_256s => slh_dsa_core::ParameterSetId::Sha2_256s,
        ParameterSetId::Sha2_256f => slh_dsa_core::ParameterSetId::Sha2_256f,
    };
    slh_dsa_core::sk_len_for(core_id).unwrap_or(0)
}

fn vk_len(param: ParameterSetId) -> usize {
    let core_id = match param {
        ParameterSetId::Shake128s => slh_dsa_core::ParameterSetId::Shake128s,
        ParameterSetId::Shake128f => slh_dsa_core::ParameterSetId::Shake128f,
        ParameterSetId::Shake192s => slh_dsa_core::ParameterSetId::Shake192s,
        ParameterSetId::Shake192f => slh_dsa_core::ParameterSetId::Shake192f,
        ParameterSetId::Shake256s => slh_dsa_core::ParameterSetId::Shake256s,
        ParameterSetId::Shake256f => slh_dsa_core::ParameterSetId::Shake256f,
        ParameterSetId::Sha2_128s => slh_dsa_core::ParameterSetId::Sha2_128s,
        ParameterSetId::Sha2_128f => slh_dsa_core::ParameterSetId::Sha2_128f,
        ParameterSetId::Sha2_192s => slh_dsa_core::ParameterSetId::Sha2_192s,
        ParameterSetId::Sha2_192f => slh_dsa_core::ParameterSetId::Sha2_192f,
        ParameterSetId::Sha2_256s => slh_dsa_core::ParameterSetId::Sha2_256s,
        ParameterSetId::Sha2_256f => slh_dsa_core::ParameterSetId::Sha2_256f,
    };
    slh_dsa_core::vk_len_for(core_id).unwrap_or(0)
}

fn sig_len(param: ParameterSetId) -> usize {
    let core_id = match param {
        ParameterSetId::Shake128s => slh_dsa_core::ParameterSetId::Shake128s,
        ParameterSetId::Shake128f => slh_dsa_core::ParameterSetId::Shake128f,
        ParameterSetId::Shake192s => slh_dsa_core::ParameterSetId::Shake192s,
        ParameterSetId::Shake192f => slh_dsa_core::ParameterSetId::Shake192f,
        ParameterSetId::Shake256s => slh_dsa_core::ParameterSetId::Shake256s,
        ParameterSetId::Shake256f => slh_dsa_core::ParameterSetId::Shake256f,
        ParameterSetId::Sha2_128s => slh_dsa_core::ParameterSetId::Sha2_128s,
        ParameterSetId::Sha2_128f => slh_dsa_core::ParameterSetId::Sha2_128f,
        ParameterSetId::Sha2_192s => slh_dsa_core::ParameterSetId::Sha2_192s,
        ParameterSetId::Sha2_192f => slh_dsa_core::ParameterSetId::Sha2_192f,
        ParameterSetId::Sha2_256s => slh_dsa_core::ParameterSetId::Sha2_256s,
        ParameterSetId::Sha2_256f => slh_dsa_core::ParameterSetId::Sha2_256f,
    };
    slh_dsa_core::sig_len_for(core_id).unwrap_or(0)
}

#[napi]
pub fn slh_dsa_parameter_name(param: ParameterSetId) -> String {
    param_name(param).to_string()
}

#[napi]
pub fn slh_dsa_signing_key_len(param: ParameterSetId) -> u64 {
    sk_len(param) as u64
}

#[napi]
pub fn slh_dsa_verifying_key_len(param: ParameterSetId) -> u64 {
    vk_len(param) as u64
}

#[napi]
pub fn slh_dsa_signature_len(param: ParameterSetId) -> u64 {
    sig_len(param) as u64
}

#[napi]
pub fn slh_dsa_keypair_generate(param: ParameterSetId) -> Keypair {
    match param {
        ParameterSetId::Shake128s => keypair_for::<Shake128s>(),
        ParameterSetId::Shake128f => keypair_for::<Shake128f>(),
        ParameterSetId::Shake192s => keypair_for::<Shake192s>(),
        ParameterSetId::Shake192f => keypair_for::<Shake192f>(),
        ParameterSetId::Shake256s => keypair_for::<Shake256s>(),
        ParameterSetId::Shake256f => keypair_for::<Shake256f>(),
        ParameterSetId::Sha2_128s => keypair_for::<Sha2_128s>(),
        ParameterSetId::Sha2_128f => keypair_for::<Sha2_128f>(),
        ParameterSetId::Sha2_192s => keypair_for::<Sha2_192s>(),
        ParameterSetId::Sha2_192f => keypair_for::<Sha2_192f>(),
        ParameterSetId::Sha2_256s => keypair_for::<Sha2_256s>(),
        ParameterSetId::Sha2_256f => keypair_for::<Sha2_256f>(),
    }
}

fn keypair_for<P: ParameterSet>() -> Keypair {
    let mut rng = OsRng;
    let (sk, vk) = slh_dsa_core::generate_keypair::<P, _>(&mut rng);
    Keypair {
        signing_key: Buffer::from(sk.to_vec()),
        verifying_key: Buffer::from(vk.to_vec()),
    }
}

#[napi]
pub fn slh_dsa_sign(
    param: ParameterSetId,
    signing_key: Buffer,
    msg: Buffer,
    ctx: Buffer,
) -> SignResult {
    let expected_sk_len = sk_len(param);
    match param {
        ParameterSetId::Shake128s => {
            sign_for::<Shake128s>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Shake128f => {
            sign_for::<Shake128f>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Shake192s => {
            sign_for::<Shake192s>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Shake192f => {
            sign_for::<Shake192f>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Shake256s => {
            sign_for::<Shake256s>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Shake256f => {
            sign_for::<Shake256f>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Sha2_128s => {
            sign_for::<Sha2_128s>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Sha2_128f => {
            sign_for::<Sha2_128f>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Sha2_192s => {
            sign_for::<Sha2_192s>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Sha2_192f => {
            sign_for::<Sha2_192f>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Sha2_256s => {
            sign_for::<Sha2_256s>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Sha2_256f => {
            sign_for::<Sha2_256f>(&signing_key, &msg, &ctx, expected_sk_len)
        }
    }
}

fn sign_for<P: ParameterSet>(
    signing_key: &[u8],
    msg: &[u8],
    ctx: &[u8],
    expected_sk_len: usize,
) -> SignResult {
    if signing_key.len() != expected_sk_len {
        return SignResult {
            status: SlhDsaStatus::InvalidLength,
            signature: Buffer::from(Vec::new()),
        };
    }

    let sk = match slh_dsa_core::SigningKey::<P>::try_from(signing_key) {
        Ok(key) => key,
        Err(_) => {
            return SignResult {
                status: SlhDsaStatus::DecodeError,
                signature: Buffer::from(Vec::new()),
            };
        }
    };

    let mut rng = OsRng;
    let sig = match slh_dsa_core::sign_with_rng::<P, _>(&sk, &mut rng, msg, ctx) {
        Ok(signature) => signature,
        Err(_) => {
            return SignResult {
                status: SlhDsaStatus::DecodeError,
                signature: Buffer::from(Vec::new()),
            };
        }
    };

    SignResult {
        status: SlhDsaStatus::Ok,
        signature: Buffer::from(sig.to_vec()),
    }
}

#[napi]
pub fn slh_dsa_sign_deterministic(
    param: ParameterSetId,
    signing_key: Buffer,
    msg: Buffer,
    ctx: Buffer,
) -> SignResult {
    let expected_sk_len = sk_len(param);
    match param {
        ParameterSetId::Shake128s => {
            sign_det_for::<Shake128s>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Shake128f => {
            sign_det_for::<Shake128f>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Shake192s => {
            sign_det_for::<Shake192s>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Shake192f => {
            sign_det_for::<Shake192f>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Shake256s => {
            sign_det_for::<Shake256s>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Shake256f => {
            sign_det_for::<Shake256f>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Sha2_128s => {
            sign_det_for::<Sha2_128s>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Sha2_128f => {
            sign_det_for::<Sha2_128f>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Sha2_192s => {
            sign_det_for::<Sha2_192s>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Sha2_192f => {
            sign_det_for::<Sha2_192f>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Sha2_256s => {
            sign_det_for::<Sha2_256s>(&signing_key, &msg, &ctx, expected_sk_len)
        }
        ParameterSetId::Sha2_256f => {
            sign_det_for::<Sha2_256f>(&signing_key, &msg, &ctx, expected_sk_len)
        }
    }
}

fn sign_det_for<P: ParameterSet>(
    signing_key: &[u8],
    msg: &[u8],
    ctx: &[u8],
    expected_sk_len: usize,
) -> SignResult {
    if signing_key.len() != expected_sk_len {
        return SignResult {
            status: SlhDsaStatus::InvalidLength,
            signature: Buffer::from(Vec::new()),
        };
    }

    let sk = match slh_dsa_core::SigningKey::<P>::try_from(signing_key) {
        Ok(key) => key,
        Err(_) => {
            return SignResult {
                status: SlhDsaStatus::DecodeError,
                signature: Buffer::from(Vec::new()),
            };
        }
    };

    let sig = match slh_dsa_core::sign_deterministic::<P>(&sk, msg, ctx) {
        Ok(signature) => signature,
        Err(_) => {
            return SignResult {
                status: SlhDsaStatus::DecodeError,
                signature: Buffer::from(Vec::new()),
            };
        }
    };

    SignResult {
        status: SlhDsaStatus::Ok,
        signature: Buffer::from(sig.to_vec()),
    }
}

#[napi]
pub fn slh_dsa_verifying_key_from_signing_key(
    param: ParameterSetId,
    signing_key: Buffer,
) -> KeyResult {
    let expected_sk_len = sk_len(param);
    match param {
        ParameterSetId::Shake128s => vk_from_sk_for::<Shake128s>(&signing_key, expected_sk_len),
        ParameterSetId::Shake128f => vk_from_sk_for::<Shake128f>(&signing_key, expected_sk_len),
        ParameterSetId::Shake192s => vk_from_sk_for::<Shake192s>(&signing_key, expected_sk_len),
        ParameterSetId::Shake192f => vk_from_sk_for::<Shake192f>(&signing_key, expected_sk_len),
        ParameterSetId::Shake256s => vk_from_sk_for::<Shake256s>(&signing_key, expected_sk_len),
        ParameterSetId::Shake256f => vk_from_sk_for::<Shake256f>(&signing_key, expected_sk_len),
        ParameterSetId::Sha2_128s => vk_from_sk_for::<Sha2_128s>(&signing_key, expected_sk_len),
        ParameterSetId::Sha2_128f => vk_from_sk_for::<Sha2_128f>(&signing_key, expected_sk_len),
        ParameterSetId::Sha2_192s => vk_from_sk_for::<Sha2_192s>(&signing_key, expected_sk_len),
        ParameterSetId::Sha2_192f => vk_from_sk_for::<Sha2_192f>(&signing_key, expected_sk_len),
        ParameterSetId::Sha2_256s => vk_from_sk_for::<Sha2_256s>(&signing_key, expected_sk_len),
        ParameterSetId::Sha2_256f => vk_from_sk_for::<Sha2_256f>(&signing_key, expected_sk_len),
    }
}

fn vk_from_sk_for<P: ParameterSet>(signing_key: &[u8], expected_sk_len: usize) -> KeyResult {
    if signing_key.len() != expected_sk_len {
        return KeyResult {
            status: SlhDsaStatus::InvalidLength,
            verifying_key: Buffer::from(Vec::new()),
        };
    }

    let sk = match slh_dsa_core::SigningKey::<P>::try_from(signing_key) {
        Ok(key) => key,
        Err(_) => {
            return KeyResult {
                status: SlhDsaStatus::DecodeError,
                verifying_key: Buffer::from(Vec::new()),
            };
        }
    };

    KeyResult {
        status: SlhDsaStatus::Ok,
        verifying_key: Buffer::from(sk.as_ref().to_vec()),
    }
}

#[napi]
pub fn slh_dsa_verify(
    param: ParameterSetId,
    verifying_key: Buffer,
    msg: Buffer,
    ctx: Buffer,
    signature: Buffer,
) -> VerifyResult {
    let expected_vk_len = vk_len(param);
    let expected_sig_len = sig_len(param);
    match param {
        ParameterSetId::Shake128s => verify_for::<Shake128s>(
            &verifying_key,
            &msg,
            &ctx,
            &signature,
            expected_vk_len,
            expected_sig_len,
        ),
        ParameterSetId::Shake128f => verify_for::<Shake128f>(
            &verifying_key,
            &msg,
            &ctx,
            &signature,
            expected_vk_len,
            expected_sig_len,
        ),
        ParameterSetId::Shake192s => verify_for::<Shake192s>(
            &verifying_key,
            &msg,
            &ctx,
            &signature,
            expected_vk_len,
            expected_sig_len,
        ),
        ParameterSetId::Shake192f => verify_for::<Shake192f>(
            &verifying_key,
            &msg,
            &ctx,
            &signature,
            expected_vk_len,
            expected_sig_len,
        ),
        ParameterSetId::Shake256s => verify_for::<Shake256s>(
            &verifying_key,
            &msg,
            &ctx,
            &signature,
            expected_vk_len,
            expected_sig_len,
        ),
        ParameterSetId::Shake256f => verify_for::<Shake256f>(
            &verifying_key,
            &msg,
            &ctx,
            &signature,
            expected_vk_len,
            expected_sig_len,
        ),
        ParameterSetId::Sha2_128s => verify_for::<Sha2_128s>(
            &verifying_key,
            &msg,
            &ctx,
            &signature,
            expected_vk_len,
            expected_sig_len,
        ),
        ParameterSetId::Sha2_128f => verify_for::<Sha2_128f>(
            &verifying_key,
            &msg,
            &ctx,
            &signature,
            expected_vk_len,
            expected_sig_len,
        ),
        ParameterSetId::Sha2_192s => verify_for::<Sha2_192s>(
            &verifying_key,
            &msg,
            &ctx,
            &signature,
            expected_vk_len,
            expected_sig_len,
        ),
        ParameterSetId::Sha2_192f => verify_for::<Sha2_192f>(
            &verifying_key,
            &msg,
            &ctx,
            &signature,
            expected_vk_len,
            expected_sig_len,
        ),
        ParameterSetId::Sha2_256s => verify_for::<Sha2_256s>(
            &verifying_key,
            &msg,
            &ctx,
            &signature,
            expected_vk_len,
            expected_sig_len,
        ),
        ParameterSetId::Sha2_256f => verify_for::<Sha2_256f>(
            &verifying_key,
            &msg,
            &ctx,
            &signature,
            expected_vk_len,
            expected_sig_len,
        ),
    }
}

fn verify_for<P: ParameterSet>(
    verifying_key: &[u8],
    msg: &[u8],
    ctx: &[u8],
    signature: &[u8],
    expected_vk_len: usize,
    expected_sig_len: usize,
) -> VerifyResult {
    if verifying_key.len() != expected_vk_len || signature.len() != expected_sig_len {
        return VerifyResult {
            status: SlhDsaStatus::InvalidLength,
            valid: false,
        };
    }

    let vk = match slh_dsa_core::VerifyingKey::<P>::try_from(verifying_key) {
        Ok(key) => key,
        Err(_) => {
            return VerifyResult {
                status: SlhDsaStatus::DecodeError,
                valid: false,
            };
        }
    };

    let sig = match slh_dsa_core::Signature::<P>::try_from(signature) {
        Ok(sig) => sig,
        Err(_) => {
            return VerifyResult {
                status: SlhDsaStatus::DecodeError,
                valid: false,
            };
        }
    };

    match slh_dsa_core::verify_with_context::<P>(&vk, msg, ctx, &sig) {
        Ok(_) => VerifyResult {
            status: SlhDsaStatus::Ok,
            valid: true,
        },
        Err(_) => VerifyResult {
            status: SlhDsaStatus::VerifyFailed,
            valid: false,
        },
    }
}
