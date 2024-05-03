---
title: "Crypto Layer on Architecture"
date: 2024-04-30 00:00:00 +0800
categories: [Architecture]
tags: [crypto layer, hsm]
mermaid: true
---

# Architecture

```mermaid
flowchart TB
    subgraph Legende
        TeamNetwatch[Team NetWatch]
        j&s-soft
        Android
    end
    subgraph CryptoAbstractionLayer [Crypto Abstraction Layer]
        AndroidProvider[Android Provider]
        KeystoreWrapper[Keystore Wrapper]
    end

    AndroidProvider --> KeystoreWrapper
    KeystoreWrapper --> KeystoreAPI
    KeystoreAPI --> HSM
       
    
    style KeystoreAPI fill:#99cc99,stroke:#333,stroke-width:2px;
    style HSM fill:#99cc99,stroke:#333,stroke-width:2px;
    style CryptoAbstractionLayer fill:#FFFF99,stroke:#333,stroke-width:2px,fontColor:#000000;
    style AndroidProvider fill:#ADD8E6;
    style KeystoreWrapper fill:#ADD8E6;
    style TeamNetwatch fill:#ADD8E6,stroke:#333,stroke-width:2px;
    style j&s-soft fill:#FFFF99,stroke:#333,stroke-width:2px;
    style Android fill:#99cc99,stroke:#333,stroke-width:2px;
```

## Crypto Abstraction Layer
The Crypto Abstraction Layer is being developed by j&s-soft GmbH and invokes methods from the Android Providers.
## Android Provider
The Android Provider furnishes the necessary functionalities required by the Crypto Abstraction Layer. Within the Android Provider, data is formatted in a way that the Keystore API can handle, and conversely, data returned by the Keystore API is reformatted for the Crypto Abstraction Layer.
## Keystore Wrapper
The Keystore Wrapper receives essential data and parameters from the Android Provider, which it then forwards to the Keystore API.
## Keystore API
The Keystore API enables communication with the HSM.
## HSM
The HSM is the hardware responsible for performing cryptographic procedures and securely storing the key.
