use robusta_jni::bridge;

#[bridge]
mod jni {
    use crate::key_generation::builder::Builder;
    use crate::key_generation::key_pair_generator::jni::KeyPairGenerator;
    use crate::key_store::cipher::jni::Cipher;
    use crate::key_store::key_store::jni::KeyStore;
    use crate::key_store::signature::jni::Signature as SignatureJni;
    use crate::logger::init_android_logger;
    use base64::engine::general_purpose;
    use base64::Engine;
    use log::{debug, error};
    use robusta_jni::convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue};
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::objects::{AutoLocal, JObject};
    use robusta_jni::jni::JNIEnv;

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.example.greetings)]
    pub struct CryptoLayer<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> CryptoLayer<'env, 'borrow> {
        pub extern "jni" fn verifyDataRust(
            self,
            env: &JNIEnv,
            data: String,
            signature: Box<[u8]>,
            key_name: String,
        ) -> JniResult<bool> {
            let key_store = KeyStore::getInstance(env, "AndroidKeyStore".to_string()).unwrap();
            let key_store_load = key_store.load(env, None);
            debug!("KeyStore.load() OK: {}", key_store_load.is_ok());

            let s = SignatureJni::getInstance(env, "SHA256withECDSA".to_string()).unwrap();
            debug!("Signature: {}", s.toString(env).unwrap());

            let cert = key_store.getCertificate(env, key_name.to_owned()).unwrap();

            s.initVerify(env, cert).map_err(|e| {
                error!("Error initializing verification: {:?}", e);
                e
            })?;

            let data_bytes = data.clone().into_bytes().into_boxed_slice();
            match s.update(env, data_bytes) {
                Ok(_) => (),
                Err(e) => error!("Error updating signature: {:?}", e),
            }
            debug!("Signature Init: {}", s.toString(env).unwrap());

            let output = s.verify(env, signature).unwrap();
            debug!("Signature: {:?}", output);

            Ok(output)
        }

        pub extern "jni" fn signDataRust(
            self,
            env: &JNIEnv,
            data: String,
            key_name: String,
        ) -> JniResult<Box<[u8]>> {
            init_android_logger("RUST_SIGN_DATA", None);

            let key_store = KeyStore::getInstance(env, "AndroidKeyStore".to_string()).unwrap();
            let key_store_load = key_store.load(env, None);
            debug!("KeyStore.load() OK: {}", key_store_load.is_ok());

            let private_key = key_store
                .getKey(env, key_name.to_owned(), JObject::null())
                .unwrap();

            let s = SignatureJni::getInstance(env, "SHA256withECDSA".to_string()).unwrap();
            debug!("Signature: {}", s.toString(env).unwrap());

            let _ = s.initSign(env, private_key.raw.as_obj());

            let data_bytes = data.clone().into_bytes().into_boxed_slice();
            match s.update(env, data_bytes) {
                Ok(_) => (),
                Err(e) => error!("Error updating signature: {:?}", e),
            }
            debug!("Signature Init: {}", s.toString(env).unwrap());

            let output = s.sign(env).unwrap();
            debug!("Signature: {:?}", output);

            Ok(output)
        }

        pub extern "jni" fn encryptTextRust(self, env: &JNIEnv, text: String) -> JniResult<String> {
            init_android_logger("RUST_ENCRYPT_TEXT", None);

            let key_store = KeyStore::getInstance(env, "AndroidKeyStore".to_string()).unwrap();
            let key_store_load = key_store.load(env, None);
            debug!("KeyStore.load() OK: {}", key_store_load.is_ok());

            let public_key = key_store
                .getCertificate(env, "key123".to_string())
                .unwrap()
                .getPublicKey(env)
                .unwrap();

            let public_key_str = public_key.toString(env).unwrap();
            debug!("PublicKey.toString(): {}", public_key_str);

            let cipher = Cipher::getInstance(env, "RSA/ECB/PKCS1Padding".to_string()).unwrap();
            let ciper_init = cipher.init(env, 1, public_key.raw.as_obj());
            debug!("cipher.init() OK: {}", ciper_init.is_ok());

            let text_bytes = text.clone().into_bytes();
            let bytes = cipher.doFinal(env, text_bytes).unwrap();

            let encrypted_text = general_purpose::URL_SAFE.encode(&bytes);
            debug!("Encrypted text: {:?}", encrypted_text);

            Ok(encrypted_text)
        }

        pub extern "jni" fn generateNewKeyRust(
            self,
            env: &JNIEnv,
            key_name: String,
            algorithm: String,
            provider: String,
            purposes: i32,
        ) -> JniResult<String> {
            init_android_logger("RUST_GENERATE_NEW_KEY", None);

            let kpg = KeyPairGenerator::getInstance(env, algorithm, provider).unwrap();
            let output = kpg.toString(env).unwrap();
            debug!("KeyPairGenerator.toString(): {}", output);

            let key_gen_param_spec = Builder::new(env, key_name.to_string(), purposes)
                .unwrap()
                .set_digests(env, vec!["SHA-256".to_string(), "SHA-512".to_string()])
                .unwrap()
                .set_encryption_paddings(env, vec!["PKCS1Padding".to_string()])
                .unwrap()
                .build(env)
                .unwrap();

            let key_gen_param_spec_obj = key_gen_param_spec.raw.as_obj();

            let _ = kpg.initialize(env, key_gen_param_spec_obj);

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
    }
}
