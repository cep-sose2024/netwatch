use robusta_jni::bridge;

#[bridge]
pub mod jni {
    use robusta_jni::{
        convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue},
        jni::{
            errors::Result as JniResult,
            objects::{AutoLocal, JObject, JValue},
            sys::jbyteArray,
            JNIEnv,
        },
    };

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(javax.crypto)]
    pub struct Cipher<'env: 'borrow, 'borrow> {
        #[instance]
        pub raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> Cipher<'env, 'borrow> {
        pub extern "java" fn getInstance(
            env: &'borrow JNIEnv<'env>,
            transformation: String,
        ) -> JniResult<Self> {
        }

        pub extern "java" fn init(
            &self,
            env: &'borrow JNIEnv<'env>,
            opmode: i32,
            #[input_type("Ljava/security/Key;")] key: JObject,
        ) -> JniResult<()> {
        }

        pub fn doFinal(&self, env: &JNIEnv, input: Vec<u8>) -> JniResult<Vec<u8>> {
            let input_array = env.byte_array_from_slice(&input)?;

            let output = env.call_method(
                self.raw.as_obj(),
                "doFinal",
                "([B)[B",
                &[JValue::from(input_array)],
            )?;

            let output_array = output.l()?.into_inner() as jbyteArray;
            let output_vec = env.convert_byte_array(output_array).unwrap();

            Ok(output_vec)
        }
    }
}
