extern crate netwatch;
use netwatch::utils::logging::hello_rust;

#[test]
fn test_hello_rust_integration() {
    assert_eq!(hello_rust(), "Hello Rust");
}
