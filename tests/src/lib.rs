#![no_std]

use gstd::{msg, prelude::*};
use test_components::value_primitives::ValueU32Component;
use test_components::value_primitives::ValueU8Component;
use test_components::value_struct::ValueData;
pub use test_components::value_struct::ValueStructComponent;

#[derive(Debug, Default)]
struct Counter {
    pub value: u128,
}

#[no_mangle]
extern fn init() {
    ValueStructComponent.register();
    ValueU8Component.register();
    ValueU32Component.register();
}

#[no_mangle]
extern fn handle() {
    // let c = ValueStructComponent.get();
    // ValueStructComponent.set(ValueData { value: c.value + 1 });

    let c = ValueU8Component.get();
    ValueU8Component.set(c + 10);
}
