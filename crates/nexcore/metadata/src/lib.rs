#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::vec::Vec;
use gstd::{Decode, Encode, TypeInfo};
use temple_types::action::SystemAction;

pub struct NexcoreMetadata;

impl Metadata for NexcoreMetadata {
    type Init = ();
    type Handle = In<SystemAction>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<StateQuery, StateReply>;
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateQuery {
    GetRecord(u8),
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateReply {
    Record(u32),
}
