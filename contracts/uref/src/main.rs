#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_std]
#![no_main]

extern crate alloc;
use alloc::{collections::BTreeMap, string::String, vec::Vec};

use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    AccessRights,
    CLType::{self, Tuple3},
    Key, URef,
};

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    runtime::remove_key("gotref");
    //----------uref1 start OK:
    let uref = URef::from_formatted_str(
        "uref-df41315b28147b690a9afa2ae60123e4d7efea794f1258c1e1a4b3e9b7c1bd34-007",
    )
    .unwrap();

    let value: BTreeMap<String, (i32, bool, String)> = storage::read(uref).unwrap().unwrap();

    runtime::put_key("gotref", storage::new_uref(value).into());
    //------------uref end
    // //------------uref2 start NG:
    // let uref1 = URef::from_formatted_str(
    //     "uref-ec586dd94af5b691ec7d0310c43a84df78b1035915988bdc0876f71750fe7cc5-007",
    // )
    // .unwrap();

    // //uref2 gave below error because it is not under my account(secret key)
    // //```
    // //"Forged reference: URef(ec586dd94af5b691ec7d0310c43a84df78b1035915988bdc0876f71750fe7cc5, READ_ADD_WRITE)"
    // //```

    // let value: String = storage::read(uref1).unwrap().unwrap();

    // runtime::put_key("gotref1", storage::new_uref(value).into());
    // //------------uref2 end
}
