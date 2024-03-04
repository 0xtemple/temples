use gstd::collections::HashMap;
use gstd::Vec;
use temple_types::world::WorldMetadata;
pub static mut TEMPLE_WORLD_STORAGE: Option<WorldMetadata> = None;
pub static mut TEMPLE_STORAGE_VALUE: Option<HashMap<[u8; 32], Vec<u8>>> = None;
