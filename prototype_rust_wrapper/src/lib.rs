use core::panic;
use std::borrow::Borrow;

use crypto_layer::{
    common::{
        crypto::{
            algorithms::{self, encryption::AsymmetricEncryption},
            KeyUsage,
        },
        error::SecurityModuleError,
        factory::{SecModules, SecurityModule},
    },
    tpm::core::instance::TpmType,
};
use jni::{
    objects::{JByteArray, JClass, JString},
    sys::{jboolean, jbyteArray},
    JNIEnv,
};
use tracing::{debug, error, warn};

fn generate_new_key(key: String, algorithm: String) -> Result<(), SecurityModuleError> {
    debug!("generating key: {key}");
    let provider = SecModules::get_instance(
        key.clone(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
    )
    .ok_or(SecurityModuleError::InitializationError(
        "couldn't create key provider".to_owned(),
    ))?;

    let mut provider = provider.lock().unwrap();

    let key_usage = vec![
        KeyUsage::Decrypt,
        KeyUsage::SignEncrypt,
        KeyUsage::CreateX509,
    ];
    let algorithm = match algorithm.borrow() {
        "RSA" => AsymmetricEncryption::Rsa(algorithms::KeyBits::Bits512),
        "EC" => AsymmetricEncryption::Ecc(algorithms::encryption::EccSchemeAlgorithm::Null),
        _ => AsymmetricEncryption::Rsa(algorithms::KeyBits::Bits512),
    };

    provider.initialize_module(algorithm, None, None, key_usage)?;
    provider.create_key(&key)?;
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_netwatch_RustNetWatch_generateNewKey(
    mut env: JNIEnv,
    _: JClass,
    key_id: JString,
    algorithm: JString,
) {
    let key_id: String = env.get_string(&key_id).expect("Couldn't get key ID").into();
    let algorithm: String = env
        .get_string(&algorithm)
        .expect("Couldn't get algo")
        .into();

    if let Err(e) = generate_new_key(key_id, algorithm) {
        handle_error(&mut env, e);
    }
}

fn encrypt(key: String, bytes: &[u8]) -> Result<Vec<u8>, SecurityModuleError> {
    let provider = SecModules::get_instance(
        key.clone(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
    )
    .ok_or(SecurityModuleError::InitializationError(
        "couldn't create key provider".to_owned(),
    ))?;

    let mut provider = provider.lock().unwrap();

    let key_usage = vec![
        KeyUsage::Decrypt,
        KeyUsage::SignEncrypt,
        KeyUsage::CreateX509,
    ];
    provider
        .initialize_module(
            AsymmetricEncryption::Rsa(algorithms::KeyBits::Bits512),
            None,
            None,
            key_usage,
        )
        .unwrap();
    provider.load_key(&key).unwrap();

    let bytes = provider.encrypt_data(bytes)?;

    return Ok(bytes);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_netwatch_RustNetwatch_encrypt(
    mut env: JNIEnv,
    _: JClass,
    key_id: JString,
    array_ref: JByteArray,
) -> jbyteArray {
    let key_id: String = env.get_string(&key_id).expect("Couldn't get key ID").into();

    let length = env.get_array_length(&array_ref).unwrap();
    let mut bytes = vec![0; length as usize];
    env.get_byte_array_region(&array_ref, 0, &mut bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let bytes = bytemuck::cast_slice::<i8, u8>(bytes.as_slice());

    match encrypt(key_id, bytes) {
        Ok(bytes) => {
            // now we need to turn them back into i8
            let bytes = bytemuck::cast_slice::<u8, i8>(bytes.as_slice());

            let output = env
                .new_byte_array(bytes.len() as i32)
                .expect("Couldn't create java array");
            env.set_byte_array_region(&output, 0, bytes).unwrap();
            **output
        }
        Err(e) => {
            handle_error(&mut env, e);
            jni::objects::JObject::null().as_raw()
        }
    }
}

fn decrypt(key_id: String, bytes: &[u8]) -> Result<Vec<u8>, SecurityModuleError> {
    let provider = SecModules::get_instance(
        key_id.clone(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
    )
    .ok_or(SecurityModuleError::InitializationError(
        "couldn't create key provider".to_owned(),
    ))?;

    let mut provider = provider.lock().unwrap();

    let key_usage = vec![
        KeyUsage::Decrypt,
        KeyUsage::SignEncrypt,
        KeyUsage::CreateX509,
    ];
    provider
        .initialize_module(
            AsymmetricEncryption::Rsa(algorithms::KeyBits::Bits512),
            None,
            None,
            key_usage,
        )
        .unwrap();
    provider.load_key(&key_id).unwrap();

    let bytes = provider.decrypt_data(bytes)?;
    return Ok(bytes);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_netwatch_RustNetWatch_decrypt(
    mut env: JNIEnv,
    _: JClass,
    key_id: JString,
    array_ref: JByteArray,
) -> jbyteArray {
    let key_id: String = env.get_string(&key_id).expect("Couldn't get key ID").into();

    let length = env.get_array_length(&array_ref).unwrap();
    let mut bytes = vec![0; length as usize];
    env.get_byte_array_region(&array_ref, 0, &mut bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let bytes = bytemuck::cast_slice::<i8, u8>(bytes.as_slice());

    match decrypt(key_id, bytes) {
        Ok(bytes) => {
            // now we need to turn them back into i8
            let bytes = bytemuck::cast_slice::<u8, i8>(bytes.as_slice());

            let output = env
                .new_byte_array(bytes.len() as i32)
                .expect("Couldn't create java array");
            env.set_byte_array_region(&output, 0, bytes).unwrap();
            **output
        }
        Err(e) => {
            handle_error(&mut env, e);
            jni::objects::JObject::null().as_raw()
        }
    }
}

fn sign(key_id: String, bytes: &[u8]) -> Result<Vec<u8>, SecurityModuleError> {
    let provider = SecModules::get_instance(
        key_id.clone(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
    )
    .ok_or(SecurityModuleError::InitializationError(
        "couldn't create key provider".to_owned(),
    ))?;

    let mut provider = provider.lock().unwrap();

    let key_usage = vec![
        KeyUsage::Decrypt,
        KeyUsage::SignEncrypt,
        KeyUsage::CreateX509,
    ];
    provider
        .initialize_module(
            AsymmetricEncryption::Rsa(algorithms::KeyBits::Bits512),
            None,
            None,
            key_usage,
        )
        .unwrap();
    provider.load_key(&key_id).unwrap();

    let bytes = provider.sign_data(bytes)?;
    return Ok(bytes);
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_netwatch_RustNetWatch_sign(
    mut env: JNIEnv,
    _: JClass,
    key_id: JString,
    array_ref: JByteArray,
) -> jbyteArray {
    let key_id: String = env.get_string(&key_id).expect("Couldn't get key ID").into();

    let length = env.get_array_length(&array_ref).unwrap();
    let mut bytes = vec![0; length as usize];
    env.get_byte_array_region(&array_ref, 0, &mut bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let bytes = bytemuck::cast_slice::<i8, u8>(bytes.as_slice());

    match sign(key_id, bytes) {
        Ok(bytes) => {
            // now we need to turn them back into i8
            let bytes = bytemuck::cast_slice::<u8, i8>(bytes.as_slice());

            let output = env
                .new_byte_array(bytes.len() as i32)
                .expect("Couldn't create java array");
            env.set_byte_array_region(&output, 0, bytes).unwrap();
            **output
        }
        Err(e) => {
            handle_error(&mut env, e);
            jni::objects::JObject::null().as_raw()
        }
    }
}

fn verify(
    key_id: String,
    data_bytes: &[u8],
    signature_bytes: &[u8],
) -> Result<bool, SecurityModuleError> {
    let provider = SecModules::get_instance(
        key_id.clone(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
    )
    .ok_or(SecurityModuleError::InitializationError(
        "couldn't create key provider".to_owned(),
    ))?;

    let mut provider = provider.lock().unwrap();

    let key_usage = vec![
        KeyUsage::Decrypt,
        KeyUsage::SignEncrypt,
        KeyUsage::CreateX509,
    ];
    provider
        .initialize_module(
            AsymmetricEncryption::Rsa(algorithms::KeyBits::Bits512),
            None,
            None,
            key_usage,
        )
        .unwrap();
    provider.load_key(&key_id).unwrap();

    provider.verify_signature(data_bytes, signature_bytes)
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_netwatch_RustNetWatch_verify(
    mut env: JNIEnv,
    _: JClass,
    key_id: JString,
    data_ref: JByteArray,
    signature_ref: JByteArray,
) -> jboolean {
    let key_id: String = env.get_string(&key_id).expect("Couldn't get key ID").into();

    let data_length = env.get_array_length(&data_ref).unwrap();
    let mut data_bytes = vec![0; data_length as usize];
    env.get_byte_array_region(&data_ref, 0, &mut data_bytes)
        .unwrap();

    let signature_length = env.get_array_length(&signature_ref).unwrap();
    let mut signature_bytes = vec![0; signature_length as usize];
    env.get_byte_array_region(&signature_ref, 0, &mut signature_bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let data_bytes = bytemuck::cast_slice::<i8, u8>(data_bytes.as_slice());
    let signature_bytes = bytemuck::cast_slice::<i8, u8>(signature_bytes.as_slice());

    match verify(key_id, data_bytes, signature_bytes) {
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
    if let Err(_) = env.throw_new("java/lang/Exception", error.to_string()) {
        error!("Couldn't throw java exception, panicking");
        panic!()
    }
}
