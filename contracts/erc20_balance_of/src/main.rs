#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use casper_contract::contract_api::{runtime, storage};

use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, U256};

#[no_mangle]
pub extern "C" fn call() {
    let lower_contracthash =
        "contract-4120116565bd608fAe6a45078055F320a2f429f426C86797b072B4EFD15B186A".to_lowercase();
    let contract_hash = ContractHash::from_formatted_str(&lower_contracthash).unwrap();

    let raw_address =
        "account-hash-ad7e091267d82c3b9ed1987cb780a005a550e6b3d1ca333b743e2dba70680877"
            // "account-hash-2293223427D59eBB331aC2221c3fcd1b3656a5Cb72BE924A6CdC9d52CdB6dB0F" jdk2
            .to_lowercase();
    let address = Key::from_formatted_str(
        //
        &raw_address,
    )
    .unwrap();

    let args = runtime_args! {
        "address" => address,

    };

    let balances: U256 = runtime::call_contract(contract_hash, "balance_of", args);
    runtime::put_key("mybalance", storage::new_uref(balances).into());

    U256::from(999999999);
}
