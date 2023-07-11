package xyz.mcxross.kfastcrypto

object Kfc {
  init {
    loadNativeLib()
  }

  val sigs: Sigs
    get() = Sigs

  val ecvrf: Ecvrf
    get() = Ecvrf

  val encode: Encode
    get() = Encode

}
