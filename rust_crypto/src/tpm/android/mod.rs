pub mod knox;
pub(crate) mod wrapper;

use robusta_jni::jni::JavaVM;
use tracing::{info, instrument};

use crate::common::crypto::algorithms::hashes::Hash;
use crate::common::crypto::KeyUsage;
use crate::common::error::SecurityModuleError;
use crate::common::traits::key_handle::KeyHandle;
use crate::common::{
    crypto::algorithms::encryption::{AsymmetricEncryption, BlockCiphers},
    traits::module_provider::Provider,
};

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
    fn create_key(
        &mut self,
        key_id: &str,
    ) -> Result<(), crate::common::error::SecurityModuleError> {
        info!("generating key!");
        let env = self.vm.as_ref().unwrap().get_env().unwrap();

        let kps = wrapper::key_generation::builder::Builder::new(&env, "KEY".to_owned(), 1 | 2)
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
    fn load_key(&mut self, key_id: &str) -> Result<(), crate::common::error::SecurityModuleError> {
        Ok(())
    }

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
        self.vm = Some(get_java_vm().map_err(|e| SecurityModuleError::Tpm(e))?);
        Ok(())
    }
}

impl KeyHandle for AndroidProvider {
    fn sign_data(&self, data: &[u8]) -> Result<Vec<u8>, SecurityModuleError> {
        todo!()
    }

    fn decrypt_data(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, SecurityModuleError> {
        let env = self.vm.as_ref().unwrap().get_env().unwrap();

        let keystore = wrapper::key_store::key_store::jni::KeyStore::getInstance(
            &env,
            "AndroidKeyStore".to_owned(),
        )
        .unwrap();
        keystore.load(&env, None).unwrap();

        let key = keystore.getKey(&env, "KEY".to_owned(), None).unwrap();

        let cipher = wrapper::key_store::cipher::jni::Cipher::getInstance(
            &env,
            "RSA/ECB/PKCS1Padding".to_owned(),
        )
        .unwrap();
        cipher.init(&env, 2, key.raw.as_obj()).unwrap();

        let decrypted = cipher.doFinal(&env, encrypted_data.to_vec()).unwrap();

        Ok(decrypted)
    }

    fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, SecurityModuleError> {
        let env = self.vm.as_ref().expect("ECode1").get_env().expect("ECode2");

        let keystore = wrapper::key_store::key_store::jni::KeyStore::getInstance(
            &env,
            "AndroidKeyStore".to_owned(),
        )
        .expect("ECode3");

        keystore.load(&env, None).unwrap();

        let key = keystore
            .getCertificate(&env, "KEY".to_owned())
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

    fn verify_signature(
        &self,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, SecurityModuleError> {
        todo!()
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
