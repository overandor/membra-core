# pub-sub-event-sdk

Exactly-once event bus with ordered streams and consumer groups.

## Languages
rust, go, java

## Interfaces
- `EventBus`
- `Publisher`
- `Subscriber`
- `StreamRouter`

## Protocols
- `kafka`
- `nats`
- `redisstreams`

## Build

```bash
cargo build --all-features
```

## License
MIT
