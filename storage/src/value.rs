use crate::emitter;
use alloc::string::ToString;
use core::marker::PhantomData;
use gstd::{vec, Decode, Encode, String, TypeInfo, Vec};
use temple_types::component::{ComponentMetadata, ComponentType};
use temple_types::utils;

use crate::io::TEMPLE_STORAGE_VALUE;
use crate::io::TEMPLE_WORLD_STORAGE;
#[derive(Debug, Clone)]
pub struct StorageValue<Value> {
    name: String,
    component_id: [u8; 32],
    _phantom: PhantomData<Value>,
}

impl<Value> StorageValue<Value>
where
    Value: Encode + Decode + Default + TypeInfo,
{
    pub fn new(name: String, component_id: [u8; 32]) -> Self {
        Self {
            name,
            component_id,
            _phantom: PhantomData,
        }
    }

    pub fn component_type(&self) -> ComponentType {
        if self.component_id[0..2] == [0, 0] {
            ComponentType::Onchain
        } else {
            ComponentType::Offchain
        }
    }

    pub fn metadata(&self) -> ComponentMetadata {
        let name = self.name.to_string();
        let (value_names, value_types) = utils::parse_type::<Value>();
        ComponentMetadata {
            name,
            key_names: vec![],
            key_types: vec![],
            value_names,
            value_types,
            ty: self.component_type(),
        }
    }

    /// Get the storage key.
    pub fn register(&self) {
        let temple_world_storage =
            unsafe { TEMPLE_WORLD_STORAGE.get_or_insert(Default::default()) };
        let metadata = self.metadata();
        temple_world_storage.register_component(&self.name, &metadata);
        emitter::emit_register_event(
            self.component_id,
            metadata.key_names,
            metadata.key_types,
            metadata.value_names,
            metadata.value_types,
        )
    }

    pub fn get(&self) -> Value {
        let temple_storage_value =
            unsafe { TEMPLE_STORAGE_VALUE.get_or_insert(Default::default()) };
        Value::decode(
            &mut &temple_storage_value
                .get(&self.component_id)
                .unwrap_or(&Vec::new())[..],
        )
        .unwrap_or(Default::default())
    }

    /// Store a value under this key into the provided storage instance.
    ///
    /// this uses the query type rather than the underlying value.
    pub fn set(&self, value: Value) {
        let temple_storage_value =
            unsafe { TEMPLE_STORAGE_VALUE.get_or_insert(Default::default()) };
        let component_data = value.encode();
        temple_storage_value.insert(self.component_id, component_data.clone());
        emitter::emit_set_event(self.component_id, vec![], component_data.clone());
    }
}
