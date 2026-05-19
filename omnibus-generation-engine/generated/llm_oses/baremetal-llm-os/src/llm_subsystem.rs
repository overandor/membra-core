//! LLM Subsystem — direct_dma
//!
//! No abstractions; user code directly programs GPU DMA rings and token decode loops.

use core::sync::atomic::{AtomicUsize, Ordering};

static TOKEN_COUNT: AtomicUsize = AtomicUsize::new(0);

pub mod direct_dma {
    pub fn init() {
        // Initialize direct_dma
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
