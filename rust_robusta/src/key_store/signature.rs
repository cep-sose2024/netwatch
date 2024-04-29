use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use crate::key_store::key_store::jni::Certificate;
    use robusta_jni::{
        convert::{IntoJavaValue, Signature as JavaSignature, TryFromJavaValue, TryIntoJavaValue},
        jni::{
            errors::Result as JniResult,
            objects::{AutoLocal, JObject, JValue},
            JNIEnv,
        },
    };

    #[derive(JavaSignature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(java.security)]
    pub struct Signature<'env: 'borrow, 'borrow> {
        #[instance]
        pub raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> Signature<'env, 'borrow> {
        pub extern "java" fn getInstance(
            env: &'borrow JNIEnv<'env>,
            algorithm: String,
        ) -> JniResult<Self> {
        }

        pub fn sign(&self, env: &JNIEnv) -> JniResult<Box<[u8]>> {
            let result = env.call_method(self.raw.as_obj(), "sign", "()[B", &[])?;

            let byte_array = result.l()?.into_inner();
            let output = env.convert_byte_array(byte_array)?;

            Ok(output.into_boxed_slice())
        }

        pub fn initSign(&self, env: &JNIEnv, privateKey: JObject) -> JniResult<()> {
            env.call_method(
                self.raw.as_obj(),
                "initSign",
                "(Ljava/security/PrivateKey;)V",
                &[JValue::from(privateKey)],
            )?;

            Ok(())
        }

        pub fn initVerify(&self, env: &JNIEnv, certificate: Certificate) -> JniResult<()> {
            let certificate_obj = certificate.raw.as_obj();

            env.call_method(
                self.raw.as_obj(),
                "initVerify",
                "(Ljava/security/cert/Certificate;)V",
                &[JValue::from(JObject::from(certificate_obj))],
            )?;

            Ok(())
        }
        pub extern "java" fn verify(&self, _env: &JNIEnv, signature: Box<[u8]>) -> JniResult<bool> {
        }

        pub extern "java" fn update(&self, _env: &JNIEnv, data: Box<[u8]>) -> JniResult<()> {}
        pub extern "java" fn toString(&self, _env: &JNIEnv) -> JniResult<String> {}
    }
}
