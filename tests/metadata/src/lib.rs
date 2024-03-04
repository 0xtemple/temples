#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::prelude::*;

pub struct TestMetadata;

impl Metadata for TestMetadata {
    type Init = ();
    type Handle = ();
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = ();
}
