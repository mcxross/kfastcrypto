package xyz.mcxross.kfc

expect object Encode {
  fun base64ToHex(bytes: String): String
  fun hexToBase64(bytes: String): String
}
