# Omnibus Generation Engine

**20 Network SDKs + 30 LLM-Capable OS Architectures**

This directory contains a generative framework that materializes 50 distinct system-level projects from a single `manifest.json` specification. It is designed to extend the membra ecosystem with pluggable network primitives and operating-system architectures optimized for LLM inference.

## Quick Start

```bash
# Regenerate everything from the manifest
python3 network_sdk_factory.py
python3 llm_os_factory.py
```

Output lands in `generated/network_sdks/` and `generated/llm_oses/`.

## What's Inside

### 20 Network SDKs (`generated/network_sdks/`)

| # | SDK | Protocols | Languages |
|---|-----|-----------|-----------|
| 1 | `mesh-network-sdk` | olsr2, babel, 802.11s | Rust, Python, C |
| 2 | `p2p-content-sdk` | libp2p, bitswap, graphsync | Rust, TS, Go |
| 3 | `blockchain-consensus-sdk` | tendermint, hotstuff, pbft | Rust, Go |
| 4 | `dag-ledger-sdk` | iota, hashgraph, phantom | Rust, Go, Python |
| 5 | `federated-learning-sdk` | fedavg, fedprox, scaffold | Python, Rust, C++ |
| 6 | `iot-sensor-mesh-sdk` | mqtt-sn, coap, lora | C, Rust, MicroPython |
| 7 | `cdn-edge-sdk` | http3, quic, srt | Rust, Go, TS |
| 8 | `neural-inference-network-sdk` | nccl, grpc, rdma | Rust, C++, Python |
| 9 | `zero-trust-overlay-sdk` | wireguard, spiffe, dice | Rust, Go, C |
| 10 | `pub-sub-event-sdk` | kafka, nats, redisstreams | Rust, Go, Java |
| 11 | `gossip-protocol-sdk` | swim, plumtree, scuttlebutt | Rust, Erlang, Go |
| 12 | `byzantine-fault-tolerant-sdk` | pbft, sbft, minbft | Rust, Go, C++ |
| 13 | `sharded-state-sdk` | 2pc, saga, calvin | Rust, Go, Java |
| 14 | `cross-chain-bridge-sdk` | ibc, layerzero, wormhole | Rust, Solidity, Go |
| 15 | `time-synchronisation-sdk` | ptp, ntp-bis, huygens | Rust, C, Go |
| 16 | `anonymous-mixnet-sdk` | sphinx, loopix, nym | Rust, Go, Python |
| 17 | `storage-dht-sdk` | kademlia, s-kademlia, pastry | Rust, Go, Python |
| 18 | `compute-grid-sdk` | boinc, folding, seti | Rust, Python, Go |
| 19 | `identity-did-sdk` | did-core, vc-data-model, oidc4vc | Rust, TS, Python |
| 20 | `streaming-media-sdk` | webrtc, srt, dash | Rust, C++, TS |

Each SDK includes:
- `Cargo.toml` or `pyproject.toml`
- Trait/class definitions for its core interfaces
- Protocol driver stubs
- `examples/` and `tests/`
- `README.md`

### 30 LLM OSes (`generated/llm_oses/`)

| # | OS | Kernel Type | LLM Subsystem |
|---|-----|-------------|---------------|
| 1 | `microkernel-llm-os` | microkernel | inference_server |
| 2 | `exokernel-llm-os` | exokernel | hardware_multiplexer |
| 3 | `unikernel-llm-os` | unikernel | monolithic_inference |
| 4 | `distributed-llm-os` | distributed | process_migrator |
| 5 | `real-time-llm-os` | real-time | bounded_inference |
| 6 | `capability-based-llm-os` | capability | capability_gpu |
| 7 | `event-driven-llm-os` | event-driven | event_router |
| 8 | `actor-model-llm-os` | actor-model | token_actor |
| 9 | `dataflow-llm-os` | dataflow | tensor_graph |
| 10 | `reactive-llm-os` | reactive | observable_output |
| 11 | `container-native-llm-os` | container-native | inference_container |
| 12 | `serverless-llm-os` | serverless | hot_weight_cache |
| 13 | `peer-to-peer-llm-os` | p2p | distributed_weights |
| 14 | `mesh-llm-os` | mesh | nearest_gpu_router |
| 15 | `hierarchical-llm-os` | hierarchical | tiered_model |
| 16 | `flat-address-llm-os` | flat-address | global_kv_cache |
| 17 | `object-capability-llm-os` | object-capability | delegated_generation |
| 18 | `language-based-llm-os` | language-based | shape_prover |
| 19 | `virtualized-llm-os` | virtualized | gpu_passthrough |
| 20 | `baremetal-llm-os` | baremetal | direct_dma |
| 21 | `mobile-llm-os` | mobile | thermal_quantizer |
| 22 | `embedded-llm-os` | embedded | tinyml_federator |
| 23 | `edge-llm-os` | edge | shard_cache |
| 24 | `cloud-llm-os` | cloud | gpu_timeslicer |
| 25 | `hybrid-llm-os` | hybrid | cost_router |
| 26 | `deterministic-llm-os` | deterministic | reproducible_engine |
| 27 | `probabilistic-llm-os` | probabilistic | entropy_scheduler |
| 28 | `self-hosting-llm-os` | self-hosting | kernel_codegen |
| 29 | `recursive-llm-os` | recursive | speculative_vm |
| 30 | `symbiotic-llm-os` | symbiotic | fair_arbiter |

Each OS includes:
- Bootloader stub (ASM or Rust)
- Kernel main with memory/scheduler/LLM subsystem init
- `Makefile` and `docs/design.md`
- `README.md`

## Reference Implementations

The `examples/` directory contains fully fleshed-out demonstrations of how to use the generated stubs:

- `neural_inference_reference.rs` — 4-stage pipeline router + KV-cache manager
- `p2p_content_reference.py` — Merkle tree content store + Bitswap exchange
- `microkernel_llm_os_expanded.rs` — Capability-based IPC + inference server service
- `distributed_llm_os_expanded.rs` — Process migration toward GPU-equipped nodes

## Architecture

```
omnibus-generation-engine/
├── manifest.json              → canonical spec of all 50 systems
├── network_sdk_factory.py     → reads manifest, emits 20 SDKs
├── llm_os_factory.py          → reads manifest, emits 30 OSes
├── generated/
│   ├── network_sdks/          → 20 generated projects
│   └── llm_oses/              → 30 generated projects
├── examples/                  → reference implementations
└── README.md                  → this file
```

## Adding a New SDK or OS

1. Edit `manifest.json`
2. Add the entry to `network_sdks` or `llm_oses`
3. Re-run the relevant factory
4. Commit the generated output if desired

## Integration with Membra

These SDKs and OS architectures are designed to compose with the existing membra stack:

- `neural-inference-network-sdk` feeds the `membra-l3` inference runtime
- `p2p-content-sdk` backs the `membra-chat-to-chain` proof distribution layer
- `distributed-llm-os` and `microkernel-llm-os` provide host targets for the `cpp_llmgpt` validator

## License
MIT
