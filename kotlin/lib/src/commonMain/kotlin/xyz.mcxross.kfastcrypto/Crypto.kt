package xyz.mcxross.kfastcrypto

interface Crypto {
  fun keygen(): Keypair

  fun verify(output: String, proof: String, input: String, publicKey: String)
}
