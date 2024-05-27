#![no_std]

mod shemas;
mod systems;

use crate::shemas::{CounterMap, CounterValue};
use counter_metadata::SystemAction;
use gstd::prog::ProgramGenerator;
use gstd::{msg, ActorId};
use hex_literal::hex;
use temple_storage::nexcore_manager;

#[gstd::async_init]
async fn init() {
    // ProgramGenerator returs ProgramId
    let (_, nexcore) = ProgramGenerator::create_program_with_gas(
        hex!("725d96aa0add2da5285ae9fee22017ba8873ec7b63757655304be3a4cce16760").into(),
        b"",
        10_000_000_000,
        0,
    )
    .expect("Unable to create program");

    nexcore_manager::new(nexcore);
    CounterValue::register(nexcore).await;
    CounterMap::register(nexcore).await;
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
