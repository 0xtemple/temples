use gstd::{Decode, Encode, String, TypeInfo, Vec};

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum StorageType {
    Onchain,
    Offchain,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct ComponentMetadata {
    pub name: String,
    pub key_names: Vec<String>,
    pub key_types: Vec<String>,
    pub value_names: Vec<String>,
    pub value_types: Vec<String>,
    pub ty: StorageType,
}
