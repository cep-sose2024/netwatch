use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use log::debug;
    use robusta_jni::{
        convert::{IntoJavaValue, Signature as JavaSignature, TryFromJavaValue, TryIntoJavaValue},
        jni::{
            errors::Result as JniResult,
            objects::{AutoLocal, JObject, JString, JValue},
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
            let class_obj =
                env.call_method(self.raw.as_obj(), "getClass", "()Ljava/lang/Class;", &[])?;
            let class_name =
                env.call_method(class_obj.l()?, "getName", "()Ljava/lang/String;", &[])?;
            let class_name: String = env
                .get_string(JString::from(class_name.l()?))
                .unwrap()
                .into();
            debug!("Object class: {}", class_name);

            env.call_method(
                self.raw.as_obj(),
                "initSign",
                "(Ljava/security/PrivateKey;)V",
                &[JValue::from(privateKey)],
            )?;

            Ok(())
        }

        pub extern "java" fn toString(&self, _env: &JNIEnv) -> JniResult<String> {}
        pub extern "java" fn update(&self, env: &JNIEnv, data: Box<[u8]>) -> JniResult<()> {}
    }
}
