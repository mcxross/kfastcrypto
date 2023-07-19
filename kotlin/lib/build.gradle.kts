val kotlinWrappersVersion = "1.0.0-pre.598"

plugins {
  kotlin("multiplatform") version "1.8.0"
  kotlin("plugin.serialization") version "1.8.0"
  id("maven-publish")
}

group = "xyz.mcxross.kfastcrypto"

version = "0.1.0"

repositories { mavenCentral() }

kotlin {
  jvm {
    jvmToolchain(8)
    withJava()
    testRuns["test"].executionTask.configure { useJUnitPlatform() }
  }

  js {
    browser {}
    nodejs {}
  }

  val hostOs = System.getProperty("os.name")
  val isMingwX64 = hostOs.startsWith("Windows")

  when {
    hostOs == "Mac OS X" -> macosX64("native")
    hostOs == "Linux" -> linuxX64("native")
    isMingwX64 -> mingwX64("native")
    else -> throw GradleException("Host OS is not supported in Kotlin/Native.")
  }

  sourceSets {
    val commonMain by getting {
      dependencies {
        implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.5.0")
      }
    }
    val commonTest by getting { dependencies { implementation(kotlin("test")) } }
    val jvmMain by getting
    val jvmTest by getting
    val jsMain by getting {
      dependencies {
        implementation(enforcedPlatform("org.jetbrains.kotlin-wrappers:kotlin-wrappers-bom:$kotlinWrappersVersion"))
        implementation("org.jetbrains.kotlin-wrappers:kotlin-web")
      }
    }
    val jsTest by getting
    val nativeMain by getting
    val nativeTest by getting
  }
}

val publishTargets =
  listOf(
    "jvmWinAmd" to "jvm-win-amd",
    "jvmWinAarch" to "jvm-win-aarch",
    "jvmLinuxAmd" to "jvm-linux-amd",
    "jvmLinuxAarch" to "jvm-linux-aarch",
  )
    .associate { (target, platform) -> target.capitalize() to platform }

val archs = listOf("amd64", "aarch64")
val platformTargets = listOf("win", "linux", "mac")

val copyNativeLibs by
tasks.register<Copy>("copyNativeLibs") {
  from("${project.rootDir}/lib")
  into("build/classes/kotlin/main/lib")
}

val publishingConfigurations = configurations.create("publishingConfigurations")

publishing {
  repositories {
    maven {
      name = "MavenCentral"
      url = uri("https://oss.sonatype.org/service/local/staging/deploy/maven2/")
      credentials {
        username = System.getenv("MAVEN_CENTRAL_USERNAME")
        password = System.getenv("MAVEN_CENTRAL_PASSWORD")
      }
    }
  }
}

tasks.named<Jar>("jvmJar") {
  dependsOn(copyNativeLibs)
  into("lib") {
    from("build/classes/kotlin/main/lib")
  }
}
