# Design Document: dataflow-llm-os

## Overview
OS structured as a dataflow graph; LLM layers are nodes, tensors flow on edges.

## Kernel Type
dataflow

## LLM Subsystem
tensor_graph

## Boot Sequence
1. Firmware → Bootloader
2. Bootloader → kernel_main()
3. kernel_main → memory_init()
4. kernel_main → scheduler_init_dataflow()
5. kernel_main → llm_subsystem_tensor_graph_init()
6. Scheduler loop begins
