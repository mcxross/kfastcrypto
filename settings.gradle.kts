rootProject.name = "kfastcrypto"

include(":kotlin:lib", ":kotlin:sample:jvm", ":kotlin:sample:js")

project(":kotlin:lib").name = "kfastcrypto"
project(":kotlin:sample:js").name = "kfastcrypto-sample"
