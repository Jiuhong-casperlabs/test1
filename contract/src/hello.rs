extern crate alloc;

use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::Parameters, CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Key, RuntimeArgs, URef,
};

// #[no_mangle]
// pub extern "C" fn world() {}

#[no_mangle]
pub extern "C" fn hello() {
    runtime::put_key("hello", storage::new_uref(1).into())
}