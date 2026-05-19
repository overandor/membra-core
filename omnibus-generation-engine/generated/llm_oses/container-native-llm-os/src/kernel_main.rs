#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;

mod llm_subsystem;
mod memory;
mod scheduler;

/// Kernel is a container orchestrator; every LLM inference is a sandboxed container with cgroup v3.
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    memory::init();
    scheduler::init_container-native();
    llm_subsystem::inference_container::init();
    loop {
        scheduler::yield_next();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
