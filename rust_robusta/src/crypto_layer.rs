use robusta_jni::bridge;

#[bridge]
mod jni {
    use crate::key_generation::builder::Builder;
    use crate::key_generation::key_pair_generator::jni::KeyPairGenerator;
    use crate::key_generation::secure_random::jni::SecureRandom;
    use crate::logger::init_android_logger;
    use log::{debug, error};
    use robusta_jni::convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue};
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::objects::{AutoLocal, JValue};
    use robusta_jni::jni::JNIEnv;

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.example.greetings)]
    pub struct CryptoLayer<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> CryptoLayer<'env, 'borrow> {
        pub extern "jni" fn generateNewKeyRust(
            self,
            env: &JNIEnv,
            algorithm: String,
            provider: String,
        ) -> JniResult<String> {
            init_android_logger("KEY_GEN_TEST", None);

            let kpg = KeyPairGenerator::getInstance(env, algorithm, provider).unwrap();
            let output = kpg.toString(env).unwrap();
            debug!("KeyPairGenerator.toString(): {}", output);

            let key_gen_param_spec = Builder::new(env, "test".to_string(), 1 | 2)
                .unwrap()
                .set_digests(env, vec!["SHA-256".to_string(), "SHA-512".to_string()])
                .unwrap()
                .set_encryption_paddings(env, vec!["PKCS1Padding".to_string()])
                .unwrap()
                .build(env)
                .unwrap();

            let sr: SecureRandom<'_, '_> = SecureRandom::new(env).unwrap();
            let sr_alg = sr.getAlgorithm(env).unwrap();
            debug!("SecureRandom algorithm: {}", sr_alg);

            // let digests = key_gen_param_spec.getDigests(env);
            let key_gen_param_spec_obj = key_gen_param_spec.raw.as_obj();

            if let Err(e) = env.call_method(
                kpg.raw.as_obj(),
                "initialize",
                "(Ljava/security/spec/AlgorithmParameterSpec;Ljava/security/SecureRandom;)V",
                &[
                    JValue::Object(key_gen_param_spec_obj),
                    JValue::Object(sr.raw.as_obj()),
                ],
            ) {
                error!("Couldn't call method via classic JNI: {}", e);
                if env.exception_check().unwrap_or(false) {
                    let ex = env.exception_occurred().unwrap();
                    let _ = env.exception_clear();
                    let res = env
                        .call_method(ex, "toString", "()Ljava/lang/String;", &[])
                        .unwrap()
                        .l()
                        .unwrap();
                    let ex_msg: String = env.get_string(Into::into(res)).unwrap().into();
                    error!("check_jni_error: {}", ex_msg);
                }
            }

            let output = kpg.getAlgorithm(env).unwrap();
            debug!("KeyPairGenerator.getAlgorithm(): {}", output);

            let key_pair = kpg.generateKeyPair(env).unwrap();
            let public = key_pair.getPublic(env).unwrap();
            let private = key_pair.getPrivate(env).unwrap();

            let public_alg = public.getAlgorithm(env).unwrap();
            let private_alg = private.getAlgorithm(env).unwrap();
            debug!("Public Alg: {}, Private Alg: {}", public_alg, private_alg);
            let public_format = public.toString(env).unwrap();
            let private_format = private.toString(env).unwrap();
            debug!("PublicKey: {}", public_format);
            debug!("PrivateKey: {}", private_format);

            Ok("output".to_string())
        }

        pub extern "java" fn encryptText(env: &JNIEnv, text: String) -> JniResult<String> {}
        pub extern "java" fn decryptText(env: &JNIEnv, text: String) -> JniResult<String> {}
    }
}
