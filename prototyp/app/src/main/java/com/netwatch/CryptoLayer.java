package com.netwatch;

import android.security.keystore.KeyGenParameterSpec;
import android.security.keystore.KeyProperties;
import android.util.Base64;
import android.util.Log;

import java.nio.charset.StandardCharsets;
import java.security.KeyPairGenerator;
import java.security.KeyStore;
import java.security.PrivateKey;
import java.security.PublicKey;
import java.security.Signature;

import javax.crypto.Cipher;

public class CryptoLayer {
    public static final String ANDROID_KEYSTORE = "AndroidKeyStore";
    public static final String KEYNAME = "key123";

    public static native String generateNewKeyRust(String keyName, String algorithm, String provider, int purposes);

    public static native String encryptTextRust(String text);

    public static native String decryptTextRust(String text);

    public static native byte[] signDataRust(String data, String keyName);

    public static native boolean verifyDataRust(String data, byte[] signature, String keyName);

    public static void generateNewKey(String keyName) throws Exception {
        KeyPairGenerator gen = KeyPairGenerator.getInstance(KeyProperties.KEY_ALGORITHM_RSA, ANDROID_KEYSTORE);

        KeyGenParameterSpec spec = new KeyGenParameterSpec.Builder(
                keyName,
                KeyProperties.PURPOSE_ENCRYPT | KeyProperties.PURPOSE_DECRYPT)
                .setDigests(KeyProperties.DIGEST_SHA256, KeyProperties.DIGEST_SHA512)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_RSA_PKCS1)
                .build();

        gen.initialize(spec);
        gen.generateKeyPair();
    }

    public static void generateNewKeyForSigning(String keyName) throws Exception {
        KeyPairGenerator gen = KeyPairGenerator.getInstance(KeyProperties.KEY_ALGORITHM_RSA, ANDROID_KEYSTORE);

        KeyGenParameterSpec spec = new KeyGenParameterSpec.Builder(
                keyName,
                KeyProperties.PURPOSE_SIGN | KeyProperties.PURPOSE_VERIFY)
                .setDigests(KeyProperties.DIGEST_SHA256, KeyProperties.DIGEST_SHA512)
                .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_RSA_PKCS1)
                .setSignaturePaddings(KeyProperties.SIGNATURE_PADDING_RSA_PKCS1)
                .build();

        gen.initialize(spec);
        gen.generateKeyPair();
    }


    public static String encryptText(String text) throws Exception {
        KeyStore keyStore = KeyStore.getInstance(ANDROID_KEYSTORE);
        keyStore.load(null);

        PrivateKey privateKey = (PrivateKey) keyStore.getKey(KEYNAME, null);
        PublicKey publicKey = keyStore.getCertificate(KEYNAME).getPublicKey();

        Cipher cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
        byte[] encrypted;
        cipher.init(Cipher.ENCRYPT_MODE, publicKey);
        encrypted = cipher.doFinal(text.getBytes());

        return Base64.encodeToString(encrypted, Base64.URL_SAFE);
    }

    public static String decryptText(String text) throws Exception {
        Log.d("TAG", "decryptText: " + text);
        byte[] encrypted;
        encrypted = Base64.decode(text, Base64.URL_SAFE);

        KeyStore keyStore = KeyStore.getInstance(ANDROID_KEYSTORE);
        keyStore.load(null);
        PrivateKey privateKey = (PrivateKey) keyStore.getKey(KEYNAME, null);
        PublicKey publicKey = keyStore.getCertificate(KEYNAME).getPublicKey();

        Cipher cipher = Cipher.getInstance("RSA/ECB/PKCS1Padding");
        cipher.init(Cipher.DECRYPT_MODE, privateKey);

        byte[] decrypted = cipher.doFinal(encrypted);
        return new String(decrypted, StandardCharsets.UTF_8);
    }

    /*
     * Use a PrivateKey in the KeyStore to create a signature over
     * some data.
     */
    public static byte[] signData(String data, String keyName) throws Exception {
        KeyStore ks = KeyStore.getInstance(ANDROID_KEYSTORE);
        ks.load(null);

        Signature s = Signature.getInstance("SHA256withRSA");
        s.initSign((PrivateKey) ks.getKey(keyName, null));
        s.update(data.getBytes());
        return s.sign();
    }

    /*
     * Verify a signature previously made by a private key in the
     * KeyStore. This uses the X.509 certificate attached to the
     * private key in the KeyStore to validate a previously
     * generated signature.
     */
    public static boolean verifyData(String data, byte[] signature, String keyName) throws Exception {
        KeyStore ks = KeyStore.getInstance(ANDROID_KEYSTORE);
        ks.load(null);

        Signature s = Signature.getInstance("SHA256withRSA");
        s.initVerify(ks.getCertificate(keyName));
        s.update(data.getBytes());
        return s.verify(signature);
    }
}
