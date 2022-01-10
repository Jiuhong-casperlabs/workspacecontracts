#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};

use casper_contract::contract_api::{
    runtime::{self, remove_key},
    storage,
};
use casper_types::{AccessRights, ContractHash, Key, RuntimeArgs, URef, U256, UREF_ADDR_LENGTH};
use core::convert::{TryFrom, TryInto};

const REPLACEMENT_DATA: &str = "bawitdaba";
const ARG_CONTRACT_HASH: &str = "contract_hash";

#[no_mangle]
pub extern "C" fn call() {
    // let a = storage::new_uref(1);
    // a.with_access_rights(AccessRights::NONE);
    // storage::add(a, 2);

    // // let c = URef::new(b.addr(), AccessRights::NONE);

    // runtime::put_key("stringvalue", a.into());

    let a = storage::new_uref(1);
    let b = a.with_access_rights(AccessRights::NONE);
    runtime::remove_key("stringvalue1");
    runtime::put_key("stringvalue1", b.into());

    let aaa: URef = runtime::get_key("stringvalue1")
        .unwrap()
        .try_into()
        .unwrap();
    storage::add(aaa, 2);

    let a: i32 = storage::read(aaa).unwrap().unwrap();
}
