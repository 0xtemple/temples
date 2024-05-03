use gstd::{collections::BTreeMap, Decode, Default, Encode, String, TypeInfo, Vec};

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SchemaType {
    Onchain,
    Offchain,
}

impl Default for SchemaType {
    fn default() -> Self {
        SchemaType::Onchain
    }
}

#[derive(Debug, Clone, Encode, Decode, Default, TypeInfo)]
pub struct SchemaMetadata {
    pub name: String,
    pub key_names: Vec<String>,
    pub key_types: Vec<String>,
    pub value_names: Vec<String>,
    pub value_types: Vec<String>,
    pub ty: SchemaType,
}

#[derive(Debug, Clone, Encode, Decode, Default, TypeInfo)]
pub struct Schema {
    pub metadata: SchemaMetadata,
    pub raw_data: BTreeMap<Vec<u8>, Vec<u8>>,
}
