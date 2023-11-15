val kotlinWrappersVersion = "1.0.0-pre.598"

plugins {
  kotlin("multiplatform")
  kotlin("plugin.serialization")
  id("com.android.library")
  id("maven-publish")
}

group = "xyz.mcxross.kfc"

version = "0.1.0"

repositories {
  mavenCentral()
  google()
}

kotlin {

  androidTarget { publishLibraryVariants("release", "debug") }

  iosArm64()
  iosSimulatorArm64()

  jvm {
    jvmToolchain(17)
    testRuns["test"].executionTask.configure { useJUnitPlatform() }
  }

  js {
    browser {
      webpackTask {
        outputFileName = "fastcrypto.js"
      }
    }
    nodejs {}
    binaries.executable()
  }

  linuxX64()
  macosArm64()
  macosX64()
  mingwX64()

  sourceSets {
    commonMain.dependencies {
      implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.5.1")
    }
    commonTest.dependencies {
      implementation(kotlin("test"))
    }
    val jvmAndroidMain by creating {
      dependsOn(commonMain.get())
    }
    val jvmAndroidTest by creating {
      dependsOn(commonTest.get())
    }
    val androidMain by getting {
      dependsOn(jvmAndroidMain)
    }
    val jvmMain by getting {
      dependsOn(jvmAndroidMain)
    }
    jsMain.dependencies {
      implementation("org.jetbrains.kotlin-wrappers:kotlin-web")
    }
  }
}

java.toolchain.languageVersion.set(JavaLanguageVersion.of(17))

android {
  namespace = "mcxross.kfc"
  defaultConfig {
    minSdk = 24
    compileSdk = 33
  }
  sourceSets {
    named("main") {
      manifest.srcFile("src/androidMain/AndroidManifest.xml")
      jniLibs.srcDirs("src/main/jniLibs")
      res.srcDirs("src/androidMain/res")
    }
  }
}
