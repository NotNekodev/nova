use crate::*;
use crate::shared::*;

pub fn main(shared: SharedData){
    if let Ok(mut _lock) = shared.logic_init.lock() {
        info!(shared,"Logic thread initialized");
    } 

}