pub mod asset;
pub mod logger;
pub mod macros;

use logger::*;

use std::sync::{Arc, Mutex, OnceLock, mpsc::Sender};

use crate::shared::asset::AssetRequest;

//Static (baked in) data
const VERSION: &str = env!("CARGO_PKG_VERSION");

//Global data that can only be set once
pub struct GlobalData {
    pub project_dir: String
}

static CONSTANT_DATA: OnceLock<GlobalData> = OnceLock::new();

//Shared data that is accessed by threads
#[derive(Clone)]
pub struct RenderStats {
    pub framerate: u32, // in frames per second
    pub frametime: u32, // in microseconds
}

#[derive(Clone)]
pub struct SharedData {
    //init locks
    pub logic_init:     Arc<Mutex<()>>,
    pub io_init:        Arc<Mutex<()>>,
    pub audio_init:     Arc<Mutex<()>>,

    //channels
    pub asset_tx:       Sender<AssetRequest>,

    //statistics
    pub render_stats:   Arc<Mutex<RenderStats>>,

    //logger
    pub logger:         Arc<Mutex<Logger>>,
}