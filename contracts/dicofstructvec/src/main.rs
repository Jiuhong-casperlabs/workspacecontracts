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

    let vec1_uref = storage::new_dictionary("vec1_uref").unwrap();
    storage::dictionary_put(vec1_uref, "mem1", mem1);
    storage::dictionary_put(vec1_uref, "mem2", mem2);

    let vec2_uref = storage::new_dictionary("vec2_uref").unwrap();
    storage::dictionary_put(vec2_uref, "mem3", mem3);
    storage::dictionary_put(vec2_uref, "mem4", mem4);

    let dic_uref = storage::new_dictionary("mydic").unwrap();
    storage::dictionary_put(dic_uref, "vec1", vec1_uref);
    storage::dictionary_put(dic_uref, "vec2", vec2_uref);
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

    // get vec1
    let return_vec1_uref: URef = storage::dictionary_get(return_dic_uref, "vec1")
        .unwrap()
        .unwrap();
    let return_mem1: Obj = storage::dictionary_get(return_vec1_uref, "mem1")
        .unwrap()
        .unwrap();

    let return_mem2: Obj = storage::dictionary_get(return_vec1_uref, "mem2")
        .unwrap()
        .unwrap();

    // get vec2
    let return_vec2_uref: URef = storage::dictionary_get(return_dic_uref, "vec2")
        .unwrap()
        .unwrap();
    let return_mem3: Obj = storage::dictionary_get(return_vec2_uref, "mem3")
        .unwrap()
        .unwrap();

    let return_mem4: Obj = storage::dictionary_get(return_vec2_uref, "mem4")
        .unwrap()
        .unwrap();
}
