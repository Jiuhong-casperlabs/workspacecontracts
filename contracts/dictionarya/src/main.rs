#![no_std]
#![no_main]
// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{string::String, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, RuntimeArgs,
};

pub const DICTIONARY_REF: &str = "dict_name";
pub const DEFAULT_DICTIONARY_NAME: &str = "abc_name";
pub const DEFAULT_DICTIONARY_VALUE: &str = "abc_value";

#[no_mangle]
pub extern "C" fn test() {
    let dictionary_uref = storage::new_dictionary(DICTIONARY_REF).unwrap_or_revert();

    storage::dictionary_put(
        dictionary_uref,
        DEFAULT_DICTIONARY_NAME,
        DEFAULT_DICTIONARY_VALUE,
    );

    let maybe_value =
        storage::dictionary_get(dictionary_uref, DEFAULT_DICTIONARY_NAME).unwrap_or_revert();
    // Whether the value exists or not we're mostly interested in validation of access
    // rights
    let value: String = maybe_value.unwrap_or_default();
    runtime::put_key("dic_value", storage::new_uref(value).into());
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "test",
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Session,
    ));

    let (contract_hash, _) = storage::new_locked_contract(entry_points, None, None, None);
    runtime::put_key("dictionarycontract", contract_hash.into());
    runtime::call_contract(contract_hash, "test", RuntimeArgs::default())
}
