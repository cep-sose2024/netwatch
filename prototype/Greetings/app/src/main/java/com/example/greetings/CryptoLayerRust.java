package com.example.greetings;

import android.security.keystore.KeyGenParameterSpec;
import android.security.keystore.KeyProperties;
import android.util.Base64;
import android.util.Log;

import java.nio.charset.StandardCharsets;
import java.security.KeyPairGenerator;
import java.security.KeyStore;
import java.security.PrivateKey;
import java.security.PublicKey;
import java.security.SecureRandom;
import java.util.Arrays;

import javax.crypto.Cipher;

public class CryptoLayerRust {

    static final String ec_key = "KEY_EC";
    static final String rsa_key = "KEY_RSA";

    public static void generateRSAKey() {
        RustGreetings.generateNewKey(rsa_key, "RSA");
    }

    public static void generateECKey() {
        Log.d("CryptoLayerRust", "generating ec key");
        RustGreetings.generateNewKey(ec_key, "EC");
    }

    public static String signText(String text) throws Exception {
        byte[] encrypted = RustGreetings.sign(ec_key, text.getBytes());

        Log.i("CryptoLayer", "signature: " + Arrays.toString(encrypted));

        return Base64.encodeToString(encrypted, Base64.URL_SAFE);
    }

    public static boolean verifyText(String text, String signature) throws Exception {
        byte[] signatureBytes = Base64.decode(signature, Base64.URL_SAFE);

        boolean verified = RustGreetings.verify(ec_key, text.getBytes(), signatureBytes);

        Log.i("CryptoLayer", "verified: " + verified);

        return verified;
    }

    public static String encryptText(String text) throws Exception {
        byte[] encrypted = RustGreetings.encrypt(rsa_key, text.getBytes());

        Log.i("CryptoLayer", "encrypted array: " + Arrays.toString(encrypted));

        return Base64.encodeToString(encrypted, Base64.URL_SAFE);
    }

    public static String decryptText(String text) throws Exception {
        byte[] encrypted = Base64.decode(text, Base64.URL_SAFE);
        byte[] decrypted = RustGreetings.decrypt(rsa_key, encrypted);
        Log.i("CryptoLayer", "decrypted array: " + Arrays.toString(encrypted));
//        Log.i("CryptoLayer", "encrypted array text: " + new String(encrypted, StandardCharsets.UTF_8));
        return new String(decrypted, StandardCharsets.UTF_8);
    }
}
