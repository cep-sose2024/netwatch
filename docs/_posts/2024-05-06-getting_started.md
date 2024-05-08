---
title: "Getting Started"
date: 2024-05-06 00:00:00 +0800
categories: [Start here]
tags: [started]
---

## What is NetWatch?

**NetWatch** is part of the Cyber Security Development Project (CEP) at the Mannheim University of Applied Science. The main goal is to develop an interface wrapper that ensures secure interaction with the Android Keystore system. The wrapper is specifically optimized for use on Google Pixel devices and integrates Hardware Security Modules (HSM) to maximize data security. This is achieved through improved and trustworthy storage and transmission of sensitive data, specifically tailored to the security architecture of mobile devices.

## Who is behind NetWatch?

NetWatch is a team of students from the [Mannheim University of Applied Sciences](https://www.english.hs-mannheim.de/the-university.html) in collaboration with [j&s-soft GmbH]([Start - j&s-soft (js-soft.com)](https://www.js-soft.com/)) based in Heidelberg, Germany. This project is part of the  [**Enmeshed**](https://enmeshed.de/en/) platform, which aims to provide open-source solutions for secure and efficient digital communication and collaboration.

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
