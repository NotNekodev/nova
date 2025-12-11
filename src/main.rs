mod render;
mod shared;

use render::*;
use shared::*;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let shared = Arc::new(Mutex::new(shared_data{
        frame_count: 0
    }));

    let test_shared = Arc::clone(&shared);
    thread::spawn(move || {
         let mut framecount = test_shared.lock().unwrap().frame_count;
         framecount += 1;
    });

    render_main(&shared)
}
