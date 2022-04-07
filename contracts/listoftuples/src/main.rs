#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use casper_contract::contract_api::{runtime, storage};
use casper_types::Key;

#[no_mangle]
pub extern "C" fn call() {
    let mut a = Vec::new();
    let value1 = ("key1", "value1");
    let value2 = ("key2", "value2");

    a.push(value1);
    a.push(value2);

    runtime::put_key("listoftuples", storage::new_uref(a).into());
}
