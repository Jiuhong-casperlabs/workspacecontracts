#![no_std]
#![no_main]
extern crate alloc;
use alloc::vec::Vec;
use casper_contract::contract_api::runtime;
use casper_types::ContractHash;

#[no_mangle]
pub extern "C" fn call() {
    let contracts: Vec<ContractHash> = runtime::get_named_arg("contracts");
    runtime::put_key("received_contract0", contracts[0].into());
    runtime::put_key("received_contract1", contracts[1].into());
}
