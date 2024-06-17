package com.example.netwatch;

import android.util.Base64;
import android.util.Log;

import java.nio.charset.StandardCharsets;
import java.util.Arrays;

public class CryptoLayerRust {

    static final String ec_key = "KEY_EC";
    static final String rsa_key = "KEY_RSA";
    static final String aes_key = "KEY_AES";

    public static void generateAESKey() {
        RustNetwatch.generateNewKey(aes_key, "AES");
    }

    public static void generateRSAKey() {
        RustNetwatch.generateNewKey(rsa_key, "RSA");
    }

    public static void generateKey(String algorithm) {
        RustNetwatch.generateNewKey("key" + algorithm, algorithm);
    }

    public static void generateECKey() {
        Log.d("CryptoLayerRust", "generating ec key");
        RustNetwatch.generateNewKey(ec_key, "EC");
    }

    public static String signText(String text, String algorithm) throws Exception {
        byte[] encrypted = RustNetwatch.sign("key" + algorithm, text.getBytes(), algorithm);

        Log.i("CryptoLayer", "signature: " + Arrays.toString(encrypted));

        return Base64.encodeToString(encrypted, Base64.URL_SAFE);
    }

    public static boolean verifyText(String text, String signature, String algorithm) throws Exception {
        byte[] signatureBytes = Base64.decode(signature, Base64.URL_SAFE);

        boolean verified = RustNetwatch.verify("key" + algorithm, text.getBytes(), signatureBytes, algorithm);

        Log.i("CryptoLayer", "verified: " + verified);

        return verified;
    }

    public static byte[] encryptFile(byte[] file, String algorithm) {
        return RustNetwatch.encrypt("key" + algorithm, file, algorithm);
    }

    public static byte[] decryptFile(byte[] file, String algorithm) {
        return RustNetwatch.decrypt("key" + algorithm, file, algorithm);
    }

    public static String encryptText(String text, String algorithm) throws Exception {
        byte[] encrypted = RustNetwatch.encrypt("key" + algorithm, text.getBytes(), algorithm);

        Log.i("CryptoLayer", "encrypted array: " + Arrays.toString(encrypted));

        return Base64.encodeToString(encrypted, Base64.URL_SAFE);
    }

    public static String decryptText(String text, String algorithm) throws Exception {
        byte[] encrypted = Base64.decode(text, Base64.URL_SAFE);
        byte[] decrypted = RustNetwatch.decrypt("key" + algorithm, encrypted, algorithm);
        Log.i("CryptoLayer", "decrypted array: " + Arrays.toString(encrypted));
//        Log.i("CryptoLayer", "encrypted array text: " + new String(encrypted, StandardCharsets.UTF_8));
        return new String(decrypted, StandardCharsets.UTF_8);
    }
}
