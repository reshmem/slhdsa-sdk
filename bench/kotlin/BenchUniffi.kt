package bench

import uniffi.slh_dsa_uniffi.ParameterSetId
import uniffi.slh_dsa_uniffi.SlhDsaStatus
import uniffi.slh_dsa_uniffi.slhDsaKeypairGenerate
import uniffi.slh_dsa_uniffi.slhDsaSignDeterministic
import uniffi.slh_dsa_uniffi.slhDsaVerify

fun main() {
    val sizes = listOf(32, 64, 256, 1024, 4096, 8128, 32768)
    val iters = System.getenv("BENCH_ITERS")?.toIntOrNull() ?: 5
    val warmup = System.getenv("BENCH_WARMUP")?.toIntOrNull() ?: 1

    val param = ParameterSetId.SHAKE256F

    println("param_set,SLH-DSA-SHAKE-256f")
    println("size_bytes,sign_ms,verify_ms,total_ms")

    for (size in sizes) {
        val msg = ByteArray(size) { i -> ((i * 31) and 0xFF).toByte() }
        val ctx = ByteArray(0)

        val keypair = slhDsaKeypairGenerate(param)

        repeat(warmup) {
            val sig = slhDsaSignDeterministic(param, keypair.signingKey, msg, ctx)
            slhDsaVerify(param, keypair.verifyingKey, msg, ctx, sig.signature)
        }

        var signTotal = 0.0
        var verifyTotal = 0.0

        repeat(iters) {
            val startSign = System.nanoTime()
            val sig = slhDsaSignDeterministic(param, keypair.signingKey, msg, ctx)
            signTotal += (System.nanoTime() - startSign).toDouble() / 1_000_000.0

            val startVerify = System.nanoTime()
            val verify = slhDsaVerify(param, keypair.verifyingKey, msg, ctx, sig.signature)
            verifyTotal += (System.nanoTime() - startVerify).toDouble() / 1_000_000.0
            if (verify.status != SlhDsaStatus.OK || !verify.valid) {
                error("Verify failed")
            }
        }

        val signAvg = signTotal / iters
        val verifyAvg = verifyTotal / iters
        val total = signAvg + verifyAvg
        println("${size},${"%.6f".format(signAvg)},${"%.6f".format(verifyAvg)},${"%.6f".format(total)}")
    }
}
