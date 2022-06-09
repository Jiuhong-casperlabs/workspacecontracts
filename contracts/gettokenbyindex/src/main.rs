#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, CLTyped, Key, U256};

pub fn key_and_value_to_str<T: CLTyped + ToBytes>(key: &Key, value: &T) -> String {
    let mut bytes_a = key.to_bytes().unwrap_or_revert();
    let mut bytes_b = value.to_bytes().unwrap_or_revert();

    bytes_a.append(&mut bytes_b);

    let bytes = runtime::blake2b(bytes_a);
    hex::encode(bytes)
}

#[no_mangle]
pub extern "C" fn call() {
    let key = Key::from_formatted_str(
        "account-hash-9a770006ffda6f5b40f9f2752e8e82ee4c7f0dc11d1e83ecda5b1d25598195a9",
    )
    .unwrap();
    let str = key_and_value_to_str(&key, &U256::from(0u8));

    // let uref = URef::from_formatted_str(
    //     "uref-25966a58fbb5e54ad519509c844953bb5673f7d4474f2295d7bd8786dfdf7172-007",
    // )
    // .unwrap();

    runtime::put_key("my_str", storage::new_uref(str).into());
}
