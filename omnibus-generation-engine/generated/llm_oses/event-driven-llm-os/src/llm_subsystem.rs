//! LLM Subsystem — event_router
//!
//! Kernel dispatch is entirely event-driven; LLM prompts are high-priority kernel events.

use core::sync::atomic::{AtomicUsize, Ordering};

static TOKEN_COUNT: AtomicUsize = AtomicUsize::new(0);

pub mod event_router {
    pub fn init() {
        // Initialize event_router
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
