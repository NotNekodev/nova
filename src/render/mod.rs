pub mod vulkantest;
pub mod window;

use crate::shared::*;
use std::sync::{Arc, Mutex};

pub fn render_main(data: &Arc<Mutex<shared_data>>){
    vulkantest::vk_init();

    window::create_window_app(data);
}