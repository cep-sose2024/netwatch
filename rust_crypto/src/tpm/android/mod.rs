pub(crate) mod error;
pub mod knox;
pub(crate) mod wrapper;

use log::{debug, info};
use robusta_jni::jni::objects::JObject;
use robusta_jni::jni::JavaVM;
use tracing::instrument;

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
use crate::tpm::core::error::ToTpmError;
use crate::tpm::core::error::TpmError;

use self::wrapper::get_java_vm;

const ANDROID_KEYSTORE: &str = "AndroidKeyStore";

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
    fn create_key(&mut self, key_id: &str) -> Result<(), SecurityModuleError> {
        android_logger::init_once(
            android_logger::Config::default()
                .with_tag("CRYPTO_LAYER_LOG")
                .with_max_level(log::LevelFilter::Debug),
        );

        info!("generating key! {}", key_id);
        info!("generating key!");

        // error if not initialized
        let key_algo = self
            .key_algo
            .as_ref()
            .ok_or(SecurityModuleError::InitializationError(
                "Module is not initialized".to_owned(),
            ))?;

        // check if we need RSA or EC
        let algorithm = match key_algo {
            AsymmetricEncryption::Rsa(_) => "RSA",
            AsymmetricEncryption::Ecc(_) => "EC",
        };

        let env = self
            .vm
            .as_ref()
            .expect("cannot happen, already checked")
            .get_env()
            .map_err(|_| {
                TpmError::InitializationError(
                    "Could not get java environment, this should never happen".to_owned(),
                )
            })?;

        let kps =
            wrapper::key_generation::builder::Builder::new(&env, key_id.to_owned(), 1 | 2 | 4 | 8)
                .err_internal()?
                .set_digests(&env, vec!["SHA-256".to_owned(), "SHA-512".to_owned()])
                .err_internal()?
                .set_encryption_paddings(&env, vec!["PKCS1Padding".to_owned()])
                .err_internal()?
                .build(&env)
                .err_internal()?;

        debug!("Key Id: {}", key_id);
        let algorithm = if key_id == "KEY" { "RSA" } else { "EC" };
        debug!("Key algorithm: {}", algorithm);

        let kpg = wrapper::key_generation::key_pair_generator::jni::KeyPairGenerator::getInstance(
            &env,
            algorithm.to_owned(),
            ANDROID_KEYSTORE.to_owned(),
        )
        .err_internal()?;

        kpg.initialize(&env, kps.raw.as_obj()).err_internal()?;

        let key_pair = kpg.generateKeyPair(&env).unwrap();
        let public = key_pair.getPublic(&env).unwrap();
        let private = key_pair.getPrivate(&env).unwrap();

        let public_alg = public.getAlgorithm(&env).unwrap();
        let private_alg = private.getAlgorithm(&env).unwrap();
        debug!("Public Alg: {}, Private Alg: {}", public_alg, private_alg);
        let public_format = public.toString(&env).unwrap();
        let private_format = private.toString(&env).unwrap();
        debug!("PublicKey: {}", public_format);
        debug!("PrivateKey: {}", private_format);

        debug!("key generated");

        Ok(())
    }

    #[instrument]
    fn load_key(&mut self, key_id: &str) -> Result<(), SecurityModuleError> {
        self.key_id = key_id.to_owned();
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
        self.vm = Some(get_java_vm()?);
        Ok(())
    }
}

impl KeyHandle for AndroidProvider {
    #[instrument]
    fn sign_data(&self, data: &[u8]) -> Result<Vec<u8>, SecurityModuleError> {
        // check that signing is allowed
        if !self
            .key_usages
            .as_ref()
            .ok_or(SecurityModuleError::InitializationError(
                "Module is not initialized".to_owned(),
            ))?
            .contains(&KeyUsage::SignEncrypt)
        {
            return Err(TpmError::UnsupportedOperation(
                "KeyUsage::SignEncrypt was not provided".to_owned(),
            )
            .into());
        }

        let env = self
            .vm
            .as_ref()
            .ok_or_else(|| TpmError::InitializationError("Module is not initialized".to_owned()))?
            .get_env()
            .map_err(|_| {
                TpmError::InitializationError(
                    "Could not get java environment, this should never happen".to_owned(),
                )
            })?;

        let key_store = KeyStore::getInstance(&env, ANDROID_KEYSTORE.to_string()).err_internal()?;
        let key_store_load = key_store.load(&env, None);
        debug!("KeyStore.load() OK: {}", key_store_load.is_ok());

        debug!("self.key_id.clone(): {}", self.key_id.clone());

        key_store.load(&env, None).err_internal()?;

        let private_key = key_store
            .getKey(&env, self.key_id.clone(), JObject::null())
            .err_internal()?;

        let s = Signature::getInstance(&env, "SHA256withECDSA".to_string()).err_internal()?;
        debug!("Signature: {}", s.toString(&env).unwrap());

        s.initSign(&env, private_key.raw.as_obj()).err_internal()?;

        let data_bytes = data.to_vec().into_boxed_slice();

        s.update(&env, data_bytes).err_internal()?;
        debug!("Signature Init: {}", s.toString(&env).unwrap());

        let output = s.sign(&env).err_internal()?;
        debug!("Signature: {:?}", output);

        Ok(output)
    }

