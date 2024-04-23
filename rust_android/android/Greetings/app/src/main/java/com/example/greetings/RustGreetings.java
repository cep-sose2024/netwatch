package com.example.greetings;

public class RustGreetings {

    private static native String greeting(final String pattern);
    public static native void generateNewKey();
    public static native byte[] encrypt(final byte[] bytes);
    public static native byte[] decrypt(final byte[] bytes);

    public String sayHello(String to) {
        return greeting(to);
    }
}

