use gstd::vec;
use gstd::ToString;
use gstd::{Decode, Encode};
use temple_derive::Schema;
use temple_types::schema::SchemaMetadata;

#[derive(Schema, Encode, Decode, Default, Debug)]
pub struct CounterValue {
    pub value: u128,
}

#[derive(Schema, Encode, Decode, Default, Debug)]
pub struct CounterMap {
    #[key]
    pub key: u128,
    pub value: u128,
}
