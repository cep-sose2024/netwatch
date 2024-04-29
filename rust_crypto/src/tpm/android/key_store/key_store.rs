use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use crate::key_generation::key::jni::PublicKey;
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
    pub struct KeyStore<'env: 'borrow, 'borrow> {
        #[instance]
        pub raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> KeyStore<'env, 'borrow> {
        pub extern "java" fn getInstance(
            env: &'borrow JNIEnv<'env>,
            type1: String,
        ) -> JniResult<Self> {
        }

        pub extern "java" fn getCertificate(
            &self,
            env: &'borrow JNIEnv<'env>,
            alias: String,
        ) -> JniResult<Certificate> {
        }

        pub fn load(&self, env: &JNIEnv, param: Option<JObject>) -> JniResult<()> {
            let param_obj = param.unwrap_or(JObject::null());
            env.call_method(
                self.raw.as_obj(),
                "load",
                "(Ljava/security/KeyStore$LoadStoreParameter;)V",
                &[Into::into(param_obj)],
            )?;
            Ok(())
        }
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(java.security.cert)]
    pub struct Certificate<'env: 'borrow, 'borrow> {
        #[instance]
        pub raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> Certificate<'env, 'borrow> {
        pub extern "java" fn getPublicKey(
            &self,
            env: &'borrow JNIEnv<'env>,
        ) -> JniResult<PublicKey<'env, 'borrow>> {
        }
    }
}
