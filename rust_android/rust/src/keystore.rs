use jni::JavaVM;

use crate::{get_java_vm, logger::JavaLogger};


pub struct Keystore {
    keyname: String,
    javavm: JavaVM,
}

impl Keystore {
    pub fn generate_new_key(&mut self) {
        let mut env = self.javavm.get_env().unwrap();
        let mut logger = JavaLogger::new(&mut env, "Keystore").unwrap();
        let mut env = self.javavm.get_env().unwrap();
        logger.debug(&mut env, "generating key in rust").expect("Unable to log message");
    }

    pub fn encrypt(&self, text: &[i8]) -> Vec<i8> {
        let mut env = self.javavm.get_env().unwrap();
        let mut logger = JavaLogger::new(&mut env, "Keystore").unwrap();
        let mut env = self.javavm.get_env().unwrap();
        logger.debug(&mut env, "encrypting in rust").expect("Unable to log message");

        "encrypted".as_bytes().iter().map(|&x| x as i8).collect()
    }

    pub fn decrypt(&self, bytes: &[i8]) -> Vec<i8> {
        let mut env = self.javavm.get_env().unwrap();
        let mut logger = JavaLogger::new(&mut env, "Keystore").unwrap();
        let mut env = self.javavm.get_env().unwrap();
        logger.debug(&mut env, "decrypting in rust").expect("Unable to log message");

        "decrypted".as_bytes().iter().map(|&x| x as i8).collect()
    }
    
}


impl Default for Keystore {
    fn default() -> Self {
        let vm = get_java_vm().unwrap();
        Keystore {
            keyname: "key123".to_owned(),
            javavm: vm,
        }
    }
}