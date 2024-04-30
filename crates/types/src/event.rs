use gstd::{ActorId, Decode, Encode, String, TypeInfo, Vec};

#[derive(Clone, Debug)]
pub struct WorldSpawned {
    pub caller: ActorId,
}

/// The event emitted when a entity is registered to a World.
#[derive(Clone, Debug, Encode, Decode)]
pub struct ComponentRegistered {
    pub component_id: [u8; 32],
    pub key_names: Vec<String>,
    pub key_types: Vec<String>,
    pub value_names: Vec<String>,
    pub value_types: Vec<String>,
}

/// The event emmitted when a entity value of an entity is set.
#[derive(Clone, Debug, Encode, Decode)]
pub struct ComponentSetRecord {
    pub component_id: [u8; 32],
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

/// The event emmitted when a entity is deleted from an entity.
#[derive(Clone, Debug, Encode, Decode)]
pub struct ComponentDelRecord {
    pub component_id: [u8; 32],
    pub key: Vec<u8>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum NexCoreEvent {
    SetRecordSuccess,
    DelRecordSuccess(u32),
    GetRecordSuccess(u32),
}
