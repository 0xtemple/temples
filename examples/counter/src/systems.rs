use crate::shemas::{CounterMap, CounterValue};
use temple_storage::nexcore_manager;

pub async fn set_value(value: u128) {
    CounterValue::set(nexcore_manager::get(), CounterValue { value }).await;
}

pub async fn set_map(key: u128, value: u128) {
    CounterMap::set(nexcore_manager::get(), key, CounterMap { key, value }).await;
}

pub async fn add_value() {
    let counter = CounterValue::get(nexcore_manager::get()).await;
    CounterValue::set(
        nexcore_manager::get(),
        CounterValue {
            value: counter.value + 1,
        },
    )
    .await;
}

pub async fn add_map(key: u128) {
    let counter = CounterMap::get(nexcore_manager::get(), key).await;
    CounterMap::set(
        nexcore_manager::get(),
        key,
        CounterMap {
            key,
            value: counter.value + 1,
        },
    )
    .await;
}
