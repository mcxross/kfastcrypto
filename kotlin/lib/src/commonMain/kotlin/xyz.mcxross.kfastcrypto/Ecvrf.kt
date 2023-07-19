package xyz.mcxross.kfastcrypto

import xyz.mcxross.kfastcrypto.model.Keypair
import xyz.mcxross.kfastcrypto.model.Proof

expect object Ecvrf {

  fun keygen(): Keypair

  fun prove(input: String, secretKey: String): Proof

  fun verify(output: String, proof: String, input: String, publicKey: String): Boolean

}
