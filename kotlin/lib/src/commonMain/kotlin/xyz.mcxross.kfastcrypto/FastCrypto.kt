package xyz.mcxross.kfastcrypto

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.serializer

@Serializable data class Keypair(val publicKey: String, val privateKey: String)

public object FastCrypto : CryptoApi {

  private val format = Json { prettyPrint = true }

  init {
    loadNativeLib()
  }

  public override fun keygen(): Keypair = format.decodeFromString(serializer(), generateKeypair())

  public override fun prove(
      input: String,
  ) {}

  public override fun verify(
      output: String,
      proof: String,
      input: String,
      publicKey: String,
  ) {}
}
