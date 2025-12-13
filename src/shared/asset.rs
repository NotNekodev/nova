use std::sync::mpsc;
use crate::*;

use super::SharedData;

#[allow(dead_code)]
// Make Asset public so it can appear in a public function’s signature.
pub enum Asset {
    Shader(Vec<u32>),
    None
}

pub struct AssetRequest {
    pub name: String,
    pub sender: mpsc::Sender<Asset>
}

#[allow(dead_code)]
pub fn get_asset(shared: &SharedData,name: &str) -> Asset {
    //reply channel
    let (reply_tx, reply_rx) = mpsc::channel::<Asset>();

    //send command
    shared.asset_tx.send(AssetRequest { name: name.to_string(), sender: reply_tx });

    return reply_rx.recv().unwrap_or_else(|e| {
        err!(shared, "Failed to load Asset \"{name}\": {e}");
        return Asset::None
    })
}