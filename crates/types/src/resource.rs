use core::panic;
use gstd::{Decode, Default, Encode, String, ToString, TypeInfo};

pub type ResourceId = [u8; 32];

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum ResourceType {
    OnchainSchema,
    OffchainSchema,
    Namespace,
    System,
}

impl Default for ResourceType {
    fn default() -> Self {
        Self::OnchainSchema
    }
}

#[derive(Debug, Clone, Encode, Decode, Default, TypeInfo)]
pub struct Resource {
    namespace: [u8; 15],
    name: [u8; 16],
    ty: ResourceType,
}

impl Resource {
    pub fn new(namespace_str: &str, name_str: &str, ty: ResourceType) -> Self {
        if namespace_str.len() > 15 || name_str.len() > 16 {
            panic!("");
        };
        let mut namespace: [u8; 15] = [0; 15];
        let mut name: [u8; 16] = [0; 16];
        namespace[..namespace_str.len()].copy_from_slice(namespace_str.as_bytes());
        name[..name_str.len()].copy_from_slice(name_str.as_bytes());
        Resource {
            namespace,
            name,
            ty,
        }
    }

    pub fn namespace(&self) -> String {
        let index = self
            .namespace
            .iter()
            .position(|&x| x == 0)
            .unwrap_or(self.namespace.len());
        let namespace_slice = &self.namespace[..index];
        String::from_utf8_lossy(namespace_slice).to_string()
    }

    pub fn name(&self) -> String {
        let index = self
            .name
            .iter()
            .position(|&x| x == 0)
            .unwrap_or(self.name.len());
        let name_slice = &self.name[..index];
        String::from_utf8_lossy(name_slice).to_string()
    }

    pub fn ty(self) -> ResourceType {
        self.ty
    }

    pub fn resource_id(&self) -> ResourceId {
        Self::encode(&self).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::resource::{Resource, ResourceType};
    use gstd::string::ToString;
    use parity_scale_codec::Encode;

    #[test]
    fn test_resource_id() {
        env_logger::init();
        let r = Resource::new("nexcore", "test", ResourceType::OnchainSchema);
        assert_eq!(32, r.encode().len());
        log::debug!("{:?}", r.encode());
        log::debug!("{:?}", r.namespace());
        log::debug!("{:?}", r.resource_id());
        assert_eq!(r.namespace(), "nexcore".to_string());
        assert_eq!(r.name(), "test".to_string());
    }
}
