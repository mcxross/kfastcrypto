plugins {
  kotlin("jvm")
  application
}

group = "xyz.mcxross.kfastcrypto"

version = "0.1.0"

repositories {
  mavenCentral()
  mavenLocal()
}

dependencies {
  implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.5.0")
  implementation("xyz.mcxross.kfastcrypto:kfastcrypto-jvm-win:0.1.0")
  testImplementation("org.junit.jupiter:junit-jupiter-api:5.9.2")
  testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.9.2")
}

tasks.getByName<Test>("test") { useJUnitPlatform() }
