package xyz.mcxross.kfastcrypto

import xyz.mcxross.kfastcrypto.model.Keypair
import xyz.mcxross.kfastcrypto.model.SignatureScheme

expect object Sigs {

  fun keygen(scheme: SignatureScheme, seed: String = ""): Keypair

  fun sign(
    msg: String, sk: String, scheme: String
  ): String

  fun verify(
    msg: String,
    sig: String,
    pk: String,
    scheme: String,
  ): Boolean
}
