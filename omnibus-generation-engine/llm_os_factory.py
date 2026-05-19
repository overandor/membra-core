#!/usr/bin/env python3
"""
LLM OS Factory — generates 30 LLM-capable OS architectures from manifest.json.
Each OS gets kernel stubs, bootloader, LLM subsystem, build scripts, and design docs.
"""

import json
import os
import shutil
from pathlib import Path
from typing import Any

MANIFEST = Path(__file__).with_name("manifest.json")
OUT_DIR = Path(__file__).parent / "generated" / "llm_oses"


def write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content)


def cargo_toml(name: str, os_def: dict) -> str:
    return f'''[package]
name = "{name.replace("-", "_")}"
version = "0.1.0"
edition = "2021"
description = "{os_def['description']}"
license = "MIT"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
lto = true

[dependencies]

[dependencies.bootloader]
version = "0.9"
features = ["map_physical_memory"]
'''


def linker_script() -> str:
    return '''ENTRY(_start)

SECTIONS {
    . = 0x8000;
    .text : {
        *(.text._start)
        *(.text*)
    }
    .rodata : { *(.rodata*) }
    .data : { *(.data*) }
    .bss : { *(.bss*) }
}
'''


def bootloader_asm() -> str:
    return '''; Minimal bootloader stub for x86_64
section .text
bits 32
global _start
_start:
    ; Set up stack
    mov esp, 0x90000
    ; TODO: enable long mode, paging, load kernel
    call kernel_main
    hlt
'''


def bootloader_rust(name: str) -> str:
    return f'''#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {{
    kernel::main();
    loop {{}}
}}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {{
    loop {{}}
}}

mod kernel {{
    pub fn main() {{
        // {name} kernel entry
    }}
}}
'''


def kernel_main_rs(name: str, os_def: dict) -> str:
    subsystem = os_def["llm_subsystem"]
    kernel_type = os_def["kernel_type"]
    return f'''#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;

mod llm_subsystem;
mod memory;
mod scheduler;

/// {os_def['description']}
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {{
    memory::init();
    scheduler::init_{kernel_type}();
    llm_subsystem::{subsystem}::init();
    loop {{
        scheduler::yield_next();
    }}
}}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {{
    loop {{}}
}}
'''


def kernel_main_c(name: str, os_def: dict) -> str:
    subsystem = os_def["llm_subsystem"]
    kernel_type = os_def["kernel_type"]
    return f'''// {os_def['description']}
#include <stdint.h>
#include "memory.h"
#include "scheduler.h"
#include "llm_subsystem.h"

void kernel_main(void) {{
    memory_init();
    scheduler_init_{kernel_type}();
    llm_subsystem_{subsystem}_init();
    while (1) {{
        scheduler_yield_next();
    }}
}}
'''


def llm_subsystem_rs(name: str, os_def: dict) -> str:
    subsystem = os_def["llm_subsystem"]
    desc = os_def["description"]
    return f'''//! LLM Subsystem — {subsystem}
//!
//! {desc}

use core::sync::atomic::{{AtomicUsize, Ordering}};

static TOKEN_COUNT: AtomicUsize = AtomicUsize::new(0);

pub mod {subsystem} {{
    pub fn init() {{
        // Initialize {subsystem}
    }}

    /// Perform one inference step.
    pub fn step(input_token: u32) -> u32 {{
        // TODO: forward through model layers
        let _ = input_token;
        0
    }}

    pub fn kv_cache_len() -> usize {{
        super::TOKEN_COUNT.load(Ordering::Relaxed)
    }}
}}
'''


def llm_subsystem_c(name: str, os_def: dict) -> str:
    subsystem = os_def["llm_subsystem"]
    desc = os_def["description"]
    return f'''// LLM Subsystem — {subsystem}
// {desc}

#include <stdint.h>
#include <stddef.h>

static size_t token_count = 0;

void llm_subsystem_{subsystem}_init(void) {{
    token_count = 0;
}}

uint32_t llm_subsystem_{subsystem}_step(uint32_t input_token) {{
    (void)input_token;
    return 0;
}}

size_t llm_subsystem_{subsystem}_kv_cache_len(void) {{
    return token_count;
}}
'''


def memory_rs() -> str:
    return '''//! Kernel memory manager

pub fn init() {
    // TODO: set up bump allocator or buddy system
}

pub unsafe fn alloc_frame() -> *mut u8 {
    core::ptr::null_mut()
}
'''


