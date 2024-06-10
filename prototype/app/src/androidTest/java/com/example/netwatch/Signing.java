package com.example.netwatch;

import static org.junit.Assert.assertEquals;

import android.util.Log;

import org.junit.Assert;
import org.junit.Test;

public class Signing {
    public static final String KEYNAME = "key123";
    public static final String ANDROID_KEYSTORE = "AndroidKeyStore";
    static{
        System.loadLibrary("prototype_rust_wrapper");
        Log.e("","TESTTT");
    }
    public byte[] generateRandomBytes(int length)
    {
        byte[] definedBytes = { '0','1','2','3','4','5','6','7','8','9',
                'a','b','c','d','e','f','g','h','i','j','k','l','m','o','p','q','r','s','t','u','v','w','x','y','z',
                'A','B','C','D','E','F','G','H','I','J','K','L','M','O','P','Q','R','S','T','U','V','W','X','Y','Z'};
        byte[] result = new byte[length];
        for(int i = 0;i<length;i++) {
            int rand = (int)(Math.random() * 58) + 1;
            result[i] = definedBytes[rand];
        }

        return result;
    }

    @Test
    public void signing_Test_1()
    {
        RustNetwatch.generateNewKey("2322","RSA");
        byte[] expectedResult = generateRandomBytes(10);
        byte[] result = RustNetwatch.sign("2322",expectedResult,"RSA");
        boolean x = RustNetwatch.verify("2322",expectedResult,result,"RSA");
        Assert.assertTrue(x);

    }

    @Test
    public void signing_Test_2()
    {
        RustNetwatch.generateNewKey("2322","RSA");
        byte[] expectedResult = generateRandomBytes(53);
        byte[] result = RustNetwatch.sign("2322",expectedResult,"RSA");
        boolean x = RustNetwatch.verify("2322",expectedResult,result,"RSA");
        Assert.assertTrue(x);

    }

    @Test(expected = Exception.class)
    public void signing_Test_3()
    {
        RustNetwatch.generateNewKey("2322","RSA");
        byte[] expectedResult = generateRandomBytes(54);
        byte[] result = RustNetwatch.sign("2322",expectedResult,"RSA");
        boolean x = RustNetwatch.verify("2322",expectedResult,result,"RSA");
        Assert.assertTrue(x);

    }

    @Test(expected = OutOfMemoryError.class)
    public void signing_Test_4()
    {
        RustNetwatch.generateNewKey("2322","RSA");
        byte[] expectedResult = generateRandomBytes(100000000);
        byte[] result = RustNetwatch.sign("2322",expectedResult,"RSA");
        boolean x = RustNetwatch.verify("2322",expectedResult,result,"RSA");
        Assert.assertTrue(x);

    }
    @Test(expected = OutOfMemoryError.class)
    public void signing_Test_5()
    {
        RustNetwatch.generateNewKey("2322","RSA");
        byte[] expectedResult = generateRandomBytes(0);
        byte[] result = RustNetwatch.sign("2322",expectedResult,"RSA");
        boolean x = RustNetwatch.verify("2322",expectedResult,result,"RSA");
        Assert.assertTrue(x);

    }



    @Test
    public void signing_Test_AES_1()
    {
        RustNetwatch.generateNewKey("2322","AES");
        byte[] expectedResult = generateRandomBytes(10);
        byte[] result = RustNetwatch.sign("2322",expectedResult,"AES");
        boolean x = RustNetwatch.verify("2322",expectedResult,result,"AES");
        Assert.assertTrue(x);

    }

    @Test
    public void signing_Test_AES_2()
    {
        RustNetwatch.generateNewKey("2322","AES");
        byte[] expectedResult = generateRandomBytes(53);
        byte[] result = RustNetwatch.sign("2322",expectedResult,"AES");
        boolean x = RustNetwatch.verify("2322",expectedResult,result,"AES");
        Assert.assertTrue(x);

    }

    @Test(expected = Exception.class)
    public void signing_Test_AES_3()
    {
        RustNetwatch.generateNewKey("2322","AES");
        byte[] expectedResult = generateRandomBytes(54);
        byte[] result = RustNetwatch.sign("2322",expectedResult,"AES");
        boolean x = RustNetwatch.verify("2322",expectedResult,result,"AES");
        Assert.assertTrue(x);

    }

    @Test(expected = OutOfMemoryError.class)
    public void signing_Test_AES_4()
    {
        RustNetwatch.generateNewKey("2322","AES");
        byte[] expectedResult = generateRandomBytes(100000000);
        byte[] result = RustNetwatch.sign("2322",expectedResult,"AES");
        boolean x = RustNetwatch.verify("2322",expectedResult,result,"AES");
        Assert.assertTrue(x);

    }
    @Test(expected = OutOfMemoryError.class)
    public void signing_Test_AES_5()
    {
        RustNetwatch.generateNewKey("2322","AES");
        byte[] expectedResult = generateRandomBytes(0);
        byte[] result = RustNetwatch.sign("2322",expectedResult,"AES");
        boolean x = RustNetwatch.verify("2322",expectedResult,result,"AES");
        Assert.assertTrue(x);

    }



}
