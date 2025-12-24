//! C-compatible FFI wrapper for slh-dsa-core.

use getrandom::fill;
use rand_core::{CryptoRng, RngCore};
use slh_dsa_core::{
    ParameterSet, ParameterSetId, Signature, SigningKey, VerifyingKey, Sha2_128f, Sha2_128s,
    Sha2_192f, Sha2_192s, Sha2_256f, Sha2_256s, Shake128f, Shake128s, Shake192f, Shake192s,
    Shake256f, Shake256s,
};
use std::ffi::c_char;
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SlhDsaStatus {
    Ok = 0,
    NullPtr = 1,
    InvalidParam = 2,
    InvalidLength = 3,
    DecodeError = 4,
    VerifyFailed = 5,
}

pub use slh_dsa_core::ParameterSetId as SlhDsaParameterSetId;

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

fn read_slice<'a>(ptr: *const u8, len: usize) -> Result<&'a [u8], SlhDsaStatus> {
    if ptr.is_null() {
        if len == 0 {
            return Ok(&[]);
        }
        return Err(SlhDsaStatus::NullPtr);
    }
    // SAFETY: caller guarantees the pointer is valid for len bytes.
    Ok(unsafe { std::slice::from_raw_parts(ptr, len) })
}

fn write_slice<'a>(ptr: *mut u8, len: usize) -> Result<&'a mut [u8], SlhDsaStatus> {
    if ptr.is_null() {
        if len == 0 {
            return Ok(&mut []);
        }
        return Err(SlhDsaStatus::NullPtr);
    }
    // SAFETY: caller guarantees the pointer is valid for len bytes.
    Ok(unsafe { std::slice::from_raw_parts_mut(ptr, len) })
}

fn parameter_name(param: ParameterSetId) -> *const c_char {
    match param {
        ParameterSetId::Shake128s => b"SLH-DSA-SHAKE-128s\0".as_ptr() as *const c_char,
        ParameterSetId::Shake128f => b"SLH-DSA-SHAKE-128f\0".as_ptr() as *const c_char,
        ParameterSetId::Shake192s => b"SLH-DSA-SHAKE-192s\0".as_ptr() as *const c_char,
        ParameterSetId::Shake192f => b"SLH-DSA-SHAKE-192f\0".as_ptr() as *const c_char,
        ParameterSetId::Shake256s => b"SLH-DSA-SHAKE-256s\0".as_ptr() as *const c_char,
        ParameterSetId::Shake256f => b"SLH-DSA-SHAKE-256f\0".as_ptr() as *const c_char,
        ParameterSetId::Sha2_128s => b"SLH-DSA-SHA2-128s\0".as_ptr() as *const c_char,
        ParameterSetId::Sha2_128f => b"SLH-DSA-SHA2-128f\0".as_ptr() as *const c_char,
        ParameterSetId::Sha2_192s => b"SLH-DSA-SHA2-192s\0".as_ptr() as *const c_char,
        ParameterSetId::Sha2_192f => b"SLH-DSA-SHA2-192f\0".as_ptr() as *const c_char,
        ParameterSetId::Sha2_256s => b"SLH-DSA-SHA2-256s\0".as_ptr() as *const c_char,
        ParameterSetId::Sha2_256f => b"SLH-DSA-SHA2-256f\0".as_ptr() as *const c_char,
    }
}

fn sk_len(param: ParameterSetId) -> usize {
    slh_dsa_core::sk_len_for(param).unwrap_or(0)
}

fn vk_len(param: ParameterSetId) -> usize {
    slh_dsa_core::vk_len_for(param).unwrap_or(0)
}

fn sig_len(param: ParameterSetId) -> usize {
    slh_dsa_core::sig_len_for(param).unwrap_or(0)
}

fn keypair_generate<P: ParameterSet>(sk_out: &mut [u8], vk_out: &mut [u8]) -> Result<(), SlhDsaStatus> {
    let mut rng = OsRng;
    let (sk, vk) = slh_dsa_core::generate_keypair::<P, _>(&mut rng);
    let sk_bytes = sk.to_bytes();
    let vk_bytes = vk.to_bytes();
    sk_out.copy_from_slice(sk_bytes.as_slice());
    vk_out.copy_from_slice(vk_bytes.as_slice());
    Ok(())
}

fn sign<P: ParameterSet>(
    sk_bytes: &[u8],
    msg: &[u8],
    ctx: &[u8],
    sig_out: &mut [u8],
) -> Result<(), SlhDsaStatus> {
    let sk = SigningKey::<P>::try_from(sk_bytes).map_err(|_| SlhDsaStatus::DecodeError)?;
    let mut rng = OsRng;
    let sig = slh_dsa_core::sign_with_rng::<P, _>(&sk, &mut rng, msg, ctx)
        .map_err(|_| SlhDsaStatus::InvalidParam)?;
    let sig_bytes = sig.to_bytes();
    sig_out.copy_from_slice(sig_bytes.as_slice());
    Ok(())
}

