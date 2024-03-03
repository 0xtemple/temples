use serde::{Deserialize, Serialize};

/// Represents a system's entity dependency.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dependency {
    /// Name of the entity.
    pub name: String,
    pub read: bool,
    pub write: bool,
}
