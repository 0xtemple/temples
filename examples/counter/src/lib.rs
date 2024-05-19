#![no_std]

mod shemas;
mod systems;

use crate::shemas::{CounterMap, CounterValue};
use counter_metadata::SystemAction;
use gstd::{msg, ActorId};
use hex_literal::hex;
use temple_storage::nexcore_manager;

#[gstd::async_init]
async fn init() {
    nexcore_manager::new(ActorId::from(hex![
        "9cd7aeec68189c75b22b1778d221dd4c578ce425282cb42bc53861bb204184fb"
    ]));
    CounterValue::register(nexcore_manager::get()).await;
    CounterMap::register(nexcore_manager::get()).await;
}

#[gstd::async_main]
async fn main() {
    let action: SystemAction = msg::load().expect("Unable to load the system action");
    match action {
        SystemAction::SetValue(value) => systems::set_value(value).await,
        SystemAction::SetMap(key, value) => systems::set_map(key, value).await,
        SystemAction::AddValue => systems::add_value().await,
        SystemAction::AddMap(key) => systems::add_map(key).await,
    }
}
