# Design Document: microkernel-llm-os

## Overview
Microkernel where LLM inference runs as a privileged server with capability-based IPC.

## Kernel Type
microkernel

## LLM Subsystem
inference_server

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_microkernel()
5. kernel_main → llm_subsystem_inference_server_init()
6. Scheduler loop begins
