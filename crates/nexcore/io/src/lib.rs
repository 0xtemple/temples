#![no_std]

use gstd::vec::Vec;
use gstd::collections::HashMap;

pub static mut SCHEMA1: Option<HashMap<u8, u32>> = None;
