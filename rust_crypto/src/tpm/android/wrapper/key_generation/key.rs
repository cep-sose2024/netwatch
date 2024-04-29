use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use robusta_jni::{
        convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue},
        jni::{errors::Result as JniResult, objects::AutoLocal, JNIEnv},
    };

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(java.security)]
    pub struct PublicKey<'env: 'borrow, 'borrow> {
        #[instance]
        pub raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> PublicKey<'env, 'borrow> {
        pub extern "java" fn toString(&self, _env: &JNIEnv) -> JniResult<String> {}
        pub extern "java" fn getAlgorithm(&self, _env: &JNIEnv) -> JniResult<String> {}
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(java.security)]
    pub struct PrivateKey<'env: 'borrow, 'borrow> {
        #[instance]
        pub raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> PrivateKey<'env, 'borrow> {
        pub extern "java" fn toString(&self, _env: &JNIEnv) -> JniResult<String> {}
        pub extern "java" fn getAlgorithm(&self, _env: &JNIEnv) -> JniResult<String> {}
    }
}
