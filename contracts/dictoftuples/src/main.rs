#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use casper_contract::contract_api::storage;

#[no_mangle]
pub extern "C" fn call() {
    let value1 = ("key1", "value1");
    let value2 = ("key2", "value2");
    let value3 = ("key3", "value3");
    let value4 = ("key4", "value4");

    let vec_uref = storage::new_dictionary("vec_tuple_uref").unwrap();
    storage::dictionary_put(vec_uref, "value1", value1);
    storage::dictionary_put(vec_uref, "value2", value2);
    storage::dictionary_put(vec_uref, "value3", value3);
    storage::dictionary_put(vec_uref, "value4", value4);
}
