package xyz.mcxross.kfastcrypto

import xyz.mcxross.kfastcrypto.model.Keypair
import xyz.mcxross.kfastcrypto.model.SignatureScheme

actual object Sigs {
  actual fun keygen(
    scheme: SignatureScheme,
    seed: String
  ): Keypair {
    TODO("Not yet implemented")
  }

  actual fun sign(msg: String, sk: String, scheme: String): String {
    TODO("Not yet implemented")
  }

  actual fun verify(
    msg: String,
    sig: String,
    pk: String,
    scheme: String
  ): Boolean {
    TODO("Not yet implemented")
  }
}
