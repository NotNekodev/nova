use crate::*;
use crate::shared::*;

pub fn main(shared: SharedData){
    if let Ok(mut _lock) = shared.io_init.lock() {
        info!(shared,"IO thread initialized");
    } 

}