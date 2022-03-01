use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl Size {
    pub const fn new(w: u32, h: u32) -> Size {
        Size { w, h }
    }
}

/// Get the current Unix time in nanoseconds.
pub fn time_ns() -> u128 {
    let time_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    time_since_epoch.as_nanos()
}
