## CSML IOS

### Build form source:

First clone the csml project in your machine 

Open your android project and  add the rust gradle plugin to your root build.gradle, like:

```C
buildscript {
    repositories {
        maven {
            url "https://plugins.gradle.org/m2/"
        }
    }
    dependencies {
        classpath "gradle.plugin.org.mozilla.rust-android-gradle:plugin:0.9.0"
        ...
    }
}
```

In your project's build.gradle, apply plugin and add the cargo configuration:

```C
apply plugin: 'org.mozilla.rust-android-gradle.rust-android'

cargo {
    module  = "../csml_andorid"       // Or whatever directory contains your Cargo.toml
    libname = "csml_andorid"
    targets = ["arm", "x86"]  // See bellow for a longer list of options
}
```

Install the rust toolchains for your target platforms:

```C
rustup target add armv7-linux-androideabi   # for arm
rustup target add i686-linux-android        # for x86
rustup target add aarch64-linux-android     # for arm64
rustup target add x86_64-linux-android      # for x86_64
rustup target add x86_64-unknown-linux-gnu  # for linux-x86-64
rustup target add x86_64-apple-darwin       # for darwin (macOS)
rustup target add x86_64-pc-windows-gnu     # for win32-x86-64-gnu
rustup target add x86_64-pc-windows-msvc    # for win32-x86-64-msvc
...
```

Finally, in the android project root run the cargoBuild task to cross compile:

```C
./gradlew cargoBuild
```

create a new java class in order to use the csml lib 

```C
public class CsmlBindings {
    static {
        System.loadLibrary("csml_android");
    }

    private static native String GetOpenConversation(final String pattern);


    public String call_get_open_conversation(String to) {
        return GetOpenConversation(to);
    }
}
```