package xyz.mcxross.kfastcrypto

actual object Kfc {

    init {
      loadNativeLib()
    }

    val sigs: Sigs by lazy { Sigs }

    val ecvrf: Ecvrf by lazy { Ecvrf }

    val encode: Encode by lazy { Encode }

}