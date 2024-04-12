mod api;
mod crypto_abstraction;
mod hsm_wrapper;

use netwatch::utils;

fn main() {
    println!("{}", utils::logging::hello_rust());
}
