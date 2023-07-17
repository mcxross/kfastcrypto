package xyz.mcxross.kfastcrypto

import xyz.mcxross.kfastcrypto.model.SignatureScheme

fun main(args: Array<String>) {
  println(Kfc.sigs.keygen(SignatureScheme.ED25519))
}
