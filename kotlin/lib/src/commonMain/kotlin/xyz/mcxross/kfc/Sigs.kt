package xyz.mcxross.kfc

import xyz.mcxross.kfc.model.Keypair
import xyz.mcxross.kfc.model.SignatureScheme

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
