#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

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
        storage::{self, read},
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

    let mut vec1 = Vec::new();
    vec1.push(mem1);
    vec1.push(mem2);

    let mut vec2 = Vec::new();
    vec2.push(mem3);
    vec2.push(mem4);

    // let mut map = BTreeMap::new();

    // map.insert("key1".to_string(), vec1);
    // map.insert("key2".to_string(), vec2);

    let dic_uref = storage::new_dictionary("mydic").unwrap();
    storage::dictionary_put(dic_uref, "key1", vec1);
    storage::dictionary_put(dic_uref, "key2", vec2);
    runtime::ret(CLValue::from_t(dic_uref).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let test_entry_point = EntryPoint::new(
        "test",
        vec![],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(test_entry_point);
    let (contract_hash, _) = storage::new_locked_contract(entry_points, None, None, None);
    runtime::put_key("mycontract", contract_hash.into());

    let return_dic_uref: URef =
        runtime::call_contract(contract_hash, "test", RuntimeArgs::default());
    // store value
    let value1: Vec<Obj> = storage::dictionary_get(return_dic_uref, "key1")
        .unwrap()
        .unwrap();
    runtime::put_key("returnvalue1", storage::new_uref(value1).into());

    let value2: Vec<Obj> = storage::dictionary_get(return_dic_uref, "key2")
        .unwrap()
        .unwrap();
    runtime::put_key("returnvalue2", storage::new_uref(value2).into());
}
