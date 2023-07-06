package xyz.mcxross.kfastcrypto.model

enum class SignatureScheme {
  ED25519 {
    override fun value(): String = "ed25519"
  },
  SECP256K1 {
    override fun value(): String = "secp256k1"
  },
  SECP256K1RECOVERABLE {
    override fun value(): String = "secp256k1-rec"
  },
  SECP256R1 {
    override fun value(): String = "secp256r1"
  },
  SECP256R1RECOVERABLE {
    override fun value(): String = "secp256r1-rec"
  },
  BLS12381MINSIG {
    override fun value(): String = "bls12381-minsig"
  },
  BLS12381MINPK {
    override fun value(): String = "bls12381-minpk"
  };

  abstract fun value(): String
}
