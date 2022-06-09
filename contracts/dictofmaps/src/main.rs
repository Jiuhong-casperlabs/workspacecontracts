#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::collections::BTreeMap;

use casper_contract::contract_api::storage;

#[no_mangle]
pub extern "C" fn call() {
    let mut value1 = BTreeMap::new();
    value1.insert("key1", "value1");
    value1.insert("key2", "value2");

    let mut value2 = BTreeMap::new();
    value2.insert("key3", "value3");
    value2.insert("key4", "value4");

    let vec_uref = storage::new_dictionary("vec_map_uref").unwrap();
    storage::dictionary_put(vec_uref, "value1", value1);
    storage::dictionary_put(vec_uref, "value2", value2);
}
