#![no_std]
#![no_main]
extern crate alloc;
use casper_contract::contract_api::runtime;
use casper_types::ContractHash;

#[no_mangle]
pub extern "C" fn call() {
    let mycontract: ContractHash = runtime::get_named_arg("mycontract");
    runtime::put_key("received_contract", mycontract.into());
}
