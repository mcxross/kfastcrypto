package xyz.mcxross.kfastcrypto

import xyz.mcxross.kfastcrypto.model.Keypair
import xyz.mcxross.kfastcrypto.model.Proof

actual object Ecvrf {
  actual fun keygen(): Keypair {
    TODO("Not yet implemented")
  }

  actual fun prove(
    input: String,
    secretKey: String
  ): Proof {
    TODO("Not yet implemented")
  }

  actual fun verify(
    output: String,
    proof: String,
    input: String,
    publicKey: String
  ): Boolean {
    TODO("Not yet implemented")
  }
}
