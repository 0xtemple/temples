#![no_std]

use gstd::collections::{BTreeMap, HashMap};
use gstd::Decode;
use gstd::Encode;
use gstd::TypeInfo;
use gstd::Vec;
use temple_types::schema::{Schema, SchemaMetadata};
use temple_types::world::WorldMetadata;

pub static mut TEMPLE_WORLD_STORAGE: Option<WorldMetadata> = None;
pub static mut TEMPLE_STORAGE_VALUE: Option<HashMap<[u8; 32], Vec<u8>>> = None;
pub static mut TEMPLE_STORAGE_MAP: Option<BTreeMap<[u8; 32], BTreeMap<Vec<u8>, Vec<u8>>>> = None;

#[derive(Debug, Clone, Encode, Decode, Default, TypeInfo)]
pub struct Storage {
    value: BTreeMap<[u8; 32], Schema>,
}

pub static mut TEMPLES_STORAGE: Option<Storage> = None;

pub fn register(schema_id: [u8; 32], metadata: SchemaMetadata) {
    let temples_storage = unsafe { TEMPLES_STORAGE.get_or_insert(Default::default()) };
    let schema = Schema {
        metadata,
        ..Default::default()
    };
    temples_storage.value.insert(schema_id, schema);
}

pub fn get(schema_id: &[u8; 32], key: &[u8]) -> Vec<u8> {
    let temples_storage = unsafe { TEMPLES_STORAGE.get_or_insert(Default::default()) };
    let schema = temples_storage
        .value
        .get(schema_id)
        .expect("Schema is not registered.");
    schema
        .raw_data
        .get(key)
        .expect("The key does not exist.")
        .to_vec()
}

pub fn set(schema_id: [u8; 32], key: Vec<u8>, value: Vec<u8>) {
    let temples_storage = unsafe { TEMPLES_STORAGE.get_or_insert(Default::default()) };
    let schema = temples_storage
        .value
        .get_mut(&schema_id)
        .expect("Schema is not registered.");
    schema.raw_data.insert(key, value);
}

pub fn remove(schema_id: &[u8; 32], key: &[u8]) {
    let temples_storage = unsafe { TEMPLES_STORAGE.get_or_insert(Default::default()) };
    let schema = temples_storage
        .value
        .get_mut(schema_id)
        .expect("Schema is not registered.");
    schema.raw_data.remove(key);
}

pub fn mutate<F>(schema_id: &[u8; 32], key: &[u8], f: F)
where
    F: FnOnce(&mut Vec<u8>),
{
    let temples_storage = unsafe { TEMPLES_STORAGE.get_or_insert(Default::default()) };
    let schema = temples_storage
        .value
        .get_mut(schema_id)
        .expect("Schema is not registered.");
    let value = schema
        .raw_data
        .get_mut(key)
        .expect("The key does not exist.");
    f(value);
}

pub fn schema_exists(schema_id: &[u8; 32]) -> bool {
    let temples_storage = unsafe { TEMPLES_STORAGE.get_or_insert(Default::default()) };
    temples_storage.value.contains_key(schema_id)
}

pub fn exists(schema_id: &[u8; 32], key: &[u8]) -> bool {
    let temples_storage = unsafe { TEMPLES_STORAGE.get_or_insert(Default::default()) };
    let schema = temples_storage
        .value
        .get_mut(schema_id)
        .expect("Schema is not registered.");
    schema.raw_data.contains_key(key)
}

#[cfg(test)]
mod tests {
    use crate::{exists, get, mutate, register, remove, schema_exists, set};
    use gstd::vec;
    use temple_types::schema::SchemaMetadata;

    #[test]
    fn test_storage() {
        let schema_id = [0u8; 32];
        let key = vec![1, 2, 3];
        let value = vec![1, 2, 3];
        let new_value = vec![1, 2, 3, 4];

        assert_eq!(schema_exists(&schema_id), false);

        register(schema_id.clone(), SchemaMetadata::default());
        assert_eq!(schema_exists(&schema_id), true);
        assert_eq!(exists(&schema_id, &key), false);

        set(schema_id, key.clone(), value.clone());
        assert_eq!(get(&schema_id, &key), value);
        assert_eq!(exists(&schema_id, &key), true);

        mutate(&schema_id, &key, |value| {
            *value = new_value.clone();
        });
        assert_eq!(get(&schema_id, &key), new_value);
        assert_eq!(exists(&schema_id, &key), true);

        remove(&schema_id, &key);
        assert_eq!(exists(&schema_id, &key), false);
    }
}
