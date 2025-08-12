pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

plugins {
    id("com.android.application") version "8.12.0" apply false
}

rootProject.name = "luminara"

include(":app")
