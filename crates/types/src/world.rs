use crate::component::ComponentMetadata;
use alloc::borrow::ToOwned;
use gstd::collections::BTreeMap;
use gstd::ToString;
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
    pub components: BTreeMap<String, ComponentMetadata>,
}

impl WorldMetadata {
    pub fn name(&self) -> Option<String> {
        self.name.to_owned()
    }

    pub fn description(&self) -> Option<String> {
        self.description.to_owned()
    }

    /// Retrieves the metadata of a entity.
    pub fn component(&self, name: impl AsRef<str>) -> Option<&ComponentMetadata> {
        self.components.get(name.as_ref())
    }

    /// Retrieves the metadata of a entity.
    pub fn register_component(&mut self, name: &String, metadata: &ComponentMetadata) {
        self.components.insert(name.to_string(), metadata.clone());
    }
}
