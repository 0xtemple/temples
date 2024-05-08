use gstd::{Decode, Default, Encode, String};
use temple_derive::Schema;
use temple_types::schema::SchemaMetadata;

#[derive(Schema, Encode, Decode, Default, Debug)]
pub struct MyValueStruct {
    #[key]
    t1: u8,
    t2: u32,
    t3: String,
    t4: String,
}

#[test]
fn macro_tests() {
    env_logger::init();

    log::debug!("{:?}", MyValueStruct::resource_id());
    log::debug!("{:?}", MyValueStruct::metadata());
}
