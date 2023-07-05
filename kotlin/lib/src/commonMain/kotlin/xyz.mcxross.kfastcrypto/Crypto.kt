package xyz.mcxross.kfastcrypto

import xyz.mcxross.kfastcrypto.model.Keypair

interface Crypto {
  fun keygen(): Keypair

  fun verify(output: String, proof: String, input: String, publicKey: String): Boolean
}