def memory_c() -> str:
    return '''// Kernel memory manager
#include <stddef.h>

void memory_init(void) {
}

void* memory_alloc_frame(void) {
    return NULL;
}
'''


def scheduler_rs(os_def: dict) -> str:
    ktype = os_def["kernel_type"]
    return f'''//! Scheduler — {ktype}

pub fn init_{ktype}() {{
    // TODO: initialize {ktype} scheduler structures
}}

pub fn yield_next() {{
    // TODO: context switch to next runnable task
}}
'''


def scheduler_c(os_def: dict) -> str:
    ktype = os_def["kernel_type"]
    return f'''// Scheduler — {ktype}

void scheduler_init_{ktype}(void) {{
}}

void scheduler_yield_next(void) {{
}}
'''


def makefile(name: str, os_def: dict) -> str:
    return f'''ARCH ?= x86_64
TARGET = {name.replace("-", "_")}

.PHONY: all clean run

all:
\t@echo "Build {name} for $(ARCH)"
\t@echo "Languages: {', '.join(os_def['languages'])}"
\t@echo "Kernel type: {os_def['kernel_type']}"
\t@echo "LLM subsystem: {os_def['llm_subsystem']}"

clean:
\trm -rf build/

run: all
\tqemu-system-$(ARCH) -kernel build/$(TARGET).bin
'''


def build_rs(name: str) -> str:
    return f'''use std::{{env, path::PathBuf}};

fn main() {{
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let kernel = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rerun-if-changed={{}}", kernel.join("src").display());
    println!("cargo:rustc-link-arg=-T{{}}", out_dir.join("linker.ld").display());
}}
'''


def readme(name: str, os_def: dict) -> str:
    langs = ", ".join(os_def["languages"])
    return f'''# {name}

{os_def['description']}

## Attributes
- **Kernel type**: {os_def['kernel_type']}
- **LLM subsystem**: {os_def['llm_subsystem']}
- **Languages**: {langs}

## Architecture

```
bootloader/   → minimal stub or GRUB multiboot
kernel/       → kernel main, scheduler, memory manager
llm_subsystem/→ token inference, KV-cache, model loader
drivers/      → UART, framebuffer, NVMe, GPU
```

## Build

```bash
make
```

## Run in QEMU

```bash
make run ARCH=x86_64
```

## License
MIT
'''


def generate(os_def: dict) -> None:
    name = os_def["name"]
    root = OUT_DIR / name
    if root.exists():
        shutil.rmtree(root)
    root.mkdir(parents=True)

    langs = os_def["languages"]
    primary = "rust" if "rust" in langs else "c"

    write(root / "README.md", readme(name, os_def))
    write(root / "Makefile", makefile(name, os_def))

    if primary == "rust":
        write(root / "Cargo.toml", cargo_toml(name, os_def))
        write(root / "build.rs", build_rs(name))
        write(root / "src" / "linker.ld", linker_script())
        write(root / "src" / "main.rs", bootloader_rust(name))
        write(root / "src" / "kernel_main.rs", kernel_main_rs(name, os_def))
        write(root / "src" / "llm_subsystem.rs", llm_subsystem_rs(name, os_def))
        write(root / "src" / "memory.rs", memory_rs())
        write(root / "src" / "scheduler.rs", scheduler_rs(os_def))
    else:
        write(root / "bootloader" / "stub.asm", bootloader_asm())
        write(root / "kernel" / "main.c", kernel_main_c(name, os_def))
        write(root / "kernel" / "llm_subsystem.c", llm_subsystem_c(name, os_def))
        write(root / "kernel" / "memory.c", memory_c())
        write(root / "kernel" / "scheduler.c", scheduler_c(os_def))

    write(root / "docs" / "design.md", f'''# Design Document: {name}

## Overview
{os_def['description']}

## Kernel Type
{os_def['kernel_type']}

## LLM Subsystem
{os_def['llm_subsystem']}

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_{os_def['kernel_type']}()
5. kernel_main → llm_subsystem_{os_def['llm_subsystem']}_init()
6. Scheduler loop begins
''')


def main() -> None:
    manifest = json.loads(MANIFEST.read_text())
    for os_def in manifest["llm_oses"]:
        generate(os_def)
        print(f"Generated LLM OS: {os_def['name']}")
    print(f"\nDone. Output: {OUT_DIR}")


if __name__ == "__main__":
    main()
