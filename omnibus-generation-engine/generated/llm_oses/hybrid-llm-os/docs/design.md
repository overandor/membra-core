# Design Document: hybrid-llm-os

## Overview
Seamlessly offloads between on-device NPU, edge GPU, and cloud TPU based on latency cost.

## Kernel Type
hybrid

## LLM Subsystem
cost_router

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_hybrid()
5. kernel_main → llm_subsystem_cost_router_init()
6. Scheduler loop begins
