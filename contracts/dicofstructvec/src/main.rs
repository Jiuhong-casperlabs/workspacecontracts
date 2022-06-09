#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use alloc::string::String;

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

    let vec1_uref = storage::new_dictionary("vec1_uref").unwrap();
    storage::dictionary_put(vec1_uref, "mem1", mem1);
    storage::dictionary_put(vec1_uref, "mem2", mem2);

    let vec2_uref = storage::new_dictionary("vec2_uref").unwrap();
    storage::dictionary_put(vec2_uref, "mem3", mem3);
    storage::dictionary_put(vec2_uref, "mem4", mem4);

    runtime::ret(CLValue::from_t(vec![vec1_uref, vec2_uref]).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let test_entry_point = EntryPoint::new(
        "test",
        vec![],
        CLType::List(Box::new(URef::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(test_entry_point);
    let (contract_hash, _) = storage::new_locked_contract(entry_points, None, None, None);
    runtime::put_key("mycontract", contract_hash.into());

    let vec_urefs: Vec<URef> =
        runtime::call_contract(contract_hash, "test", RuntimeArgs::default());

    // get vec1
    // get mem1
    let return_mem1: Obj = storage::dictionary_get(vec_urefs[0], "mem1")
        .unwrap()
        .unwrap();
    runtime::put_key("mem1", storage::new_uref(return_mem1).into());

    // get mem2
    let return_mem2: Obj = storage::dictionary_get(vec_urefs[0], "mem2")
        .unwrap()
        .unwrap();
    runtime::put_key("mem2", storage::new_uref(return_mem2).into());

    // get vec2
    // get mem3
    let return_mem3: Obj = storage::dictionary_get(vec_urefs[1], "mem3")
        .unwrap()
        .unwrap();
    runtime::put_key("mem3", storage::new_uref(return_mem3).into());

    // get mem4
    let return_mem4: Obj = storage::dictionary_get(vec_urefs[1], "mem4")
        .unwrap()
        .unwrap();
    runtime::put_key("mem4", storage::new_uref(return_mem4).into());
}
