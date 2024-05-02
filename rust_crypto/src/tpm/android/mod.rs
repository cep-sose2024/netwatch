pub mod knox;
pub(crate) mod wrapper;

use robusta_jni::jni::objects::JObject;
use robusta_jni::jni::JavaVM;
use tracing::{debug, error, info, instrument};

use crate::common::crypto::algorithms::hashes::Hash;
use crate::common::crypto::KeyUsage;
use crate::common::error::SecurityModuleError;
use crate::common::traits::key_handle::KeyHandle;
use crate::common::{
    crypto::algorithms::encryption::{AsymmetricEncryption, BlockCiphers},
    traits::module_provider::Provider,
};
use crate::tpm::android::wrapper::key_store::key_store::jni::KeyStore;
use crate::tpm::android::wrapper::key_store::signature::jni::Signature;

use self::wrapper::get_java_vm;

/// A TPM-based cryptographic provider for managing cryptographic keys and performing
/// cryptographic operations in a Windows environment.
///
/// This provider leverages the Windows Cryptography API: Next Generation (CNG) to interact
/// with a Trusted Platform Module (TPM) for operations like signing, encryption, and decryption.
/// It provides a secure and hardware-backed solution for managing cryptographic keys and performing
/// cryptographic operations on Windows platforms.
pub(crate) struct AndroidProvider {
    key_id: String,
    key_algo: Option<AsymmetricEncryption>,
    sym_algo: Option<BlockCiphers>,
    hash: Option<Hash>,
    key_usages: Option<Vec<KeyUsage>>,
    vm: Option<JavaVM>,
}

impl AndroidProvider {
    /// Constructs a new `AndroidProvider`.
    ///
    /// # Arguments
    ///
    /// * `key_id` - A string identifier for the cryptographic key to be managed by this provider.
    ///
    /// # Returns
    ///
    /// A new instance of `AndroidProvider` with the specified `key_id`.
    #[instrument]
    pub fn new(key_id: String) -> Self {
        Self {
            key_id,
            key_algo: None,
            sym_algo: None,
            hash: None,
            key_usages: None,
            vm: None,
        }
    }
}

impl Provider for AndroidProvider {
    #[instrument]
    fn create_key(
        &mut self,
        key_id: &str,
    ) -> Result<(), crate::common::error::SecurityModuleError> {
        info!("generating key!");
        let env = self.vm.as_ref().unwrap().get_env().unwrap();

        let kps =
            wrapper::key_generation::builder::Builder::new(&env, key_id.to_owned(), 1 | 2 | 4 | 8)
                .unwrap()
                .set_digests(&env, vec!["SHA-256".to_owned(), "SHA-512".to_owned()])
                .unwrap()
                .set_encryption_paddings(&env, vec!["PKCS1Padding".to_owned()])
                .unwrap()
                .build(&env)
                .unwrap();

        let kpg = wrapper::key_generation::key_pair_generator::jni::KeyPairGenerator::getInstance(
            &env,
            "RSA".to_owned(),
            "AndroidKeyStore".to_owned(),
        )
        .unwrap();

        kpg.initialize(&env, kps.raw.as_obj()).unwrap();

        kpg.generateKeyPair(&env).unwrap();

        Ok(())
    }

    /// As a Provider can only hold one Key, there is no need to load a key,
    #[instrument]
    fn load_key(&mut self, key_id: &str) -> Result<(), crate::common::error::SecurityModuleError> {
        Ok(())
    }

    #[instrument]
    fn initialize_module(
        &mut self,
        key_algorithm: AsymmetricEncryption,
        sym_algorithm: Option<BlockCiphers>,
        hash: Option<Hash>,
        key_usages: Vec<KeyUsage>,
    ) -> Result<(), SecurityModuleError> {
        self.key_algo = Some(key_algorithm);
        self.sym_algo = sym_algorithm;
        self.hash = hash;
        self.key_usages = Some(key_usages);
        self.vm = Some(get_java_vm().map_err(SecurityModuleError::Tpm)?);
        Ok(())
    }
}

