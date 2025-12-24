package test

import uniffi.slh_dsa_uniffi.ParameterSetId
import uniffi.slh_dsa_uniffi.SlhDsaStatus
import uniffi.slh_dsa_uniffi.slhDsaKeypairGenerate
import uniffi.slh_dsa_uniffi.slhDsaSign
import uniffi.slh_dsa_uniffi.slhDsaVerify

fun main() {
    val param = ParameterSetId.SHAKE128F
    val keypair = slhDsaKeypairGenerate(param)

    if (keypair.signingKey.isEmpty() || keypair.verifyingKey.isEmpty()) {
        error("Generated empty keypair")
    }

    val message = "hello from kotlin".toByteArray()
    val ctx = "kotlin-test".toByteArray()

    val signResult = slhDsaSign(param, keypair.signingKey, message, ctx)
    if (signResult.status != SlhDsaStatus.OK) {
        error("Sign failed: ${signResult.status}")
    }

    val verifyResult = slhDsaVerify(param, keypair.verifyingKey, message, ctx, signResult.signature)
    if (verifyResult.status != SlhDsaStatus.OK || !verifyResult.valid) {
        error("Verify failed")
    }

    println("kotlin test ok")
}
