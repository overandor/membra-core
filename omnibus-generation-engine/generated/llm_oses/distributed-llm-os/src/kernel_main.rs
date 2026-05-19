#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;

mod llm_subsystem;
mod memory;
mod scheduler;

/// OS spanning multiple nodes where processes migrate toward GPU-equipped hosts automatically.
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    memory::init();
    scheduler::init_distributed();
    llm_subsystem::process_migrator::init();
    loop {
        scheduler::yield_next();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
