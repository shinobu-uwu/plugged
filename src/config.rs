use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub sounds: Sounds,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Sounds {
    pub connected: PathBuf,
    pub disconnected: PathBuf,
}
