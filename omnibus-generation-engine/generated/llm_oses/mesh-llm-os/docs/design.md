# Design Document: mesh-llm-os

## Overview
Mesh-topology OS for edge clusters; inference workloads route through nearest GPU hop.

## Kernel Type
mesh

## LLM Subsystem
nearest_gpu_router

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_mesh()
5. kernel_main → llm_subsystem_nearest_gpu_router_init()
6. Scheduler loop begins
