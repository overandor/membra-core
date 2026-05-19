# Design Document: language-based-llm-os

## Overview
OS implemented in a linearly-typed language where memory safety proves tensor shape correctness.

## Kernel Type
language-based

## LLM Subsystem
shape_prover

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_language-based()
5. kernel_main → llm_subsystem_shape_prover_init()
6. Scheduler loop begins
