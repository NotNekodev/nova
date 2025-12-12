pub mod asset;
pub mod logger;
pub mod macros;

use logger::*;

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SharedData {
    pub frame_count: Arc<Mutex<i32>>,
    pub logger: Arc<Mutex<Logger>>
}