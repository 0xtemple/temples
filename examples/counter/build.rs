use counter_world::metadata::CounterMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<CounterMetadata>();
}
