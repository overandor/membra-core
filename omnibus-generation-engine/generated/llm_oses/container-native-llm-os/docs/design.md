# Design Document: container-native-llm-os

## Overview
Kernel is a container orchestrator; every LLM inference is a sandboxed container with cgroup v3.

## Kernel Type
container-native

## LLM Subsystem
inference_container

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_container-native()
5. kernel_main → llm_subsystem_inference_container_init()
6. Scheduler loop begins
