//! Protocol driver for `http3`

use crate::{Error};

/// Configuration for `http3` integration.
#[derive(Debug, Clone)]
pub struct Config {
    pub endpoint: String,
    pub timeout_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            endpoint: "127.0.0.1:0".into(),
            timeout_ms: 5000,
        }
    }
}

/// Run a single protocol handshake.
pub async fn handshake(cfg: &Config) -> Result<(), Error> {
    let _ = cfg;
    Ok(())
}
