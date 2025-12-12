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
    //This structure has the references to all the shared memory in the engine
    let shared = SharedData {
        frame_count: Arc::new(Mutex::new(0)),
        logger: Arc::new(Mutex::new(Logger::new(true, "logs.txt")))
    };

    //spawn all threads
    let logic_shared = shared.clone();
    let logic_handle = thread::Builder::new().name("logic".to_string()).spawn(move || {
        logic_main(logic_shared);
    }).unwrap_or_else(|_| {
        fatal!(*shared.logger.lock().unwrap(),"Failed to spawn logic thread")
    });

    let render_shared = shared.clone();
    render_main(render_shared);


    //join all threads
    logic_handle.join().unwrap_or_else(|_| {
        fatal!(*shared.logger.lock().unwrap(),"Render thread panicked, can't continue operation!")
    });

}
