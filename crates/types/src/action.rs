use crate::schema::SchemaMetadata;
use gstd::Decode;
use gstd::Encode;
use gstd::Vec;
use scale_info::TypeInfo;

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum SystemAction {
    RegisterSchema([u8; 32], SchemaMetadata),
    SetRecord([u8; 32], Vec<u8>, Vec<u8>),
    DelRecord([u8; 32], Vec<u8>),
    GetRecord([u8; 32], Vec<u8>),
}
