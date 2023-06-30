package xyz.mcxross.kfastcrypto

import kotlinx.serialization.json.Json
import kotlinx.serialization.serializer

object Ecvrf : Crypto {

  private val format = Json { prettyPrint = true }

  override fun keygen(): Keypair = format.decodeFromString(serializer(), generateKeypair())

  fun prove() {
    TODO("Not yet implemented")
  }

  override fun verify(output: String, proof: String, input: String, publicKey: String) {
    TODO("Not yet implemented")
  }

}
