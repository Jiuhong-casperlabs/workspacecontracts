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
    let value1 = vec![12, 3, 4];
    let value2 = vec![22, 333, 44];

    let a: Vec<Vec<i32>> = vec![value1, value2];
    runtime::put_key("listoflists", storage::new_uref(a).into());
}
