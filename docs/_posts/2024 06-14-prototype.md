---
title: "Prototype"
date: 2024-06-14 00:00:00 +0800
categories: [Architecture]
tags: [prototype]
mermaid: true
---

# Architecture

```mermaid
sequenceDiagram
  participant Storage
  participant Application
  participant RustLibrary
  participant AndroidKeystoreAPI
  participant SecureElement

  Application->>RustLibrary: Generate Key Request
  alt Secure Element Available
    RustLibrary->>AndroidKeystoreAPI: Request Key (via Keystore API)
    AndroidKeystoreAPI->>SecureElement: Request Key Generation
    SecureElement->>SecureElement: Store Generated Key
    SecureElement->>AndroidKeystoreAPI: Key Generated
    AndroidKeystoreAPI->>RustLibrary: Key Generated
    RustLibrary->>Application: Key Generated
  else Secure Element Not Available
    RustLibrary->>AndroidKeystoreAPI: Request Key (via Keystore API)
    AndroidKeystoreAPI->>RustLibrary: Error (No Secure Element)
    RustLibrary->>Application: Error (No Secure Element)
  end

  Application->>RustLibrary: File to Encrypt (Bytes)
  RustLibrary->>AndroidKeystoreAPI: Encrypt using Generated Key (via Keystore API)
  AndroidKeystoreAPI->>SecureElement: Encrypt using Generated Key
  SecureElement->>AndroidKeystoreAPI: Encrypted Bytes
  AndroidKeystoreAPI->>RustLibrary: Encrypted Bytes
  RustLibrary->>Application: Encrypted File
  Application->>Storage: Save File to Storage
```

# Prototype Overview

The prototype is a Java application designed to test and demonstrate the functionality of interacting with the Android implementation of the Crypto-Abstraction-Layer. This layer provides a unified interface for cryptographic operations, abstracting the underlying complexities of different cryptographic algorithms and hardware implementations.

# Prototype Functionality

1. Key Generation: The creation of cryptographic keys for various algorithms.
2. Symmetric and Asymmetric Encryption of Text and Images: Encrypting data using both symmetric (e.g., AES) and asymmetric (e.g., RSA) algorithms.
3. Symmetric and Asymmetric Decryption of Text and Images: Decrypting data that was encrypted using symmetric and asymmetric algorithms.
4. Signing Encrypted Bytes: Generating digital signatures for encrypted data to ensure authenticity and integrity.
5. Verification of Signatures: Verifying the authenticity of digital signatures to confirm the integrity of the data.

# Prototype UI

![image1](/docs/assets/img/prototype_tab1.jpg)

![image2](/docs/assets/img/prototype_tab2.jpg)

# Key Generation Process

1. The user specifies the algorithm, padding, and purposes for the key.
2. The user clicks a button to generate the key.
3. The Java application sends this instruction to the library.
4. The library requests the Keystore to generate a key on the device's Secure Element.

# Encryption Process

1. After the key is generated, the user selects a file to encrypt.
2. The Java application sends the file bytes to the library.
3. The library uses the generated key to encrypt the data.
4. The library sends the encrypted bytes back to the Java application.

Decryption, signing, and verification follow a similar process to encryption.
