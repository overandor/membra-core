# Design Document: reactive-llm-os

## Overview
Reactive streams at kernel level; LLM outputs propagate as observable hot streams.

## Kernel Type
reactive

## LLM Subsystem
observable_output

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_reactive()
5. kernel_main → llm_subsystem_observable_output_init()
6. Scheduler loop begins
