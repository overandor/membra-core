# Design Document: event-driven-llm-os

## Overview
Kernel dispatch is entirely event-driven; LLM prompts are high-priority kernel events.

## Kernel Type
event-driven

## LLM Subsystem
event_router

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_event-driven()
5. kernel_main → llm_subsystem_event_router_init()
6. Scheduler loop begins
