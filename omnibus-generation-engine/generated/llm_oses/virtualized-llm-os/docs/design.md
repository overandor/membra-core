# Design Document: virtualized-llm-os

## Overview
Type-1 hypervisor hosting para-virtualized LLM guests with direct GPU passthrough.

## Kernel Type
virtualized

## LLM Subsystem
gpu_passthrough

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_virtualized()
5. kernel_main → llm_subsystem_gpu_passthrough_init()
6. Scheduler loop begins
