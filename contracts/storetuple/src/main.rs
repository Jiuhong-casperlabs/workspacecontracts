#![no_main]
#![no_std]

extern crate alloc;

use casper_contract::contract_api::{runtime, storage};

#[no_mangle]
pub extern "C" fn call() {
    let uref = storage::new_uref((22, true, "hello"));

    runtime::put_key("teststoretuple", uref.into());
}
