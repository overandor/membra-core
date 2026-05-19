# Design Document: flat-address-llm-os

## Overview
Single global 64-bit address space; LLM KV-cache is memory-mapped across the entire cluster.

## Kernel Type
flat-address

## LLM Subsystem
global_kv_cache

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_flat-address()
5. kernel_main → llm_subsystem_global_kv_cache_init()
6. Scheduler loop begins
