# Design Document: symbiotic-llm-os

## Overview
Human and LLM share equal scheduling rights; the kernel arbitrates CPU time between human tasks and model thoughts.

## Kernel Type
symbiotic

## LLM Subsystem
fair_arbiter

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_symbiotic()
5. kernel_main → llm_subsystem_fair_arbiter_init()
6. Scheduler loop begins
