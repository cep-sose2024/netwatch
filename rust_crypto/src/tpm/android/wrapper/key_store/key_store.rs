use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use crate::tpm::android::wrapper::key_generation::key::jni::{Key, PublicKey};
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

        pub extern "java" fn getKey(
            &self,
            env: &'borrow JNIEnv<'env>,
            alias: String,
            #[input_type("[C")] password: JObject,
        ) -> JniResult<Key> {
        }

        // pub fn getKey(
        //     &self,
        //     env: &'borrow JNIEnv<'env>,
        //     alias: String,
        //     password: Option<Vec<char>>,
        // ) -> JniResult<PrivateKey> {
        //     let char_ptr = match password {
        //         Some(password) => {
        //             let char_array = env.new_char_array(password.len() as i32)?;
        //             let u16_password = password.into_iter().map(|c| c as u16).collect::<Vec<_>>();
        //             env.set_char_array_region(char_array, 0, &u16_password)?;
        //             char_array
        //         }
        //         None => *robusta_jni::jni::objects::JObject::null(),
        //     };

        //     let str_ptr = env.new_string(alias)?;

        //     let value = env.call_method(
        //         self.raw.as_obj(),
        //         "getKey",
        //         "(Ljava/lang/String;[C)Ljava/security/Key;",
        //         &[Into::into(str_ptr), Into::into(char_ptr)],
        //     )?;

        //     let key = value.l()?;

        //     Ok(PrivateKey {
        //         raw: env.auto_local(key),
        //     })
        // }

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

        pub extern "java" fn toString(&self, _env: &JNIEnv) -> JniResult<String> {}
    }
}
