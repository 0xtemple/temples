#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{ActorId, Decode, Encode, TypeInfo};

pub struct CounterMetadata;

impl Metadata for CounterMetadata {
    type Init = ();
    type Handle = In<SystemAction>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = ();
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum SystemAction {
    SetValue(u128),
    SetMap(u128, u128),
    AddValue,
    AddMap(u128),
}
