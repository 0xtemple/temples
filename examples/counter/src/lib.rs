#![no_std]

pub use counter_components::counter::Counter;
use counter_metadata::{StateQuery, StateReply, SystemAction};
use gstd::msg;

#[no_mangle]
extern fn init() {
    Counter::register();
}

#[no_mangle]
extern fn handle() {
    let action: SystemAction = msg::load().expect("Unable to load the system action");
    match action {
        SystemAction::Add => counter_systems::counter::add_number(),
    }
}

#[no_mangle]
extern fn state() {
    let query: StateQuery = msg::load().expect("Unable to load the state query");
    match query {
        StateQuery::GetCurrentNumber => {
            let counter = Counter::get();
            msg::reply(StateReply::CurrentNumber(counter.value), 0)
                .expect("Unable to share the state");
        }
    }
}
