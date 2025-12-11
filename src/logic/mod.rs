use crate::shared::*;
use std::sync::{Arc, Mutex};

pub fn logic_main(data: &Arc<Mutex<SharedData>>){
    // Keep this for now but mark as intentionally unused.
    let mut _framecount = data.lock().unwrap().frame_count;
    _framecount += 1;
}