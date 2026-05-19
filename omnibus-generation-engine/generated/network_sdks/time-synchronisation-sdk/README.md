# time-synchronisation-sdk

Sub-microsecond clock sync with Byzantine fault-tolerant time master election.

## Languages
rust, c, go

## Interfaces
- `ClockSync`
- `MasterElection`
- `DelayEstimator`
- `LeapSecondHandler`

## Protocols
- `ptp`
- `ntp-bis`
- `huygens`

## Build

```bash
cargo build --all-features
```

## License
MIT
