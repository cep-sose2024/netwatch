use core::panic;
use std::{any::Any, panic::catch_unwind};

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
    objects::{JByteArray, JClass, JString},
    sys::{jboolean, jbyteArray},
    JNIEnv,
};
use tracing::error;

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_generateNewKey(
    mut env: JNIEnv,
    _: JClass,
    key_id: JString,
) {
    let key_id: String = env.get_string(&key_id).expect("Couldn't get key ID").into();
    let catch_res = catch_unwind(|| {
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
        provider
            .initialize_module(algorithm, None, None, key_usage)
            .unwrap();
        provider.create_key("KEY").expect("can't create key");
    });

    if let Err(e) = catch_res {
        handle_error(&mut env, e);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_encrypt(
    mut env: JNIEnv,
    _: JClass,
    array_ref: JByteArray,
) -> jbyteArray {
    let length = env.get_array_length(&array_ref).unwrap();
    let mut bytes = vec![0; length as usize];
    env.get_byte_array_region(&array_ref, 0, &mut bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let bytes = bytemuck::cast_slice::<i8, u8>(bytes.as_slice());

    let catch_res = catch_unwind(|| {
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
        provider
            .initialize_module(algorithm, None, None, key_usage)
            .unwrap();
        provider.load_key("KEY").unwrap();

        let bytes = provider.encrypt_data(bytes).expect("encryption failed");

        return bytes;
    });

    match catch_res {
        Ok(bytes) => {
            // now we need to turn them back into i8
            let bytes = bytemuck::cast_slice::<u8, i8>(bytes.as_slice());

            let output = env
                .new_byte_array(bytes.len() as i32)
                .expect("Couldn't create java array");
            env.set_byte_array_region(&output, 0, bytes).unwrap();
            **output
        },
        Err(e) => {
            handle_error(&mut env, e);
            jni::objects::JObject::null().as_raw()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_decrypt(
    mut env: JNIEnv,
    _: JClass,
    array_ref: JByteArray,
) -> jbyteArray {
    let length = env.get_array_length(&array_ref).unwrap();
    let mut bytes = vec![0; length as usize];
    env.get_byte_array_region(&array_ref, 0, &mut bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let bytes = bytemuck::cast_slice::<i8, u8>(bytes.as_slice());

    let catch_res = catch_unwind(|| {
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
        provider
            .initialize_module(algorithm, None, None, key_usage)
            .unwrap();
        provider.load_key("KEY").unwrap();

        let bytes = provider.decrypt_data(bytes).expect("encryption failed");
        return bytes;
    });

    match catch_res {
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

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_sign(
    mut env: JNIEnv,
    _: JClass,
    array_ref: JByteArray,
) -> jbyteArray {
    let length = env.get_array_length(&array_ref).unwrap();
    let mut bytes = vec![0; length as usize];
    env.get_byte_array_region(&array_ref, 0, &mut bytes)
        .unwrap();

    // the bytes are i8 right now, we need to reinterpret them to u8
    let bytes = bytemuck::cast_slice::<i8, u8>(bytes.as_slice());

    let catch_res = catch_unwind(|| {
        let provider = SecModules::get_instance(
            "KEY SIGN".to_owned(),
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
        provider
            .initialize_module(algorithm, None, None, key_usage)
            .unwrap();
        provider.load_key("KEY SIGN").unwrap();

        let bytes = provider.sign_data(bytes).expect("signing failed");
        return bytes;
    });

    match catch_res {
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

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_verify(
    mut env: JNIEnv,
    _: JClass,
    data_ref: JByteArray,
    signature_ref: JByteArray,
) -> jboolean {
    let data_length = env.get_array_length(&data_ref).unwrap();
    let mut data_bytes = vec![0; data_length as usize];
    env.get_byte_array_region(&data_ref, 0, &mut data_bytes)
        .unwrap();

    let signature_length = env.get_array_length(&signature_ref).unwrap();
    let mut signature_bytes = vec![0; signature_length as usize];
    env.get_byte_array_region(&signature_ref, 0, &mut signature_bytes)
        .unwrap();

    let catch_res = catch_unwind(|| {
        // the bytes are i8 right now, we need to reinterpret them to u8
        let data_bytes = bytemuck::cast_slice::<i8, u8>(data_bytes.as_slice());
        let signature_bytes = bytemuck::cast_slice::<i8, u8>(signature_bytes.as_slice());

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
        provider
            .initialize_module(algorithm, None, None, key_usage)
            .unwrap();
        provider.load_key("KEY SIGN").unwrap();

        let result = provider.verify_signature(data_bytes, signature_bytes);

        match result {
            Ok(_) => 1,
            Err(_) => 0,
        }
    });

    match catch_res {
        Ok(v) => v,
        Err(e) => {
            handle_error(&mut env, e);
            0
        }
    }
}

fn handle_error(env: &mut JNIEnv, error: Box<dyn Any>) {
    // check if there was a java exception
    if let Ok(true) = env.exception_check() {
        error!("There was a java exception, trying to print it:");
        if let Err(e) = env.exception_describe() {
            error!("Couldn't print it: {}", e);
        }
        // because we didn't clear it, the exception will keep going when we return
    } else {
        error!("rust panicked without a java exception");
        // try and get the error
        match error.downcast_ref::<String>() {
            Some(s) => error!(s),
            None => error!("couldn't downcast rust error"),
        }
        // throw java exception
        if let Err(_) = env.throw_new("java/lang/Exception", "rust panic") {
            error!("Couldn't throw java exception, panicking");
            panic!()
        }
    }
}