fn sign_deterministic<P: ParameterSet>(
    sk_bytes: &[u8],
    msg: &[u8],
    ctx: &[u8],
    sig_out: &mut [u8],
) -> Result<(), SlhDsaStatus> {
    let sk = SigningKey::<P>::try_from(sk_bytes).map_err(|_| SlhDsaStatus::DecodeError)?;
    let sig = slh_dsa_core::sign_deterministic::<P>(&sk, msg, ctx)
        .map_err(|_| SlhDsaStatus::InvalidParam)?;
    let sig_bytes = sig.to_bytes();
    sig_out.copy_from_slice(sig_bytes.as_slice());
    Ok(())
}

fn verifying_key_from_signing_key<P: ParameterSet>(
    sk_bytes: &[u8],
    vk_out: &mut [u8],
) -> Result<(), SlhDsaStatus> {
    let sk = SigningKey::<P>::try_from(sk_bytes).map_err(|_| SlhDsaStatus::DecodeError)?;
    let vk = sk.as_ref().clone();
    let vk_bytes = vk.to_bytes();
    vk_out.copy_from_slice(vk_bytes.as_slice());
    Ok(())
}

fn verify<P: ParameterSet>(
    vk_bytes: &[u8],
    msg: &[u8],
    ctx: &[u8],
    sig_bytes: &[u8],
) -> Result<(), SlhDsaStatus> {
    let vk = VerifyingKey::<P>::try_from(vk_bytes).map_err(|_| SlhDsaStatus::DecodeError)?;
    let sig = Signature::<P>::try_from(sig_bytes).map_err(|_| SlhDsaStatus::DecodeError)?;
    slh_dsa_core::verify_with_context::<P>(&vk, msg, ctx, &sig)
        .map_err(|_| SlhDsaStatus::VerifyFailed)
}

fn dispatch_keypair(
    param: ParameterSetId,
    sk_out: &mut [u8],
    vk_out: &mut [u8],
) -> Result<(), SlhDsaStatus> {
    match param {
        ParameterSetId::Shake128s => keypair_generate::<Shake128s>(sk_out, vk_out),
        ParameterSetId::Shake128f => keypair_generate::<Shake128f>(sk_out, vk_out),
        ParameterSetId::Shake192s => keypair_generate::<Shake192s>(sk_out, vk_out),
        ParameterSetId::Shake192f => keypair_generate::<Shake192f>(sk_out, vk_out),
        ParameterSetId::Shake256s => keypair_generate::<Shake256s>(sk_out, vk_out),
        ParameterSetId::Shake256f => keypair_generate::<Shake256f>(sk_out, vk_out),
        ParameterSetId::Sha2_128s => keypair_generate::<Sha2_128s>(sk_out, vk_out),
        ParameterSetId::Sha2_128f => keypair_generate::<Sha2_128f>(sk_out, vk_out),
        ParameterSetId::Sha2_192s => keypair_generate::<Sha2_192s>(sk_out, vk_out),
        ParameterSetId::Sha2_192f => keypair_generate::<Sha2_192f>(sk_out, vk_out),
        ParameterSetId::Sha2_256s => keypair_generate::<Sha2_256s>(sk_out, vk_out),
        ParameterSetId::Sha2_256f => keypair_generate::<Sha2_256f>(sk_out, vk_out),
    }
}

fn dispatch_sign(
    param: ParameterSetId,
    sk_bytes: &[u8],
    msg: &[u8],
    ctx: &[u8],
    sig_out: &mut [u8],
) -> Result<(), SlhDsaStatus> {
    match param {
        ParameterSetId::Shake128s => sign::<Shake128s>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Shake128f => sign::<Shake128f>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Shake192s => sign::<Shake192s>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Shake192f => sign::<Shake192f>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Shake256s => sign::<Shake256s>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Shake256f => sign::<Shake256f>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Sha2_128s => sign::<Sha2_128s>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Sha2_128f => sign::<Sha2_128f>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Sha2_192s => sign::<Sha2_192s>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Sha2_192f => sign::<Sha2_192f>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Sha2_256s => sign::<Sha2_256s>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Sha2_256f => sign::<Sha2_256f>(sk_bytes, msg, ctx, sig_out),
    }
}

