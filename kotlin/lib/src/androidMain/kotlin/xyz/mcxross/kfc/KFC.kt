package xyz.mcxross.kfc

fun loadNativeLib() {
  System.loadLibrary("fastcrypto")
}

actual object KFC {

  init {
    loadNativeLib()
  }

  val sigs: Sigs by lazy { Sigs }

  val ecvrf: Ecvrf by lazy { Ecvrf }

  val encode: Encode by lazy { Encode }

}
