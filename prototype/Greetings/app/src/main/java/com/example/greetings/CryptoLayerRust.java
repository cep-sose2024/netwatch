package com.example.greetings;

import android.util.Base64;

import java.nio.charset.StandardCharsets;

public class CryptoLayerRust {

    public static void generateNewKey(String keyId) {
        RustGreetings.generateNewKey(keyId);
    }

    public static byte[] signText(String text) throws Exception {
        byte[] textBytes = Base64.decode(text, Base64.URL_SAFE);
        return RustGreetings.sign(textBytes);
    }

    public static boolean verifyText(String text, byte[] signature) throws Exception {
        byte[] textBytes = Base64.decode(text, Base64.URL_SAFE);
        return RustGreetings.verify(textBytes, signature);
    }

    public static String encryptText(String text) throws Exception {
        byte[] encrypted = RustGreetings.encrypt(text.getBytes());
        return Base64.encodeToString(encrypted, Base64.URL_SAFE);
    }

    public static String decryptText(String text) throws Exception {
        byte[] encrypted = Base64.decode(text, Base64.URL_SAFE);
        byte[] decrypted = RustGreetings.decrypt(encrypted);
        return new String(decrypted, StandardCharsets.UTF_8);
    }
}
