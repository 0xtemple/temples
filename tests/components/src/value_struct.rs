use gstd::scale_info::IntoPortable;
use gstd::{vec, TypeInfo};
use gstd::{Decode, Encode};
use lazy_static::lazy_static;
use temple_storage::value::StorageValue;
use temple_types::component::{ComponentMetadata, ComponentType};
use temple_types::primitive::Primitive;

const COMPONENT_ID: [u8; 32] = [8; 32];

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
pub struct ValueData {
    pub value: u128,
}

lazy_static! {
    pub static ref ValueStructComponent: StorageValue<ValueData> =
        StorageValue::new("Counter".into(), COMPONENT_ID);
}
