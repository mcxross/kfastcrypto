package xyz.mcxross.kfc

import xyz.mcxross.kfc.model.Keypair
import xyz.mcxross.kfc.model.Proof

expect object Ecvrf {

  fun keygen(): Keypair

  fun prove(input: String, secretKey: String): Proof

  fun verify(output: String, proof: String, input: String, publicKey: String): Boolean

}
