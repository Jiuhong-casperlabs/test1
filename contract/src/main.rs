#![no_std]
#![no_main]

extern crate alloc;

mod hello;

use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    contracts::Parameters, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Key, RuntimeArgs, URef,
};

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = {
        let mut entry_points = EntryPoints::new();

        let entry_point = EntryPoint::new(
            "hello",
            Parameters::default(),
            CLType::String,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );

        entry_points.add_entry_point(entry_point);

        entry_points
    };
    let (contract_hash, _contract_version) = storage::new_contract(entry_points, None, None, None);

    runtime::put_key("call", contract_hash.into());
    runtime::call_contract(contract_hash, "hello", RuntimeArgs::default())
}
