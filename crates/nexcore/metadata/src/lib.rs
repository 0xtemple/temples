#![no_std]

use gmeta::{InOut, Metadata};
use temple_types::action::SystemAction;
use temple_types::event::NexCoreEvent;

pub struct NexcoreMetadata;

impl Metadata for NexcoreMetadata {
    type Init = ();
    type Handle = InOut<SystemAction, NexCoreEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = ();
}
