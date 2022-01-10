#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_std]
#![no_main]

use casper_contract::contract_api::runtime;

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    let named_keys = runtime::list_named_keys();

    for (key, _) in named_keys {
        if key.starts_with("cep47-nft-2"){
            runtime::remove_key(&key);
        }
    }
}
