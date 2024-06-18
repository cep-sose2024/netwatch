package com.example.netwatch

import android.app.Activity
import android.content.ContentResolver
import android.content.Context
import android.content.Intent
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.net.IpSecAlgorithm
import android.net.Uri
import android.os.Bundle
import android.os.PersistableBundle
import android.util.Log
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.compose.setContent
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.wrapContentHeight
import androidx.compose.foundation.layout.wrapContentWidth
import androidx.compose.material3.BasicAlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ExposedDropdownMenuBox
import androidx.compose.material3.ExposedDropdownMenuDefaults
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Tab
import androidx.compose.material3.TabRow
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.focusModifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.unit.dp
import androidx.lifecycle.lifecycleScope
import com.example.netwatch.ui.theme.NetWatchTheme
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
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
                NetWatchTheme {
                    // A surface container using the 'background' color from the theme
                    Surface(
                        modifier = Modifier.fillMaxSize(),
                        color = MaterialTheme.colorScheme.background
                    ) {
                        Column {
                            TabScreen(contentResolver, this@MainActivity)
                        }
                    }
                }
            }
        } catch (e: Exception) {
            Log.e("setContent", "Exception: " + e.message)
        }
    }
}

suspend fun decryptImage(inputUri: Uri, outputUri: Uri, contentResolver: ContentResolver, context: Context) = withContext(Dispatchers.IO) {
    val startTime = System.currentTimeMillis()
    var imageSize: Int? = null

    contentResolver.openInputStream(inputUri)?.use { inputStream ->
        val byteArray = inputStream.readBytes()
        imageSize = byteArray.size
        Log.d("ImageDecryption", "Input file: $byteArray")

        val decryptedByteArray = CryptoLayerRust.decryptFile(byteArray, "AES")
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
            context,
            "Image decrypted successfully in $duration ms",
            Toast.LENGTH_SHORT
        ).show()
    }
    Log.d("ImageDecryption", "Output file: $outputUri")
}

suspend fun encryptImage(inputUri: Uri, outputUri: Uri, contentResolver: ContentResolver, context: Context) = withContext(Dispatchers.IO) {
    val startTime = System.currentTimeMillis()

    contentResolver.openInputStream(inputUri)?.use { inputStream ->
        val byteArray = inputStream.readBytes()
        val imageSize = byteArray.size

        val encryptedByteArray = CryptoLayerRust.encryptFile(byteArray, "AES")

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
                context,
                "Image encrypted successfully in $duration ms",
                Toast.LENGTH_SHORT
            ).show()
        }
        Log.d("ImageEncryption", "Output file: $outputUri")
    }
}

@Composable
fun TabScreen(
    contentResolver: ContentResolver,
    context: Context,
    ) {
    var tabIndex by remember { mutableStateOf(0) }

    val tabs = listOf("Text", "Image")

    Column(modifier = Modifier.fillMaxWidth()) {
        TabRow(selectedTabIndex = tabIndex) {
            tabs.forEachIndexed { index, title ->
                Tab(text = { Text(title) },
                    selected = tabIndex == index,
                    onClick = { tabIndex = index }
                )
            }
        }
        when (tabIndex) {
            0 -> TextEncrypter()
            1 -> ImageEncrypter(contentResolver, context)
        }
    }
}

@Composable
fun TextEncrypter() {
    var text by remember { mutableStateOf("Hello World") }
    var encText by remember { mutableStateOf("") }
    var showAlert by remember { mutableStateOf(false) }
    var exceptionName by remember { mutableStateOf("") }
    var algorithms = RustNetwatch.getCapabilities()
    var algorithm by remember { mutableStateOf("AES") }
    var signatureText by remember { mutableStateOf("") }
    var verificationStatus by remember { mutableStateOf("") }

    val alert = { e: Exception ->
        showAlert = true
        exceptionName = e.toString()
    }

        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(32.dp),
        ){
            Demo_ExposedDropdownMenuBox(algorithms) { algorithm = it }

            Button(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 32.dp)
                    .align(Alignment.CenterHorizontally),
                onClick = {
                try {
                    thread {
                        CryptoLayerRust.generateKey(algorithm)
                    }
                } catch (e: Exception) {
                    alert(e)
                }
            }) {
                Text("Generate Key")
            }
            TextField(value = text, onValueChange = { text = it }, label = { Text("Message") }, modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 32.dp)
                .align(Alignment.CenterHorizontally))
            Button(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 32.dp)
                    .align(Alignment.CenterHorizontally),
                onClick = {
                try {
                    encText = CryptoLayerRust.encryptText(text, algorithm)
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
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 32.dp)
                    .align(Alignment.CenterHorizontally)
            )
            Button(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 32.dp)
                    .align(Alignment.CenterHorizontally),
                onClick = {
                try {
                    val dec = CryptoLayerRust.decryptText(encText, algorithm)
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
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 32.dp)
                    .align(Alignment.CenterHorizontally),
                label = { Text("Signature") })
            Button(modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 32.dp)
                .align(Alignment.CenterHorizontally),
                onClick = {
                try {
                    signatureText = CryptoLayerRust.signText(encText, algorithm)
                } catch (e: Exception) {
                    alert(e)
                }
            }) {
                Text("Sign")
            }
            Button(modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 32.dp)
                .align(Alignment.CenterHorizontally),
                onClick = {
                try {
                    val verified = CryptoLayerRust.verifyText(encText, signatureText, algorithm)
                    verificationStatus = if (verified) "Verified" else "Not Verified"
                } catch (e: Exception) {
                    alert(e)
                }
            }) {
                Text("Verify Signature")
            }
            Text(text = verificationStatus, modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 32.dp)
                .align(Alignment.CenterHorizontally))
        }

}

