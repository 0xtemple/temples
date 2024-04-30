use nexcore_metadata::NexcoreMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<NexcoreMetadata>();
}
