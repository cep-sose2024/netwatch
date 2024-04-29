use crypto_layer::{
    common::{
        crypto::{
            algorithms::{self, encryption::AsymmetricEncryption},
            KeyUsage,
        },
        factory::{SecModules, SecurityModule},
    },
    tpm::core::instance::TpmType,
};
use jni::{
    objects::{JByteArray, JClass},
    sys::jbyteArray,
    JNIEnv,
};

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_generateNewKey(
    _env: JNIEnv,
    _: JClass,
) {
    let provider = SecModules::get_instance(
        "KEY".to_owned(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
    )
    .expect("keystore provider not found");

    let mut provider = provider.lock().unwrap();
    
    let key_usage = vec![
        KeyUsage::Decrypt,
        KeyUsage::SignEncrypt,
        KeyUsage::CreateX509,
    ];
    let algorithm = AsymmetricEncryption::Rsa(algorithms::KeyBits::Bits512);
    provider.initialize_module(algorithm, None, None, key_usage).unwrap();
    provider.create_key("KEY").expect("can't create key");
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_encrypt(
    env: JNIEnv,
    _: JClass,
    array_ref: JByteArray,
) -> jbyteArray {
    let length = env.get_array_length(&array_ref).unwrap();
    let mut bytes = vec![0; length as usize];
    env.get_byte_array_region(&array_ref, 0, &mut bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let bytes = bytemuck::cast_slice::<i8, u8>(bytes.as_slice());

    let provider = SecModules::get_instance(
        "KEY".to_owned(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
    )
    .expect("keystore provider not found");

    let mut provider = provider.lock().unwrap();

    let key_usage = vec![
        KeyUsage::Decrypt,
        KeyUsage::SignEncrypt,
        KeyUsage::CreateX509,
    ];
    let algorithm = AsymmetricEncryption::Rsa(algorithms::KeyBits::Bits512);
    provider.initialize_module(algorithm, None, None, key_usage).unwrap();
    provider
        .load_key("KEY")
        .unwrap();

    let bytes = provider.encrypt_data(bytes).expect("encryption failed");

    // now we need to turn them back into i8
    let bytes = bytemuck::cast_slice::<u8, i8>(bytes.as_slice());

    let output = env
        .new_byte_array(bytes.len() as i32)
        .expect("Couldn't create java array");
    env.set_byte_array_region(&output, 0, bytes).unwrap();
    **output
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_decrypt(
    env: JNIEnv,
    _: JClass,
    array_ref: JByteArray,
) -> jbyteArray {
    let length = env.get_array_length(&array_ref).unwrap();
    let mut bytes = vec![0; length as usize];
    env.get_byte_array_region(&array_ref, 0, &mut bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let bytes = bytemuck::cast_slice::<i8, u8>(bytes.as_slice());

    let provider = SecModules::get_instance(
        "KEY".to_owned(),
        SecurityModule::Tpm(TpmType::Android(
            crypto_layer::tpm::core::instance::AndroidTpmType::Keystore,
        )),
    )
    .expect("keystore provider not found");

    let mut provider = provider.lock().unwrap();

    let key_usage = vec![
        KeyUsage::Decrypt,
        KeyUsage::SignEncrypt,
        KeyUsage::CreateX509,
    ];
    let algorithm = AsymmetricEncryption::Rsa(algorithms::KeyBits::Bits512);
    provider.initialize_module(algorithm, None, None, key_usage).unwrap();
    provider
        .load_key("KEY")
        .unwrap();

    let bytes = provider.decrypt_data(bytes).expect("encryption failed");

    // now we need to turn them back into i8
    let bytes = bytemuck::cast_slice::<u8, i8>(bytes.as_slice());

    let output = env
        .new_byte_array(bytes.len() as i32)
        .expect("Couldn't create java array");
    env.set_byte_array_region(&output, 0, bytes).unwrap();
    **output
}
