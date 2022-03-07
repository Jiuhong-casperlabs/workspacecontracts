#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, RuntimeArgs,
};
use casper_types::{CLTyped, CLValue, Key};
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

    let mut vec = Vec::new();
    vec.push(mem1);
    vec.push(mem2);

    runtime::ret(CLValue::from_t(vec).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let test_entry_point = EntryPoint::new(
        "test",
        vec![],
        Vec::<Obj>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(test_entry_point);
    let (contract_hash, _) = storage::new_locked_contract(entry_points, None, None, None);
    runtime::put_key("mycontract", contract_hash.into());

    let returnvalue: Vec<Obj> =
        runtime::call_contract(contract_hash, "test", RuntimeArgs::default());
    runtime::put_key("returnvalue", storage::new_uref(returnvalue).into());
}
