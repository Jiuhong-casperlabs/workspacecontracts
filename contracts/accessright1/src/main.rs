#![no_std]
#![no_main]

extern crate alloc;

use casper_contract::contract_api::{runtime, storage};
use casper_types::{AccessRights, URef};
use core::convert::TryInto;

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

    let _a: i32 = storage::read(aaa).unwrap().unwrap();
}
