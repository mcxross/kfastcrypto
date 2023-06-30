package xyz.mcxross.kfastcrypto

object Kfc {
  init {
    loadNativeLib()
  }

  fun sigs(): Sigs = Sigs

  fun ecvrf(): Ecvrf = Ecvrf

  fun encode(): Encode = Encode

}
