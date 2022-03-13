#![no_std]
#![no_main]

extern crate alloc;
use alloc::{collections::BTreeMap, vec};

use casper_contract::contract_api::{runtime, storage};
use casper_types::{AsymmetricType, PublicKey};
const PUBLIC_KEY_STR: &str = "01a018bf278f32fdb7b06226071ce399713ace78a28d43a346055060a660ba7aa9";

#[no_mangle]
pub extern "C" fn call() {
    for (k, _) in runtime::list_named_keys() {
        runtime::remove_key(&k);
    }
    let pk1 = PublicKey::from_hex(PUBLIC_KEY_STR).unwrap();
    let pk2 = PublicKey::from_hex(PUBLIC_KEY_STR).unwrap();

    let value1 = vec![111u32, 222u32];
    let value2 = vec![333u32, 444u32];

    let mut map = BTreeMap::new();

    map.insert(pk1, value1);
    map.insert(pk2, value2);

    runtime::put_key("mymap", storage::new_uref(map).into());
}