impl KeyHandle for AndroidProvider {
    #[instrument]
    fn sign_data(&self, data: &[u8]) -> Result<Vec<u8>, SecurityModuleError> {
        let env = self.vm.as_ref().unwrap().get_env().unwrap();

        let key_store = KeyStore::getInstance(&env, "AndroidKeyStore".to_string()).unwrap();
        let key_store_load = key_store.load(&env, None);
        debug!("KeyStore.load() OK: {}", key_store_load.is_ok());

        let private_key = key_store
            .getKey(&env, "KEY SIGN".to_owned(), JObject::null())
            .unwrap();

        let s = Signature::getInstance(&env, "SHA256withECDSA".to_string()).unwrap();
        debug!("Signature: {}", s.toString(&env).unwrap());

        s.initSign(&env, private_key.raw.as_obj()).unwrap();

        let data_bytes = data.to_vec().into_boxed_slice();

        match s.update(&env, data_bytes) {
            Ok(_) => (),
            Err(e) => error!("Error updating signature: {:?}", e),
        }
        debug!("Signature Init: {}", s.toString(&env).unwrap());

        let output = s.sign(&env).unwrap();
        debug!("Signature: {:?}", output);

        Ok(output)
    }

    #[instrument]
    fn decrypt_data(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, SecurityModuleError> {
        let env = self.vm.as_ref().unwrap().get_env().unwrap();

        let keystore = wrapper::key_store::key_store::jni::KeyStore::getInstance(
            &env,
            "AndroidKeyStore".to_owned(),
        )
        .unwrap();
        keystore.load(&env, None).unwrap();

        let key = keystore
            .getKey(&env, self.key_id.to_owned(), JObject::null())
            .unwrap();

        let cipher = wrapper::key_store::cipher::jni::Cipher::getInstance(
            &env,
            "RSA/ECB/PKCS1Padding".to_owned(),
        )
        .unwrap();
        cipher.init(&env, 2, key.raw.as_obj()).unwrap();

        let decrypted = cipher.doFinal(&env, encrypted_data.to_vec()).unwrap();

        Ok(decrypted)
    }

    #[instrument]
    fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, SecurityModuleError> {
        let env = self.vm.as_ref().expect("ECode1").get_env().expect("ECode2");

        info!("CRYPTO_LAYER: encrypt_data call");

        let keystore = wrapper::key_store::key_store::jni::KeyStore::getInstance(
            &env,
            "AndroidKeyStore".to_owned(),
        )
        .expect("ECode3");

        keystore.load(&env, None).unwrap();

        let key = keystore
            .getCertificate(&env, self.key_id.to_owned())
            .expect("ECode4")
            .getPublicKey(&env)
            .expect("ECode5");

        let cipher = wrapper::key_store::cipher::jni::Cipher::getInstance(
            &env,
            "RSA/ECB/PKCS1Padding".to_owned(),
        )
        .expect("ECode6");

        cipher.init(&env, 1, key.raw.as_obj()).expect("ECode7");

        let encrypted = cipher.doFinal(&env, data.to_vec()).expect("ECode8");

        Ok(encrypted)
    }

    #[instrument]
    fn verify_signature(&self, data: &[u8], signature: &[u8]) -> Result<bool, SecurityModuleError> {
        let env = self.vm.as_ref().expect("ECode1").get_env().expect("ECode2");

        let key_store = KeyStore::getInstance(&env, "AndroidKeyStore".to_string()).unwrap();
        let key_store_load = key_store.load(&env, None);
        debug!("KeyStore.load() OK: {}", key_store_load.is_ok());

        let s = Signature::getInstance(&env, "SHA256withECDSA".to_string()).unwrap();
        debug!("Signature: {}", s.toString(&env).unwrap());

        let cert = key_store
            .getCertificate(&env, "KEY SIGN".to_owned())
            .unwrap();

        match s.initVerify(&env, cert) {
            Ok(_) => (),
            Err(e) => error!("Error initializing verification: {:?}", e),
        }

        let data_bytes = data.to_vec().into_boxed_slice();
        match s.update(&env, data_bytes) {
            Ok(_) => (),
            Err(e) => error!("Error updating signature: {:?}", e),
        }
        debug!("Signature Init: {}", s.toString(&env).unwrap());

        let signature_boxed = signature.to_vec().into_boxed_slice();
        let output = s.verify(&env, signature_boxed).unwrap();
        debug!("Signature: {:?}", output);

        Ok(output)
    }
}

impl std::fmt::Debug for AndroidProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AndroidProvider")
            .field("key_id", &self.key_id)
            .field("key_algo", &self.key_algo)
            .field("sym_algo", &self.sym_algo)
            .field("hash", &self.hash)
            .field("key_usages", &self.key_usages)
            .finish()
    }
}
