plugins {
  kotlin("multiplatform") apply false
  kotlin("plugin.serialization") apply false
  id("com.android.library") apply false
}

group = "xyz.mcxross.kfc"

version = "0.1.0"

repositories { mavenCentral() }
