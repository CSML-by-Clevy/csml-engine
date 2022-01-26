## CSML IOS

### Instalation:

Open your xcode porject add the **libcsmllib.a** file in the General > Frameworks, Libraries and Embedded Content section.

<h1 align="center">
  <br>
  <a href="https://www.csml.dev"><img src="./images/general_add_lib_.png?raw=true" width="500"></a>
  <br>
</h1>

After that, go to the Build Settings tab, search for search paths and add the **header** and **library** search paths

<h1 align="center">
  <br>
  <a href="https://www.csml.dev"><img src="./images/add_paths.png?raw=true" width="500"></a>
  <br>
</h1>

Finally, let’s add the Objective-C Bridging header. Search for bridging header in the Build Settings tab:

<h1 align="center">
  <br>
  <a href="https://www.csml.dev"><img src="./images/add_header.png?raw=true" width="500"></a>
  <br>
</h1>

```cpp
    // Example: Swift code
    // The interaction whit the API is only made via Strings

    let result = hello_csml("Alexis")

    // make a Swift String
    let sr = String(cString: result!)

    // IMPORTANT: once we get the result we have to release the pointer.
    release_string(UnsafeMutablePointer(mutating: result))
```

### build form source

In order to build the csml lib for IOS you will need to install some targets in your machine.

```cpp
// aarch64-apple-ios
// armv7-apple-ios
// armv7s-apple-ios
// x86_64-apple-ios i386-apple-ios

rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios

```

after that build the project using the build.sh in order to get **libcsmllib.a** and **libcsmllib.a**

```cpp
sh build.sh
```