use crate::shared::*;
use std::sync::{Arc, Mutex};

pub fn logic_main(data: SharedData){
    // Keep this for now but mark as intentionally unused.
    let mut _framecount = data.frame_count.lock().unwrap();
    *_framecount += 1;
}