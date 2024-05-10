#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_key_success() {
        let mut provider = AndroidProvider::new("test_key".to_string());
        provider
            .initialize_module(
                AsymmetricEncryption::Rsa(2048),
                None,
                Some(Hash::Sha256),
                vec![KeyUsage::SignEncrypt],
            )
            .unwrap();

        let result = provider.create_key("test_key");
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_key_not_initialized() {
        let mut provider = AndroidProvider::new("test_key".to_string());

        let result = provider.create_key("test_key");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            SecurityModuleError::InitializationError("Module is not initialized".to_owned())
        );
    }

    #[test]
    fn test_create_key_invalid_algorithm() {
        let mut provider = AndroidProvider::new("test_key".to_string());
        provider
            .initialize_module(
                AsymmetricEncryption::Ecc(256),
                None,
                Some(Hash::Sha256),
                vec![KeyUsage::SignEncrypt],
            )
            .unwrap();

        let result = provider.create_key("test_key");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            SecurityModuleError::InternalError("Invalid algorithm for key generation".to_owned())
        );
    }

    #[test]
    fn test_create_key_invalid_digest() {
        let mut provider = AndroidProvider::new("test_key".to_string());
        provider
            .initialize_module(
                AsymmetricEncryption::Rsa(2048),
                None,
                Some(Hash::Sha1),
                vec![KeyUsage::SignEncrypt],
            )
            .unwrap();

        let result = provider.create_key("test_key");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            SecurityModuleError::InternalError("Invalid digest for key generation".to_owned())
        );
    }
}
