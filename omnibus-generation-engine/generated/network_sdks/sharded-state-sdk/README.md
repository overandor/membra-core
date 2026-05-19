# sharded-state-sdk

Consistent-hashing state shards with cross-shard transaction atomicity.

## Languages
rust, go, java

## Interfaces
- `ShardManager`
- `CrossShardTx`
- `ConsistentHasher`
- `Rebalancer`

## Protocols
- `two-phase-commit`
- `saga`
- `calvin`

## Build

```bash
cargo build --all-features
```

## License
MIT
