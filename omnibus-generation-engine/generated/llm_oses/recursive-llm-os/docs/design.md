# Design Document: recursive-llm-os

## Overview
OS runs a smaller OS inside itself to validate speculative LLM outputs before committing to host state.

## Kernel Type
recursive

## LLM Subsystem
speculative_vm

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_recursive()
5. kernel_main → llm_subsystem_speculative_vm_init()
6. Scheduler loop begins
