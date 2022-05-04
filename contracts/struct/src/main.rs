#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use core::convert::TryInto;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use alloc::string::String;

use casper_contract::{
    contract_api::{
        runtime,
        storage::{self, read},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr,
    bytesrepr::{FromBytes, ToBytes},
    CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, RuntimeArgs, URef,
};
use casper_types::{CLTyped, CLValue, Key};
// use casper_types_derive::{CLTyped, FromBytes, ToBytes};

pub struct Schedule {
    pub unlock_time: i64,
    pub unlock_percent: i64,
}

impl CLTyped for Schedule {
    fn cl_type() -> CLType {
        CLType::ByteArray(16)
    }
}

impl ToBytes for Schedule {
    #[inline(always)]
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut preimage = Vec::new();
        preimage.append(&mut self.unlock_time.to_bytes().unwrap());
        preimage.append(&mut self.unlock_percent.to_bytes().unwrap());
        Ok(preimage)
    }

    #[inline(always)]
    fn serialized_length(&self) -> usize {
        128
    }

    fn into_bytes(self) -> Result<Vec<u8>, casper_types::bytesrepr::Error>
    where
        Self: Sized,
    {
        self.to_bytes()
    }
}

impl FromBytes for Schedule {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (unlock_time, remainder1) = i64::from_bytes(bytes).unwrap();
        let (unlock_percent, remainder2) = i64::from_bytes(remainder1).unwrap();
        let schedule = Schedule {
            unlock_time,
            unlock_percent,
        };
        Ok((schedule, remainder2))
    }
}

#[no_mangle]
fn test() {
    let mem = Schedule {
        unlock_time: 1,
        unlock_percent: 20,
    };

    runtime::ret(CLValue::from_t(mem).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let test_entry_point = EntryPoint::new(
        "test",
        vec![],
        Schedule::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(test_entry_point);
    let (contract_hash, _) = storage::new_locked_contract(entry_points, None, None, None);
    runtime::put_key("mycontract", contract_hash.into());

    let returnvalue: Schedule =
        runtime::call_contract(contract_hash, "test", RuntimeArgs::default());
    // store value
    runtime::put_key("returnvalue", storage::new_uref(returnvalue).into());

    // get value again
    let getkey: URef = runtime::get_key("returnvalue").unwrap().try_into().unwrap();
    let getvalue: Schedule = storage::read(getkey).unwrap().unwrap();
}
