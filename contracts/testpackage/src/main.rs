#![no_main]
#![no_std]

use casper_contract::contract_api::{runtime, storage};
use casper_types::{contracts::NamedKeys, EntryPoints};

#[no_mangle]
pub extern "C" fn call() {
    let (mycontracthash, myuref) = storage::create_contract_package_at_hash();

    runtime::put_key("mycontracthash", mycontracthash.into()); //"hash-0fc7e9d680d241c6cbd8918fa5660a990faf2fd535f24c6fe9815dac58294910"
    runtime::put_key("myuref", myuref.into()); //72b192704af8232f85f81ebd141ecb70cf32092c875e54571e0581f4cd39e0c3

    let entrypoints = EntryPoints::new();

    let namedkeys = NamedKeys::new();

    storage::add_contract_version(mycontracthash, entrypoints, namedkeys);
}
