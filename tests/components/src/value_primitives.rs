use gstd::scale_info::IntoPortable;
use gstd::{vec, TypeInfo};
use gstd::{Decode, Encode};
use lazy_static::lazy_static;
use temple_storage::value::StorageValue;
use temple_types::component::{ComponentMetadata, ComponentType};
use temple_types::primitive::Primitive;

const COMPONENT_ID: [u8; 32] = [1; 32];

lazy_static! {
    pub static ref ValueU8Component: StorageValue<u8> = StorageValue::new("U8".into(), [1; 32]);
    pub static ref ValueU32Component: StorageValue<u32> = StorageValue::new("U32".into(), [2; 32]);
}
