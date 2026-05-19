# Design Document: baremetal-llm-os

## Overview
No abstractions; user code directly programs GPU DMA rings and token decode loops.

## Kernel Type
baremetal

## LLM Subsystem
direct_dma

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_baremetal()
5. kernel_main → llm_subsystem_direct_dma_init()
6. Scheduler loop begins
