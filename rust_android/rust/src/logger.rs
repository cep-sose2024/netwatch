
use jni::objects::{JClass, JObject, JValue};

use jni::JNIEnv;

pub struct JavaLogger<'a> {
    log_class: JClass<'a>,
    tag: String,
}


impl<'a> JavaLogger<'a> {
    pub fn new(env: &'a mut JNIEnv, tag: &str) -> Result<Self, jni::errors::Error> {
        Ok(Self {
            log_class: env.find_class("android/util/Log")?,
            tag: tag.to_owned(),
        })
    }

    pub fn debug(&mut self, env: &mut JNIEnv, message: impl AsRef<str>) -> Result<(), jni::errors::Error>{

        env.call_static_method(
            &self.log_class,
            "d",
            "(Ljava/lang/String;Ljava/lang/String;)I",
            &[
                JValue::Object(&JObject::from(env.new_string(&self.tag)?)),
                JValue::Object(&JObject::from(env.new_string(message)?)),
            ],
        )?;
        Ok(())
    }

}