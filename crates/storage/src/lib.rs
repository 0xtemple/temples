#![no_std]

use gstd::collections::{BTreeMap, HashMap};
use gstd::Decode;
use gstd::Encode;
use gstd::TypeInfo;
use gstd::Vec;
use temple_types::world::WorldMetadata;

pub static mut TEMPLE_WORLD_STORAGE: Option<WorldMetadata> = None;
pub static mut TEMPLE_STORAGE_VALUE: Option<HashMap<[u8; 32], Vec<u8>>> = None;
pub static mut TEMPLE_STORAGE_MAP: Option<BTreeMap<[u8; 32], BTreeMap<Vec<u8>, Vec<u8>>>> = None;

#[derive(Debug, Clone, Encode, Decode, Default, TypeInfo)]
pub struct Storage {
    value: BTreeMap<[u8; 32], BTreeMap<Vec<u8>, Vec<u8>>>,
}

pub static mut TEMPLES_STORAGE: Option<Storage> = None;

pub fn get(schema_id: &[u8; 32], key: &[u8]) -> Vec<u8> {
    let temples_storage = unsafe { TEMPLES_STORAGE.get_or_insert(Default::default()) };
    let schema = temples_storage.value.get(schema_id).unwrap();
    schema.get(key).unwrap().to_vec()
}

pub fn set(schema_id: [u8; 32], key: Vec<u8>, value: Vec<u8>) {
    let temples_storage = unsafe { TEMPLES_STORAGE.get_or_insert(Default::default()) };
    if temples_storage.value.contains_key(&schema_id) {
        let schema = temples_storage.value.get_mut(&schema_id).unwrap();
        schema.insert(key, value);
    } else {
        let mut schema = BTreeMap::new();
        schema.insert(key, value);
        temples_storage.value.insert(schema_id, schema);
    };
}

pub fn mutate<F>(schema_id: &[u8; 32], key: &[u8], f: F)
where
    F: FnOnce(&mut Vec<u8>),
{
    let temples_storage = unsafe { TEMPLES_STORAGE.get_or_insert(Default::default()) };
    if let Some(schema) = temples_storage.value.get_mut(schema_id) {
        if let Some(value) = schema.get_mut(key) {
            f(value);
        }
    }
}

pub fn schema_exists(schema_id: &[u8; 32]) -> bool {
    let temples_storage = unsafe { TEMPLES_STORAGE.get_or_insert(Default::default()) };
    temples_storage.value.contains_key(schema_id)
}

pub fn exists(schema_id: &[u8; 32], key: &[u8]) -> bool {
    let temples_storage = unsafe { TEMPLES_STORAGE.get_or_insert(Default::default()) };
    if !temples_storage.value.contains_key(schema_id) {
        false
    } else {
        let schema = temples_storage.value.get(schema_id).unwrap();
        schema.contains_key(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::{exists, get, mutate, schema_exists, set};
    use gstd::vec;

    #[test]
    fn test_storage() {
        let schema_id = [0u8; 32];
        let key = vec![1, 2, 3];
        let value = vec![1, 2, 3];
        let new_value = vec![1, 2, 3, 4];

        assert_eq!(schema_exists(&schema_id), false);
        assert_eq!(exists(&schema_id, &key), false);

        set(schema_id, key.clone(), value.clone());
        assert_eq!(get(&schema_id, &key), value);
        assert_eq!(schema_exists(&schema_id), true);
        assert_eq!(exists(&schema_id, &key), true);

        mutate(&schema_id, &key, |value| {
            *value = new_value.clone();
        });
        assert_eq!(get(&schema_id, &key), new_value);
        assert_eq!(schema_exists(&schema_id), true);
        assert_eq!(exists(&schema_id, &key), true);
    }
}
