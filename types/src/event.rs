use crate::component::Struct;
use gstd::ActorId;

#[derive(Clone, Debug)]
pub struct WorldSpawned {
    pub caller: ActorId,
}

/// The event emitted when a entity is registered to a World.
#[derive(Clone, Debug)]
pub struct ComponentRegistered {
    pub name: String,
    pub schema: Struct,
}

/// The event emmitted when a entity value of an entity is set.
#[derive(Clone, Debug)]
pub struct StoreSetRecord {
    pub table_id: Vec<u8>,
    pub key_names: Vec<String>,
    pub key_types: Vec<String>,
    pub key_values: Vec<u8>,
}

/// The event emmitted when a entity is deleted from an entity.
#[derive(Clone, Debug)]
pub struct StoreDelRecord {
    pub table_id: Vec<u8>,
    pub keys: Vec<u8>,
}
