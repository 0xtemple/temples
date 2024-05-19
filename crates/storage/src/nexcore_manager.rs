use gstd::ActorId;

pub static mut NEXCORE_ACTOR_ID: Option<ActorId> = None;

pub fn new(nexcore: ActorId) {
    let manager = unsafe { NEXCORE_ACTOR_ID.get_or_insert(Default::default()) };
    *manager = nexcore;
}

pub fn get() -> ActorId {
    let manager = unsafe { NEXCORE_ACTOR_ID.get_or_insert(Default::default()) };
    *manager
}
