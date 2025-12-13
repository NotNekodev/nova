use crate::*;
use crate::shared::*;

pub fn main(shared: SharedData){
    if let Ok(mut _lock) = shared.audio_init.lock() {
        info!(shared,"Audio thread initialized");
    } 

}