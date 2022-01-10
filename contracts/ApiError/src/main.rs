#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_std]
#![no_main]

use casper_contract::contract_api::runtime;
use casper_types::ApiError;

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    runtime::revert(ApiError::User(2))
}
