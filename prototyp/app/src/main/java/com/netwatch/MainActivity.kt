package com.netwatch

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.wrapContentHeight
import androidx.compose.foundation.layout.wrapContentWidth
import androidx.compose.material3.BasicAlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
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
import androidx.compose.ui.unit.dp
import com.netwatch.ui.theme.Theme
import kotlin.concurrent.thread


class MainActivity : ComponentActivity() {

    init {
        Log.i("init", "loading lib")
        System.loadLibrary("prototype_rust_wrapper")
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        try {
            setContent {
                Theme {
                    // A surface container using the 'background' color from the theme
                    Surface(
                        modifier = Modifier.fillMaxSize(),
                        color = MaterialTheme.colorScheme.background
                    ) {
                        EncryptTest()
                    }
                }
            }
        } catch (e: Exception) {
            Log.e("setContent", "Exception: " + e.message)
        }

    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun EncryptTest() {
    var text by remember { mutableStateOf("Hello World") }
    var encText by remember { mutableStateOf("") }
    var signatureText by remember { mutableStateOf("") }
    var verificationStatus by remember { mutableStateOf("") }
    var showAlert by remember { mutableStateOf(false) }
    var exceptionName by remember { mutableStateOf("") }

    val alert = { e: Exception ->
        showAlert = true
        exceptionName = e.toString()
    }

    Column {
        Button(onClick = {
            try {
                thread {
                    CryptoLayerRust.generateRSAKey()
                }
            } catch (e: Exception) { alert(e) } })
        {
            Text("Generate Encryption Key")
        }
        TextField(
            value = text,
            onValueChange = { text = it },
            label = { Text("Message") }
        )
        Button(onClick = {
            try {
                encText = CryptoLayerRust.encryptText(text)
            } catch (e: Exception) {
                alert(e)
            }
        }) {
            Text("Encrypt")
        }
        TextField(
            value = encText,
            onValueChange = {},
            enabled = false,
        )
        Button(onClick = {
            try {
                val dec = CryptoLayerRust.decryptText(encText)
                encText = dec
            } catch (e: Exception) {
                alert(e)
            }
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
            try {
                signatureText = CryptoLayerRust.signText(encText)
            } catch (e: Exception) {
                alert(e)
            }
        }) {
            Text("Sign")
        }
        Button(onClick = {
            try {
                val verified = CryptoLayerRust.verifyText(encText, signatureText)
                verificationStatus = if (verified) "Verified" else "Not Verified"
            } catch (e: Exception) {
                alert(e)
            }
        }) {
            Text("Verify Signature")
        }
        Text(text = verificationStatus)
        Button(onClick = { try {
            CryptoLayerRust.generateECKey()
        } catch (e: Exception) {
            alert(e)
        }
        }) {
            Text("Generate Signing Key")
        }
    }

    when {
        showAlert -> {
            BasicAlertDialog(onDismissRequest = { showAlert = false },
                content = {
                    Surface(
                        modifier = Modifier
                            .wrapContentWidth()
                            .wrapContentHeight(),
                        shape = MaterialTheme.shapes.large
                    ) {
                        Column(modifier = Modifier.padding(16.dp)) {
                            Text(text = exceptionName)
                        }
                    }
                })
        }
    }
}