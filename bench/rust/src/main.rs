use slh_dsa_core::{
    sign_deterministic, verify_with_context, ParameterSet, ParameterSetId, Shake256f,
};
use std::time::{Duration, Instant};

fn bench_for_size<P: ParameterSet>(msg_size: usize, iters: usize, warmup: usize) {
    let mut msg = vec![0u8; msg_size];
    for (i, byte) in msg.iter_mut().enumerate() {
        *byte = (i as u8).wrapping_mul(31);
    }
    let ctx: [u8; 0] = [];

    let (sk, vk) = slh_dsa_core::generate_keypair::<P, _>(&mut bench_rng());

    for _ in 0..warmup {
        let sig = sign_deterministic(&sk, &msg, &ctx).expect("sign failed");
        verify_with_context(&vk, &msg, &ctx, &sig).expect("verify failed");
    }

    let mut sign_total = Duration::from_secs(0);
    let mut verify_total = Duration::from_secs(0);

    for _ in 0..iters {
        let start = Instant::now();
        let sig = sign_deterministic(&sk, &msg, &ctx).expect("sign failed");
        sign_total += start.elapsed();

        let start = Instant::now();
        verify_with_context(&vk, &msg, &ctx, &sig).expect("verify failed");
        verify_total += start.elapsed();
    }

    let sign_avg = sign_total.as_secs_f64() / iters as f64;
    let verify_avg = verify_total.as_secs_f64() / iters as f64;

    println!(
        "{msg_size}, {:.6}, {:.6}, {:.6}",
        sign_avg * 1000.0,
        verify_avg * 1000.0,
        (sign_avg + verify_avg) * 1000.0
    );
}

fn bench_rng() -> impl rand_core::CryptoRng + rand_core::RngCore {
    struct OsRng;
    impl rand_core::RngCore for OsRng {
        fn next_u32(&mut self) -> u32 {
            let mut bytes = [0u8; 4];
            getrandom::fill(&mut bytes).expect("rng");
            u32::from_le_bytes(bytes)
        }
        fn next_u64(&mut self) -> u64 {
            let mut bytes = [0u8; 8];
            getrandom::fill(&mut bytes).expect("rng");
            u64::from_le_bytes(bytes)
        }
        fn fill_bytes(&mut self, dst: &mut [u8]) {
            getrandom::fill(dst).expect("rng");
        }
    }
    impl rand_core::CryptoRng for OsRng {}
    OsRng
}

fn main() {
    let sizes = [32usize, 64, 256, 1024, 4096, 8128, 32768];
    let iters: usize = std::env::var("BENCH_ITERS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5);
    let warmup: usize = std::env::var("BENCH_WARMUP")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);

    println!("param_set,{}", ParameterSetId::Shake256f.name());
    println!("size_bytes,sign_ms,verify_ms,total_ms");
    for size in sizes {
        bench_for_size::<Shake256f>(size, iters, warmup);
    }
}
