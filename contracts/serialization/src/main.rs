#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use casper_contract::contract_api::{runtime, storage};

// Import
use serde::Deserialize;
// use serde_json_core;

// The field is a ref that will point directly in the parsed buffer
#[derive(Deserialize)]
struct MyStruct<'a> {
    the_str: &'a str,
}

#[no_mangle]
pub extern "C" fn call() {
    // Deserialize buffer
    match serde_json_core::de::from_slice::<MyStruct>(b"{\"the_str\": \"jsoncore\"}") {
        Err(_e) => {
            // Err
        }
        Ok(deser) => {
            // result in deser.0.the_str
            runtime::put_key("test", storage::new_uref(deser.0.the_str).into())
        }
    }
}
