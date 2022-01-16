#![no_main]
#![no_std]

extern crate alloc;
use alloc::string::String;
use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, AsymmetricType, ContractHash, PublicKey, RuntimeArgs, U512};

const PUBLIC_KEY_STR: &str = "01a018bf278f32fdb7b06226071ce399713ace78a28d43a346055060a660ba7aa9";
const CONTRACT_HASH_STR: &str =
    "contract-5bfb3233216d08490ab74ddd04b0ff7df3450a63d15976efcf57facd2ebaf5bf";

fn storeoption() {
    let runtime_args = runtime_args! {
        "name" => String::from("optionstring"),
        "value"=> Some("abc")
    };

    storegeneral("store_option", runtime_args);
}

fn storegoodresult() {
    let good_result: Result<String, String> = Ok(String::from("goodresult"));

    let runtime_args = runtime_args! {
        "name" => String::from("resultgoodvalue"),
        "value"=> good_result
    };

    storegeneral("store_good_result", runtime_args);
}

fn storebadresult() {
    let bad_result: Result<String, String> = Err(String::from("badresult"));

    let runtime_args = runtime_args! {
        "name" => String::from("resultbadvalue"),
        "value"=> bad_result
    };
    storegeneral("store_bad_result", runtime_args);
}

fn storetuple1() {
    let tuple1value = String::from("tuple1value");

    let runtime_args = runtime_args! {
        "name" => String::from("tuple1value"),
        "value"=> (tuple1value, )  //single-element tuple (value, )
    };

    storegeneral("store_tuple1", runtime_args);
}

fn storegeneral(entry_point_name: &str, args: RuntimeArgs) {
    let contract_hash = ContractHash::from_formatted_str(CONTRACT_HASH_STR).unwrap();

    runtime::call_contract(contract_hash, entry_point_name, args)
}

fn storetuple2() {
    let runtime_args = runtime_args! {
        "name" => "tuple2value",
        "value" => (String::from("abc"), U512::from(1))
    };
    storegeneral("store_tuple2", runtime_args);
}

fn storetuple3() {
    let pk = PublicKey::from_hex(PUBLIC_KEY_STR).unwrap();

    let runtime_args = runtime_args! {
        "name" => "tuple3value",
        "value" => (pk,Some("abc"),U512::from(2))
    };
    storegeneral("store_tuple3", runtime_args);
}

#[no_mangle]
pub extern "C" fn call() {
    //***store option string***
    storeoption();

    //***store good result***
    storegoodresult();

    // *** store bad result
    storebadresult();

    // store tuple1
    storetuple1();

    // store tuple2
    storetuple2();

    // store tuple3
    storetuple3();
}

// https://testnet.cspr.live/deploy/04d5c8e65d9e6fb15241902661bc2860ede4f2845be1f5b416e53eaeefbcc50f

// {
//     "id": 1,
//     "jsonrpc":"2.0",
//     "method":"state_get_item",
//     "params":
//     ["5a94f7f9dd98352392def4bee6ca388ee689b1f1f9752659c9f254b42c9656fe",
//     "hash-5bfb3233216d08490ab74ddd04b0ff7df3450a63d15976efcf57facd2ebaf5bf"
//      ]
// }
