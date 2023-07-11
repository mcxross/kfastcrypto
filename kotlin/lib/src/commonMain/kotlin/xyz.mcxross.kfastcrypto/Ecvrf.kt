package xyz.mcxross.kfastcrypto

import kotlinx.serialization.json.Json
import kotlinx.serialization.serializer
import xyz.mcxross.kfastcrypto.model.Keypair
import xyz.mcxross.kfastcrypto.model.Proof

object Ecvrf {

  private val format = Json { prettyPrint = true }

  fun keygen(): Keypair = format.decodeFromString(serializer(), generateKeypair())

  fun prove(input: String, secretKey: String): Proof =
    format.decodeFromString(serializer(), xyz.mcxross.kfastcrypto.prove(input, secretKey))

  fun verify(output: String, proof: String, input: String, publicKey: String): Boolean =
    xyz.mcxross.kfastcrypto.verify(output, proof, input, publicKey)

}
