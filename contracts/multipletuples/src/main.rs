#![no_std]
#![no_main]

use casper_contract::contract_api::{runtime, storage};

#[no_mangle]
pub extern "C" fn call() {
    // for (k, _) in runtime::list_named_keys() {
    //     runtime::remove_key(&k);
    // }

    let value1 = (123, true, (1, 2, ((11, 22, (111, "hello", 333)), 5, 6)));

    let uref = storage::new_uref(value1);

    runtime::put_key("mutipletuple", uref.into());
}

//reference https://github.com/casper-network/casper-node/blob/e2027dbe979ebf91f10ba8a90ffba1fddbd9fb09/types/src/cl_value/jsonrepr.rs
