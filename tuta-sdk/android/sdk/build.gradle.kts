plugins {
	id("com.android.library")
	id("org.jetbrains.kotlin.android")
	id("org.mozilla.rust-android-gradle.rust-android")
}

var targetAbi = ""
if (gradle.startParameter.taskNames.isNotEmpty()) {
	if (gradle.startParameter.taskNames.size == 1) {
		val targetTask = gradle.startParameter.taskNames[0].lowercase()
		if (targetTask.contains("arm64")) {
			targetAbi = "arm64"
		} else if (targetTask.contains("arm")) {
			targetAbi = "arm"
		}
	}
}

android {
	namespace = "com.example.tutasdk"
	compileSdk = 34

	defaultConfig {
		minSdk = 26

		testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
		consumerProguardFiles("consumer-rules.pro")
	}

	buildTypes {
		release {
			isMinifyEnabled = false
			proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
		}
	}
	compileOptions {
		sourceCompatibility = JavaVersion.VERSION_1_8
		targetCompatibility = JavaVersion.VERSION_1_8
	}
	kotlinOptions {
		jvmTarget = "1.8"
	}
	sourceSets["main"].java.srcDirs(file("${layout.buildDirectory.asFile.get()}/generated-sources/tuta-sdk"))
}

dependencies {
	implementation("net.java.dev.jna:jna:5.13.0@aar")
	implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.6.4")
	implementation("androidx.annotation:annotation:1.8.0")
	testImplementation("junit:junit:4.13.2")
	androidTestImplementation("androidx.test.ext:junit:1.1.5")
	androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
}

cargo {
	module = "../../rust"
	libname = "tutasdk"
	prebuiltToolchains = true
	pythonCommand = "python3"
	targets = when {
		targetAbi.isBlank() -> listOf("arm", "arm64", "x86_64")
		targetAbi == "arm" -> listOf("arm")
		targetAbi == "arm64" -> listOf("arm64")
		else -> listOf("arm", "arm64")
	}
}

tasks.register<Exec>("generateBinding") {
	dependsOn("cargoBuild")
	workingDir("../../rust")
	executable("cargo")
	// FIXME pick the first target
	val anyTargetAbi = "arm64-v8a"
	args("run", "--bin", "uniffi-bindgen", "generate", "--library", "${layout.buildDirectory.asFile.get()}/rustJniLibs/android/${anyTargetAbi}/libtutasdk.so", "--language", "kotlin", "--out-dir", "${layout.buildDirectory.asFile.get()}/generated-sources/tuta-sdk")
}

tasks.whenTaskAdded {
	when (name) {
		"mergeDebugJniLibFolders", "mergeReleaseJniLibFolders" -> dependsOn("generateBinding")
	}
}