package com.example.netwatch;

public class RustNetwatch {
    public static native void generateNewKey(String key_id, String algo);
    public static native byte[] encrypt(String key_id, final byte[] bytes);
    public static native byte[] decrypt(String key_id, final byte[] bytes);
    public static native byte[] sign(String key_id, final byte[] bytes);
    public static native boolean verify(String key_id, byte[] data, byte[] signature);
}

