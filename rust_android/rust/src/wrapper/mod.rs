use std::rc::Rc;

use jni::{objects::{JObject, JValue, JValueGen}, sys::jstring, JavaVM};


enum KeyProperties {
    Rsa,
    Ec,
}

impl KeyProperties {
    fn to_str(&self) -> &str {
        match self {
            KeyProperties::Rsa => "RSA",
            KeyProperties::Ec => "EC",
        }
    }
}

struct KeyPairGenerator<'a> {
    inner: JObject<'a>,
    vm: &'a JavaVM,
}

impl<'a> KeyPairGenerator<'a> {
    fn get_instance(vm: &'a JavaVM, key_props: KeyProperties, keystore: &str) -> Result<KeyPairGenerator<'a>, jni::errors::Error> {
        let mut env = vm.get_env()?;

        let key_pair_generator_obj = env.call_static_method(
            "java/security/KeyPairGenerator",
            "getInstance",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/security/KeyPairGenerator;",
            &[
                JValue::Object(&JObject::from(env.new_string(key_props.to_str())?)),
                JValue::Object(&JObject::from(env.new_string(keystore)?)),
            ],
        )?;

        Ok(KeyPairGenerator{
            inner: key_pair_generator_obj.l()?,
            vm: vm,
        })
    }

    fn initialize(&self, param_spec: AlgorithmParameterSpec) -> Result<(), jni::errors::Error> {
        let mut env = self.vm.get_env()?;
        env.call_method(
            &self.inner,
            "initialize",
            "(I)V",  // TODO: figure out what the actual signature is
            &[JValue::Object(&param_spec.inner)],
        )?;
        Ok(())
    }
}

struct AlgorithmParameterSpec<'a> {
    inner: JObject<'a>,
}

