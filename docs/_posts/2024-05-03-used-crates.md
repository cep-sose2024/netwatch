---
title: "USed Rust Crates"
date: 2024-05-03 00:00:00 +0800
categories: [Architecture]
tags: [crypto layer, hsm]
---

# Used Rust Crates
Additionally to the Rust libraries used by the upstream Crypto Abstraction Layer by j&s-soft, we use the following 4 libraries:

## jni

Provides Java Native Interface bindings for Rust, allowing calling Java libraries from Rust and Rust functions from Java. It is needed to access the Android Keystore API and cannot be replaced.

## robusta-jni

Robusta provides a macro to generate jni bindings from function definitions without having to convert types manually. It could be replaced by manual jni bindings in the future.

## libloading

Allows dynamically loading libraries into the running process without using unsafe code. This is only needed because some native jvm functions don't get loaded on Android. It could be replaced by the std loading mechanism it wraps, which would require unsafe code.

## tracing-android

Binds the tracing logging framework used by the Crypto Abstraction Layer to the Android logcat debug output.