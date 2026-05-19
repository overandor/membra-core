# blockchain-consensus-sdk

Pluggable BFT consensus engine with hot-swap validator sets.

## Languages
rust, go

## Interfaces
- `ConsensusEngine`
- `ValidatorSet`
- `BlockProposer`
- `FinalityGadget`

## Protocols
- `tendermint`
- `hotstuff`
- `pbft`

## Build

```bash
cargo build --all-features
```

## License
MIT
