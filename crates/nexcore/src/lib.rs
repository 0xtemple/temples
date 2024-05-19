#![no_std]

use gstd::msg;
use temple_types::action::SystemAction;
pub use temple_types::event::NexCoreEvent;

#[gstd::async_main]
async fn main() {
    let action: SystemAction = msg::load().expect("Unable to load the system action");
    match action {
        SystemAction::RegisterSchema(schema_id, metadata) => {
            temple_storage::register(schema_id, metadata.clone());
            msg::reply(NexCoreEvent::RegisterSchemaSuccess(schema_id, metadata), 0)
                .expect("Error during replying with `NexCoreEvent::RegisterSchemaSuccess`");
        }
        SystemAction::SetRecord(schema_id, key, value) => {
            temple_storage::set(schema_id.clone(), key.clone(), value.clone());
            msg::reply(NexCoreEvent::SetRecordSuccess(schema_id, key, value), 0)
                .expect("Error during replying with `NexCoreEvent::SetRecordSuccess`");
        }
        SystemAction::DelRecord(schema_id, key) => {
            temple_storage::remove(&schema_id, &key);
            msg::reply(NexCoreEvent::DelRecordSuccess(schema_id, key), 0)
                .expect("Error during replying with `NexCoreEvent::DelRecord`");
        }
        SystemAction::GetRecord(schema_id, key) => {
            let value = temple_storage::get(&schema_id, &key);
            msg::reply(NexCoreEvent::GetRecordSuccess(value), 0)
                .expect("Error during replying with `NexCoreEvent::GetRecordSuccess`");
        }
    }
}
