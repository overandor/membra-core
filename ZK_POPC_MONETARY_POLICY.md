# ZK-Proof-of-Productive-Capacity (ZK-PoPC) Monetary Policy

## 1. Core Thesis

ZK-PoPC is **proof-backed monetary issuance**. New tokens mint only when collateral-backed, externally useful work is proven, independently attested, ZK-validated, and nullifier-protected.

## 2. Five Required Components

| # | Component | Purpose |
|---|-----------|---------|
| 1 | Collateral Commitment | Economic weight against Sybil spam |
| 2 | Productive Work | Inference, compression, deployment, benchmarking, appraisal, compute rental, microtask |
| 3 | Verifier Attestation | Independent quorum confirms quality |
| 4 | Zero-Knowledge Proof | Validates claim without exposing private data |
| 5 | Nullifier | One-time claim; prevents replay inflation |

Missing any component = rejection.

## 3. Collateral Tiers

| Tier | Threshold (USD) | Ceiling Multiplier |
|------|-----------------|-------------------|
| 1 | ≥ $1 | 1x |
| 2 | ≥ $10 | 2x |
| 3 | ≥ $100 | 5x |
| 4 | ≥ $1,000 | 10x |
| 5 | ≥ $10,000 | 20x |

Collateral alone does NOT mint. It only qualifies minting when paired with verified work.

## 4. Verifier Rules

- **Minimum verifiers:** 2
- **Maximum verifiers:** 5
- Attestations weighted by reputation, stake, and diversity
- Colluding verifiers subject to slashing and reputation decay
- Challenge periods allow disputing attestations before minting

## 5. Mint Formula

```
mint_amount = min(
  base_reward * difficulty * tier_multiplier * verifier_bonus * network_adjustment / scale,
  max_mint_per_proof,
  epoch_mint_cap_remaining
)
```

- `base_reward` = 1,000,000 smallest units
- `scale` = 10^16
- `max_mint_per_proof` = 100,000,000,000 units
- `network_adjustment` counter-cyclical: compresses issuance when demand is high

## 6. Nullifiers

Every proof generates a unique nullifier hash. Once a proof is minted, its nullifier is recorded on-chain. Any subsequent submission with the same nullifier is rejected. This converts productive work into a **one-time monetary claim**.

## 7. Counter-Cyclical Control

Network adjustment factor responds to measured conditions:
- Low utilization → higher adjustment, incentivizing capacity
- High utilization / inflation → lower adjustment, compressing issuance
- Emergency pause available via governance multisig

## 8. Governance & Emergency

- **Mint approval class:** RISKY (requires explicit policy inclusion)
- **Emergency pause:** governance multisig can halt all issuance
- **Epoch caps:** per-epoch mint limits prevent runaway expansion
- **Upgrade path:** monetary parameters adjustable via governance vote with timelock

## 9. Risk Register

| Risk | Mitigation |
|------|-----------|
| Verifier collusion | Reputation scoring, diversity requirements, slashing |
| Fake productivity | Collateral minimums, quality thresholds, challenge periods |
| Replay attacks | Nullifier registry enforced at mint time |
| Oracle manipulation | Multi-source oracles, median aggregation, deviation checks |
| Over-minting | Max per-proof cap, epoch cap, counter-cyclical adjustment |
| Epoch boundary exploits | Rollover atomicity, grace-period attestation carryover |

## 10. Production Readiness Checklist

- [x] Mint calculation tests pass
- [x] Collateral tier derivation tests pass
- [x] Max mint cap enforced
- [x] Proof lifecycle tests pass
- [x] Duplicate attestation rejected
- [x] Unverified proof cannot mint
- [x] Nullifier prevents double mint
- [x] Counter-cyclical adjustment verified
- [ ] Adversarial collusion simulation (pending)
- [ ] Formal audit of monetary math (pending)
- [ ] Devnet stress test (pending)
