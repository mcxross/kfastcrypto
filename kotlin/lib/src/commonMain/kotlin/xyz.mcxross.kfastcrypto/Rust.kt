/** This file is used as a namespace for all the exported Rust functions. */
@file:JvmName("FastCryptoApi")

package xyz.mcxross.kfastcrypto

import kotlin.jvm.JvmName

external fun generateKeypair(): String

external fun sign()

external fun prove()

external fun verify()
