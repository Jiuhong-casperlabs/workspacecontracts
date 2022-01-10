#![no_std]
#![no_main]

extern crate alloc;
use alloc::{collections::BTreeMap, string::String};

use casper_contract::contract_api::{runtime, storage};

#[no_mangle]
pub extern "C" fn call() {
    for (k, _) in runtime::list_named_keys() {
        runtime::remove_key(&k);
    }
    let key1 = String::from("first");
    let key2 = String::from("second");

    let uref1 = storage::new_uref((111, 222));
    let uref2 = storage::new_uref((333, 444));

    let value1 = (123, true, uref1);
    let value2 = (999, false, uref2);

    let mut map = BTreeMap::new();

    map.insert(key1, value1);
    map.insert(key2, value2);

    let mut map2 = BTreeMap::new();
    map2.insert("map2key1", (1, true, map.clone()));
    map2.insert("map2key2", (2, false, map.clone()));

    let mut map3 = BTreeMap::new();
    map3.insert("map3key1", (11, map2.clone(), true));
    map3.insert("map3key2", (22, map2.clone(), false));

    let uref = storage::new_uref(map);
    let uref2 = storage::new_uref(map2);
    let uref3 = storage::new_uref(map3);

    runtime::put_key("map1", uref.into());
    runtime::put_key("map2", uref2.into());
    runtime::put_key("map3", uref3.into());
}

//reference https://github.com/casper-network/casper-node/blob/e2027dbe979ebf91f10ba8a90ffba1fddbd9fb09/types/src/cl_value/jsonrepr.rs
