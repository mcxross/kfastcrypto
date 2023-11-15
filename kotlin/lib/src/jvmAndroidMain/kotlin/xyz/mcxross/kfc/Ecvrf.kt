package xyz.mcxross.kfc

import kotlinx.serialization.json.Json
import kotlinx.serialization.serializer
import xyz.mcxross.kfc.model.Keypair
import xyz.mcxross.kfc.model.Proof

actual object Ecvrf {

  private val format = Json { prettyPrint = true }
  actual fun keygen(): Keypair = format.decodeFromString(serializer(), generateKeypair())

  actual fun prove(
    input: String,
    secretKey: String
  ): Proof = format.decodeFromString(serializer(), xyz.mcxross.kfc.prove(input, secretKey))

  actual fun verify(
    output: String,
    proof: String,
    input: String,
    publicKey: String
  ): Boolean = xyz.mcxross.kfc.verify(output, proof, input, publicKey)

}
