package xyz.mcxross.kfastcrypto

import kotlinx.serialization.json.Json
import kotlinx.serialization.serializer
import xyz.mcxross.kfastcrypto.model.Keypair
import xyz.mcxross.kfastcrypto.model.SignatureScheme

object Sigs {

  private val format = Json { prettyPrint = true }
  fun keygen(scheme: SignatureScheme, seed: String = ""): Keypair =
    format.decodeFromString(serializer(), sigsGenerateKeypair(scheme.value(), seed))

  fun sign(
    msg: String, sk: String, scheme: String
  ): String = sigsSign(msg, sk, scheme)

  fun verify(
    msg: String,
    sig: String,
    pk: String,
    scheme: String,
  ): Boolean = sigsVerify(msg, sig, pk, scheme)
}
