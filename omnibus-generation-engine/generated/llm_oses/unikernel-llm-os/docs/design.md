# Design Document: unikernel-llm-os

## Overview
Single-address-space unikernel optimized for single-tenant LLM inference on cloud hypervisors.

## Kernel Type
unikernel

## LLM Subsystem
monolithic_inference

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_unikernel()
5. kernel_main → llm_subsystem_monolithic_inference_init()
6. Scheduler loop begins
