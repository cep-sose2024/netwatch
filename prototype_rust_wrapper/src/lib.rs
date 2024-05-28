use core::panic;
use std::borrow::Borrow;

use crypto_layer::{
    common::{
        crypto::{
            algorithms::{self, encryption::{AsymmetricEncryption, BlockCiphers, SymmetricMode}, hashes::{Hash, Sha2Bits}, KeyBits},
            KeyUsage,
        },
        error::SecurityModuleError,
        factory::{SecModules, SecurityModule}
    },
    tpm::{android::{android_logger::DefaultAndroidLogger, config::{AndroidConfig, EncryptionMode}}, core::instance::TpmType},
};
use robusta_jni::jni::{
    objects::{JClass, JString, JObject},
    sys::{jboolean, jbyteArray},
    JNIEnv, JavaVM,
};
use tracing::{debug, error, warn};

fn generate_new_key(key: String, algorithm: String, vm: JavaVM) -> Result<(), SecurityModuleError> {

    let mode = match algorithm.borrow() {
        "RSA" => EncryptionMode::ASym {algo: AsymmetricEncryption::Rsa(KeyBits::Bits1024), digest: Hash::Sha2(Sha2Bits::Sha256)},
        "EC" => EncryptionMode::ASym {algo: AsymmetricEncryption::Ecc(algorithms::encryption::EccSchemeAlgorithm::Null), digest: Hash::Sha2(Sha2Bits::Sha256)},
        "AES" => EncryptionMode::Sym(BlockCiphers::Aes(SymmetricMode::Ecb, KeyBits::Bits256)),
        _ => panic!(),
    };

    let config = AndroidConfig {
        mode: mode,
        hardware_backed: true,
        key_usages: vec![
            KeyUsage::Decrypt,
            KeyUsage::SignEncrypt,
            KeyUsage::CreateX509,
        ],
        vm: Some(vm),
    };

    debug!("generating key: {key}");
    let provider = SecModules::get_instance(
        key.clone(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
        Some(Box::new(DefaultAndroidLogger)),
    )
    .ok_or(SecurityModuleError::InitializationError(
        "couldn't create key provider".to_owned(),
    ))?;

    let mut provider = provider.lock().unwrap();

    provider.initialize_module()?;
    provider.create_key(&key, Box::new(config))?;
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_netwatch_RustNetwatch_generateNewKey(
    mut env: JNIEnv,
    _: JClass,
    key_id: JString,
    algorithm: JString,
) {
    let key_id: String = env.get_string(key_id).expect("Couldn't get key ID").into();
    let algorithm: String = env
        .get_string(algorithm)
        .expect("Couldn't get algo")
        .into();

    let vm = env.get_java_vm().unwrap();

    if let Err(e) = generate_new_key(key_id, algorithm, vm) {
        handle_error(&mut env, e);
    }
}

fn encrypt(key: String, bytes: &[u8], vm: JavaVM) -> Result<Vec<u8>, SecurityModuleError> {
    let provider = SecModules::get_instance(
        key.clone(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
        Some(Box::new(DefaultAndroidLogger)),
    )
    .ok_or(SecurityModuleError::InitializationError(
        "couldn't create key provider".to_owned(),
    ))?;

    let mut provider = provider.lock().unwrap();

    let algorithm = "RSA";

    let mode = match algorithm.borrow() {
        "RSA" => EncryptionMode::ASym {algo: AsymmetricEncryption::Rsa(KeyBits::Bits512), digest: Hash::Sha2(Sha2Bits::Sha256)},
        "EC" => EncryptionMode::ASym {algo: AsymmetricEncryption::Ecc(algorithms::encryption::EccSchemeAlgorithm::Null), digest: Hash::Sha2(Sha2Bits::Sha256)},
        "AES" => EncryptionMode::Sym(BlockCiphers::Aes(SymmetricMode::Ecb, KeyBits::Bits256)),
        _ => panic!(),
    };

    let config = AndroidConfig {
        mode: mode,
        hardware_backed: true,
        key_usages: vec![
            KeyUsage::Decrypt,
            KeyUsage::SignEncrypt,
            KeyUsage::CreateX509,
        ],
        vm: Some(vm),
    };
    provider
        .initialize_module()
        .unwrap();
    provider.load_key(&key, Box::new(config)).unwrap();

    let bytes = provider.encrypt_data(bytes)?;

    Ok(bytes)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_netwatch_RustNetwatch_encrypt(
    mut env: JNIEnv,
    _: JClass,
    key_id: JString,
    array_ref: jbyteArray,
) -> jbyteArray {
    let key_id: String = env.get_string(key_id).expect("Couldn't get key ID").into();

    let length = env.get_array_length(array_ref).unwrap();
    let mut bytes = vec![0; length as usize];
    env.get_byte_array_region(array_ref, 0, &mut bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let bytes = bytemuck::cast_slice::<i8, u8>(bytes.as_slice());

    let vm = env.get_java_vm().unwrap();

    match encrypt(key_id, bytes, vm) {
        Ok(bytes) => {
            // now we need to turn them back into i8
            let bytes = bytemuck::cast_slice::<u8, i8>(bytes.as_slice());

            let output = env
                .new_byte_array(bytes.len() as i32)
                .expect("Couldn't create java array");
            env.set_byte_array_region(output, 0, bytes).unwrap();
            output
        }
        Err(e) => {
            handle_error(&mut env, e);
            *JObject::null()
        }
    }
}

fn decrypt(key_id: String, bytes: &[u8], vm: JavaVM) -> Result<Vec<u8>, SecurityModuleError> {
    let provider = SecModules::get_instance(
        key_id.clone(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
        Some(Box::new(DefaultAndroidLogger)),
    )
    .ok_or(SecurityModuleError::InitializationError(
        "couldn't create key provider".to_owned(),
    ))?;

    let mut provider = provider.lock().unwrap();

    let algorithm = "RSA";

    let mode = match algorithm.borrow() {
        "RSA" => EncryptionMode::ASym {algo: AsymmetricEncryption::Rsa(KeyBits::Bits512), digest: Hash::Sha2(Sha2Bits::Sha256)},
        "EC" => EncryptionMode::ASym {algo: AsymmetricEncryption::Ecc(algorithms::encryption::EccSchemeAlgorithm::Null), digest: Hash::Sha2(Sha2Bits::Sha256)},
        "AES" => EncryptionMode::Sym(BlockCiphers::Aes(SymmetricMode::Ecb, KeyBits::Bits256)),
        _ => panic!(),
    };

    let config = AndroidConfig {
        mode: mode,
        hardware_backed: true,
        key_usages: vec![
            KeyUsage::Decrypt,
            KeyUsage::SignEncrypt,
            KeyUsage::CreateX509,
        ],
        vm: Some(vm),
    };

    provider
        .initialize_module()
        .unwrap();
    provider.load_key(&key_id, Box::new(config)).unwrap();

    let bytes = provider.decrypt_data(bytes)?;
    Ok(bytes)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_netwatch_RustNetwatch_decrypt(
    mut env: JNIEnv,
    _: JClass,
    key_id: JString,
    array_ref: jbyteArray,
) -> jbyteArray {
    let key_id: String = env.get_string(key_id).expect("Couldn't get key ID").into();

    let length = env.get_array_length(array_ref).unwrap();
    let mut bytes = vec![0; length as usize];
    env.get_byte_array_region(array_ref, 0, &mut bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let bytes = bytemuck::cast_slice::<i8, u8>(bytes.as_slice());

    let vm = env.get_java_vm().unwrap();

    match decrypt(key_id, bytes, vm) {
        Ok(bytes) => {
            // now we need to turn them back into i8
            let bytes = bytemuck::cast_slice::<u8, i8>(bytes.as_slice());

            let output = env
                .new_byte_array(bytes.len() as i32)
                .expect("Couldn't create java array");
            env.set_byte_array_region(output, 0, bytes).unwrap();
            output
        }
        Err(e) => {
            handle_error(&mut env, e);
            *JObject::null()
        }
    }
}

fn sign(key_id: String, bytes: &[u8], vm: JavaVM) -> Result<Vec<u8>, SecurityModuleError> {
    let provider = SecModules::get_instance(
        key_id.clone(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
        Some(Box::new(DefaultAndroidLogger)),
    )
    .ok_or(SecurityModuleError::InitializationError(
        "couldn't create key provider".to_owned(),
    ))?;

    let mut provider = provider.lock().unwrap();

    let algorithm = "RSA";

    let mode = match algorithm.borrow() {
        "RSA" => EncryptionMode::ASym {algo: AsymmetricEncryption::Rsa(KeyBits::Bits512), digest: Hash::Sha2(Sha2Bits::Sha256)},
        "EC" => EncryptionMode::ASym {algo: AsymmetricEncryption::Ecc(algorithms::encryption::EccSchemeAlgorithm::Null), digest: Hash::Sha2(Sha2Bits::Sha256)},
        "AES" => EncryptionMode::Sym(BlockCiphers::Aes(SymmetricMode::Ecb, KeyBits::Bits256)),
        _ => panic!(),
    };

    let config = AndroidConfig {
        mode: mode,
        hardware_backed: true,
        key_usages: vec![
            KeyUsage::Decrypt,
            KeyUsage::SignEncrypt,
            KeyUsage::CreateX509,
        ],
        vm: Some(vm),
    };

    provider
        .initialize_module()
        .unwrap();


    provider.load_key(&key_id, Box::new(config)).unwrap();

    let bytes = provider.sign_data(bytes)?;
    Ok(bytes)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_netwatch_RustNetwatch_sign(
    mut env: JNIEnv,
    _: JClass,
    key_id: JString,
    array_ref: jbyteArray,
) -> jbyteArray {
    let key_id: String = env.get_string(key_id).expect("Couldn't get key ID").into();

    let length = env.get_array_length(array_ref).unwrap();
    let mut bytes = vec![0; length as usize];
    env.get_byte_array_region(array_ref, 0, &mut bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let bytes = bytemuck::cast_slice::<i8, u8>(bytes.as_slice());

    let vm = env.get_java_vm().unwrap();

    match sign(key_id, bytes, vm) {
        Ok(bytes) => {
            // now we need to turn them back into i8
            let bytes = bytemuck::cast_slice::<u8, i8>(bytes.as_slice());

            let output = env
                .new_byte_array(bytes.len() as i32)
                .expect("Couldn't create java array");
            env.set_byte_array_region(output, 0, bytes).unwrap();
            output
        }
        Err(e) => {
            handle_error(&mut env, e);
            *JObject::null()
        }
    }
}

fn verify(
    key_id: String,
    data_bytes: &[u8],
    signature_bytes: &[u8],
    vm: JavaVM
) -> Result<bool, SecurityModuleError> {
    let provider = SecModules::get_instance(
        key_id.clone(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
        Some(Box::new(DefaultAndroidLogger)),
    )
    .ok_or(SecurityModuleError::InitializationError(
        "couldn't create key provider".to_owned(),
    ))?;

    let mut provider = provider.lock().unwrap();

    let algorithm = "RSA";

    let mode = match algorithm.borrow() {
        "RSA" => EncryptionMode::ASym {algo: AsymmetricEncryption::Rsa(KeyBits::Bits512), digest: Hash::Sha2(Sha2Bits::Sha256)},
        "EC" => EncryptionMode::ASym {algo: AsymmetricEncryption::Ecc(algorithms::encryption::EccSchemeAlgorithm::Null), digest: Hash::Sha2(Sha2Bits::Sha256)},
        "AES" => EncryptionMode::Sym(BlockCiphers::Aes(SymmetricMode::Ecb, KeyBits::Bits256)),
        _ => panic!(),
    };

    let config = AndroidConfig {
        mode: mode,
        hardware_backed: true,
        key_usages: vec![
            KeyUsage::Decrypt,
            KeyUsage::SignEncrypt,
            KeyUsage::CreateX509,
        ],
        vm: Some(vm),
    };

    provider
        .initialize_module()
        .unwrap();
    provider.load_key(&key_id, Box::new(config)).unwrap();

    provider.verify_signature(data_bytes, signature_bytes)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_netwatch_RustNetwatch_verify(
    mut env: JNIEnv,
    _: JClass,
    key_id: JString,
    data_ref: jbyteArray,
    signature_ref: jbyteArray,
) -> jboolean {
    let key_id: String = env.get_string(key_id).expect("Couldn't get key ID").into();

    let data_length = env.get_array_length(data_ref).unwrap();
    let mut data_bytes = vec![0; data_length as usize];
    env.get_byte_array_region(data_ref, 0, &mut data_bytes)
        .unwrap();

    let signature_length = env.get_array_length(signature_ref).unwrap();
    let mut signature_bytes = vec![0; signature_length as usize];
    env.get_byte_array_region(signature_ref, 0, &mut signature_bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let data_bytes = bytemuck::cast_slice::<i8, u8>(data_bytes.as_slice());
    let signature_bytes = bytemuck::cast_slice::<i8, u8>(signature_bytes.as_slice());

    let vm = env.get_java_vm().unwrap();

    match verify(key_id, data_bytes, signature_bytes, vm) {
        Ok(true) => 1,
        Ok(false) => 0,
        Err(e) => {
            handle_error(&mut env, e);
            0
        }
    }
}

fn handle_error(env: &mut JNIEnv, error: SecurityModuleError) {
    warn!("{}", error);
    // throw java exception
    if env.throw_new("java/lang/Exception", error.to_string()).is_err() {
        error!("Couldn't throw java exception, panicking");
        panic!()
    }
}