fn dispatch_sign_deterministic(
    param: ParameterSetId,
    sk_bytes: &[u8],
    msg: &[u8],
    ctx: &[u8],
    sig_out: &mut [u8],
) -> Result<(), SlhDsaStatus> {
    match param {
        ParameterSetId::Shake128s => sign_deterministic::<Shake128s>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Shake128f => sign_deterministic::<Shake128f>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Shake192s => sign_deterministic::<Shake192s>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Shake192f => sign_deterministic::<Shake192f>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Shake256s => sign_deterministic::<Shake256s>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Shake256f => sign_deterministic::<Shake256f>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Sha2_128s => sign_deterministic::<Sha2_128s>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Sha2_128f => sign_deterministic::<Sha2_128f>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Sha2_192s => sign_deterministic::<Sha2_192s>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Sha2_192f => sign_deterministic::<Sha2_192f>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Sha2_256s => sign_deterministic::<Sha2_256s>(sk_bytes, msg, ctx, sig_out),
        ParameterSetId::Sha2_256f => sign_deterministic::<Sha2_256f>(sk_bytes, msg, ctx, sig_out),
    }
}

fn dispatch_vk_from_sk(
    param: ParameterSetId,
    sk_bytes: &[u8],
    vk_out: &mut [u8],
) -> Result<(), SlhDsaStatus> {
    match param {
        ParameterSetId::Shake128s => verifying_key_from_signing_key::<Shake128s>(sk_bytes, vk_out),
        ParameterSetId::Shake128f => verifying_key_from_signing_key::<Shake128f>(sk_bytes, vk_out),
        ParameterSetId::Shake192s => verifying_key_from_signing_key::<Shake192s>(sk_bytes, vk_out),
        ParameterSetId::Shake192f => verifying_key_from_signing_key::<Shake192f>(sk_bytes, vk_out),
        ParameterSetId::Shake256s => verifying_key_from_signing_key::<Shake256s>(sk_bytes, vk_out),
        ParameterSetId::Shake256f => verifying_key_from_signing_key::<Shake256f>(sk_bytes, vk_out),
        ParameterSetId::Sha2_128s => verifying_key_from_signing_key::<Sha2_128s>(sk_bytes, vk_out),
        ParameterSetId::Sha2_128f => verifying_key_from_signing_key::<Sha2_128f>(sk_bytes, vk_out),
        ParameterSetId::Sha2_192s => verifying_key_from_signing_key::<Sha2_192s>(sk_bytes, vk_out),
        ParameterSetId::Sha2_192f => verifying_key_from_signing_key::<Sha2_192f>(sk_bytes, vk_out),
        ParameterSetId::Sha2_256s => verifying_key_from_signing_key::<Sha2_256s>(sk_bytes, vk_out),
        ParameterSetId::Sha2_256f => verifying_key_from_signing_key::<Sha2_256f>(sk_bytes, vk_out),
    }
}

