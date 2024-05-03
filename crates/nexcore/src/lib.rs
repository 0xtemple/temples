#![no_std]

use gstd::msg;
use temple_types::action::SystemAction;
pub use temple_types::event::NexCoreEvent;

#[gstd::async_main]
async fn main() {
    let action: SystemAction = msg::load().expect("Unable to load the system action");
    match action {
        SystemAction::RegisterSchema(shcema_id, schema_metadata) => {
            msg::reply(
                NexCoreEvent::RegisterSchemaSuccess(shcema_id, schema_metadata),
                0,
            )
            .expect("Error during replying with `NexCoreEvent::RegisterSchemaSuccess`");
        }
        SystemAction::SetRecord(shcema_id, key, value) => {
            temple_storage::set(shcema_id.clone(), key.clone(), value.clone());
            msg::reply(NexCoreEvent::SetRecordSuccess(shcema_id, key, value), 0)
                .expect("Error during replying with `NexCoreEvent::SetRecordSuccess`");
        }
        SystemAction::DelRecord(shcema_id, key) => {
            temple_storage::remove(&shcema_id, &key);
            msg::reply(NexCoreEvent::DelRecordSuccess(shcema_id, key), 0)
                .expect("Error during replying with `NexCoreEvent::DelRecord`");
        }
        SystemAction::GetRecord(shcema_id, key) => {
            let value = temple_storage::get(&shcema_id, &key);
            msg::reply(NexCoreEvent::GetRecordSuccess(value), 0)
                .expect("Error during replying with `NexCoreEvent::GetRecordSuccess`");
        }
    }
}
