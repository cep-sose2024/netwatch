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

import javax.crypto.Cipher;

public class CryptoLayerRust {

    public static void generateNewKey() {
       RustGreetings.generateNewKey();
    }

    public static String encryptText(String text) throws Exception {
        byte[] encrypted = RustGreetings.encrypt(text.getBytes());

        Log.i("CryptoLayer", "encryped array: " + encrypted.toString());

        return Base64.encodeToString(encrypted, Base64.URL_SAFE);
    }

    public static String decryptText(String text) throws Exception {
        byte [] encrypted = Base64.decode(text, Base64.URL_SAFE);
        byte[] decrypted = RustGreetings.decrypt(encrypted);
        Log.i("CryptoLayer", "decryped array: " + encrypted.toString());
        Log.i("CryptoLayer", "encryped array text: " + new String(encrypted, StandardCharsets.UTF_8));
        return new String(decrypted, StandardCharsets.UTF_8);

    }

}
