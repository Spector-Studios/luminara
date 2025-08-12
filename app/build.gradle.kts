plugins {
    id("com.android.application") version "8.12.0"
}

repositories {
    google()
    mavenCentral()
}

java {
    toolchain.languageVersion.set(JavaLanguageVersion.of(21)) // or 11
}

android {
    compileSdk = 30
    ndkVersion = "27.1.12297006"
    defaultConfig {
        applicationId = "dev.spectorstudios.luminara"
        namespace = "dev.spectorstudios.luminara"
        minSdk = 28
        targetSdk = 33
        versionCode = 1
        versionName = "1.0"
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    signingConfigs {
        // TODO Create a seperate debug signing
        create("releaseSigning") {
            val inCI = System.getenv("CI")?.toBoolean() ?: false

            val storePath: String?
            val storePassword: String?
            val keyAlias: String?
            val keyPassword: String?

            if (inCI) {
                storePath = System.getenv("KEYSTORE_PATH")
                storePassword = System.getenv("KEYSTORE_PASSWORD")
                keyAlias = System.getenv("KEY_ALIAS")
                keyPassword = System.getenv("KEY_PASSWORD")
            } else {
                storePath = project.findProperty("keyStoreFile") as? String
                storePassword = project.findProperty("keyStorePassword") as? String
                keyAlias = project.findProperty("luminaraKeyAlias") as? String
                keyPassword = project.findProperty("luminaraKeyPassword") as? String
            }

            if (storePath != null && storePassword != null && keyAlias != null && keyPassword != null) {
                println("✅ Using custom signing config")
                storeFile = file(storePath)
                this.storePassword = storePassword
                this.keyAlias = keyAlias
                this.keyPassword = keyPassword
            } else {
                println("⚠️ No signing config found — will use default Android debug keystore")
                // Do NOT set any properties — Android will fall back to default
            }
        }
    }

    buildTypes {
        getByName("debug") {
            // Only assign signing config if it was successfully configured
            if (signingConfigs.findByName("releaseSigning")?.storeFile != null) {
                signingConfig = signingConfigs.getByName("releaseSigning")
            }
        }

        getByName("release") {
            if (signingConfigs.findByName("releaseSigning")?.storeFile != null) {
                signingConfig = signingConfigs.getByName("releaseSigning")
            }
            isMinifyEnabled = false
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }
    }
}

tasks.withType<JavaCompile>().configureEach {
    options.compilerArgs.add("-Xlint:deprecation")
}

android.applicationVariants.all {
    if (buildType.name == "debug") {
        preBuildProvider.configure {
            dependsOn(buildRustLibDebug)
            dependsOn(syncAssets)
        }
    }

    if (buildType.name == "release") {
        preBuildProvider.configure {
            dependsOn(buildRustLibRelease)
            dependsOn(syncAssets)
        }
    }
}

val syncAssets by tasks.registering(Sync::class) {
    from(rootProject.layout.projectDirectory.dir("assets"))
    into(layout.projectDirectory.dir("src/main/assets"))
}

fun buildCargoCommand(isRelease: Boolean): List<String> {
    val flavor = if (isRelease) "release" else "debug"
    val outDir = layout.projectDirectory.dir("src/${flavor}/jniLibs").asFile.absolutePath

    val profile = if (isRelease) "android-release" else "android-dev"

    val abis = buildList {
        add("arm64-v8a")
        if (isRelease) {
            addAll(listOf("armeabi-v7a", "x86", "x86_64"))
        }
    }
    
    val command = buildList {
        addAll(listOf("cargo", "ndk"))
        addAll(listOf("--platform", "${android.defaultConfig.minSdk}"))
        addAll(listOf("-o", outDir))
        addAll(abis.flatMap {listOf("-t", it)})
        add("build")
        addAll(listOf("--profile", profile))
    }

    return command
}

val buildRustLibDebug by tasks.registering(Exec::class) {
    environment("CARGO_TERM_COLOR", "always")
    workingDir = rootProject.layout.projectDirectory.asFile
    commandLine(buildCargoCommand(false))
}

val buildRustLibRelease by tasks.registering(Exec::class) {
    environment("CARGO_TERM_COLOR", "always")
    workingDir = rootProject.layout.projectDirectory.asFile
    commandLine(buildCargoCommand(true))
}

dependencies {
    testImplementation("junit:junit:4.13.1")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test:monitor:1.6.1")
    androidTestImplementation("junit:junit:4.13.2")
}
