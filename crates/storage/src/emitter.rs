use gstd::String;
use gstd::{msg, ActorId, Vec};
use temple_types::event::{ComponentDelRecord, ComponentRegistered, ComponentSetRecord};

pub fn emit_register_event(
    component_id: [u8; 32],
    key_names: Vec<String>,
    key_types: Vec<String>,
    value_names: Vec<String>,
    value_types: Vec<String>,
) {
    msg::send(
        ActorId::default(),
        ComponentRegistered {
            component_id,
            key_names,
            key_types,
            value_names,
            value_types,
        },
        0,
    )
    .expect("Sending message failed");
}

pub fn emit_set_event(component_id: [u8; 32], key: Vec<u8>, value: Vec<u8>) {
    msg::send(
        ActorId::default(),
        ComponentSetRecord {
            component_id,
            key,
            value,
        },
        0,
    )
    .expect("Sending message failed");
}

pub fn emit_del_event(component_id: [u8; 32], key: Vec<u8>) {
    msg::send(
        ActorId::default(),
        ComponentDelRecord { component_id, key },
        0,
    )
    .expect("Sending message failed");
}
