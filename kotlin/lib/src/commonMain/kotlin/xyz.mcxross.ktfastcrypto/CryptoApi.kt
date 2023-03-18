package xyz.mcxross.ktfastcrypto

interface CryptoApi {
    fun keygen(): Keypair
    fun prove(input: String)
    fun verify(output: String, proof: String, input: String, publicKey: String)
}