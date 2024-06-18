---
icon: fas fa-flask
title: "Testing"
date: 2024-06-14 00:00:00 +0800
categories: [Test]
---

## Introduction


Testing has always been a very important topic to determine the functionality of the methods and to narrow it down to create a perfect implementation that fits the description and requirements that the company wants.

The company "J&S" had its own test. However, the tests didn't work in our favor or rather didn't work entirely. So we had to create our own tests to create an understanding/depiction for the reader and programmer of how the code works.

Testing is an important part of software development since it ensures that implemented functionality meet established specifications and client needs. 
This is especially clear in the instance of the contracted company J&S, whose existing testing suite proved insufficient for the project's requirements. As a result, we were obligated to create our own tests  that would provide a thorough explanation of the code's behavior to both the reader and the programmer.


Overall, we use White-box testing methods to test.
which consists of looking at the code, inspecting it, anitcipating the behaviour and then writing the test cases correspondingly
![Poster](assets/img/logo/whiteboxTesting.png)

Our code's reasonable size enables us to adopt an extensive testing strategy. We attempt to test all functions that lend themselves to testing, using a variety of approaches to provide a high level of test coverage.

## What Does Need Testing

Hardware Verification: The initial focus is on verifying the functionality of the Secure Element (SE) or Chip.This includes validating the presence of the necessary hardware and version, notably the Titan Chip M found in Google Pixel 6 and 7 devices. Without a functional hardware foundation, software testing is impossible.

Software Testing: Once the hardware has been verified, we can proceed to test the software components. Our software is made up of seven main techniques, some of which depend on one another, such as encryption/decryption and signature/key creation/loading.

## How Do We Test

**Our testing strategy utilizes two approaches:**

1. **Unit Testing:** This method utilizes Rust unit tests to verify the functionality of individual software components in isolation. These tests are executed directly on our Android devices for a realistic testing environment.



## What do we Test

**Our tests focus on  evaluating the functionalities related to the Secure Element (SE) and Trusted Execution Environment (TEE) within the device.** This includes:

- **Option and Parameter Handling:** We thoroughly test the device's ability to handle various configuration options and parameters for relevant functions.
- **Supported Enums:** We ensure the system correctly recognizes and processes all expected enum values.
- **Supported Hashing Algorithms:** We verify that the device supports and operates with the intended hashing algorithms.
- **Encryption/Decryption Methods and Key Lengths:** We test the functionality of different encryption and decryption methods offered by the device, along with their compatibility with various key bit lengths.
- **Digital Signature Creation and Verification:** We test the signing functionalities, ensuring signatures are created correctly using existing Keystore API methods. Additionally, we verify the system's ability to validate the authenticity of signatures using the Keystore API.
