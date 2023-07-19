/** This file is used as a namespace for all the exported Rust functions. */
@file:JvmName("FastCryptoApi")

package xyz.mcxross.kfastcrypto

import kotlin.jvm.JvmName

external fun sigsGenerateKeypair(scheme: String, seed: String): String

external fun sigsSign(msg: String, sk: String, scheme: String): String

external fun sigsVerify(
  msg: String,
  sig: String,
  pk: String,
  scheme: String
): Boolean

external fun generateKeypair(): String

external fun prove(
  input: String,
  secretKey: String,
): String

external fun verify(
  output: String,
  proof: String,
  input: String,
  publicKey: String
): Boolean

external fun base64ToHex(bytes: String): String
external fun hexToBase64(bytes: String): String
