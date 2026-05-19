# zero-trust-overlay-sdk

Identity-aware overlay with mTLS, attestation, and micro-segmentation.

## Languages
rust, go, c

## Interfaces
- `IdentityBroker`
- `Attestor`
- `PolicyEngine`
- `TunnelManager`

## Protocols
- `wireguard`
- `spiffe`
- `dice`

## Build

```bash
cargo build --all-features
```

## License
MIT
