import Foundation

@main
struct TestMain {
    static func main() {
        let param = ParameterSetId.shake128f
        let keypair = slhDsaKeypairGenerate(param: param)

        if keypair.signingKey.isEmpty || keypair.verifyingKey.isEmpty {
            fatalError("Generated empty keypair")
        }

        let message = "hello from swift".data(using: .utf8)!
        let ctx = "swift-test".data(using: .utf8)!

        let signResult = slhDsaSign(param: param, signingKey: keypair.signingKey, msg: message, ctx: ctx)
        if signResult.status != SlhDsaStatus.ok {
            fatalError("Sign failed: \(signResult.status)")
        }

        let verifyResult = slhDsaVerify(
            param: param,
            verifyingKey: keypair.verifyingKey,
            msg: message,
            ctx: ctx,
            signature: signResult.signature
        )

        if verifyResult.status != SlhDsaStatus.ok || !verifyResult.valid {
            fatalError("Verify failed")
        }

        print("swift test ok")
    }
}
