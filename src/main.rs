mod render;
mod logic;
mod shared;

use render::*;
use shared::*;
use logic::*;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let shared = Arc::new(Mutex::new(SharedData {
        frame_count: 0
    }));

    let test_shared = Arc::clone(&shared);
    thread::spawn(move || {
        logic_main(&test_shared);
    });

    render_main(&shared)
}
