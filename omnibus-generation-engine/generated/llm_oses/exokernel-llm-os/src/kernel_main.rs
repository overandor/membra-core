#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;

mod llm_subsystem;
mod memory;
mod scheduler;

/// Exokernel exposing raw GPU/TPU hardware to user-space LLM runtimes with secure multiplexing.
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    memory::init();
    scheduler::init_exokernel();
    llm_subsystem::hardware_multiplexer::init();
    loop {
        scheduler::yield_next();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
