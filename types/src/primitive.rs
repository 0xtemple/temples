// use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum Primitive {
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    Bool,
    ActorId,
}
