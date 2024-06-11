package com.example.netwatch

import android.app.Activity
import android.content.Intent
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.net.Uri
import android.os.Bundle
import android.util.Log
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.wrapContentHeight
import androidx.compose.foundation.layout.wrapContentWidth
import androidx.compose.material3.BasicAlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.lifecycleScope
import com.example.netwatch.ui.theme.NetWatchTheme
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import kotlin.concurrent.thread


class MainActivity : ComponentActivity() {
    private var encryptionState by mutableStateOf("image")
    private var imageUri: Uri? by mutableStateOf(null)
    private var encryptedImageUri: Uri? by mutableStateOf(null)
    private lateinit var createEncryptLauncher: ActivityResultLauncher<Intent>
    private lateinit var createDecryptLauncher: ActivityResultLauncher<Intent>
    private var progress by mutableFloatStateOf(0f)

    private suspend fun decryptImage(inputUri: Uri, outputUri: Uri) = withContext(Dispatchers.IO) {
        val startTime = System.currentTimeMillis()
        var imageSize: Int? = null

        contentResolver.openInputStream(inputUri)?.use { inputStream ->
            val byteArray = inputStream.readBytes()
            imageSize = byteArray.size
            Log.d("ImageDecryption", "Input file: $byteArray")

            val decryptedByteArray = CryptoLayerRust.decryptFile(byteArray)
            Log.d("ImageDecryption", "Decrypted file: $decryptedByteArray")

            contentResolver.openOutputStream(outputUri)?.use { outputStream ->
                val bitmap =
                    BitmapFactory.decodeByteArray(decryptedByteArray, 0, decryptedByteArray.size)
                bitmap.compress(Bitmap.CompressFormat.PNG, 100, outputStream)
            }
        }

        val endTime = System.currentTimeMillis()
        val duration = endTime - startTime

        imageSize?.let {
            Log.d("ImageDecryption", "Input file size: $it bytes")
            val sizeInMB = it / (1024.0 * 1024.0)
            val durationInSeconds = duration / 1000.0
            val speed = sizeInMB / durationInSeconds
            Log.d("ImageDecryption", "Decryption speed: $speed MB/s")
        }
        Log.i("ImageDecryption", "Image decrypted successfully in $duration ms")
        withContext(Dispatchers.Main) {
            Toast.makeText(
                this@MainActivity,
                "Image decrypted successfully in $duration ms",
                Toast.LENGTH_SHORT
            ).show()
        }
        Log.d("ImageDecryption", "Output file: $outputUri")
    }

    private suspend fun encryptImage(inputUri: Uri, outputUri: Uri) = withContext(Dispatchers.IO) {
        val startTime = System.currentTimeMillis()

        contentResolver.openInputStream(inputUri)?.use { inputStream ->
            val byteArray = inputStream.readBytes()
            val imageSize = byteArray.size

            val encryptedByteArray = CryptoLayerRust.encryptFile(byteArray)

            contentResolver.openOutputStream(outputUri)?.use { outputStream ->
                outputStream.write(encryptedByteArray)
            }

            val endTime = System.currentTimeMillis()
            val duration = endTime - startTime

            Log.d("ImageEncryption", "Input image size: $imageSize bytes")
            val sizeInMB = imageSize / (1024.0 * 1024.0)
            val durationInSeconds = duration / 1000.0
            val speed = sizeInMB / durationInSeconds
            Log.d("ImageEncryption", "Encryption speed: $speed MB/s")
            Log.i("ImageEncryption", "Image encrypted successfully in $duration ms")

            withContext(Dispatchers.Main) {
                Toast.makeText(
                    this@MainActivity,
                    "Image encrypted successfully in $duration ms",
                    Toast.LENGTH_SHORT
                ).show()
            }
            Log.d("ImageEncryption", "Output file: $outputUri")
        }
    }

    // Sony Xperia 1 IV in MB/s
    // 0.07506234690404226
    // 0.10176130368104697
    // 0.07977867229892223
    // 0.0796026996638611
    // 0.08055670324265024
    // 0.07002658860240217
    // 0.06617695868388186
    // 0.06754749488123747
    // 0.07535768545601139
    //

    private fun encryptAndSaveImage() {
        val createFileIntent = Intent(Intent.ACTION_CREATE_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "image/png"
            putExtra(Intent.EXTRA_TITLE, "encrypted")
        }

        createEncryptLauncher.launch(createFileIntent)
    }

    private fun decryptAndSaveImage() {
        val createFileIntent = Intent(Intent.ACTION_CREATE_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "image/png"
            putExtra(Intent.EXTRA_TITLE, "decrypted")
        }

        createDecryptLauncher.launch(createFileIntent)
    }

