use crate::shared::*;
use std::sync::{Arc, Mutex};

pub fn logic_main(data: &Arc<Mutex<shared_data>>){
    let mut framecount = data.lock().unwrap().frame_count;
    framecount += 1;
}