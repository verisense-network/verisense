use crate::nucleus::{Gluon, Nucleus};
use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Cage {
    sender: Sender<Gluon>,
    tunnel: Vec<Gluon>,
}

impl Cage {
    // 1. we need runtime storage to read wasm binary
    // 2. we need p2p networking
    // 3. we need offchain kv storage
    // 4. we need keystore
}
