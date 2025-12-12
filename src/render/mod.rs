pub mod vulkantest;
pub mod window;

use crate::*;
use crate::shared::*;

pub fn render_main(data: SharedData){
    info!(data.logger.lock().unwrap(),"Nova Engine finished initializing");
    window::create_window_app(data);
}