fn dispatch_verify(
    param: ParameterSetId,
    vk_bytes: &[u8],
    msg: &[u8],
    ctx: &[u8],
    sig_bytes: &[u8],
) -> Result<(), SlhDsaStatus> {
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

#[no_mangle]
pub extern "C" fn slh_dsa_parameter_name(param: ParameterSetId) -> *const c_char {
    parameter_name(param)
}

#[no_mangle]
pub extern "C" fn slh_dsa_signing_key_len(param: ParameterSetId) -> usize {
    sk_len(param)
}

#[no_mangle]
pub extern "C" fn slh_dsa_verifying_key_len(param: ParameterSetId) -> usize {
    vk_len(param)
}

#[no_mangle]
pub extern "C" fn slh_dsa_signature_len(param: ParameterSetId) -> usize {
    sig_len(param)
}

#[no_mangle]
pub extern "C" fn slh_dsa_keypair_generate(
    param: ParameterSetId,
    signing_key_out: *mut u8,
    signing_key_len: usize,
    verifying_key_out: *mut u8,
    verifying_key_len: usize,
) -> SlhDsaStatus {
    let expected_sk = sk_len(param);
    let expected_vk = vk_len(param);
    if signing_key_len != expected_sk || verifying_key_len != expected_vk {
        return SlhDsaStatus::InvalidLength;
    }

    let sk_out = match write_slice(signing_key_out, signing_key_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };
    let vk_out = match write_slice(verifying_key_out, verifying_key_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };

    match dispatch_keypair(param, sk_out, vk_out) {
        Ok(()) => SlhDsaStatus::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub extern "C" fn slh_dsa_sign(
    param: ParameterSetId,
    signing_key: *const u8,
    signing_key_len: usize,
    msg: *const u8,
    msg_len: usize,
    ctx: *const u8,
    ctx_len: usize,
    signature_out: *mut u8,
    signature_len: usize,
) -> SlhDsaStatus {
    let expected_sk = sk_len(param);
    let expected_sig = sig_len(param);
    if signing_key_len != expected_sk || signature_len != expected_sig {
        return SlhDsaStatus::InvalidLength;
    }

    let sk_bytes = match read_slice(signing_key, signing_key_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };
    let msg_bytes = match read_slice(msg, msg_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };
    let ctx_bytes = match read_slice(ctx, ctx_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };
    let sig_out = match write_slice(signature_out, signature_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };

    match dispatch_sign(param, sk_bytes, msg_bytes, ctx_bytes, sig_out) {
        Ok(()) => SlhDsaStatus::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub extern "C" fn slh_dsa_sign_deterministic(
    param: ParameterSetId,
    signing_key: *const u8,
    signing_key_len: usize,
    msg: *const u8,
    msg_len: usize,
    ctx: *const u8,
    ctx_len: usize,
    signature_out: *mut u8,
    signature_len: usize,
) -> SlhDsaStatus {
    let expected_sk = sk_len(param);
    let expected_sig = sig_len(param);
    if signing_key_len != expected_sk || signature_len != expected_sig {
        return SlhDsaStatus::InvalidLength;
    }

    let sk_bytes = match read_slice(signing_key, signing_key_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };
    let msg_bytes = match read_slice(msg, msg_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };
    let ctx_bytes = match read_slice(ctx, ctx_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };
    let sig_out = match write_slice(signature_out, signature_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };

    match dispatch_sign_deterministic(param, sk_bytes, msg_bytes, ctx_bytes, sig_out) {
        Ok(()) => SlhDsaStatus::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub extern "C" fn slh_dsa_verifying_key_from_signing_key(
    param: ParameterSetId,
    signing_key: *const u8,
    signing_key_len: usize,
    verifying_key_out: *mut u8,
    verifying_key_len: usize,
) -> SlhDsaStatus {
    let expected_sk = sk_len(param);
    let expected_vk = vk_len(param);
    if signing_key_len != expected_sk || verifying_key_len != expected_vk {
        return SlhDsaStatus::InvalidLength;
    }

    let sk_bytes = match read_slice(signing_key, signing_key_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };
    let vk_out = match write_slice(verifying_key_out, verifying_key_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };

    match dispatch_vk_from_sk(param, sk_bytes, vk_out) {
        Ok(()) => SlhDsaStatus::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub extern "C" fn slh_dsa_verify(
    param: ParameterSetId,
    verifying_key: *const u8,
    verifying_key_len: usize,
    msg: *const u8,
    msg_len: usize,
    ctx: *const u8,
    ctx_len: usize,
    signature: *const u8,
    signature_len: usize,
) -> SlhDsaStatus {
    let expected_vk = vk_len(param);
    let expected_sig = sig_len(param);
    if verifying_key_len != expected_vk || signature_len != expected_sig {
        return SlhDsaStatus::InvalidLength;
    }

    let vk_bytes = match read_slice(verifying_key, verifying_key_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };
    let msg_bytes = match read_slice(msg, msg_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };
    let ctx_bytes = match read_slice(ctx, ctx_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };
    let sig_bytes = match read_slice(signature, signature_len) {
        Ok(slice) => slice,
        Err(status) => return status,
    };

    match dispatch_verify(param, vk_bytes, msg_bytes, ctx_bytes, sig_bytes) {
        Ok(()) => SlhDsaStatus::Ok,
        Err(status) => status,
    }
}

#[no_mangle]
pub extern "C" fn slh_dsa_status_string(status: SlhDsaStatus) -> *const c_char {
    match status {
        SlhDsaStatus::Ok => b"OK\0".as_ptr() as *const c_char,
        SlhDsaStatus::NullPtr => b"Null pointer\0".as_ptr() as *const c_char,
        SlhDsaStatus::InvalidParam => b"Invalid parameter\0".as_ptr() as *const c_char,
        SlhDsaStatus::InvalidLength => b"Invalid length\0".as_ptr() as *const c_char,
        SlhDsaStatus::DecodeError => b"Decode error\0".as_ptr() as *const c_char,
        SlhDsaStatus::VerifyFailed => b"Verification failed\0".as_ptr() as *const c_char,
    }
}

#[no_mangle]
pub extern "C" fn slh_dsa_parameter_set_is_valid(raw: u32) -> bool {
    ParameterSetId::from_u32(raw).is_some()
}

#[no_mangle]
pub extern "C" fn slh_dsa_parameter_set_from_u32(raw: u32) -> ParameterSetId {
    ParameterSetId::from_u32(raw).unwrap_or(ParameterSetId::Shake256f)
}

#[no_mangle]
pub extern "C" fn slh_dsa_parameter_set_to_u32(param: ParameterSetId) -> u32 {
    param as u32
}

#[no_mangle]
pub extern "C" fn slh_dsa_signature_verify_result_to_bool(status: SlhDsaStatus) -> bool {
    status == SlhDsaStatus::Ok
}

#[no_mangle]
pub extern "C" fn slh_dsa_zeroize(ptr: *mut u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }
    // SAFETY: caller guarantees the pointer is valid for len bytes.
    unsafe {
        ptr::write_bytes(ptr, 0, len);
    }
}
