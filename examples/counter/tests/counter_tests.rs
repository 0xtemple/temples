use counter_world::metadata::SystemAction;
use gtest::{Program, System};

#[test]
fn counter_tests() {
    let system = System::new();
    system.init_logger();

    let counter_program = Program::current(&system);

    let res = counter_program.send_bytes(42, "INIT");
    assert!(!res.main_failed());
    println!("{:?}", res.log()[1].payload());
    // payload -> ComponentRegistered

    let res = counter_program.send(42, SystemAction::Add);
    assert!(!res.main_failed());
    println!("{:?}", res.log()[1].payload());
    // payload -> ComponentSetRecord
}
