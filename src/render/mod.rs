pub mod vulkantest;
pub mod window;

use crate::*;
use crate::shared::*;

pub fn render_main(data: SharedData){
    info!(data,"Nova Engine finished initializing");
    window::create_window_app(data);
}