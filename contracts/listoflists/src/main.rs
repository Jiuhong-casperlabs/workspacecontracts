#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::vec;

use casper_contract::contract_api::{runtime, storage};

#[no_mangle]
pub extern "C" fn call() {
    let value1 = vec![12, 3, 4];
    let value2 = vec![22, 333, 44];

    // a.push(value1);
    // a.push(value2);
    let mut a = vec![value1.clone(), value2.clone()];
    let value3 = vec![5, 6, 7];
    a.push(value3);
    let _result = a[0][1];
    a[0][2] = 9;

    let b = vec![value1, value2];

    let c = vec![a, b];
    let _d = c[0][0][0];
    runtime::put_key("listoflists", storage::new_uref(c).into());
}
