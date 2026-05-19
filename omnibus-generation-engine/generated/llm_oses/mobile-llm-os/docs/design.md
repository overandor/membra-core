# Design Document: mobile-llm-os

## Overview
Battery-aware scheduler quantizing models to NPU and falling back to CPU on thermal throttling.

## Kernel Type
mobile

## LLM Subsystem
thermal_quantizer

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_mobile()
5. kernel_main → llm_subsystem_thermal_quantizer_init()
6. Scheduler loop begins
