#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;

mod llm_subsystem;
mod memory;
mod scheduler;

/// Human and LLM share equal scheduling rights; the kernel arbitrates CPU time between human tasks and model thoughts.
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    memory::init();
    scheduler::init_symbiotic();
    llm_subsystem::fair_arbiter::init();
    loop {
        scheduler::yield_next();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
