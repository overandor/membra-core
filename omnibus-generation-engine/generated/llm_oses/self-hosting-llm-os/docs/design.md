# Design Document: self-hosting-llm-os

## Overview
OS can rewrite its own kernel source via LLM and hot-reload without reboot.

## Kernel Type
self-hosting

## LLM Subsystem
kernel_codegen

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_self-hosting()
5. kernel_main → llm_subsystem_kernel_codegen_init()
6. Scheduler loop begins
