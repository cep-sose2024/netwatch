use crate::tpm::android::wrapper::key_generation::key_gen_parameter_spec::jni::KeyGenParameterSpec;

use robusta_jni::jni::errors::Result as JniResult;
use robusta_jni::jni::objects::{AutoLocal, JObject, JValue};
use robusta_jni::jni::sys::jsize;
use robusta_jni::jni::JNIEnv;

pub struct Builder<'env: 'borrow, 'borrow> {
    raw: AutoLocal<'env, 'borrow>,
}

impl<'env: 'borrow, 'borrow> Builder<'env, 'borrow> {
    pub fn new(
        env: &'borrow JNIEnv<'env>,
        keystore_alias: String,
        purposes: i32,
    ) -> JniResult<Self> {
        let class = env.find_class("android/security/keystore/KeyGenParameterSpec$Builder")?;
        let jstring_keystore_alias = env.new_string(keystore_alias)?;
        let args = [Into::into(jstring_keystore_alias), JValue::from(purposes)];
        let obj = env.new_object(class, "(Ljava/lang/String;I)V", &args)?;
        Ok(Self {
            raw: AutoLocal::new(env, Into::<JObject>::into(obj)),
        })
    }

    pub fn set_digests(
        mut self,
        env: &'borrow JNIEnv<'env>,
        digests: Vec<String>,
    ) -> JniResult<Self> {
        let string_class = env.find_class("java/lang/String")?;
        let digest_array =
            env.new_object_array(digests.len() as jsize, string_class, JObject::null())?;
        for (i, digest) in digests.iter().enumerate() {
            let jstring_digest = env.new_string(digest)?;
            env.set_object_array_element(digest_array, i as jsize, jstring_digest)?;
        }

        let result = env.call_method(
            self.raw.as_obj(),
            "setDigests",
            "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
            &[digest_array.into()],
        )?;
        self.raw = AutoLocal::new(env, result.l()?);
        Ok(self)
    }

    pub fn set_encryption_paddings(
        mut self,
        env: &'borrow JNIEnv<'env>,
        paddings: Vec<String>,
    ) -> JniResult<Self> {
        let string_class = env.find_class("java/lang/String")?;
        let padding_array =
            env.new_object_array(paddings.len() as jsize, string_class, JObject::null())?;
        for (i, padding) in paddings.iter().enumerate() {
            let jstring_padding = env.new_string(padding)?;
            env.set_object_array_element(padding_array, i as jsize, jstring_padding)?;
        }

        let result = env.call_method(
            self.raw.as_obj(),
            "setEncryptionPaddings",
            "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
            &[padding_array.into()],
        )?;
        self.raw = AutoLocal::new(env, result.l()?);
        Ok(self)
    }

    pub fn set_signature_paddings(
        mut self,
        env: &'borrow JNIEnv<'env>,
        paddings: Vec<String>,
    ) -> JniResult<Self> {
        let string_class = env.find_class("java/lang/String")?;
        let padding_array =
            env.new_object_array(paddings.len() as jsize, string_class, JObject::null())?;
        for (i, padding) in paddings.iter().enumerate() {
            let jstring_padding = env.new_string(padding)?;
            env.set_object_array_element(padding_array, i as jsize, jstring_padding)?;
        }

        let result = env.call_method(
            self.raw.as_obj(),
            "setSignaturePaddings",
            "([Ljava/lang/String;)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
            &[padding_array.into()],
        )?;
        self.raw = AutoLocal::new(env, result.l()?);
        Ok(self)
    }

    pub fn set_is_strongbox_backed(
        mut self,
        env: &'borrow JNIEnv<'env>,
        is_strongbox_backed: bool,
    ) -> JniResult<Self> {
        let result = env.call_method(
            self.raw.as_obj(),
            "setIsStrongBoxBacked",
            "(Z)Landroid/security/keystore/KeyGenParameterSpec$Builder;",
            &[JValue::Bool(is_strongbox_backed.into())],
        )?;
        self.raw = AutoLocal::new(env, result.l()?);
        Ok(self)
    }

    pub fn build(
        self,
        env: &'borrow JNIEnv<'env>,
    ) -> JniResult<KeyGenParameterSpec<'env, 'borrow>> {
        let result = env.call_method(
            self.raw.as_obj(),
            "build",
            "()Landroid/security/keystore/KeyGenParameterSpec;",
            &[],
        )?;
        Ok(KeyGenParameterSpec {
            raw: AutoLocal::new(env, result.l()?),
        })
    }
}
