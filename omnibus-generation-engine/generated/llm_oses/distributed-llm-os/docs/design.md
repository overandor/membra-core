# Design Document: distributed-llm-os

## Overview
OS spanning multiple nodes where processes migrate toward GPU-equipped hosts automatically.

## Kernel Type
distributed

## LLM Subsystem
process_migrator

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_distributed()
5. kernel_main → llm_subsystem_process_migrator_init()
6. Scheduler loop begins
