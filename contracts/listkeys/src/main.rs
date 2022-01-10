#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::vec;
use alloc::boxed::Box;
use alloc::{string::String, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, Key, RuntimeArgs, EntryPoint, EntryPoints, CLType, EntryPointAccess, EntryPointType, CLTyped, Parameter};

#[no_mangle]
pub extern "C" fn test() {
    let value: Vec<Key> = runtime::get_named_arg("hello");
    runtime::put_key("listkeys", storage::new_uref(value).into());
}

#[no_mangle]
pub extern "C" fn call() {
    let mut test = RuntimeArgs::new();
    let mut vec = Vec::new();
    let value1 = Key::from_formatted_str("account-hash-22a6800a2f7f1f9a52ebc5165069b82f0e4b2981b27f11d42f5d94e3d87aa931").unwrap();
    let value2 = Key::from_formatted_str("account-hash-22a6800a2f7f1f9a52ebc5165069b82f0e4b2981b27f11d42f5d94e3d87aa931").unwrap();
    let value3 = Key::from_formatted_str("account-hash-22a6800a2f7f1f9a52ebc5165069b82f0e4b2981b27f11d42f5d94e3d87aa931").unwrap();
    vec.push(value1);
    vec.push(value2);
    vec.push(value3);
    let _ = test.insert("hello", vec);

    // let test_entry_point = EntryPoint::new("test",CLType::List(Box::new(Key::cl_type())),CLType::Unit,EntryPointAccess::Public,EntryPointType::Contract);
    let test_entry_point = EntryPoint::new("test",vec![
        Parameter::new("hello", CLType::List(Box::new(Key::cl_type()))),
    ],CLType::Unit,EntryPointAccess::Public,EntryPointType::Contract);
   
    
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(test_entry_point);
    let (contract_hash,_) = storage::new_locked_contract(entry_points,None, None,None);
    runtime::put_key("listkeys", contract_hash.into());

    let _: () = runtime::call_contract(contract_hash, "test", test);
}
