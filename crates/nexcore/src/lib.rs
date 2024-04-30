#![no_std]

use gstd::msg;
use nexcore_io::SCHEMA1;
use nexcore_metadata::{StateQuery, StateReply};
use temple_types::action::SystemAction;
pub use temple_types::event::NexCoreEvent;

#[gstd::async_main]
async fn main() {
    let action: SystemAction = msg::load().expect("Unable to load the system action");
    match action {
        SystemAction::SetRecord(key, value) => {
            let schema = unsafe { SCHEMA1.get_or_insert(Default::default()) };
            schema.insert(key, value);
            msg::reply(NexCoreEvent::SetRecordSuccess, 0)
                .expect("Error during replying with `NFTEvent::Owner`");
        }
        SystemAction::DelRecord(key) => {
            let schema = unsafe { SCHEMA1.get_or_insert(Default::default()) };
            let value = schema.remove(&key).unwrap();
            msg::reply(NexCoreEvent::DelRecordSuccess(value), 0)
                .expect("Error during replying with `NFTEvent::Owner`");
        }
        SystemAction::GetRecord(key) => {
            let schema = unsafe { SCHEMA1.get_or_insert(Default::default()) };
            let value = schema.get(&key).unwrap();
            msg::reply(NexCoreEvent::GetRecordSuccess(*value), 0)
                .expect("Error during replying with `NFTEvent::Owner`");
        }
    }
}

#[no_mangle]
extern fn state() {
    let query: StateQuery = msg::load().expect("Unable to load the state query");
    match query {
        StateQuery::GetRecord(key) => {
            let schema = unsafe { SCHEMA1.get_or_insert(Default::default()) };
            schema.get(&key).unwrap();
            msg::reply(StateReply::Record(*schema.get(&key).unwrap()), 0)
                .expect("Unable to share the state");
        }
    }
}
