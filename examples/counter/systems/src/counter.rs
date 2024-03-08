use counter_components::counter::Counter;

pub fn add_number() {
    let counter = Counter::get();
    Counter::set(Counter {
        value: counter.value + 1,
    });
}
