use counter_components::counter::CounterComponent;

pub fn add_number() {
    let current_number = CounterComponent.get();
    CounterComponent.set(current_number);
}
