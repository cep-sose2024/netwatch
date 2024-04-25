package com.example.greetings;

import android.security.keystore.KeyGenParameterSpec;
import android.security.keystore.KeyProperties;
import android.util.Base64;
import android.util.Log;

import java.nio.charset.StandardCharsets;
import java.security.Key;
import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.KeyStore;
import java.security.NoSuchAlgorithmException;
import java.security.PrivateKey;
import java.security.PublicKey;
import java.security.SecureRandom;
import java.security.spec.AlgorithmParameterSpec;
import java.security.spec.ECGenParameterSpec;
import java.security.spec.RSAKeyGenParameterSpec;

import javax.crypto.Cipher;

public class CryptoLayer {
    public static final String ANDROID_KEYSTORE = "AndroidKeyStore";
    public static final String KEYNAME = "key123";

    public static native String genKeyInRust(String algorithm, String provider);

    public static void generateNewKey() throws Exception{
            String  gen1 = genKeyInRust(KeyProperties.KEY_ALGORITHM_RSA, ANDROID_KEYSTORE);
            Log.d("KeyPairGenerator_RUST", gen1);

            KeyPairGenerator gen = KeyPairGenerator.getInstance(KeyProperties.KEY_ALGORITHM_RSA, ANDROID_KEYSTORE);
            Log.d("KeyPairGenerator_RUST", gen.toString());

            KeyGenParameterSpec spec = new KeyGenParameterSpec.Builder(
                KEYNAME,
                KeyProperties.PURPOSE_ENCRYPT | KeyProperties.PURPOSE_DECRYPT)
                    .setDigests(KeyProperties.DIGEST_SHA256, KeyProperties.DIGEST_SHA512)
                    .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_RSA_PKCS1)
                    .build();

            gen.initialize(spec, new SecureRandom());
            gen.generateKeyPair();
    }

    public static String encryptText(String text) throws Exception {
            KeyStore keyStore = KeyStore.getInstance(ANDROID_KEYSTORE);
            keyStore.load(null);
            PrivateKey privateKey = (PrivateKey) keyStore.getKey(KEYNAME, null);
            PublicKey publicKey = keyStore.getCertificate(KEYNAME).getPublicKey();

            Cipher cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
            cipher.init(Cipher.ENCRYPT_MODE, publicKey);

            byte[] encrypted = cipher.doFinal(text.getBytes());
            return Base64.encodeToString(encrypted, Base64.URL_SAFE);

    }

    public static String decryptText(String text) throws Exception {
        byte [] encrypted = Base64.decode(text, Base64.URL_SAFE);

        KeyStore keyStore = KeyStore.getInstance(ANDROID_KEYSTORE);
        keyStore.load(null);
        PrivateKey privateKey = (PrivateKey) keyStore.getKey(KEYNAME, null);
        PublicKey publicKey = keyStore.getCertificate(KEYNAME).getPublicKey();

        Cipher cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
        cipher.init(Cipher.DECRYPT_MODE, privateKey);

        byte[] decrypted = cipher.doFinal(encrypted);
        return new String(decrypted, StandardCharsets.UTF_8);

    }
}
