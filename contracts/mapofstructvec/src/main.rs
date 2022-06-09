#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use core::convert::TryInto;

use alloc::vec;
use alloc::vec::Vec;
use alloc::{boxed::Box, collections::BTreeMap};

use alloc::string::{String, ToString};

use casper_contract::{
    contract_api::{
        runtime,
        storage::{self},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, RuntimeArgs, URef,
};
use casper_types::{CLTyped, CLValue};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};

#[derive(CLTyped, ToBytes, FromBytes)]
struct Obj {
    title: String,
    recipient: String,
}

#[no_mangle]
fn test() {
    let mem1 = Obj {
        title: String::from("kitty"),
        recipient: String::from(
            "hash-4929e7fcb71772c1cb39ebb702a70d036b0ad4f9caf420d3fd377f749dfdb17",
        ),
    };

    let mem2 = Obj {
        title: String::from("kitty"),
        recipient: String::from(
            "hash-4929e7fcb71772c1cb39ebb702a70d036b0ad4f9caf420d3fd377f749dfdb17",
        ),
    };

    let mem3 = Obj {
        title: String::from("kitty"),
        recipient: String::from(
            "hash-4929e7fcb71772c1cb39ebb702a70d036b0ad4f9caf420d3fd377f749dfdb17",
        ),
    };

    let mem4 = Obj {
        title: String::from("kitty"),
        recipient: String::from(
            "hash-4929e7fcb71772c1cb39ebb702a70d036b0ad4f9caf420d3fd377f749dfdb17",
        ),
    };

    let vec1 = vec![mem1, mem2];

    let vec2 = vec![mem3, mem4];

    let mut map = BTreeMap::new();

    map.insert("key1".to_string(), vec1);
    map.insert("key2".to_string(), vec2);

    runtime::ret(CLValue::from_t(map).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let test_entry_point = EntryPoint::new(
        "test",
        vec![],
        CLType::Map {
            key: Box::new(CLType::String),
            value: Box::new(Vec::<Obj>::cl_type()),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(test_entry_point);
    let (contract_hash, _) = storage::new_locked_contract(entry_points, None, None, None);
    runtime::put_key("mycontract", contract_hash.into());

    let returnvalue: BTreeMap<String, Vec<Obj>> =
        runtime::call_contract(contract_hash, "test", RuntimeArgs::default());
    // store value
    runtime::put_key("returnvalue", storage::new_uref(returnvalue).into());

    // get value again
    let getkey: URef = runtime::get_key("returnvalue").unwrap().try_into().unwrap();
    let getvalue: BTreeMap<String, Vec<Obj>> = storage::read(getkey).unwrap().unwrap();
    // store value again
    runtime::put_key("returngetvalue", storage::new_uref(getvalue).into());
}
