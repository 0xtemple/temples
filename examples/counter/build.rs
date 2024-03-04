use counter_metadata::CounterMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<CounterMetadata>();
}
