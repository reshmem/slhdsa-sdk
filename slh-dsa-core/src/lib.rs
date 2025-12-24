#![no_std]
//! Core SLH-DSA wrappers and helpers built on the `slh-dsa` crate.

pub use slh_dsa::{
    Sha2_128f, Sha2_128s, Sha2_192f, Sha2_192s, Sha2_256f, Sha2_256s, Shake128f, Shake128s,
    Shake192f, Shake192s, Shake256f, Shake256s,
};
pub use slh_dsa::signature;
pub use slh_dsa::{ParameterSet, Signature, SignatureLen, SigningKey, SigningKeyLen, VerifyingKey, VerifyingKeyLen};

use hybrid_array::Array;
use rand_core::{CryptoRng, RngCore};
use hybrid_array::typenum::Unsigned;

/// Supported SLH-DSA parameter sets.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParameterSetId {
    Shake128s = 1,
    Shake128f = 2,
    Shake192s = 3,
    Shake192f = 4,
    Shake256s = 5,
    Shake256f = 6,
    Sha2_128s = 7,
    Sha2_128f = 8,
    Sha2_192s = 9,
    Sha2_192f = 10,
    Sha2_256s = 11,
    Sha2_256f = 12,
}

impl ParameterSetId {
    /// Convert a raw value into a parameter set id.
    pub const fn from_u32(value: u32) -> Option<Self> {
        match value {
            1 => Some(Self::Shake128s),
            2 => Some(Self::Shake128f),
            3 => Some(Self::Shake192s),
            4 => Some(Self::Shake192f),
            5 => Some(Self::Shake256s),
            6 => Some(Self::Shake256f),
            7 => Some(Self::Sha2_128s),
            8 => Some(Self::Sha2_128f),
            9 => Some(Self::Sha2_192s),
            10 => Some(Self::Sha2_192f),
            11 => Some(Self::Sha2_256s),
            12 => Some(Self::Sha2_256f),
            _ => None,
        }
    }

    /// Human-readable FIPS-205 parameter set name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Shake128s => "SLH-DSA-SHAKE-128s",
            Self::Shake128f => "SLH-DSA-SHAKE-128f",
            Self::Shake192s => "SLH-DSA-SHAKE-192s",
            Self::Shake192f => "SLH-DSA-SHAKE-192f",
            Self::Shake256s => "SLH-DSA-SHAKE-256s",
            Self::Shake256f => "SLH-DSA-SHAKE-256f",
            Self::Sha2_128s => "SLH-DSA-SHA2-128s",
            Self::Sha2_128f => "SLH-DSA-SHA2-128f",
            Self::Sha2_192s => "SLH-DSA-SHA2-192s",
            Self::Sha2_192f => "SLH-DSA-SHA2-192f",
            Self::Sha2_256s => "SLH-DSA-SHA2-256s",
            Self::Sha2_256f => "SLH-DSA-SHA2-256f",
        }
    }
}

/// Generate a signing and verifying key pair using the provided RNG.
pub fn generate_keypair<P: ParameterSet, R: CryptoRng + RngCore>(
    rng: &mut R,
) -> (SigningKey<P>, VerifyingKey<P>) {
    let signing_key = SigningKey::<P>::new(rng);
    let verifying_key = signing_key.as_ref().clone();
    (signing_key, verifying_key)
}

/// Sign a message with a context string using a caller-supplied RNG.
pub fn sign_with_rng<P: ParameterSet, R: CryptoRng + RngCore>(
    signing_key: &SigningKey<P>,
    rng: &mut R,
    msg: &[u8],
    ctx: &[u8],
) -> Result<Signature<P>, signature::Error> {
    let mut buffer = Array::<u8, P::SkLen>::default();
    rng.fill_bytes(buffer.as_mut_slice());
    let n = sk_len::<P>() / 4;
    signing_key.try_sign_with_context(msg, ctx, Some(&buffer.as_slice()[..n]))
}

/// Deterministically sign a message with a context string.
pub fn sign_deterministic<P: ParameterSet>(
    signing_key: &SigningKey<P>,
    msg: &[u8],
    ctx: &[u8],
) -> Result<Signature<P>, signature::Error> {
    signing_key.try_sign_with_context(msg, ctx, None)
}

