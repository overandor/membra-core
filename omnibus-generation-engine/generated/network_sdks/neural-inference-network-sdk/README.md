# neural-inference-network-sdk

Distributed neural inference with pipeline parallelism and KV-cache sharding.

## Languages
rust, cpp, python

## Interfaces
- `ModelShard`
- `PipelineRouter`
- `KVCacheManager`
- `TokenizerGateway`

## Protocols
- `nccl`
- `grpc`
- `rdma`

## Build

```bash
cargo build --all-features
```

## License
MIT
