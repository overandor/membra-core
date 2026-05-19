# Design Document: edge-llm-os

## Overview
Gateway OS caching model shards and serving them to low-power clients over LoRa/WiFi.

## Kernel Type
edge

## LLM Subsystem
shard_cache

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_edge()
5. kernel_main → llm_subsystem_shard_cache_init()
6. Scheduler loop begins
