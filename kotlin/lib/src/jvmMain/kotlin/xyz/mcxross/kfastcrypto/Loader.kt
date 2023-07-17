package xyz.mcxross.kfastcrypto

import java.io.File
import java.io.FileOutputStream

/**
 * Loads the native library "fastcrypto" based on the current operating system.
 * The library file is extracted from the classpath and copied to a temporary file
 * before being loaded with the system loader.
 *
 * @throws Exception if the operating system is not supported or the native library
 * cannot be loaded.
 */
fun loadNativeLib() {
   val os = System.getProperty("os.name")
   val extension = when {
      os.startsWith("Windows") -> ".dll"
      os == "Mac OS X" -> ".dylib"
      os == "Linux" -> ".so"
      else -> throw Exception("Unsupported OS: $os")
   }
   val inputStream = ClassLoader.getSystemClassLoader().getResourceAsStream("lib/fastcrypto$extension")
   val tempFile = File.createTempFile("fastcrypto$extension", null)
   tempFile.deleteOnExit()
   inputStream.use { input ->
      FileOutputStream(tempFile).use { output ->
         input?.copyTo(output) ?: throw Exception("Failed to load native library.")
      }
   }
   System.load(tempFile.absolutePath)
}
