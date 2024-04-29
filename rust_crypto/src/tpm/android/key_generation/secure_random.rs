use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use robusta_jni::{
        convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue},
        jni::errors::Result as JniResult,
        jni::objects::AutoLocal,
        jni::JNIEnv,
    };

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(java.security)]
    pub struct SecureRandom<'env: 'borrow, 'borrow> {
        #[instance]
        pub raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> SecureRandom<'env, 'borrow> {
        #[constructor]
        pub extern "java" fn new(env: &'borrow JNIEnv<'env>) -> JniResult<Self> {}

        pub extern "java" fn getAlgorithm(&self, env: &JNIEnv<'env>) -> JniResult<String> {}
    }
}
