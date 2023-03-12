//! # Rummage
//! 
//! Collect various buildtime and runtime properties, and provides an easy way to dump those
//! properties into [`tracing`] event.


pub fn gather() {
    bosion::gather();
}


mod internal {
    include!(env!("BOSION_PATH"));
}

pub struct RummageInfo {
    pub git_commit_hash: String,
}

impl RummageInfo {
    fn new() -> Self {
        Self {
            git_commit_hash: internal::Bosion::GIT_COMMIT_HASH,
        }
    }
}