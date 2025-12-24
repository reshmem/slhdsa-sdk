//! UniFFI bindings for SLH-DSA.

use getrandom::fill;
use rand_core::{CryptoRng, RngCore};
use slh_dsa_core::{
    ParameterSet, Sha2_128f, Sha2_128s, Sha2_192f, Sha2_192s, Sha2_256f, Sha2_256s, Shake128f,
    Shake128s, Shake192f, Shake192s, Shake256f, Shake256s,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, uniffi::Enum)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, uniffi::Enum)]
pub enum SlhDsaStatus {
    Ok,
    InvalidParam,
    InvalidLength,
    DecodeError,
    VerifyFailed,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct Keypair {
    pub signing_key: Vec<u8>,
    pub verifying_key: Vec<u8>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct SignResult {
    pub status: SlhDsaStatus,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct VerifyResult {
    pub status: SlhDsaStatus,
    pub valid: bool,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct KeyResult {
    pub status: SlhDsaStatus,
    pub verifying_key: Vec<u8>,
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

fn parameter_name(param: ParameterSetId) -> &'static str {
    match param {
        ParameterSetId::Shake128s => "SLH-DSA-SHAKE-128s",
        ParameterSetId::Shake128f => "SLH-DSA-SHAKE-128f",
        ParameterSetId::Shake192s => "SLH-DSA-SHAKE-192s",
        ParameterSetId::Shake192f => "SLH-DSA-SHAKE-192f",
        ParameterSetId::Shake256s => "SLH-DSA-SHAKE-256s",
        ParameterSetId::Shake256f => "SLH-DSA-SHAKE-256f",
        ParameterSetId::Sha2_128s => "SLH-DSA-SHA2-128s",
        ParameterSetId::Sha2_128f => "SLH-DSA-SHA2-128f",
        ParameterSetId::Sha2_192s => "SLH-DSA-SHA2-192s",
        ParameterSetId::Sha2_192f => "SLH-DSA-SHA2-192f",
        ParameterSetId::Sha2_256s => "SLH-DSA-SHA2-256s",
        ParameterSetId::Sha2_256f => "SLH-DSA-SHA2-256f",
    }
}

fn signing_key_len(param: ParameterSetId) -> usize {
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

fn verifying_key_len(param: ParameterSetId) -> usize {
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

fn signature_len(param: ParameterSetId) -> usize {
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

fn keypair_generate<P: ParameterSet>() -> Keypair {
    let mut rng = OsRng;
    let (sk, vk) = slh_dsa_core::generate_keypair::<P, _>(&mut rng);
    Keypair {
        signing_key: sk.to_bytes().to_vec(),
        verifying_key: vk.to_bytes().to_vec(),
    }
}

fn sign<P: ParameterSet>(sk_bytes: &[u8], msg: &[u8], ctx: &[u8]) -> SignResult {
    let sk = match slh_dsa_core::SigningKey::<P>::try_from(sk_bytes) {
        Ok(key) => key,
        Err(_) => {
            return SignResult {
                status: SlhDsaStatus::DecodeError,
                signature: Vec::new(),
            };
        }
    };

    let mut rng = OsRng;
    let sig = match slh_dsa_core::sign_with_rng::<P, _>(&sk, &mut rng, msg, ctx) {
        Ok(sig) => sig,
        Err(_) => {
            return SignResult {
                status: SlhDsaStatus::InvalidParam,
                signature: Vec::new(),
            };
        }
    };

    SignResult {
        status: SlhDsaStatus::Ok,
        signature: sig.to_bytes().to_vec(),
    }
}

fn sign_deterministic<P: ParameterSet>(sk_bytes: &[u8], msg: &[u8], ctx: &[u8]) -> SignResult {
    let sk = match slh_dsa_core::SigningKey::<P>::try_from(sk_bytes) {
        Ok(key) => key,
        Err(_) => {
            return SignResult {
                status: SlhDsaStatus::DecodeError,
                signature: Vec::new(),
            };
        }
    };

    let sig = match slh_dsa_core::sign_deterministic::<P>(&sk, msg, ctx) {
        Ok(sig) => sig,
        Err(_) => {
            return SignResult {
                status: SlhDsaStatus::InvalidParam,
                signature: Vec::new(),
            };
        }
    };

    SignResult {
        status: SlhDsaStatus::Ok,
        signature: sig.to_bytes().to_vec(),
    }
}

fn verifying_key_from_signing_key<P: ParameterSet>(sk_bytes: &[u8]) -> KeyResult {
    let sk = match slh_dsa_core::SigningKey::<P>::try_from(sk_bytes) {
        Ok(key) => key,
        Err(_) => {
            return KeyResult {
                status: SlhDsaStatus::DecodeError,
                verifying_key: Vec::new(),
            };
        }
    };

    let vk = sk.as_ref().clone();
    KeyResult {
        status: SlhDsaStatus::Ok,
        verifying_key: vk.to_bytes().to_vec(),
    }
}

fn verify<P: ParameterSet>(vk_bytes: &[u8], msg: &[u8], ctx: &[u8], sig_bytes: &[u8]) -> VerifyResult {
    let vk = match slh_dsa_core::VerifyingKey::<P>::try_from(vk_bytes) {
        Ok(key) => key,
        Err(_) => {
            return VerifyResult {
                status: SlhDsaStatus::DecodeError,
                valid: false,
            };
        }
    };
    let sig = match slh_dsa_core::Signature::<P>::try_from(sig_bytes) {
        Ok(sig) => sig,
        Err(_) => {
            return VerifyResult {
                status: SlhDsaStatus::DecodeError,
                valid: false,
            };
        }
    };

    match slh_dsa_core::verify_with_context::<P>(&vk, msg, ctx, &sig) {
        Ok(()) => VerifyResult {
            status: SlhDsaStatus::Ok,
            valid: true,
        },
        Err(_) => VerifyResult {
            status: SlhDsaStatus::VerifyFailed,
            valid: false,
        },
    }
}

fn dispatch_keypair(param: ParameterSetId) -> Keypair {
    match param {
        ParameterSetId::Shake128s => keypair_generate::<Shake128s>(),
        ParameterSetId::Shake128f => keypair_generate::<Shake128f>(),
        ParameterSetId::Shake192s => keypair_generate::<Shake192s>(),
        ParameterSetId::Shake192f => keypair_generate::<Shake192f>(),
        ParameterSetId::Shake256s => keypair_generate::<Shake256s>(),
        ParameterSetId::Shake256f => keypair_generate::<Shake256f>(),
        ParameterSetId::Sha2_128s => keypair_generate::<Sha2_128s>(),
        ParameterSetId::Sha2_128f => keypair_generate::<Sha2_128f>(),
        ParameterSetId::Sha2_192s => keypair_generate::<Sha2_192s>(),
        ParameterSetId::Sha2_192f => keypair_generate::<Sha2_192f>(),
        ParameterSetId::Sha2_256s => keypair_generate::<Sha2_256s>(),
        ParameterSetId::Sha2_256f => keypair_generate::<Sha2_256f>(),
    }
}

fn dispatch_sign(param: ParameterSetId, sk_bytes: &[u8], msg: &[u8], ctx: &[u8]) -> SignResult {
    match param {
        ParameterSetId::Shake128s => sign::<Shake128s>(sk_bytes, msg, ctx),
        ParameterSetId::Shake128f => sign::<Shake128f>(sk_bytes, msg, ctx),
        ParameterSetId::Shake192s => sign::<Shake192s>(sk_bytes, msg, ctx),
        ParameterSetId::Shake192f => sign::<Shake192f>(sk_bytes, msg, ctx),
        ParameterSetId::Shake256s => sign::<Shake256s>(sk_bytes, msg, ctx),
        ParameterSetId::Shake256f => sign::<Shake256f>(sk_bytes, msg, ctx),
        ParameterSetId::Sha2_128s => sign::<Sha2_128s>(sk_bytes, msg, ctx),
        ParameterSetId::Sha2_128f => sign::<Sha2_128f>(sk_bytes, msg, ctx),
        ParameterSetId::Sha2_192s => sign::<Sha2_192s>(sk_bytes, msg, ctx),
        ParameterSetId::Sha2_192f => sign::<Sha2_192f>(sk_bytes, msg, ctx),
        ParameterSetId::Sha2_256s => sign::<Sha2_256s>(sk_bytes, msg, ctx),
        ParameterSetId::Sha2_256f => sign::<Sha2_256f>(sk_bytes, msg, ctx),
    }
}

fn dispatch_sign_deterministic(
    param: ParameterSetId,
    sk_bytes: &[u8],
    msg: &[u8],
    ctx: &[u8],
) -> SignResult {
    match param {
        ParameterSetId::Shake128s => sign_deterministic::<Shake128s>(sk_bytes, msg, ctx),
        ParameterSetId::Shake128f => sign_deterministic::<Shake128f>(sk_bytes, msg, ctx),
        ParameterSetId::Shake192s => sign_deterministic::<Shake192s>(sk_bytes, msg, ctx),
        ParameterSetId::Shake192f => sign_deterministic::<Shake192f>(sk_bytes, msg, ctx),
        ParameterSetId::Shake256s => sign_deterministic::<Shake256s>(sk_bytes, msg, ctx),
        ParameterSetId::Shake256f => sign_deterministic::<Shake256f>(sk_bytes, msg, ctx),
        ParameterSetId::Sha2_128s => sign_deterministic::<Sha2_128s>(sk_bytes, msg, ctx),
        ParameterSetId::Sha2_128f => sign_deterministic::<Sha2_128f>(sk_bytes, msg, ctx),
        ParameterSetId::Sha2_192s => sign_deterministic::<Sha2_192s>(sk_bytes, msg, ctx),
        ParameterSetId::Sha2_192f => sign_deterministic::<Sha2_192f>(sk_bytes, msg, ctx),
        ParameterSetId::Sha2_256s => sign_deterministic::<Sha2_256s>(sk_bytes, msg, ctx),
        ParameterSetId::Sha2_256f => sign_deterministic::<Sha2_256f>(sk_bytes, msg, ctx),
    }
}

fn dispatch_verify(
    param: ParameterSetId,
    vk_bytes: &[u8],
    msg: &[u8],
    ctx: &[u8],
    sig_bytes: &[u8],
) -> VerifyResult {
    match param {
        ParameterSetId::Shake128s => verify::<Shake128s>(vk_bytes, msg, ctx, sig_bytes),
        ParameterSetId::Shake128f => verify::<Shake128f>(vk_bytes, msg, ctx, sig_bytes),
        ParameterSetId::Shake192s => verify::<Shake192s>(vk_bytes, msg, ctx, sig_bytes),
        ParameterSetId::Shake192f => verify::<Shake192f>(vk_bytes, msg, ctx, sig_bytes),
        ParameterSetId::Shake256s => verify::<Shake256s>(vk_bytes, msg, ctx, sig_bytes),
        ParameterSetId::Shake256f => verify::<Shake256f>(vk_bytes, msg, ctx, sig_bytes),
        ParameterSetId::Sha2_128s => verify::<Sha2_128s>(vk_bytes, msg, ctx, sig_bytes),
        ParameterSetId::Sha2_128f => verify::<Sha2_128f>(vk_bytes, msg, ctx, sig_bytes),
        ParameterSetId::Sha2_192s => verify::<Sha2_192s>(vk_bytes, msg, ctx, sig_bytes),
        ParameterSetId::Sha2_192f => verify::<Sha2_192f>(vk_bytes, msg, ctx, sig_bytes),
        ParameterSetId::Sha2_256s => verify::<Sha2_256s>(vk_bytes, msg, ctx, sig_bytes),
        ParameterSetId::Sha2_256f => verify::<Sha2_256f>(vk_bytes, msg, ctx, sig_bytes),
    }
}

fn dispatch_vk_from_sk(param: ParameterSetId, sk_bytes: &[u8]) -> KeyResult {
    match param {
        ParameterSetId::Shake128s => verifying_key_from_signing_key::<Shake128s>(sk_bytes),
        ParameterSetId::Shake128f => verifying_key_from_signing_key::<Shake128f>(sk_bytes),
        ParameterSetId::Shake192s => verifying_key_from_signing_key::<Shake192s>(sk_bytes),
        ParameterSetId::Shake192f => verifying_key_from_signing_key::<Shake192f>(sk_bytes),
        ParameterSetId::Shake256s => verifying_key_from_signing_key::<Shake256s>(sk_bytes),
        ParameterSetId::Shake256f => verifying_key_from_signing_key::<Shake256f>(sk_bytes),
        ParameterSetId::Sha2_128s => verifying_key_from_signing_key::<Sha2_128s>(sk_bytes),
        ParameterSetId::Sha2_128f => verifying_key_from_signing_key::<Sha2_128f>(sk_bytes),
        ParameterSetId::Sha2_192s => verifying_key_from_signing_key::<Sha2_192s>(sk_bytes),
        ParameterSetId::Sha2_192f => verifying_key_from_signing_key::<Sha2_192f>(sk_bytes),
        ParameterSetId::Sha2_256s => verifying_key_from_signing_key::<Sha2_256s>(sk_bytes),
        ParameterSetId::Sha2_256f => verifying_key_from_signing_key::<Sha2_256f>(sk_bytes),
    }
}

#[uniffi::export]
pub fn slh_dsa_parameter_name(param: ParameterSetId) -> String {
    parameter_name(param).to_string()
}

#[uniffi::export]
pub fn slh_dsa_signing_key_len(param: ParameterSetId) -> u64 {
    signing_key_len(param) as u64
}

#[uniffi::export]
pub fn slh_dsa_verifying_key_len(param: ParameterSetId) -> u64 {
    verifying_key_len(param) as u64
}

#[uniffi::export]
pub fn slh_dsa_signature_len(param: ParameterSetId) -> u64 {
    signature_len(param) as u64
}

#[uniffi::export]
pub fn slh_dsa_keypair_generate(param: ParameterSetId) -> Keypair {
    dispatch_keypair(param)
}

#[uniffi::export]
pub fn slh_dsa_sign(
    param: ParameterSetId,
    signing_key: Vec<u8>,
    msg: Vec<u8>,
    ctx: Vec<u8>,
) -> SignResult {
    if signing_key.len() != signing_key_len(param) {
        return SignResult {
            status: SlhDsaStatus::InvalidLength,
            signature: Vec::new(),
        };
    }

    dispatch_sign(param, &signing_key, &msg, &ctx)
}

#[uniffi::export]
pub fn slh_dsa_sign_deterministic(
    param: ParameterSetId,
    signing_key: Vec<u8>,
    msg: Vec<u8>,
    ctx: Vec<u8>,
) -> SignResult {
    if signing_key.len() != signing_key_len(param) {
        return SignResult {
            status: SlhDsaStatus::InvalidLength,
            signature: Vec::new(),
        };
    }

    dispatch_sign_deterministic(param, &signing_key, &msg, &ctx)
}

#[uniffi::export]
pub fn slh_dsa_verifying_key_from_signing_key(
    param: ParameterSetId,
    signing_key: Vec<u8>,
) -> KeyResult {
    if signing_key.len() != signing_key_len(param) {
        return KeyResult {
            status: SlhDsaStatus::InvalidLength,
            verifying_key: Vec::new(),
        };
    }

    dispatch_vk_from_sk(param, &signing_key)
}

#[uniffi::export]
pub fn slh_dsa_verify(
    param: ParameterSetId,
    verifying_key: Vec<u8>,
    msg: Vec<u8>,
    ctx: Vec<u8>,
    signature: Vec<u8>,
) -> VerifyResult {
    if verifying_key.len() != verifying_key_len(param) || signature.len() != signature_len(param) {
        return VerifyResult {
            status: SlhDsaStatus::InvalidLength,
            valid: false,
        };
    }

    dispatch_verify(param, &verifying_key, &msg, &ctx, &signature)
}

uniffi::setup_scaffolding!();
