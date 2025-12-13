use std::{sync::{Arc, Mutex}, thread, time::Duration};

mod io;
mod audio;
mod logic;
mod render;
mod shared;

use crate::shared::*;

fn main() {
    // This structure has the references to all the shared memory in the engine
    // This original copy belongs to the Render thread
    let shared = SharedData {
        io_init:    Arc::new(Mutex::new(())),
        audio_init: Arc::new(Mutex::new(())),
        logic_init: Arc::new(Mutex::new(())),

        render_stats: Arc::new(Mutex::new(
            RenderStats {
                framerate: 0,
                frametime: 0 
            }
        )),

        logger: Arc::new(Mutex::new(logger::Logger::new(true, "logs.txt")))
    };

    //Spawn the logic thread
    let logic_shared = shared.clone();
    thread::Builder::new().name("logic".to_string()).spawn(move || {
        logic::main(logic_shared);
    }).unwrap_or_else(|_| {
        fatal!(shared,"Failed to spawn logic thread")
    });

    //Spawn the io thread
    let logic_shared = shared.clone();
    thread::Builder::new().name("io".to_string()).spawn(move || {
        io::main(logic_shared);
    }).unwrap_or_else(|_| {
        fatal!(shared,"Failed to spawn io thread")
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