    #[instrument]
    fn decrypt_data(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, SecurityModuleError> {
        info!("decrypting data");
        let env = self
            .vm
            .as_ref()
            .ok_or_else(|| TpmError::InitializationError("Module is not initialized".to_owned()))?
            .get_env()
            .map_err(|_| {
                TpmError::InitializationError(
                    "Could not get java environment, this should never happen".to_owned(),
                )
            })?;

        let algorithm = match self.key_algo.as_ref().unwrap() {
            AsymmetricEncryption::Rsa(_) => "RSA",
            AsymmetricEncryption::Ecc(_) => {
                return Err(TpmError::UnsupportedOperation(
                    "EC is not allowed for en/decryption on android".to_owned(),
                )
                .into());
            }
        };

        let key_store = KeyStore::getInstance(&env, ANDROID_KEYSTORE.to_owned()).err_internal()?;
        key_store.load(&env, None).err_internal()?;

        let key = key_store
            .getKey(&env, self.key_id.to_owned(), JObject::null())
            .err_internal()?;

        let cipher = wrapper::key_store::cipher::jni::Cipher::getInstance(
            &env,
            format!("{algorithm}/ECB/PKCS1Padding"),
        )
        .err_internal()?;
        cipher.init(&env, 2, key.raw.as_obj()).err_internal()?;

        let decrypted = cipher
            .doFinal(&env, encrypted_data.to_vec())
            .err_internal()?;

        debug!("decrypted data: {:?}", decrypted);
        Ok(decrypted)
    }

    #[instrument]
    fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, SecurityModuleError> {
        info!("encrypting");
        let env = self
            .vm
            .as_ref()
            .ok_or_else(|| TpmError::InitializationError("Module is not initialized".to_owned()))?
            .get_env()
            .map_err(|_| {
                TpmError::InitializationError(
                    "Could not get java environment, this should never happen".to_owned(),
                )
            })?;

        let algorithm = match self.key_algo.as_ref().unwrap() {
            AsymmetricEncryption::Rsa(_) => "RSA",
            AsymmetricEncryption::Ecc(_) => {
                return Err(TpmError::UnsupportedOperation(
                    "EC is not allowed for en/decryption on android".to_owned(),
                )
                .into());
            }
        };

        let key_store = KeyStore::getInstance(&env, ANDROID_KEYSTORE.to_owned()).err_internal()?;

        let key_store_load = key_store.load(&env, None);
        debug!("KeyStore.load() OK: {}", key_store_load.is_ok());

        let key = key_store
            .getCertificate(&env, self.key_id.to_owned())
            .err_internal()?
            .getPublicKey(&env)
            .err_internal()?;

        let public_alg = key.getAlgorithm(&env).unwrap();
        debug!("Public Alg: {}", public_alg);

        let cipher = wrapper::key_store::cipher::jni::Cipher::getInstance(
            &env,
            format!("{algorithm}/ECB/PKCS1Padding"),
        )
        .err_internal()?;

        cipher.init(&env, 1, key.raw.as_obj()).err_internal()?;

        let encrypted = cipher.doFinal(&env, data.to_vec()).err_internal()?;

        debug!("encrypted: {:?}", encrypted);
        Ok(encrypted)
    }

    #[instrument]
    fn verify_signature(&self, data: &[u8], signature: &[u8]) -> Result<bool, SecurityModuleError> {
        info!("verifiying");
        let env = self
            .vm
            .as_ref()
            .ok_or_else(|| TpmError::InitializationError("Module is not initialized".to_owned()))?
            .get_env()
            .map_err(|_| {
                TpmError::InitializationError(
                    "Could not get java environment, this should never happen".to_owned(),
                )
            })?;

        let key_store = KeyStore::getInstance(&env, ANDROID_KEYSTORE.to_string()).err_internal()?;
        let key_store_load = key_store.load(&env, None);
        debug!("KeyStore.load() OK: {}", key_store_load.is_ok());

        let s = Signature::getInstance(&env, "SHA256withECDSA".to_string()).err_internal()?;
        debug!("Signature: {}", s.toString(&env).unwrap());

        let cert = key_store
            .getCertificate(&env, self.key_id.clone())
            .err_internal()?;
        debug!("Signature: {}", cert.toString(&env).unwrap());

        s.initVerify(&env, cert).err_internal()?;
        debug!("Signature Init: {}", s.toString(&env).unwrap());

        let data_bytes = data.to_vec().into_boxed_slice();
        s.update(&env, data_bytes).err_internal()?;

        let signature_boxed = signature.to_vec().into_boxed_slice();
        let output = s.verify(&env, signature_boxed).err_internal()?;
        debug!("Signature verified: {:?}", output);

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
