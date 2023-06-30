package xyz.mcxross.kfastcrypto.model

import kotlinx.serialization.Serializable

@Serializable
data class Keypair(
  val publicKey: String,
  val privateKey: String
)
