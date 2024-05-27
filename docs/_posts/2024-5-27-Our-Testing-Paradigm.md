
Testing has always been a very important topic to determine the functionality of the methods and to narrow it down to create a perfect implementation that fits the description and requirements that the company wants.

The company "J&S" had its own test. However, they didn't work in our favor or rather didn't work entirely. So we had to create our own tests to create an understanding/depiction for the reader and programmer of how the code works.

Overall, we use Whitebox testing methods to test.
![[../assets/img/whiteboxTesting.png]]

Our concept of testing is quite simple to follow. We test every "testable" function, because we don't have many of them, we can test them with a variety of methods.


## What Does Need Testing


We will work our way from up to down.  Foremost, we need to test if the SE(Secure Element) or the Chip works, after that we will take a look at our TEE.

Hardware testing is very important, because without it, we can't get any tests to run.

If the hardware is there, and it works, we can start testing the software.

Our Software consists of 7 different main methods, some of them are based on each other, like Encryption and Decryption or creating and loading Signatures/Keys.

  

The software is cut into different sections:

1. Key creation and key loading

2. Signing data. 

3. Encrypting and Decryption data.

4. Verify Signature

## How Do We Test
We have two methods for testing. 

The first basic method:

We use Rust Unit tests for basic testing and then run them on our Android.

  
The second method:

We use Android Emulator as a GitHub Action to run the tests frequently. This way, we can better investigate which commit, change or code that made specific tests fail.



## What do we Test
We test everything related to the code or even the device. Here is a list:

1. option and parameters. 

2. Enums that are supported.

3. Supported Hashes. 

4. Encryption/Decryption Methods.

5. Encryption Bit-lengths.

6. Creating Signatures.

7. Verifying Signatures.

Therefore, we test the SE(Secure Element) and TEE(Trusted Execution Environment). 
  
Specifically, we test the Key Creation method to create a key and load a key onto the device.

Second of all, we test the Signing data functions to create a Signature using the existing methods of the Keystore API. We  test the Encryption and Decryption data for the intended encryption and decryption methods. Lastly, we verify if the Signature uses the Keystore API to validate the existence of the Signature.



