pluginManagement {
  repositories {
    google()
    gradlePluginPortal()
    mavenCentral()
    mavenLocal()
  }

  plugins {
    kotlin("jvm").version(extra["kotlin.version"] as String)
    kotlin("multiplatform").version(extra["kotlin.version"] as String)
    kotlin("plugin.serialization").version(extra["kotlin.version"] as String)
    id("com.android.library").version(extra["agp.version"] as String)
  }

}

rootProject.name = "kfastcrypto"

include(":kotlin:lib", ":kotlin:sample:jvm", ":kotlin:sample:js")

project(":kotlin:lib").name = "kfastcrypto"
project(":kotlin:sample:jvm").name = "kfc-jvm-sample"
project(":kotlin:sample:js").name = "kfc-js-sample"
