package com.example.greetings

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import com.example.greetings.ui.theme.GreetingsTheme


class MainActivity : ComponentActivity() {

    init {
        Log.i("init", "loading lib")
        System.loadLibrary("prototype_rust_wrapper")
//        System.loadLibrary("rust_robusta")
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

//        val kpg: KeyPairGenerator = KeyPairGenerator.getInstance(
//            KeyProperties.KEY_ALGORITHM_EC,
//            "AndroidKeyStore"
//        )
//        val parameterSpec: KeyGenParameterSpec = KeyGenParameterSpec.Builder(
//            "alias",
//            KeyProperties.PURPOSE_SIGN or KeyProperties.PURPOSE_VERIFY
//        ).run {
//            setDigests(KeyProperties.DIGEST_SHA256, KeyProperties.DIGEST_SHA512)
//            build()
//        }

//        kpg.initialize(parameterSpec)
//
//        val kp = kpg.generateKeyPair()

//        RobustaAndroidExample.runRustExample(applicationContext)

//        val keyName = "key123"
//        CryptoLayer.generateNewKeyRust(
//            keyName,
//            KeyProperties.KEY_ALGORITHM_RSA,
//            CryptoLayer.ANDROID_KEYSTORE,
//            KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT
//        )

        val keySignName = "keySign123";
//        CryptoLayer.generateNewKeyRust(
//            keySignName,
//            KeyProperties.KEY_ALGORITHM_EC,
//            CryptoLayer.ANDROID_KEYSTORE,
//            KeyProperties.PURPOSE_SIGN or KeyProperties.PURPOSE_VERIFY
//        )

//        val signature = CryptoLayer.signDataRust("Hello World", keySignName)
//        Log.i("main", "Signature: $signature")
//        val verified = CryptoLayer.verifyDataRust("Hello World", signature, keySignName)
//        Log.i("main", "Verified: $verified")

//        Log.i("main", "executing RustGreetings")
//        val g = RustGreetings()
//        Log.i("main", "RustGreetings done")
//        var r = g.sayHello("world")


        setContent {
            GreetingsTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    Column {
                        Greeting(name = "j&s-soft")
                        GenerateEncryptionKeyButton()
                        EncryptTest()
                        GenerateSigningKeyButton()
                    }
                }
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "$name!",
        modifier = modifier
    )
}

@Composable
fun GenerateEncryptionKeyButton() {
    Button(onClick = { CryptoLayerRust.generateNewKey("KEY123") }) {
        Text("Generate Encryption Key")
    }
}

@Composable
fun GenerateSigningKeyButton() {
    Button(onClick = { CryptoLayerRust.generateNewKey("KEY SIGN") }) {
        Text("Generate Signing Key")
    }
}

@Composable
fun SignDataButton() {
    Button(onClick = { CryptoLayerRust.generateNewKey("KEY SIGN") }) {
        Text("Generate Signing Key")
    }
}

@Composable
fun EncryptTest() {
    var text by remember { mutableStateOf("Hello World") }
    var encText by remember { mutableStateOf("") }
    var signatureText by remember { mutableStateOf("") }
    var verificationStatus by remember { mutableStateOf("") }
    Column {
        TextField(
            value = text,
            onValueChange = { text = it },
            label = { Text("Message") }
        )
        Button(onClick = {
            encText = CryptoLayerRust.encryptText(text)
            Log.i("button", "Encrypted text: $encText")
        }) {
            Text("Encrypt")
        }
        TextField(
            value = encText,
            onValueChange = {},
            enabled = false,
        )
        Button(onClick = {
            val dec = CryptoLayerRust.decryptText(encText)
            encText = dec
        }) {
            Text("Decrypt")
        }
        TextField(
            value = signatureText,
            onValueChange = {},
            enabled = false,
            label = { Text("Signature") }
        )
        Button(onClick = {
            val signature = CryptoLayerRust.signText(encText)
            signatureText = signature
        }) {
            Text("Sign")
        }
        Button(onClick = {
            val verified = CryptoLayerRust.verifyText(encText, signatureText)
            verificationStatus = if (verified) "Verified" else "Not Verified"
        }) {
            Text("Verify Signature")
        }
        Text(text = verificationStatus)
    }
}