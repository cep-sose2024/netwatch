pub(crate) mod thread_func;

use robusta_jni::bridge;
use robusta_jni::jni::objects::GlobalRef;
use robusta_jni::jni::JavaVM;
use std::sync::OnceLock;

static APP_CONTEXT: OnceLock<(JavaVM, GlobalRef)> = OnceLock::new();

#[bridge]
mod jni {
    use crate::APP_CONTEXT;
    use android_logger::Config;
    use log::{debug, info};
    use robusta_jni::convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue};
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::objects::AutoLocal;
    use robusta_jni::jni::objects::{GlobalRef, JObject, JValue};
    use robusta_jni::jni::JNIEnv;
    use std::thread;

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(java.security.KeyPairGenerator)]
    pub struct KeyPairGenerator<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.example.greetings)]
    pub struct CryptoLayer<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> CryptoLayer<'env, 'borrow> {
        pub extern "jni" fn genKeyInRust(
            self,
            env: &JNIEnv,
            algorithm: String,
            provider: String,
        ) -> JniResult<String> {
            let key_pair_generator_obj: JObject<'_> =
                self.get_key_pair_generator(&env, algorithm, provider);
            let key_pair_generator_string: JValue<'_> = env
                .call_method(
                    key_pair_generator_obj,
                    "toString",
                    "()Ljava/lang/String;",
                    &[],
                )
                .unwrap();

            let output = env
                .get_string(robusta_jni::jni::objects::JString::from(
                    key_pair_generator_string.l().unwrap(),
                ))
                .unwrap();
            let output = output.to_str().unwrap().to_string();

            Ok(output)
        }

        fn get_key_pair_generator(
            &self,
            env: &'env JNIEnv,
            algorithm: String,
            provider: String,
        ) -> JObject<'env> {
            let key_pair_generator_class =
                env.find_class("java/security/KeyPairGenerator").unwrap();
            let _ = APP_CONTEXT.set((
                env.get_java_vm().unwrap(),
                env.new_global_ref(key_pair_generator_class).unwrap(),
            ));

            let rsa_string = env.new_string(algorithm).unwrap();
            let android_key_store = env.new_string(provider).unwrap();
            let rsa_jvalue = JValue::from(rsa_string);
            let aks_jvalue = JValue::from(android_key_store);
            let key_pair_generator_call = env.call_static_method(
                "java/security/KeyPairGenerator",
                "getInstance",
                "(Ljava/lang/String;Ljava/lang/String;)Ljava/security/KeyPairGenerator;",
                &[rsa_jvalue, aks_jvalue],
            );

            debug!(
                "KeyPairGenerator.getInstance call OK: {}",
                key_pair_generator_call.is_ok()
            );

            let key_pair_generator_obj: JObject<'_> = key_pair_generator_call.unwrap().l().unwrap();

            key_pair_generator_obj
        }

        pub extern "java" fn generateNewKey(env: &JNIEnv) -> JniResult<()> {}
        pub extern "java" fn encryptText(env: &JNIEnv, text: String) -> JniResult<String> {}
        pub extern "java" fn decryptText(env: &JNIEnv, text: String) -> JniResult<String> {}
    }

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.example.greetings)]
    pub struct RobustaAndroidExample<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> RobustaAndroidExample<'env, 'borrow> {
        pub extern "jni" fn runRustExample(self, env: &JNIEnv, context: JObject<'env>) {
            android_logger::init_once(
                Config::default()
                    .with_tag("RUST_ROBUSTA_ANDROID_EXAMPLE")
                    .with_max_level(log::LevelFilter::Debug),
            );

            info!("TEST START");
            let java_class = env
                .find_class("com/example/greetings/RobustaAndroidExample")
                .unwrap();
            let _ = APP_CONTEXT.set((
                env.get_java_vm().unwrap(),
                env.new_global_ref(java_class).unwrap(),
            ));

            let app_files_dir = RobustaAndroidExample::getAppFilesDir(env, context).unwrap();
            info!("App files dir: {}", app_files_dir);

            assert_eq!(
                RobustaAndroidExample::threadTestNoClass(env, "test".to_string()).unwrap(),
                10
            );

            let test_string = env.new_string("SUPER TEST").unwrap();
            let test_string = JValue::from(test_string);
            let met_call = env.call_static_method(
                "com/example/greetings/RobustaAndroidExample",
                "threadTestNoClass",
                "(Ljava/lang/String;)I",
                &[test_string],
            );
            assert!(met_call.is_ok());

            let thread_handler = thread::Builder::new()
                .name("test_thread_fail".to_string())
                .spawn(move || crate::thread_func::thread_test_fail());
            let join_res = thread_handler.unwrap().join().unwrap();
            assert!(join_res.is_err());

            let thread_handler = thread::Builder::new()
                .name("test_thread_good".to_string())
                .spawn(move || crate::thread_func::thread_test_good());
            let join_res = thread_handler.unwrap().join().unwrap();
            assert!(join_res.is_ok());

            info!("TEST END");
        }

        pub extern "java" fn getAppFilesDir(
            env: &JNIEnv,
            #[input_type("Landroid/content/Context;")] context: JObject,
        ) -> JniResult<String> {
        }

        pub extern "java" fn threadTestNoClass(env: &JNIEnv, s: String) -> JniResult<i32> {}
        pub extern "java" fn threadTestWithClass(
            env: &JNIEnv,
            class_ref: &GlobalRef,
            s: String,
        ) -> JniResult<i32> {
        }
    }
}
