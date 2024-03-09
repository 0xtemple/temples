use crate::io::TEMPLE_WORLD_STORAGE;
use gstd::borrow::ToOwned;
use gstd::String;
use temple_types::world::WorldMetadata;

pub fn init(name: String, description: String) {
    let world = unsafe { TEMPLE_WORLD_STORAGE.get_or_insert(Default::default()) };
    world.name = Some(name);
    world.description = Some(description);
}

pub fn get() -> WorldMetadata {
    unsafe { TEMPLE_WORLD_STORAGE.get_or_insert(Default::default()) }.to_owned()
}
