use std::sync::{Arc, Mutex};

mod render;
mod logic;
mod shared;

use render::*;
use shared::*;
use logic::*;
use std::thread;

use crate::shared::logger::Logger;

fn main() {
    // This structure has the references to all the shared memory in the engine
    // This original copy belongs to the Render thread
    let shared = SharedData {
        frame_count: Arc::new(Mutex::new(0)),
        logger: Arc::new(Mutex::new(Logger::new(true, "logs.txt")))
    };

    //Spawn the logic thread
    let logic_shared = shared.clone();
    thread::Builder::new().name("logic".to_string()).spawn(move || {
        logic_main(logic_shared);
    }).unwrap_or_else(|_| {
        fatal!(shared,"Failed to spawn logic thread")
    });

    render_main(shared);
}
