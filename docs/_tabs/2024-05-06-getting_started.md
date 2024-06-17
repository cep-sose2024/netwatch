---
title: "Getting Started"
date: 2024-04-00 00:00:00 +0800
categories: [Getting Stared]
pin: true
---

> Check out the **Enmeshed Documentation** [here.](https://enmeshed.eu/) <img src="https://avatars.githubusercontent.com/u/87031446?s=200&v=4" alt="Beschreibung des Bildes" width="20" height="20">
{: .prompt-info }


## Prototype

The **NetWatch** prototype demonstrates how the wrapper accesses the Android Keystore System through an Abstraction Layer. This layer offers a more user-friendly interface for key management and security operations. The Abstraction Layer also allows various security features, which is especially important for effective application on various newer Android devices like the Google Pixel. 

## Development

### Technology Stack

The project mainly uses the [**Rust**](https://www.rust-lang.org/) programming language. As well as [**Android Studio**](https://developer.android.com/studio?hl=en)  and  [**ADB**](https://developer.android.com/tools/adb?hl=en) (*Android Debug Bridge*) to develop and test the applications on Android devices. The **Git** version control system is used to ensure management in **GitHub**.

### Prerequisites

1. **Install Rust:**
   - Visit [https://rustup.rs/](https://rustup.rs/) and follow the instructions to install the Rust toolchain.
2. **Install Android Studio:**
   - Download Android Studio from [https://developer.android.com/studio](https://developer.android.com/studio) and follow the installation instructions.

### Installation

1. **Clone the Repository:**
   - Open your terminal and run `git clone https://github.com/cep-sose2024/netwatch.git`.
2. **Install Dependencies:**
   - Switch to the project directory with `cd netwatch` and run `cargo build` to install all necessary dependencies.
3. **Conduct Local Tests:**
   - Run `cargo test` to ensure that all tests are successful.

## Content and Structure

- **Source Code:**
  - Rust modules for interfacing with the Android Keystore and managing interactions with Hardware Security Modules (HSM).

- **Documentation:**
  - The project documentation provides technical guidance and descriptions, as well as detailed explanations of the architecture, error handling, functionalities and more important information to understand the work process..

- **Tests:**
  - Automated tests are provided to ensure the integrity, security, and performance of the software.

- **Examples and Tutorials:**
  - Examples and tutorials are provided to better understand the project and its processes.
