# Design Document: probabilistic-llm-os

## Overview
Kernel schedules based on expected information gain; high-entropy prompts get priority.

## Kernel Type
probabilistic

## LLM Subsystem
entropy_scheduler

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_probabilistic()
5. kernel_main → llm_subsystem_entropy_scheduler_init()
6. Scheduler loop begins
