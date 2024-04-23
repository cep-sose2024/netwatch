// #![cfg(target_os = "android")]
#![allow(non_snake_case)]


use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jbyteArray, jstring};
use jni::JNIEnv;
use self::keystore::Keystore;

use super::*;

/// get the greeting from the rust side
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_greeting(
    mut env: JNIEnv,
    _: JClass,
    java_pattern: JString,
) -> jstring {
    // Our Java companion code might pass-in "world" as a string, hence the name.

    let world = rust_greeting(
        env.get_string(&java_pattern)
            .expect("invalid pattern string")
            .as_ptr(),
    )
    .unwrap_or(CString::new("Error in rust_greeting").unwrap());
    // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
    let output = env
        .new_string(world.to_str().unwrap())
        .expect("Couldn't create java string!");

    **output
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_generateNewKey(
    _env: JNIEnv,
    _: JClass,
) {
    let mut keystore = Keystore::default();
    keystore.generate_new_key();  
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_encrypt(
    env: JNIEnv,
    _: JClass,
    array_ref: JByteArray,
) -> jbyteArray {

    let length = env.get_array_length(&array_ref).unwrap();
    let mut bytes = vec![0; length as usize];
    env.get_byte_array_region(&array_ref, 0, &mut bytes).unwrap();

    let keystore = Keystore::default();
    let bytes = keystore.encrypt(bytes.as_slice());
    let output = env.new_byte_array(bytes.len() as i32).expect("Couldn't create java array");
    env.set_byte_array_region(&output, 0, &bytes).unwrap();
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
    env.get_byte_array_region(&array_ref, 0, &mut bytes).unwrap();

    let keystore = Keystore::default();
    let bytes = keystore.decrypt(bytes.as_slice());
    let output = env.new_byte_array(bytes.len() as i32).expect("Couldn't create java array");
    env.set_byte_array_region(&output, 0, &bytes).unwrap();
    **output
}
