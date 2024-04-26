use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use robusta_jni::convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue};
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::objects::{AutoLocal, JObject, JValue};
    use robusta_jni::jni::sys::jsize;
    use robusta_jni::jni::JNIEnv;

    use crate::android_classes::key_gen_parameter_spec::jni::KeyGenParameterSpec;

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(android.security.keystore)]
    pub struct Builder<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> Builder<'env, 'borrow> {
        pub fn new(
            env: &'borrow JNIEnv<'env>,
            keystoreAlias: String,
            purposes: i32,
        ) -> JniResult<JObject<'env>> {
            let class = env.find_class("android/security/keystore/KeyGenParameterSpec$Builder")?;
            let jstring_keystore_alias = env.new_string(keystoreAlias)?;
            let args = [Into::into(jstring_keystore_alias), JValue::from(purposes)];
            let obj = env.new_object(class, "(Ljava/lang/String;I)V", &args)?;
            Ok(obj)
        }

        pub fn setDigests(
            env: &'borrow JNIEnv<'env>,
            object: JObject<'env>,
            digests: Vec<String>,
        ) -> JniResult<JObject<'env>> {
            let string_class = env.find_class("java/lang/String")?;
            let digest_array =
                env.new_object_array(digests.len() as jsize, string_class, JObject::null())?;
            for (i, digest) in digests.iter().enumerate() {
                let jstring_digest = env.new_string(digest)?;
                env.set_object_array_element(digest_array, i as jsize, jstring_digest)?;
            }

            let result = env.call_method(
                object,
                "setDigests",
                "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
                &[digest_array.into()],
            )?;
            Ok(result.l()?)
        }

        pub fn setEncryptionPaddings(
            env: &'borrow JNIEnv<'env>,
            object: JObject<'env>,
            paddings: Vec<String>,
        ) -> JniResult<JObject<'env>> {
            let string_class = env.find_class("java/lang/String")?;
            let padding_array =
                env.new_object_array(paddings.len() as jsize, string_class, JObject::null())?;
            for (i, padding) in paddings.iter().enumerate() {
                let jstring_padding = env.new_string(padding)?;
                env.set_object_array_element(padding_array, i as jsize, jstring_padding)?;
            }

            let result = env.call_method(
                object,
                "setEncryptionPaddings",
                "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
                &[padding_array.into()],
            )?;
            Ok(result.l()?)
        }

        pub fn build(
            env: &'borrow JNIEnv<'env>,
            object: JObject<'env>,
        ) -> JniResult<KeyGenParameterSpec<'env, 'borrow>> {
            let result = env.call_method(
                object,
                "build",
                "()Landroid/security/keystore/KeyGenParameterSpec;",
                &[],
            )?;
            Ok(KeyGenParameterSpec {
                raw: AutoLocal::new(env, result.l()?),
            })
        }
    }
}
