# Design Document: hierarchical-llm-os

## Overview
Ring-0 hosts small model, delegates to ring-1 medium, ring-2 large model via hierarchical calls.

## Kernel Type
hierarchical

## LLM Subsystem
tiered_model

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_hierarchical()
5. kernel_main → llm_subsystem_tiered_model_init()
6. Scheduler loop begins
