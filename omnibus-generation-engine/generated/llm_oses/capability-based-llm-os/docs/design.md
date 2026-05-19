# Design Document: capability-based-llm-os

## Overview
Capability system where file handles, GPU contexts, and model weights are all capabilities.

## Kernel Type
capability

## LLM Subsystem
capability_gpu

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_capability()
5. kernel_main → llm_subsystem_capability_gpu_init()
6. Scheduler loop begins
