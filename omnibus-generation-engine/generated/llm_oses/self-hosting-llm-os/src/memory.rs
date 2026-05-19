//! Kernel memory manager

pub fn init() {
    // TODO: set up bump allocator or buddy system
}

pub unsafe fn alloc_frame() -> *mut u8 {
    core::ptr::null_mut()
}
