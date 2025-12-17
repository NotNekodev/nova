mod asset;
mod audio;
mod logic;
mod render;
mod shared;

use std::{collections::HashSet, sync::*, thread, time::*};

use crate::shared::{asset::*, entity::{Entity, create_entity}, logger::*, *};

fn mt_main(logger: Logger) {
    let (asset_tx,asset_rx) = mpsc::channel::<AssetRequest>();

    let shared = SharedData {
        asset_init:    Arc::new(Mutex::new(())),
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

    // Spawn the logic thread
    let logic_shared = shared.clone();
    thread::Builder::new().name("logic".to_string()).spawn(move || {
        logic::main(logic_shared);
    }).unwrap_or_else(|_| {
        fatal!(shared,"Failed to spawn logic thread")
    });

    // Spawn the asset thread
    let logic_shared = shared.clone();
    thread::Builder::new().name("asset".to_string()).spawn(move || {
        asset::main(logic_shared,asset_rx);
    }).unwrap_or_else(|_| {
        fatal!(shared,"Failed to spawn asset thread")
    });

    // Spawn the audio thread
    let logic_shared = shared.clone();
    thread::Builder::new().name("audio".to_string()).spawn(move || {
        audio::main(logic_shared);
    }).unwrap_or_else(|_| {
        fatal!(shared,"Failed to spawn audio thread")
    });

    // HACK: make sure all threads aquire the lock
    thread::sleep(Duration::from_millis(1));

    // make sure all threads are initialized
    drop(shared.logic_init.lock());
    drop(shared.asset_init.lock());
    drop(shared.audio_init.lock());

    // our main thread will be the render thread
    render::main(shared);
}

fn init_global_data(logger: &Logger) {
    //NOTE: there is no fuckin way this can fail
    GLOBAL_DATA.set(GlobalData { project_dir: ".".to_string() }).expect("Couldn't set global data");

    info_early!(logger, "Project dir: {}", GLOBAL_DATA.get().unwrap().project_dir);
}

fn main() {
    //init basic stuff
    let logger = logger::Logger::new(true, "logs.txt");

    init_global_data(&logger);

    let mut rng = rand::rng();
    let mut ents = HashSet::<Entity>::new();

    info_early!(logger, "Test entity ID: {:?}", create_entity(&mut ents, &mut rng));

    info_early!(logger, "Early initialization done!");

    // Starts mutithreaded operation
    // the structure contains information useful to all threads
    mt_main(logger);
}
