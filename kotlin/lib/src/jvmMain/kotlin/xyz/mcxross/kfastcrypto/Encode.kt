package xyz.mcxross.kfastcrypto

actual object Encode {
  actual fun base64ToHex(bytes: String): String = xyz.mcxross.kfastcrypto.base64ToHex(bytes)

  actual fun hexToBase64(bytes: String): String = xyz.mcxross.kfastcrypto.hexToBase64(bytes)
}
