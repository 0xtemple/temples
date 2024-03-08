use gstd::vec;
use gstd::ToString;
use gstd::Vec;
use gstd::{Decode, Encode};
use temple_derive::StorageValue;
use temple_types::component::ComponentMetadata;

#[derive(StorageValue, Encode, Decode, Default, Debug)]
pub struct Counter {
    pub value: u128,
}
