#![no_std]
#![no_main]

extern crate alloc;
use alloc::{collections::BTreeMap, string::String};

use casper_contract::contract_api::{runtime, storage};

#[no_mangle]
pub extern "C" fn call() {
    let key1 = String::from("first");
    let key2 = String::from("second");
    let value1 = (123, true, "hello");
    let value2 = (999, false, "world");
    let mut map = BTreeMap::new();

    map.insert(key1, value1);
    map.insert(key2, value2);

    let uref = storage::new_uref(map);

    runtime::put_key("testmap1", uref.into());
}

//reference https://github.com/casper-network/casper-node/blob/e2027dbe979ebf91f10ba8a90ffba1fddbd9fb09/types/src/cl_value/jsonrepr.rs
