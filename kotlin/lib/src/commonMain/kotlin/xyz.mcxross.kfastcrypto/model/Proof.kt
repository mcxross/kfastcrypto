package xyz.mcxross.kfastcrypto.model

import kotlinx.serialization.Serializable

@Serializable
data class Proof(
  val proofStr: String,
  val hashStr: String,
)
