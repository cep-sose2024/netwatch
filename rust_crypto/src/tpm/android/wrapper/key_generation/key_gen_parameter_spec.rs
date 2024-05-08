use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use robusta_jni::{
        convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue},
        jni::{errors::Result as JniResult, objects::AutoLocal, JNIEnv},
    };

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(android.security.keystore)]
    pub struct KeyGenParameterSpec<'env: 'borrow, 'borrow> {
        #[instance]
        pub raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> KeyGenParameterSpec<'env, 'borrow> {
        pub extern "java" fn getDigests(&self, env: &JNIEnv) -> JniResult<Vec<String>> {}

        pub extern "java" fn isStrongBoxBacked(&self, env: &JNIEnv) -> JniResult<bool> {}
    }
}
