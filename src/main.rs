use std::{sync::{Arc, Mutex, mpsc}, thread, time::Duration};

mod asset;
mod audio;
mod logic;
mod render;
mod shared;

use crate::shared::{asset::*, logger::*, *};

fn mt_main(logger: Logger) {
    let (asset_tx,asset_rx) = mpsc::channel::<AssetRequest>();

    let shared = SharedData {
        io_init:    Arc::new(Mutex::new(())),
        audio_init: Arc::new(Mutex::new(())),
        logic_init: Arc::new(Mutex::new(())),

        render_stats: Arc::new(Mutex::new(
            RenderStats {
                framerate: 0.0,
                frametime: 0 
            }
        )),

        asset_tx: asset_tx,

        logger: Arc::new(Mutex::new(logger))
    };

    //Spawn the logic thread
    let logic_shared = shared.clone();
    thread::Builder::new().name("logic".to_string()).spawn(move || {
        logic::main(logic_shared);
    }).unwrap_or_else(|_| {
        fatal!(shared,"Failed to spawn logic thread")
    });

    //Spawn the asset thread
    let logic_shared = shared.clone();
    thread::Builder::new().name("asset".to_string()).spawn(move || {
        asset::main(logic_shared,asset_rx);
    }).unwrap_or_else(|_| {
        fatal!(shared,"Failed to spawn asset thread")
    });

    //Spawn the audio thread
    let logic_shared = shared.clone();
    thread::Builder::new().name("audio".to_string()).spawn(move || {
        audio::main(logic_shared);
    }).unwrap_or_else(|_| {
        fatal!(shared,"Failed to spawn audio thread")
    });

    // HACK: make sure all threads aquire the lock before the render thread
    thread::sleep(Duration::from_millis(1));

    render::main(shared);
}

fn main() {
    //init basic stuff
    let logger = logger::Logger::new(true, "logs.txt");

    info_early!(logger, "Early initialization done!");

    // Starts mutithreaded operation
    // the structure contains information useful to all threads
    mt_main(logger);
}
