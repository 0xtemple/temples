use crate::primitive::{Primitive, PrimitiveError};

/// Represents a entity member.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Member {
    pub name: String,
    pub ty: Primitive,
    pub key: bool,
}

impl Member {
    pub fn serialize(&self) -> Result<Vec<Vec<u8>>, PrimitiveError> {
        self.ty.serialize()
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Struct {
    pub name: String,
    pub children: Vec<Member>,
}

impl Struct {
    /// Returns the struct member with the given name. Returns `None` if no such member exists.
    pub fn get(&self, field: &str) -> Option<&Primitive> {
        self.children
            .iter()
            .find(|m| m.name == field)
            .map(|m| &m.ty)
    }

    pub fn keys(&self) -> Vec<Member> {
        self.children.iter().filter(|m| m.key).cloned().collect()
    }
}

#[derive(Debug, Clone)]
pub struct ComponentMetadata {
    pub name: String,
    pub schema: Struct,
}

fn format_member(m: &Member) -> String {
    let mut str = if m.key {
        format!("  #[key]\n  {}: {}", m.name, m.ty.to_string())
    } else {
        format!("  {}: {}", m.name, m.ty.to_string())
    };
    match m.ty {
        Primitive::U8(value) => {
            if let Some(value) = value {
                str.push_str(&format!(" = {}", value));
            }
        }
        Primitive::U16(value) => {
            if let Some(value) = value {
                str.push_str(&format!(" = {}", value));
            }
        }
        Primitive::U32(value) => {
            if let Some(value) = value {
                str.push_str(&format!(" = {}", value));
            }
        }
        Primitive::U64(value) => {
            if let Some(value) = value {
                str.push_str(&format!(" = {}", value));
            }
        }
        Primitive::U128(value) => {
            if let Some(value) = value {
                str.push_str(&format!(" = {}", value));
            }
        }
        Primitive::USize(value) => {
            if let Some(value) = value {
                str.push_str(&format!(" = {}", value));
            }
        }
        Primitive::Bool(value) => {
            if let Some(value) = value {
                str.push_str(&format!(" = {}", value));
            }
        }
        Primitive::ActorId(value) => {
            if let Some(value) = value {
                str.push_str(&format!(" = {:?}", value));
            }
        }
    }
    str
}
