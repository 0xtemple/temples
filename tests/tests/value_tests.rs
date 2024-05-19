use gmeta::Type;
use gstd::{ActorId, TypeInfo};
use gtest::{Program, System};
use temple_types::event::ComponentRegistered;

// const v = ComponentRegistered {
//     component_id: [],
//     key_names: vec![],
//     key_types: vec![],
//     value_names: vec![],
//     value_types: vec![],
// };

#[test]
fn storage_value_struct_tests() {
    let system = System::new();
    system.init_logger();

    let test_program = Program::current(&system);

    let test_program_id = test_program.id();
    println!("{:?}", test_program_id);

    let res = test_program.send_bytes(42, "INIT");
    assert!(!res.main_failed());
    println!("{:?}", res.log()[1].payload());

    let res = test_program.send_bytes(42, "HANDLE");
    assert!(!res.main_failed());
    println!("{:?}", res.log()[1].payload());
    // let res = program.send_bytes(42, String::from("PING").encode());
    // let log = Log::builder().source(1).dest(42).payload_bytes("PONG");
    // assert!(res.contains(&log));

    // println!("{:?}", u8::meta_type());
    // println!("{:?}", bool::type_info());
    // println!("{:?}", ActorId::type_info());
    // println!("{:?}", ValueData::type_info());
    // println!("{:?}", Key::type_info());
    // println!("{:?}", ValueData::type_info().type_def);
}

#[test]
fn storage_value_primitives_tests() {
    let system = System::new();
    system.init_logger();

    let test_program = Program::current(&system);

    let test_program_id = test_program.id();
    println!("{:?}", test_program_id);

    let res = test_program.send_bytes(42, "INIT");
    assert!(!res.main_failed());
    println!("{:?}", res.log()[1].payload());

    let res = test_program.send_bytes(42, "HANDLE");
    assert!(!res.main_failed());
    println!("{:?}", res.log()[1].payload());

    let res = test_program.send_bytes(42, "HANDLE");
    assert!(!res.main_failed());
    println!("{:?}", res.log()[1].payload());
    // let res = program.send_bytes(42, String::from("PING").encode());
    // let log = Log::builder().source(1).dest(42).payload_bytes("PONG");
    // assert!(res.contains(&log));

    // println!("{:?}", u8::meta_type());
    // println!("{:?}", bool::type_info());
    // println!("{:?}", ActorId::type_info());
    // println!("{:?}", ValueData::type_info());
    // println!("{:?}", Key::type_info());
    // println!("{:?}", ValueData::type_info().type_def);
}
