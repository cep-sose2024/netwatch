# NetWatch

## Wrapper for Android Keystore System

## Project Goal

The main goal of the Netwatch team is to use the security features of the Android platform, namely the Android Keystore system, to provide a way for the enmeshed Crypto Abstraction Layer to interact with mobile devices. The Android Keystore system lets you store cryptographic keys in a secure container to make them more difficult to extract from the device. Sensitive operations are delegated to a secure processor reached through some kernel interface. The resulting architecture looks like this:

![alt text](static/access-to-keymaster.png)

The overall goal of the project is to create a Rust-based wrapper that connects the HSM (Android Keystore) with the Crypto-Abstraction Layer provided by [j&s-soft](https://github.com/nmshd).

Note: This repository only houses the prototype designed to demonstrate the functionality of the android CAL implementation. To view the actual implementation, go to the [official CAL fork repository page](https://github.com/cep-sose2024/rust-crypto-netwatch).

## Features

- **Encryption & Decryption**
- **Verification & Signing**
- **Device attestation**

## Getting Started

### Prerequisites

- Rust Programming Language: Install the latest version of Rust on your system. You can install Rust through [rustup](https://rustup.rs/).
- Android Studio with the **NDK** and **CMAKE** SDK tools enabled.

### Running the Prototype

1. Clone the repository:

```bash
git clone https://github.com/cep-sose2024/netwatch.git
```

2. Install `cargo-ndk`

```bash
cargo install cargo-ndk
```

3. Add android toolchain:

```bash
rustup target add armv7-linux-androideabi
```

4. Navigate to the prototype wrapper directory:

```bash
cd netwatch/prototype_rust_wrapper
```

5. Build the library:

```bash
cargo ndk -t arm64-v8a -o ../prototype_new/app/src/main/jniLibs build
```

6. Run the prototype Java application using Android Studio.

## Contributing

We welcome contributions from the community. Please submit a pull request if you detect any bugs, have ideas for enhancements, or would like to add new functionality.

## License

This project is released under the [MIT License](LICENSE).

## Acknowledgments

- Special thanks to [j&s-soft](https://github.com/nmshd) for providing the Crypto Abstraction Layer.