/// Verify a signature with a context string.
pub fn verify_with_context<P: ParameterSet>(
    verifying_key: &VerifyingKey<P>,
    msg: &[u8],
    ctx: &[u8],
    signature: &Signature<P>,
) -> Result<(), signature::Error> {
    verifying_key.try_verify_with_context(msg, ctx, signature)
}

fn sk_len<P: ParameterSet>() -> usize {
    <P::SkLen as Unsigned>::USIZE
}

fn vk_len<P: ParameterSet>() -> usize {
    <P::VkLen as Unsigned>::USIZE
}

fn sig_len<P: ParameterSet>() -> usize {
    <P::SigLen as Unsigned>::USIZE
}

/// Byte length of a signing key for the given parameter set.
pub fn sk_len_for(param: ParameterSetId) -> Option<usize> {
    Some(match param {
        ParameterSetId::Shake128s => sk_len::<Shake128s>(),
        ParameterSetId::Shake128f => sk_len::<Shake128f>(),
        ParameterSetId::Shake192s => sk_len::<Shake192s>(),
        ParameterSetId::Shake192f => sk_len::<Shake192f>(),
        ParameterSetId::Shake256s => sk_len::<Shake256s>(),
        ParameterSetId::Shake256f => sk_len::<Shake256f>(),
        ParameterSetId::Sha2_128s => sk_len::<Sha2_128s>(),
        ParameterSetId::Sha2_128f => sk_len::<Sha2_128f>(),
        ParameterSetId::Sha2_192s => sk_len::<Sha2_192s>(),
        ParameterSetId::Sha2_192f => sk_len::<Sha2_192f>(),
        ParameterSetId::Sha2_256s => sk_len::<Sha2_256s>(),
        ParameterSetId::Sha2_256f => sk_len::<Sha2_256f>(),
    })
}

/// Byte length of a verifying key for the given parameter set.
pub fn vk_len_for(param: ParameterSetId) -> Option<usize> {
    Some(match param {
        ParameterSetId::Shake128s => vk_len::<Shake128s>(),
        ParameterSetId::Shake128f => vk_len::<Shake128f>(),
        ParameterSetId::Shake192s => vk_len::<Shake192s>(),
        ParameterSetId::Shake192f => vk_len::<Shake192f>(),
        ParameterSetId::Shake256s => vk_len::<Shake256s>(),
        ParameterSetId::Shake256f => vk_len::<Shake256f>(),
        ParameterSetId::Sha2_128s => vk_len::<Sha2_128s>(),
        ParameterSetId::Sha2_128f => vk_len::<Sha2_128f>(),
        ParameterSetId::Sha2_192s => vk_len::<Sha2_192s>(),
        ParameterSetId::Sha2_192f => vk_len::<Sha2_192f>(),
        ParameterSetId::Sha2_256s => vk_len::<Sha2_256s>(),
        ParameterSetId::Sha2_256f => vk_len::<Sha2_256f>(),
    })
}

/// Byte length of a signature for the given parameter set.
pub fn sig_len_for(param: ParameterSetId) -> Option<usize> {
    Some(match param {
        ParameterSetId::Shake128s => sig_len::<Shake128s>(),
        ParameterSetId::Shake128f => sig_len::<Shake128f>(),
        ParameterSetId::Shake192s => sig_len::<Shake192s>(),
        ParameterSetId::Shake192f => sig_len::<Shake192f>(),
        ParameterSetId::Shake256s => sig_len::<Shake256s>(),
        ParameterSetId::Shake256f => sig_len::<Shake256f>(),
        ParameterSetId::Sha2_128s => sig_len::<Sha2_128s>(),
        ParameterSetId::Sha2_128f => sig_len::<Sha2_128f>(),
        ParameterSetId::Sha2_192s => sig_len::<Sha2_192s>(),
        ParameterSetId::Sha2_192f => sig_len::<Sha2_192f>(),
        ParameterSetId::Sha2_256s => sig_len::<Sha2_256s>(),
        ParameterSetId::Sha2_256f => sig_len::<Sha2_256f>(),
    })
}
