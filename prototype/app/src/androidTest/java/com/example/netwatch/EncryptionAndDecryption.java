package com.example.netwatch;

import static org.junit.Assert.assertEquals;
import org.junit.Assert;

import android.util.Log;

import org.junit.Test;

public class EncryptionAndDecryption {
    public static final String KEYNAME = "key123";
    public static final String ANDROID_KEYSTORE = "AndroidKeyStore";
    static{
            System.loadLibrary("prototype_rust_wrapper");
            Log.e("","TESTTT");
        }
    @Test
    public void testCalculate() {
        assertEquals(3, 3);
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
    public void encryption_and_decryption_random_AES_1()
    {
        RustNetwatch.generateNewKey("2324","AES");
        byte[] expectedResult = generateRandomBytes(20);
        byte[] encryptedValue = RustNetwatch.encrypt("2324",expectedResult,"AES");
        byte[] result = RustNetwatch.decrypt("2324",encryptedValue,"AES");
        Assert.assertArrayEquals( expectedResult, result);
    }



    //The LIMIT is 53
    @Test
    public void encryption_and_decryption_limit_AES_1()
    {
        RustNetwatch.generateNewKey("2325","AES");
        byte[] expectedResult = generateRandomBytes(53);
        byte[] encryptedValue = RustNetwatch.encrypt("2325",expectedResult,"AES");
        byte[] result = RustNetwatch.decrypt("2325",encryptedValue,"AES");
        Assert.assertArrayEquals( expectedResult, result);
    }

    @Test(expected = Exception.class)
    public void encryption_and_decryption_limit_AES_2()
    {
        RustNetwatch.generateNewKey("2325","AES");
        byte[] expectedResult = generateRandomBytes(54);
        byte[] encryptedValue = RustNetwatch.encrypt("2325",expectedResult,"AES");
        byte[] result = RustNetwatch.decrypt("2325",encryptedValue,"AES");
        Assert.assertArrayEquals( expectedResult, result);
    }







    @Test(expected = OutOfMemoryError.class)
    public void encryption_and_decryption_random_AES_6()
    {
        RustNetwatch.generateNewKey("2324","AES");
        byte[] expectedResult = generateRandomBytes(10000000);
        byte[] encryptedValue = RustNetwatch.encrypt("2324",expectedResult,"AES");
        byte[] result = RustNetwatch.decrypt("2324",encryptedValue,"AES");
        Assert.assertArrayEquals( expectedResult, result);
    }



    @Test(expected = OutOfMemoryError.class)
    public void encryption_and_decryption_random_AES_8()
    {
        RustNetwatch.generateNewKey("2324","AES");
        byte[] expectedResult = generateRandomBytes(1000000000);
        byte[] encryptedValue = RustNetwatch.encrypt("2324",expectedResult,"AES");
        byte[] result = RustNetwatch.decrypt("2324",encryptedValue,"AES");
        Assert.assertArrayEquals( expectedResult, result);
    }



    @Test
    public void encryption_and_decryption_empty_AES_1()
    {
        RustNetwatch.generateNewKey("2322","AES");
        byte[] expectedResult = {};
        byte[] encryptedValue = RustNetwatch.encrypt("2322",expectedResult,"AES");
        byte[] result = RustNetwatch.decrypt("2322",encryptedValue,"Aes");
        Assert.assertNotEquals(expectedResult,result);
    }
    @Test(expected = Exception.class)
    public void encryption_and_decryption_empty_Aes_2()
    {
        RustNetwatch.generateNewKey("2322","AES");
        byte[] expectedResult = {'5','b','1','D','E','z','2','G','y','7'};
        byte[] encryptedValue = RustNetwatch.encrypt("2322",expectedResult,"AES");
        byte[] emptyValue = {};
        byte[] result = RustNetwatch.decrypt("2322",emptyValue,"AES");
        Assert.assertNotEquals(expectedResult,result);
    }

/*
    @Test(expected = Exception.class)
    public void encryption_and_decryption_null_AES_2()
    {
        RustNetwatch.generateNewKey("2322","AES");
        byte[] expectedResult = {'5','b','1','D','E','z','2','G','y','7'};
        byte[] encryptedValue = RustNetwatch.encrypt("2322",expectedResult,"AES");
        byte[] result = RustNetwatch.decrypt("2322",null,"AES");
    }
*/

    //Please don't say anything about this test, it's quite obvious.. this character § is forbidden....So I had to test out all allowed characters
    @Test
    public void encryption_and_decryption_not_allowed_bytes_AES()
    {
        RustNetwatch.generateNewKey("2322","AES");
        byte[] expectedResult = {'%','!','"','=','/','?','`','*','+','~','$','&',};
        byte[] encryptedValue = RustNetwatch.encrypt("2322",expectedResult,"AES");
        byte[] result = RustNetwatch.decrypt("2322",encryptedValue,"AES");
        Assert.assertNotEquals(expectedResult,result);
    }

//Zufallsgenerator für unterschiedliche längen.
//Metadata abrufen => Liste von allen ALgorithmen durchgehen. (LATER)
//





    @Test
    public void encryption_and_decryption_random_1()
    {
        RustNetwatch.generateNewKey("2324","RSA");
        byte[] expectedResult = generateRandomBytes(20);
        byte[] encryptedValue = RustNetwatch.encrypt("2324",expectedResult,"RSA");
        byte[] result = RustNetwatch.decrypt("2324",encryptedValue,"RSA");
        Assert.assertArrayEquals( expectedResult, result);
    }



    //The LIMIT is 53
    @Test
    public void encryption_and_decryption_limit_1()
    {
        RustNetwatch.generateNewKey("2325","RSA");
        byte[] expectedResult = generateRandomBytes(53);
        byte[] encryptedValue = RustNetwatch.encrypt("2325",expectedResult,"RSA");
        byte[] result = RustNetwatch.decrypt("2325",encryptedValue,"RSA");
        Assert.assertArrayEquals( expectedResult, result);
    }

    @Test(expected = Exception.class)
    public void encryption_and_decryption_limit_2()
    {
        RustNetwatch.generateNewKey("2325","RSA");
        byte[] expectedResult = generateRandomBytes(54);
        byte[] encryptedValue = RustNetwatch.encrypt("2325",expectedResult,"RSA");
        byte[] result = RustNetwatch.decrypt("2325",encryptedValue,"RSA");
        Assert.assertArrayEquals( expectedResult, result);
    }


    @Test(expected = Exception.class)
    public void encryption_and_decryption_random_4()
    {
        RustNetwatch.generateNewKey("2326","RSA");
        byte[] expectedResult = generateRandomBytes(100000);
        byte[] encryptedValue = RustNetwatch.encrypt("2326",expectedResult,"RSA");
        byte[] result = RustNetwatch.decrypt("2326",encryptedValue,"RSA");
        Assert.assertArrayEquals( expectedResult, result);
    }

    @Test(expected = Exception.class)
    public void encryption_and_decryption_random_5()
    {
        RustNetwatch.generateNewKey("2324","RSA");
        byte[] expectedResult = generateRandomBytes(1000000);
        byte[] encryptedValue = RustNetwatch.encrypt("2324",expectedResult,"RSA");
        byte[] result = RustNetwatch.decrypt("2324",encryptedValue,"RSA");
        Assert.assertArrayEquals( expectedResult, result);
    }


    @Test(expected = Exception.class)
    public void encryption_and_decryption_random_6()
    {
        RustNetwatch.generateNewKey("2324","RSA");
        byte[] expectedResult = generateRandomBytes(10000000);
        byte[] encryptedValue = RustNetwatch.encrypt("2324",expectedResult,"RSA");
        byte[] result = RustNetwatch.decrypt("2324",encryptedValue,"RSA");
        Assert.assertArrayEquals( expectedResult, result);
    }


    @Test(expected = Exception.class)
    public void encryption_and_decryption_random_7()
    {
        RustNetwatch.generateNewKey("2324","RSA");
        byte[] expectedResult = generateRandomBytes(100000000);
        byte[] encryptedValue = RustNetwatch.encrypt("2324",expectedResult,"RSA");
        byte[] result = RustNetwatch.decrypt("2324",encryptedValue,"RSA");
        Assert.assertArrayEquals( expectedResult, result);
    }

    @Test(expected = OutOfMemoryError.class)
    public void encryption_and_decryption_random_8()
    {
        RustNetwatch.generateNewKey("2324","RSA");
        byte[] expectedResult = generateRandomBytes(1000000000);
        byte[] encryptedValue = RustNetwatch.encrypt("2324",expectedResult,"RSA");
        byte[] result = RustNetwatch.decrypt("2324",encryptedValue,"RSA");
        Assert.assertArrayEquals( expectedResult, result);
    }



    @Test
    public void encryption_and_decryption_empty_1()
    {
        RustNetwatch.generateNewKey("2322","RSA");
        byte[] expectedResult = {};
        byte[] encryptedValue = RustNetwatch.encrypt("2322",expectedResult,"RSA");
        byte[] result = RustNetwatch.decrypt("2322",encryptedValue,"RSA");
        Assert.assertNotEquals(expectedResult,result);
    }
    @Test(expected = Exception.class)
    public void encryption_and_decryption_empty_2()
    {
        RustNetwatch.generateNewKey("2322","RSA");
        byte[] expectedResult = {'5','b','1','D','E','z','2','G','y','7'};
        byte[] encryptedValue = RustNetwatch.encrypt("2322",expectedResult,"RSA");
        byte[] emptyValue = {};
        byte[] result = RustNetwatch.decrypt("2322",emptyValue,"RSA");
        Assert.assertNotEquals(expectedResult,result);
    }


    @Test(expected = Exception.class)
    public void encryption_and_decryption_null_2()
    {
        RustNetwatch.generateNewKey("2322","RSA");
        byte[] expectedResult = {'5','b','1','D','E','z','2','G','y','7'};
        byte[] encryptedValue = RustNetwatch.encrypt("2322",expectedResult,"RSA");
        byte[] result = RustNetwatch.decrypt("2322",null,"RSA");
    }


    //Please don't say anything about this test, it's quite obvious.. this character § is forbidden....So I had to test out all allowed characters
    @Test(expected = Exception.class)
    public void encryption_and_decryption_not_allowed_bytes()
    {
        RustNetwatch.generateNewKey("2322","RSA");
        byte[] expectedResult = {'%','!','"','=','/','?','`','*','+','~','$','&',};
        byte[] encryptedValue = RustNetwatch.encrypt("2322",expectedResult,"RSA");
        byte[] result = RustNetwatch.decrypt("2322",encryptedValue,"RSA");
        Assert.assertNotEquals(expectedResult,result);
    }

//Zufallsgenerator für unterschiedliche längen.
//Metadata abrufen => Liste von allen ALgorithmen durchgehen. (LATER)
//



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



}
