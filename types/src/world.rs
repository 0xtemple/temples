use crate::component::ComponentMetadata;
use gstd::collections::HashMap;
use gstd::String;
use gstd::ToString;

/// Represents the metadata of a World
#[derive(Debug, Clone, Default)]
pub struct WorldMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cover_uri: Option<String>,
    pub icon_uri: Option<String>,
    pub website: Option<String>,
    pub socials: Option<HashMap<String, String>>,
    pub components: HashMap<String, ComponentMetadata>,
}

impl WorldMetadata {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
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
