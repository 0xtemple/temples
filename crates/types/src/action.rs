use gstd::Decode;
use gstd::Encode;
use scale_info::TypeInfo;

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum SystemAction {
    SetRecord(u8, u32),
    DelRecord(u8),
    GetRecord(u8),
}
