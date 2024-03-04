use test_metadata::TestMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<TestMetadata>();
}
