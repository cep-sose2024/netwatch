use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use crate::tpm::android::wrapper::key_generation::key::jni::{PrivateKey, PublicKey};
    use robusta_jni::{
        convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue},
        jni::{errors::Result as JniResult, objects::AutoLocal, JNIEnv},
    };

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(java.security)]
    pub struct KeyPair<'env: 'borrow, 'borrow> {
        #[instance]
        pub raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> KeyPair<'env, 'borrow> {
        pub extern "java" fn toString(&self, _env: &JNIEnv) -> JniResult<String> {}
        pub extern "java" fn getPublic(&self, _env: &'borrow JNIEnv) -> JniResult<PublicKey> {}
        pub extern "java" fn getPrivate(&self, _env: &'borrow JNIEnv) -> JniResult<PrivateKey> {}
    }
}