    private val pickImage =
        registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
            if (result.resultCode == Activity.RESULT_OK) {
                // The result data contains a Uri for the image
                val inputUri = result.data?.data
                imageUri = inputUri
            }
        }

    private val pickEncryptedImage =
        registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
            if (result.resultCode == Activity.RESULT_OK) {
                // The result data contains a Uri for the image
                val inputUri = result.data?.data
                encryptedImageUri = inputUri
            }
        }


    init {
        Log.i("init", "loading lib")
        System.loadLibrary("prototype_rust_wrapper")
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        createEncryptLauncher =
            registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
                if (result.resultCode == Activity.RESULT_OK) {
                    val outputUri = result.data?.data
                    if (outputUri != null && imageUri != null) {
                        lifecycleScope.launch {
                            encryptImage(imageUri!!, outputUri)
                        }
                    }
                }
            }

        createDecryptLauncher =
            registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
                if (result.resultCode == Activity.RESULT_OK) {
                    val outputUri = result.data?.data
                    if (outputUri != null && encryptedImageUri != null) {
                        lifecycleScope.launch {
                            decryptImage(encryptedImageUri!!, outputUri)
                        }
                    }
                }
            }

        try {
            setContent {
                NetWatchTheme {
                    // A surface container using the 'background' color from the theme
                    Surface(
                        modifier = Modifier.fillMaxSize(),
                        color = MaterialTheme.colorScheme.background
                    ) {
                        Column {
                            Row {
                                Button(onClick = {
                                    encryptionState = "text"
                                }) {
                                    Text("Encrypt Text")
                                }
                                Spacer(modifier = Modifier.width(16.dp))
                                Button(onClick = {
                                    encryptionState = "image"
                                }) {
                                    Text("Encrypt Image")
                                }
                            }

                            EncryptTest(
                                encryptionState,
                                imageUri,
                                encryptedImageUri,
                                pickImage,
                                pickEncryptedImage,
                                ::encryptAndSaveImage,
                                ::decryptAndSaveImage,
                                progress,
                            )
                        }
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
fun EncryptTest(
    encryptionState: String,
    imageUri: Uri?,
    encryptedImageUri: Uri?,
    pickImage: ActivityResultLauncher<Intent>,
    pickEncryptedImage: ActivityResultLauncher<Intent>,
    encryptAndSaveImage: () -> Unit,
    decryptAndSaveImage: () -> Unit,
    progress: Float,
) {
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
                    CryptoLayerRust.generateAESKey()
                }
            } catch (e: Exception) {
                alert(e)
            }
        }) {
            Text("Generate Encryption Key")
        }
        if (encryptionState == "text") {
            TextField(value = text, onValueChange = { text = it }, label = { Text("Message") })
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
            TextField(value = signatureText,
                onValueChange = {},
                enabled = false,
                label = { Text("Signature") })
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
            Button(onClick = {
                try {
                    CryptoLayerRust.generateECKey()
                } catch (e: Exception) {
                    alert(e)
                }
            }) {
                Text("Generate Signing Key")
            }
        } else {
            Button(onClick = {
                val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
                    addCategory(Intent.CATEGORY_OPENABLE)
                    type = "image/*"
                }
                pickImage.launch(intent)
            }) {
                Text("Select an image")
            }
            Text("Selected Image URI: $imageUri")
            Button(onClick = {
                try {
                    if (imageUri != null) {
                        encryptAndSaveImage()
                    }
                } catch (e: Exception) {
                    Log.e("ImageEncryption", "Error encrypting image", e)
                }
            }) {
                Text("Encrypt")
            }

            // Decrypt image
            Button(onClick = {
                val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
                    addCategory(Intent.CATEGORY_OPENABLE)
                    type = "image/*"
                }
                pickEncryptedImage.launch(intent)
            }) {
                Text("Select an encrypted image")
            }
            Text("Selected Image URI: $encryptedImageUri")
            Button(onClick = {
                try {
                    if (encryptedImageUri != null) {
                        decryptAndSaveImage()
                    }
                } catch (e: Exception) {
                    Log.e("ImageEncryption", "Error decrypting image", e)
                }
            }) {
                Text("Decrypt")
            }

            Row(
                modifier = Modifier.padding(top = 48.dp)
            ) {
                LinearProgressIndicator(
                    progress = { progress },
                    modifier = Modifier
                        .weight(1f)
                        .padding(top = 10.dp, start = 4.dp, end = 4.dp),
                )
                Text(
                    text = (progress * 100).toInt().toString() + "%",
                    modifier = Modifier
                        .wrapContentWidth()
                        .padding(end = 4.dp),
                )
            }
        }
    }

    when {
        showAlert -> {
            BasicAlertDialog(onDismissRequest = { showAlert = false }, content = {
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