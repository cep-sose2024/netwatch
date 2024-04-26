mod crypto_layer;
mod key_generation;
mod key_store;
mod logger;
pub(crate) mod thread_func;

use robusta_jni::bridge;
use robusta_jni::jni::objects::GlobalRef;
use robusta_jni::jni::JavaVM;
use std::sync::OnceLock;

static APP_CONTEXT: OnceLock<(JavaVM, GlobalRef)> = OnceLock::new();

#[bridge]
mod jni {
    use crate::logger::init_android_logger;
    use crate::APP_CONTEXT;
    use log::info;
    use robusta_jni::convert::{IntoJavaValue, Signature, TryFromJavaValue, TryIntoJavaValue};
    use robusta_jni::jni::errors::Result as JniResult;
    use robusta_jni::jni::objects::AutoLocal;
    use robusta_jni::jni::objects::{GlobalRef, JObject, JValue};
    use robusta_jni::jni::JNIEnv;
    use std::thread;

    #[derive(Signature, TryIntoJavaValue, IntoJavaValue, TryFromJavaValue)]
    #[package(com.example.greetings)]
    pub struct RobustaAndroidExample<'env: 'borrow, 'borrow> {
        #[instance]
        raw: AutoLocal<'env, 'borrow>,
    }

    impl<'env: 'borrow, 'borrow> RobustaAndroidExample<'env, 'borrow> {
        pub extern "jni" fn runRustExample(self, env: &JNIEnv, context: JObject<'env>) {
            init_android_logger("RUST_ROBUSTA_ANDROID_EXAMPLE", None);

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
