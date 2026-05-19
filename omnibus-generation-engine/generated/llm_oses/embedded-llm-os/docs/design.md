# Design Document: embedded-llm-os

## Overview
Sub-MB kernel for MCUs; runs TinyML models with on-device fine-tuning via federated delta.

## Kernel Type
embedded

## LLM Subsystem
tinyml_federator

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_embedded()
5. kernel_main → llm_subsystem_tinyml_federator_init()
6. Scheduler loop begins
