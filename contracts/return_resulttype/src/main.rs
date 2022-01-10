#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec;

use alloc::{ string::String};

use casper_contract::{contract_api::{runtime, storage}, unwrap_or_revert::UnwrapOrRevert};

use casper_types::{CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, RuntimeArgs};

#[no_mangle]
pub extern "C" fn test() {
    let a = 1;
    let b = 2;

    let good_result:Result<(),String> = Ok(());
    let bad_result: Result<(),String> = Err(String::from("thisiserror"));

    if a==b {
    runtime::ret(CLValue::from_t(good_result).unwrap_or_revert());
    }

    runtime::ret(CLValue::from_t(bad_result).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
          "test",
        vec![],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::String),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (contract_hash,_) = storage::new_contract(entry_points, None, None, None);
    runtime::put_key("ttt", contract_hash.into());
    let a:Result<(), String> = runtime::call_contract(contract_hash, "test", RuntimeArgs::default());
    runtime::put_key("resulttype", storage::new_uref(a).into());
    
}