@Composable
fun ImageEncrypter(
    contentResolver: ContentResolver,
    context: Context,
    ) {
    var imageUri: Uri? by remember { mutableStateOf(null) }
    var encryptedImageUri: Uri? by remember { mutableStateOf(null) }
    var progress by remember { mutableFloatStateOf(0f) }
    var showAlert by remember { mutableStateOf(false) }
    var exceptionName by remember { mutableStateOf("") }
    var algorithms = RustNetwatch.getCapabilities()
    var algorithm by remember { mutableStateOf("AES") }

    val alert = { e: Exception ->
        showAlert = true
        exceptionName = e.toString()
    }

    val lifecycleOwner = LocalLifecycleOwner.current

    var createEncryptLauncher =
        rememberLauncherForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
            if (result.resultCode == Activity.RESULT_OK) {
                val outputUri = result.data?.data
                if (outputUri != null && imageUri != null) {
                    lifecycleOwner.lifecycleScope.launch {
                        encryptImage(imageUri!!, outputUri, contentResolver, context)
                    }
                }
            }
        }

    var createDecryptLauncher =
        rememberLauncherForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
            if (result.resultCode == Activity.RESULT_OK) {
                val outputUri = result.data?.data
                if (outputUri != null && encryptedImageUri != null) {
                    lifecycleOwner.lifecycleScope.launch {
                        decryptImage(encryptedImageUri!!, outputUri, contentResolver, context)
                    }
                }
            }
        }

    val pickImage =
        rememberLauncherForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
            if (result.resultCode == Activity.RESULT_OK) {
                // The result data contains a Uri for the image
                val inputUri = result.data?.data
                imageUri = inputUri
            }
        }

    val pickEncryptedImage =
        rememberLauncherForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
            if (result.resultCode == Activity.RESULT_OK) {
                // The result data contains a Uri for the image
                val inputUri = result.data?.data
                encryptedImageUri = inputUri
            }
        }

    fun encryptAndSaveImage() {
        val createFileIntent = Intent(Intent.ACTION_CREATE_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "image/png"
            putExtra(Intent.EXTRA_TITLE, "encrypted")
        }

        createEncryptLauncher.launch(createFileIntent)
    }

    fun decryptAndSaveImage() {
        val createFileIntent = Intent(Intent.ACTION_CREATE_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "image/png"
            putExtra(Intent.EXTRA_TITLE, "decrypted")
        }

        createDecryptLauncher.launch(createFileIntent)
    }

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(32.dp),
    ) {
//        Demo_ExposedDropdownMenuBox(algorithms) { algorithm = it }
        Button(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 32.dp)
                .align(Alignment.CenterHorizontally),
            onClick = {
                try {
                    thread {
                        CryptoLayerRust.generateKey(algorithm)
                    }
                } catch (e: Exception) {
                    alert(e)
                }
            }) {
            Text("Generate Key")
        }
        Button(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 32.dp)
                .align(Alignment.CenterHorizontally),
            onClick = {
                val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
                    addCategory(Intent.CATEGORY_OPENABLE)
                    type = "image/*"
                }
                pickImage.launch(intent)
            }) {
                Text("Select an image")
            }
        Text("Selected Image URI: $imageUri")
        Button(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 32.dp)
                .align(Alignment.CenterHorizontally),
            onClick = {
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
        Button(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 32.dp)
                .align(Alignment.CenterHorizontally),
            onClick = {
            val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
                addCategory(Intent.CATEGORY_OPENABLE)
                type = "image/*"
            }
            pickEncryptedImage.launch(intent)
        }) {
            Text("Select an encrypted image")
        }
        Text("Selected Image URI: $encryptedImageUri")
        Button(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 32.dp)
                .align(Alignment.CenterHorizontally),
            onClick = {
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
        Row {
            Button(onClick = {
                try {
                    thread {
                        CryptoLayerRust.generateAESKey()
                    }
                } catch (e: Exception) {
                    alert(e)
                }
            }) {
                Text("Generate Key")
            }
        }
        if (encryptionState == "text") {
            TextField(value = text, onValueChange = { text = it }, label = { Text("Message") })
            Button(onClick = {
                try {
                    encText = CryptoLayerRust.encryptText(text, "AES")
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
                    val dec = CryptoLayerRust.decryptText(encText, "AES")
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
                    signatureText = CryptoLayerRust.signText(encText, "EC")
                } catch (e: Exception) {
                    alert(e)
                }
            }) {
                Text("Sign")
            }
            Button(onClick = {
                try {
                    val verified = CryptoLayerRust.verifyText(encText, signatureText, "EC")
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

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun Demo_ExposedDropdownMenuBox(algos: Array<String>, onChange: (String) -> Unit) {
    val context = LocalContext.current
    var expanded by remember { mutableStateOf(false) }
    var selectedText by remember { mutableStateOf(algos[0]) }

    Box(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 32.dp)
    ) {
        ExposedDropdownMenuBox(
            expanded = expanded,
            onExpandedChange = {
                expanded = !expanded
            }
        ) {
            TextField(
                value = selectedText,
                onValueChange = {},
                readOnly = true,
                trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded = expanded) },
                modifier = Modifier.menuAnchor()
            )

            ExposedDropdownMenu(
                expanded = expanded,
                onDismissRequest = { expanded = false }
            ) {
                algos.forEach { item ->
                    DropdownMenuItem(
                        text = { Text(text = item) },
                        onClick = {
                            selectedText = item
                            onChange(item)
                            expanded = false
                            Toast.makeText(context, item, Toast.LENGTH_SHORT).show()
                        }
                    )
                }
            }
        }
    }
}