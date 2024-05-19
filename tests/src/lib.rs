#![no_std]

use gstd::prelude::*;

#[derive(Debug, Default)]
struct Counter {
    pub value: u128,
}

#[no_mangle]
extern fn init() {}

#[no_mangle]
extern fn handle() {}
