#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;

use casper_contract::contract_api::{runtime, storage};
use casper_types::{AccessRights, URef};
use core::convert::{TryFrom, TryInto};

#[no_mangle]
pub extern "C" fn call() {
    let a = runtime::get_key("stringvalue").unwrap();
    runtime::remove_key("stringvalue");
    let b = URef::try_from(a).unwrap();

    let c = URef::new(b.addr(), AccessRights::NONE);

    runtime::put_key("stringvalue", c.into());

    let aaa: URef = runtime::get_key("stringvalue").unwrap().try_into().unwrap();

    let _a: String = storage::read(aaa).unwrap().unwrap();
}
