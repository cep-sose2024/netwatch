package com.example.netwatch;

public class RustNetwatch {
    public static native String[] getCapabilities();
    public static native void generateNewKey(String key_id, String algo);
    public static native byte[] encrypt(String key_id, final byte[] bytes, String algo);
    public static native byte[] decrypt(String key_id, final byte[] bytes, String algo);
    public static native byte[] sign(String key_id, final byte[] bytes, String algo);
    public static native boolean verify(String key_id, byte[] data, byte[] signature, String algo);
}

