package xyz.mcxross.kfastcrypto

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.serializer

@Serializable
data class Keypair(val publicKey: String, val privateKey: String)

object Sigs : Crypto {

  private val format = Json { prettyPrint = true }

  override fun keygen(): Keypair = format.decodeFromString(serializer(), generateKeypair())

  fun sign(
    input: String,
  ) {
  }

  override fun verify(
    output: String,
    proof: String,
    input: String,
    publicKey: String,
  ) {
  }
}
