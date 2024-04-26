use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use crate::android_classes::{key_pair::jni::KeyPair, secure_random::jni::SecureRandom};
    use robusta_jni::{
        convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue},
        jni::{
            errors::Result as JniResult,
            objects::{AutoLocal, JObject},
            JNIEnv,
        },
    };

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(java.security)]
    pub struct KeyPairGenerator<'env: 'borrow, 'borrow> {
        #[instance]
        pub raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> KeyPairGenerator<'env, 'borrow> {
        pub extern "java" fn getInstance(
            env: &'borrow JNIEnv<'env>,
            algorithm: String,
            provider: String,
        ) -> JniResult<Self> {
        }

        pub extern "java" fn toString(&self, _env: &JNIEnv) -> JniResult<String> {}
        pub extern "java" fn getAlgorithm(&self, _env: &JNIEnv) -> JniResult<String> {}

        pub extern "java" fn initialize(
            &self,
            env: &JNIEnv,
            #[input_type("Ljava/security/spec/AlgorithmParameterSpec;")] params: JObject,
            random: SecureRandom,
        ) -> JniResult<()> {
        }

        pub extern "java" fn generateKeyPair(&self, _env: &'borrow JNIEnv) -> JniResult<KeyPair> {}
    }
}
