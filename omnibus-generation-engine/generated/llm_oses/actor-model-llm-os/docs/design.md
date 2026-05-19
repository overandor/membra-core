# Design Document: actor-model-llm-os

## Overview
Every process is an actor; LLM tokens are messages with backpressure and supervision trees.

## Kernel Type
actor-model

## LLM Subsystem
token_actor

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_actor-model()
5. kernel_main → llm_subsystem_token_actor_init()
6. Scheduler loop begins
