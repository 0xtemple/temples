use gstd::{msg, ActorId, Decode, Encode, String, TypeInfo, Vec};

use crate::schema::SchemaMetadata;

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
    RegisterSchemaSuccess([u8; 32], SchemaMetadata),
    SetRecordSuccess([u8; 32], Vec<u8>, Vec<u8>),
    DelRecordSuccess([u8; 32], Vec<u8>),
    GetRecordSuccess(Vec<u8>),
}

pub fn emit_event<T: Encode>(value: T) {
    msg::send(ActorId::default(), value, 0).expect("Sending message failed");
}
