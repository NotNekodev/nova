pub mod vulkantest;
pub mod window;

use crate::shared::*;
use std::sync::{Arc, Mutex};

pub fn render_main(data: &Arc<Mutex<SharedData>>){

    window::create_window_app(data.clone());
}