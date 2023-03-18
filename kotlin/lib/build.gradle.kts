import org.gradle.api.publish.maven.MavenPublication
import org.gradle.api.tasks.Copy
import org.gradle.kotlin.dsl.*
import org.jetbrains.kotlin.util.capitalizeDecapitalize.toLowerCaseAsciiOnly

plugins {
  kotlin("multiplatform") version "1.8.0"
  kotlin("plugin.serialization") version "1.8.0"
  id("maven-publish")
}

group = "xyz.mcxross.ktfastcrypto"

version = "0.1.0"

repositories { mavenCentral() }

kotlin {
  jvm {
    jvmToolchain(8)
    withJava()
    testRuns["test"].executionTask.configure { useJUnitPlatform() }
  }

  val hostOs = System.getProperty("os.name")
  val isMingwX64 = hostOs.startsWith("Windows")
  val nativeTarget =
      when {
        hostOs == "Mac OS X" -> macosX64("native")
        hostOs == "Linux" -> linuxX64("native")
        isMingwX64 -> mingwX64("native")
        else -> throw GradleException("Host OS is not supported in Kotlin/Native.")
      }

  sourceSets {
    val commonMain by getting {
      dependencies { implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.5.0") }
    }
    val commonTest by getting { dependencies { implementation(kotlin("test")) } }
    val jvmMain by getting
    val jvmTest by getting
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
val platformTargets = listOf("win", "linux" , "mac")

val copyNativeLibs by
    tasks.register<Copy>("copyNativeLibs") {
      val rootDir = project.rootDir
      platformTargets.forEach {
        archs.forEach { arch ->
          from("$rootDir/lib/${it}/${arch}") {
            include("**/*")
          }
        }
        into("build/classes/kotlin/main/lib")
      }
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
  publications {
    named<MavenPublication>("jvm") {
      artifactId = "ktfastcrypto-jvm-win"
    }
    publishTargets.forEach { (t, u) ->
      create<MavenPublication>("mavenPublication${t}") {
        artifactId = "ktfastcrypto-${u}"
        from(components["kotlin"])
        artifact(tasks.named("jvmJar"))
      }
    }
  }
}

tasks.named<Jar>("jvmJar") {
  dependsOn(copyNativeLibs)
  into("lib") { from("build/classes/kotlin/main/lib") }
}
