use gmeta::{In, InOut, Metadata};
use gstd::{Decode, Encode, TypeInfo};
use temple_types::world::WorldMetadata;

pub struct CounterMetadata;

impl Metadata for CounterMetadata {
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
    WorldMetadata,
    GetCurrentNumber,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateReply {
    WorldMetadata(WorldMetadata),
    CurrentNumber(u128),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum SystemAction {
    Add,
}
