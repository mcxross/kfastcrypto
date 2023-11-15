package xyz.mcxross.kfc

actual object Encode {
  actual fun base64ToHex(bytes: String): String = xyz.mcxross.kfc.base64ToHex(bytes)

  actual fun hexToBase64(bytes: String): String = xyz.mcxross.kfc.hexToBase64(bytes)
}
