# Design Document: exokernel-llm-os

## Overview
Exokernel exposing raw GPU/TPU hardware to user-space LLM runtimes with secure multiplexing.

## Kernel Type
exokernel

## LLM Subsystem
hardware_multiplexer

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_exokernel()
5. kernel_main → llm_subsystem_hardware_multiplexer_init()
6. Scheduler loop begins
