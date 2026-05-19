#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;

mod llm_subsystem;
mod memory;
mod scheduler;

/// Multi-tenant kernel with GPU time-slicing and spot-preemptible LLM inference jobs.
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    memory::init();
    scheduler::init_cloud();
    llm_subsystem::gpu_timeslicer::init();
    loop {
        scheduler::yield_next();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
