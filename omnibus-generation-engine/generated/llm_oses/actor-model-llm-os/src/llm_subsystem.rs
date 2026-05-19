//! LLM Subsystem — token_actor
//!
//! Every process is an actor; LLM tokens are messages with backpressure and supervision trees.

use core::sync::atomic::{AtomicUsize, Ordering};

static TOKEN_COUNT: AtomicUsize = AtomicUsize::new(0);

pub mod token_actor {
    pub fn init() {
        // Initialize token_actor
    }

    /// Perform one inference step.
    pub fn step(input_token: u32) -> u32 {
        // TODO: forward through model layers
        let _ = input_token;
        0
    }

    pub fn kv_cache_len() -> usize {
        super::TOKEN_COUNT.load(Ordering::Relaxed)
    }
}
