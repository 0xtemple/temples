use crate::metadata::StateReply;
use gstd::{prelude::*, String};

#[gmeta::metawasm]
pub mod metafns {
    pub type State = StateReply;

    pub fn name(state: State) -> Option<String> {
        if let StateReply::WorldMetadata(world_metadata) = state {
            return world_metadata.name();
        }
        None
    }

    pub fn description(state: State) -> Option<String> {
        if let StateReply::WorldMetadata(world_metadata) = state {
            return world_metadata.description();
        };
        None
    }
}
