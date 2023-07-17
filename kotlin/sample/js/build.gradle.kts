plugins {
  kotlin("multiplatform")
}

group = "xyz.mcxross.kfastcrypto"

version = "0.1.0"

repositories {
  mavenCentral()
  mavenLocal()
}

kotlin {
  js {
    browser {}
    nodejs {}
    binaries.executable()
  }
  sourceSets {
    val jsMain by getting {
      kotlin.srcDir("src/main/kotlin")
      resources.srcDir("src/main/res")
      dependencies {
        
      }
    }
  }
}
