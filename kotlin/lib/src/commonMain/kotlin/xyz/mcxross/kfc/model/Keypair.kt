package xyz.mcxross.kfc.model

import kotlinx.serialization.Serializable

@Serializable
data class Keypair(
  val publicKey: String,
  val privateKey: String
)
