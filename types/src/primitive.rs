use gstd::ActorId;
use gstd::{Decode, Encode};
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

#[derive(AsRefStr, Display, EnumIter, EnumString, Copy, Clone, Debug, PartialEq, Hash, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Primitive {
    U8(Option<u8>),
    U16(Option<u16>),
    U32(Option<u32>),
    U64(Option<u64>),
    U128(Option<u128>),
    USize(Option<u32>),
    Bool(Option<bool>),
    ActorId(Option<ActorId>),
}

#[derive(Debug, thiserror::Error)]
pub enum PrimitiveError {
    #[error("Value must have at least one FieldElement")]
    MissingFieldElement,
    #[error("Not enough FieldElements for U256")]
    NotEnoughFieldElements,
    #[error("Unsupported CairoType for SQL formatting")]
    UnsupportedType,
    #[error("Set value type mismatch")]
    TypeMismatch,
    #[error("ValueOutOfRange")]
    ValueOutOfRange,
}

/// Macro to generate setter methods for Primitive enum variants.
macro_rules! set_primitive {
    ($method_name:ident, $variant:ident, $type:ty) => {
        /// Sets the inner value of the `Primitive` enum if variant matches.
        pub fn $method_name(&mut self, value: Option<$type>) -> Result<(), PrimitiveError> {
            match self {
                Primitive::$variant(_) => {
                    *self = Primitive::$variant(value);
                    Ok(())
                }
                _ => Err(PrimitiveError::TypeMismatch),
            }
        }
    };
}

/// Macro to generate getter methods for Primitive enum variants.
macro_rules! as_primitive {
    ($method_name:ident, $variant:ident, $type:ty) => {
        /// If the `Primitive` is variant type, returns the associated vartiant value. Returns
        /// `None` otherwise.
        pub fn $method_name(&self) -> Option<$type> {
            match self {
                Primitive::$variant(value) => *value,
                _ => None,
            }
        }
    };
}

impl Primitive {
    as_primitive!(as_u8, U8, u8);
    as_primitive!(as_u16, U16, u16);
    as_primitive!(as_u32, U32, u32);
    as_primitive!(as_u64, U64, u64);
    as_primitive!(as_u128, U128, u128);
    as_primitive!(as_bool, Bool, bool);
    as_primitive!(as_usize, USize, u32);
    as_primitive!(as_actor_id, ActorId, ActorId);

    set_primitive!(set_u8, U8, u8);
    set_primitive!(set_u16, U16, u16);
    set_primitive!(set_u32, U32, u32);
    set_primitive!(set_u64, U64, u64);
    set_primitive!(set_u128, U128, u128);
    set_primitive!(set_bool, Bool, bool);
    set_primitive!(set_usize, USize, u32);
    set_primitive!(set_actor_id, ActorId, ActorId);

    pub fn to_numeric(&self) -> usize {
        match self {
            Primitive::U8(_) => 0,
            Primitive::U16(_) => 1,
            Primitive::U32(_) => 2,
            Primitive::U64(_) => 3,
            Primitive::U128(_) => 4,
            Primitive::USize(_) => 5,
            Primitive::Bool(_) => 6,
            Primitive::ActorId(_) => 7,
        }
    }

    pub fn from_numeric(value: usize) -> Option<Self> {
        Self::iter().nth(value)
    }

    pub fn deserialize(&mut self, datas: &mut Vec<Vec<u8>>) -> Result<(), PrimitiveError> {
        if datas.is_empty() {
            return Err(PrimitiveError::MissingFieldElement);
        }

        match self {
            Primitive::U8(ref mut value) => {
                *value = Some(Decode::decode(&mut &datas.remove(0)[..]).expect("Failed to decode"));
                Ok(())
            }
            Primitive::U16(ref mut value) => {
                *value = Some(Decode::decode(&mut &datas.remove(0)[..]).expect("Failed to decode"));
                Ok(())
            }
            Primitive::U32(ref mut value) => {
                *value = Some(Decode::decode(&mut &datas.remove(0)[..]).expect("Failed to decode"));
                Ok(())
            }
            Primitive::U64(ref mut value) => {
                *value = Some(Decode::decode(&mut &datas.remove(0)[..]).expect("Failed to decode"));
                Ok(())
            }
            Primitive::USize(ref mut value) => {
                *value = Some(Decode::decode(&mut &datas.remove(0)[..]).expect("Failed to decode"));
                Ok(())
            }
            Primitive::Bool(ref mut value) => {
                *value = Some(Decode::decode(&mut &datas.remove(0)[..]).expect("Failed to decode"));
                Ok(())
            }
            Primitive::U128(ref mut value) => {
                *value = Some(Decode::decode(&mut &datas.remove(0)[..]).expect("Failed to decode"));
                Ok(())
            }
            Primitive::ActorId(ref mut value) => {
                *value = Some(
                    ActorId::from_slice(datas.remove(0).as_slice()).expect("Failed to decode"),
                );
                Ok(())
            }
        }
    }

    pub fn serialize(&self) -> Result<Vec<Vec<u8>>, PrimitiveError> {
        match self {
            Primitive::U8(value) => value
                .map(|v| Ok(vec![v.encode()]))
                .unwrap_or(Err(PrimitiveError::MissingFieldElement)),
            Primitive::U16(value) => value
                .map(|v| Ok(vec![v.encode()]))
                .unwrap_or(Err(PrimitiveError::MissingFieldElement)),
            Primitive::U32(value) => value
                .map(|v| Ok(vec![v.encode()]))
                .unwrap_or(Err(PrimitiveError::MissingFieldElement)),
            Primitive::U64(value) => value
                .map(|v| Ok(vec![v.encode()]))
                .unwrap_or(Err(PrimitiveError::MissingFieldElement)),
            Primitive::USize(value) => value
                .map(|v| Ok(vec![v.encode()]))
                .unwrap_or(Err(PrimitiveError::MissingFieldElement)),
            Primitive::Bool(value) => value
                .map(|v| Ok(vec![v.encode()]))
                .unwrap_or(Err(PrimitiveError::MissingFieldElement)),
            Primitive::U128(value) => value
                .map(|v| Ok(vec![v.encode()]))
                .unwrap_or(Err(PrimitiveError::MissingFieldElement)),
            Primitive::ActorId(value) => value
                .map(|v| Ok(vec![v.as_ref().to_vec()]))
                .unwrap_or(Err(PrimitiveError::MissingFieldElement)),
        }
    }
}
