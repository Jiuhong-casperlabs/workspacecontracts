#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use casper_contract::contract_api::{runtime, storage};

#[no_mangle]
pub extern "C" fn call() {
    let mut a = Vec::new();
    let mut value1 = BTreeMap::new();

    value1.insert("key1", "value1");
    value1.insert("key2", "value2");
    a.push(value1);

    let mut value2 = BTreeMap::new();

    value2.insert("key3", "value3");
    value2.insert("key4", "value4");
    a.push(value2);

    runtime::put_key("listofmaps", storage::new_uref(a).into());
}
