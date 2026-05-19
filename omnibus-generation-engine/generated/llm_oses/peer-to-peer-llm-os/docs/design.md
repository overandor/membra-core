# Design Document: peer-to-peer-llm-os

## Overview
OS without central servers; LLM weights are erasure-coded across peer devices.

## Kernel Type
p2p

## LLM Subsystem
distributed_weights

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_p2p()
5. kernel_main → llm_subsystem_distributed_weights_init()
6. Scheduler loop begins
