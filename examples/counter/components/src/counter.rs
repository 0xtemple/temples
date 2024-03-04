#![allow(non_upper_case_globals)]

use lazy_static::lazy_static;
use temple_storage::value::StorageValue;

const COMPONENT_ID: [u8; 32] = [8; 32];

lazy_static! {
    pub static ref CounterComponent: StorageValue<u128> =
        StorageValue::new("Counter".into(), COMPONENT_ID);
}
