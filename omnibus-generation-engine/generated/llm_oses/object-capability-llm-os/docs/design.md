# Design Document: object-capability-llm-os

## Overview
Object-capability security; each LLM token generation requires a capability delegation chain.

## Kernel Type
object-capability

## LLM Subsystem
delegated_generation

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_object-capability()
5. kernel_main → llm_subsystem_delegated_generation_init()
6. Scheduler loop begins
