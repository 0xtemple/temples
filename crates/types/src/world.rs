use crate::schema::SchemaMetadata;
use gstd::collections::BTreeMap;
use gstd::{Decode, Encode, String, TypeInfo};

/// Represents the metadata of a World
#[derive(Debug, Clone, Encode, Decode, Default, TypeInfo)]
pub struct WorldMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cover_uri: Option<String>,
    pub icon_uri: Option<String>,
    pub website: Option<String>,
    pub socials: Option<BTreeMap<String, String>>,
    pub components: BTreeMap<String, SchemaMetadata>,
}
