use gstd::{Decode, Encode, String, Vec};

#[derive(Debug, Clone, Encode, Decode)]
pub enum ComponentType {
    Onchain,
    Offchain,
}

#[derive(Debug, Clone)]
pub struct ComponentMetadata {
    pub name: String,
    pub key_names: Vec<String>,
    pub key_types: Vec<String>,
    pub value_names: Vec<String>,
    pub value_types: Vec<String>,
    pub ty: ComponentType,
}
