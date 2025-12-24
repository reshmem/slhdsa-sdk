import os
import sys
import pathlib

root = pathlib.Path(__file__).resolve().parents[2]
bindings_dir = root / "bindings" / "python"

# Ensure the UniFFI library is next to the generated Python module.
lib_ext = ".dylib"
if sys.platform.startswith("linux"):
    lib_ext = ".so"
elif sys.platform.startswith("win"):
    lib_ext = ".dll"

lib_name = f"libslh_dsa_uniffi{lib_ext}"
target_dir = pathlib.Path(os.environ.get("CARGO_TARGET_DIR", root / "target")) / "release"
lib_path = target_dir / lib_name

if not lib_path.exists():
    raise RuntimeError(f"Missing {lib_path}. Run ./scripts/build.sh first.")

bindings_dir.mkdir(parents=True, exist_ok=True)

local_lib = bindings_dir / lib_name
if not local_lib.exists():
    local_lib.write_bytes(lib_path.read_bytes())

sys.path.insert(0, str(bindings_dir))

import slh_dsa_uniffi as slh

param = slh.ParameterSetId.SHAKE128F

keypair = slh.slh_dsa_keypair_generate(param)
if len(keypair.signing_key) == 0 or len(keypair.verifying_key) == 0:
    raise RuntimeError("Generated empty keypair")

message = b"hello from python"
ctx = b"python-test"

sign_result = slh.slh_dsa_sign(param, keypair.signing_key, message, ctx)
if sign_result.status != slh.SlhDsaStatus.OK:
    raise RuntimeError(f"Sign failed: {sign_result.status}")

verify_result = slh.slh_dsa_verify(
    param,
    keypair.verifying_key,
    message,
    ctx,
    sign_result.signature,
)

if verify_result.status != slh.SlhDsaStatus.OK or not verify_result.valid:
    raise RuntimeError("Verify failed")

print("python test ok")
