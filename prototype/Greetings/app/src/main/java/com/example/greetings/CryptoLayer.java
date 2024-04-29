package com.example.greetings;

import android.graphics.Point;
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
import java.security.Signature;
import java.security.spec.AlgorithmParameterSpec;
import java.security.spec.ECGenParameterSpec;
import java.security.spec.RSAKeyGenParameterSpec;
import java.util.Enumeration;

import javax.crypto.Cipher;

public class CryptoLayer {
    public static final String ANDROID_KEYSTORE = "AndroidKeyStore";
    public static final String KEYNAME = "key123";


    public static void generateNewKey() throws Exception {

        KeyPairGenerator gen = KeyPairGenerator.getInstance(KeyProperties.KEY_ALGORITHM_RSA, ANDROID_KEYSTORE);

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
        Log.d("TAG", "decryptText: " + text);
        byte[] encrypted = Base64.decode(text, Base64.URL_SAFE);

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
    public static byte[] signData(String data) throws Exception {
        KeyStore ks = KeyStore.getInstance(ANDROID_KEYSTORE);
        ks.load(null);

        KeyStore.Entry entry = ks.getEntry(KEYNAME, null);
        if (!(entry instanceof KeyStore.PrivateKeyEntry)) {
            Log.w("TAG", "Not an instance of a PrivateKeyEntry");
            return null;
        }
        Signature s = Signature.getInstance("SHA256withECDSA");
        s.initSign(((KeyStore.PrivateKeyEntry) entry).getPrivateKey());
        s.update(data.getBytes());
        return s.sign();
    }

    /*
     * Verify a signature previously made by a private key in the
     * KeyStore. This uses the X.509 certificate attached to the
     * private key in the KeyStore to validate a previously
     * generated signature.
     */
    public static boolean verifyData(String data, byte[] signature) throws Exception {
        KeyStore ks = KeyStore.getInstance(ANDROID_KEYSTORE);
        ks.load(null);

        Signature s = Signature.getInstance("SHA256withECDSA");
        s.initVerify(ks.getCertificate(KEYNAME));
        s.update(data.getBytes());
        return s.verify(signature);
    }
}
