package com.example.netwatch;

import android.util.Log;

import org.junit.Assert;
import org.junit.Test;

import java.util.Arrays;
import java.util.Random;

public class EncryptionAndDecryption {
    static {
        System.loadLibrary("prototype_rust_wrapper");
        Log.e("", "rust_wrapper loaded");
    }

    public byte[] generateRandomBytes(int length) {
        byte[] bytes = new byte[length];
        new Random().nextBytes(bytes);

        return bytes;
    }

    @Test
    public void encryption_from_capabilities() {
        String[] caps = RustNetwatch.getCapabilities();

        Arrays.stream(caps).filter((name) -> {
            return name.startsWith("ENC");
        }).forEach((mode) -> {
            String keyName = "AndroidTest-" + mode;
            RustNetwatch.generateNewKey(keyName, mode);
            byte[] expectedResult = generateRandomBytes(20);
            byte[] encryptedValue = RustNetwatch.encrypt(keyName, expectedResult, mode);
            byte[] result = RustNetwatch.decrypt(keyName, encryptedValue, mode);
            Assert.assertArrayEquals(expectedResult, result);
        });
    }

    @Test
    public void signing_from_capabilities() {
        String[] caps = RustNetwatch.getCapabilities();

        Arrays.stream(caps)
                .filter((name) -> {
                    return name.startsWith("SIG");
                })
                .forEach((mode) -> {
                    String keyName = "AndroidTest-" + mode;
                    RustNetwatch.generateNewKey(keyName, mode);
                    byte[] expectedResult = generateRandomBytes(10);
                    byte[] result = RustNetwatch.sign(keyName, expectedResult, mode);
                    boolean x = RustNetwatch.verify(keyName, expectedResult, result, mode);
                    Assert.assertTrue(x);
                });
    }

    @Test
    public void encrypt_emtpy_array() {
        String[] caps = RustNetwatch.getCapabilities();

        Arrays.stream(caps)
                .filter((name) -> {
                    return name.startsWith("ENC");
                })
                .forEach((mode) -> {
                    String keyName = "AndroidTest-" + mode;
                    RustNetwatch.generateNewKey(keyName, mode);
                    byte[] expectedResult = {};
                    byte[] encrypted = RustNetwatch.encrypt(keyName, expectedResult, mode);
                    byte[] decrypted = RustNetwatch.decrypt(keyName, encrypted, mode);
                    Assert.assertArrayEquals(expectedResult, decrypted);
                });
    }

    @Test
    public void sign_emtpy_array() {
        String[] caps = RustNetwatch.getCapabilities();

        Arrays.stream(caps)
                .filter((name) -> {
                    return name.startsWith("SIG");
                })
                .forEach((mode) -> {
                    String keyName = "AndroidTest-" + mode;
                    RustNetwatch.generateNewKey(keyName, mode);
                    byte[] expectedResult = {};
                    byte[] result = RustNetwatch.sign(keyName, expectedResult, mode);
                    boolean x = RustNetwatch.verify(keyName, expectedResult, result, mode);
                    Assert.assertTrue(x);
                });
    }

    // Java apps have a default max heap size of 16MB, so this should still work for AES.
    // RSA on the other hand can't encrypt anything more that 53 bytes
    @Test
    public void encrypt_10MB_array() {
        String[] caps = RustNetwatch.getCapabilities();

        Arrays.stream(caps)
                .filter((name) -> {
                    return name.startsWith("ENC-AES");
                })
                .forEach((mode) -> {
                    String keyName = "AndroidTest-" + mode;
                    RustNetwatch.generateNewKey(keyName, mode);
                    byte[] expectedResult = generateRandomBytes(10_000_000);
                    byte[] encrypted = RustNetwatch.encrypt(keyName, expectedResult, mode);
                    byte[] decrypted = RustNetwatch.decrypt(keyName, encrypted, mode);
                    Assert.assertArrayEquals(expectedResult, decrypted);
                });
    }

    // This should fail because the Keystore api can't handle empty arrays as ciphertext,
    // as they have no padding
    @Test(expected = Exception.class)
    public void encryption_and_decryption_empty_2() {
        RustNetwatch.generateNewKey("2322", "RSA");
        byte[] empty = {};
        RustNetwatch.decrypt("2322", empty, "RSA");
    }


    // this does not test the crypto layer, this error should already be caught in the wrapper
    @Test(expected = Exception.class)
    public void decrypt_null() {
        RustNetwatch.generateNewKey("2322", "RSA");
        byte[] result = RustNetwatch.decrypt("2322", null, "RSA");
    }
}
