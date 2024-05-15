use crate::tpm::android::*;

#[test]
fn test_new_function() {
    // Create a test key ID
    let test_key_id = String::from("test-key-id");

    // Call the new function with the test key ID
    let key = Key::new(test_key_id.clone());

    // Assert that the key ID is set correctly
    assert_eq!(key.key_id, test_key_id);

    // Assert that other fields are None
    assert!(key.key_algo.is_none());
    assert!(key.sym_algo.is_none());
    assert!(key.hash.is_none());
    assert!(key.key_usages.is_none());
    assert!(key.vm.is_none());
}
