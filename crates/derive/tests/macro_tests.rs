use gstd::{Decode, Encode, String};
use temple_derive::StorageValue;
use temple_types::component::ComponentMetadata;

// pub trait StorageValue<T> {
//     fn hello_macro();
//     fn component_id() -> [u8; 32];
//     fn metadata() -> ComponentMetadata;
//     fn register();
//     fn get(&self) -> T;
// }

#[derive(StorageValue, Encode, Decode, Default, Debug)]
struct MyStruct {
    t1: u8,
    t2: u32,
    t3: String,
    t4: String,
}

#[test]
fn macro_tests() {
    println!("{:?}", MyStruct::component_id());
    println!("{:?}", MyStruct::metadata());
    println!("{:?}", MyStruct::register());

    println!("{:?}", MyStruct::get());

    let t = MyStruct {
        t1: 8,
        t2: 8,
        t3: "Hello".into(),
        t4: "World".into(),
    };
    MyStruct::set(t);
    println!("{:?}", MyStruct::get());
}
