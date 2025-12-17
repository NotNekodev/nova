pub mod asset;
pub mod logger;
pub mod macros;
pub mod entity;

use logger::*;

use std::sync::{Arc, Mutex, OnceLock, mpsc::Sender};

use crate::shared::asset::AssetRequest;

//Static (baked in) data
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

//Global data that can only be set once
#[derive(Debug)]
pub struct GlobalData {
    pub project_dir: String
}

pub static GLOBAL_DATA: OnceLock<GlobalData> = OnceLock::new();

//Shared data that is accessed by threads
#[derive(Clone)]
pub struct RenderStats {
    pub framerate: f32, // in frames per second
    pub frametime: u128, // in microseconds
}

#[derive(Clone)]
pub struct SharedData {
    //init locks
    pub logic_init:     Arc<Mutex<()>>,
    pub asset_init:        Arc<Mutex<()>>,
    pub audio_init:     Arc<Mutex<()>>,

    //channels
    pub asset_tx:       Sender<AssetRequest>,

    //statistics
    pub render_stats:   Arc<Mutex<RenderStats>>,

    //logger
    pub logger:         Arc<Mutex<Logger>>,
}