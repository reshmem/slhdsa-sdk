import os
import pathlib
import sys
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]
BINDINGS_DIR = ROOT / "bindings" / "python"

lib_ext = ".dylib"
if sys.platform.startswith("linux"):
    lib_ext = ".so"
elif sys.platform.startswith("win"):
    lib_ext = ".dll"

lib_name = f"libslh_dsa_uniffi{lib_ext}"

TARGET_DIR = pathlib.Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target")) / "release"
LIB_PATH = TARGET_DIR / lib_name

if not LIB_PATH.exists():
    raise RuntimeError(f"Missing {LIB_PATH}. Run ./scripts/build.sh first.")

BINDINGS_DIR.mkdir(parents=True, exist_ok=True)
local_lib = BINDINGS_DIR / lib_name
if not local_lib.exists():
    local_lib.write_bytes(LIB_PATH.read_bytes())

sys.path.insert(0, str(BINDINGS_DIR))
import slh_dsa_uniffi as slh

sizes = [32, 64, 256, 1024, 4096, 8128, 32768]
param = slh.ParameterSetId.SHAKE256F

iters = int(os.environ.get("BENCH_ITERS", "5"))
warmup = int(os.environ.get("BENCH_WARMUP", "1"))

print(f"param_set,{slh.slh_dsa_parameter_name(param)}")
print("size_bytes,sign_ms,verify_ms,total_ms")

for size in sizes:
    msg = bytes((i * 31) & 0xFF for i in range(size))
    ctx = b""

    keypair = slh.slh_dsa_keypair_generate(param)

    for _ in range(warmup):
        sig = slh.slh_dsa_sign_deterministic(param, keypair.signing_key, msg, ctx)
        slh.slh_dsa_verify(param, keypair.verifying_key, msg, ctx, sig.signature)

    sign_total = 0.0
    verify_total = 0.0

    for _ in range(iters):
        start = time.perf_counter()
        sig = slh.slh_dsa_sign_deterministic(param, keypair.signing_key, msg, ctx)
        sign_total += time.perf_counter() - start

        start = time.perf_counter()
        verify = slh.slh_dsa_verify(param, keypair.verifying_key, msg, ctx, sig.signature)
        verify_total += time.perf_counter() - start
        if not verify.valid:
            raise RuntimeError("Verify failed")

    sign_avg = (sign_total / iters) * 1000.0
    verify_avg = (verify_total / iters) * 1000.0
    total = sign_avg + verify_avg
    print(f"{size},{sign_avg:.6f},{verify_avg:.6f},{total:.6f}")
