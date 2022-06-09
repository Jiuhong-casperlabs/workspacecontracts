#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};

use casper_types::{account::AccountHash, ApiError, AsymmetricType, CLValue, PublicKey};

#[no_mangle]
pub extern "C" fn test_with_restriction() {}

#[no_mangle]
pub extern "C" fn test2() {}

#[no_mangle]
pub extern "C" fn get_uref() {
    let account_hash_str = runtime::get_caller().to_string();

    let account_hash_uref = match runtime::get_key(&account_hash_str) {
        Some(uref) => uref.into_uref().unwrap(),
        None => runtime::revert(ApiError::User(1)),
    };

    let return_value = CLValue::from_t(account_hash_uref).unwrap_or_revert();
    runtime::ret(return_value)
}

#[no_mangle]
pub extern "C" fn call() {
    let public_key: PublicKey =
        PublicKey::from_hex("0119bf44096984cdfe8541bac167dc3b96c85086aa30b6b6cb0c5c38ad703166e1")
            .unwrap();
    let account_hash = AccountHash::from_public_key(&public_key, runtime::blake2b);

    runtime::put_key("key1", account_hash.into());

    let public_key1: PublicKey =
        PublicKey::from_hex("02025977ea84931dc9fa09c0ecd899c8716162512d0b2384477a015803ab375a8615")
            .unwrap();
    let account_hash1 = AccountHash::from_public_key(&public_key1, runtime::blake2b);

    runtime::put_key("key2", account_hash1.into())
}
