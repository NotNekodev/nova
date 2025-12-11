pub mod vulkantest;

use crate::shared::*;
use std::sync::{Arc, Mutex};

pub fn render_main(data: &Arc<Mutex<shared_data>>){
    vulkantest::vk_init();
    println!("frame count: {}",data.lock().unwrap().frame_count)
}