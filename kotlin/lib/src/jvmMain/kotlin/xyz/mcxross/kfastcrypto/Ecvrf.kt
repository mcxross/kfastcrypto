package xyz.mcxross.kfastcrypto

import kotlinx.serialization.json.Json
import kotlinx.serialization.serializer
import xyz.mcxross.kfastcrypto.model.Keypair
import xyz.mcxross.kfastcrypto.model.Proof

actual object Ecvrf {

  private val format = Json { prettyPrint = true }
  actual fun keygen(): Keypair = format.decodeFromString(serializer(), generateKeypair())

  actual fun prove(
    input: String,
    secretKey: String
  ): Proof = format.decodeFromString(serializer(), xyz.mcxross.kfastcrypto.prove(input, secretKey))

  actual fun verify(
    output: String,
    proof: String,
    input: String,
    publicKey: String
  ): Boolean = xyz.mcxross.kfastcrypto.verify(output, proof, input, publicKey)

}
