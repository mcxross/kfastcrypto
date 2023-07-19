package xyz.mcxross.kfastcrypto

import js.buffer.BufferSource

object WebAssembly {
  fun compile(bytes: BufferSource) = webassembly.compile(bytes)
}
