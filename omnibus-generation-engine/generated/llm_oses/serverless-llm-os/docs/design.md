# Design Document: serverless-llm-os

## Overview
Cold-start optimized kernel for ephemeral LLM functions; keeps hot model weights in kernel-resident cache.

## Kernel Type
serverless

## LLM Subsystem
hot_weight_cache

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_serverless()
5. kernel_main → llm_subsystem_hot_weight_cache_init()
6. Scheduler loop begins
