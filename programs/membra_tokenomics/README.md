# MEMBRA Early-Risk Curve / QR Tokenomics

Solana Anchor program for MEMBRA's QR Tokenomics flow — bonding curve token sales with time-decay bonuses, capped rebates, and on-chain contribution records.

## Architecture

```
Buyer scans QR → sees terms → connects wallet → contributes SOL
     ↓
Bonding curve computes price → decay formula computes early bonus
     ↓
Contribution split atomically: 80% treasury / 10% protocol / 5% validator / 5% early-reward-pool
     ↓
BuyerReceipt recorded on-chain → eligible for capped rebate after finalization
     ↓
Authority finalizes → liquidity migrated → claims enabled
```

## Instructions

| Instruction | Description | Authority |
|-------------|-------------|-----------|
| `initialize_sale` | Create a new TokenSale with bonding curve params, splits, caps | Authority signer |
| `activate_sale` | Start the sale clock, enabling contributions | Authority signer |
| `contribute` | Buyer sends SOL, receives token allocation + potential bonus | Buyer signer |
| `claim_rebate` | Eligible buyer claims capped rebate from early-reward-pool | Buyer signer |
| `finalize_sale` | Lock the sale, enable rebate claims | Authority signer |
| `migrate_liquidity` | Mark liquidity as migrated (off-chain DEX setup) | Authority signer |
| `cancel_sale` | Cancel the sale (refunds handled off-chain in v0.1) | Authority signer |
| `set_sale_pause` | Pause or resume an active sale | Authority signer |

## Guardrails

- **No guaranteed profit**: Rebates are capped (`max_rebate_per_buyer_lamports`, `early_reward_cap_lamports`) and claimable only after finalization.
- **Pool-limited**: Early reward pool has a hard cap; contributions rejected if they would exceed it.
- **Hard cap**: Total sale raise capped at `hard_cap_lamports`.
- **Minimum contribution**: `min_contribution_lamports` enforced per contribution.
- **Disclosed**: Terms are pre-signed by the buyer transaction.
- **80/10/5/5 split**: Treasury / Protocol / Validator / Early Reward enforced in lamport transfers via CPI.

## Bonding Curve

Linear bonding curve with integer-only arithmetic:

```
price = base_price + slope_bps * total_raised / 10_000
```

Base tokens computed with 6-decimal precision:

```
base_tokens = amount_lamports * 1_000_000 / current_price
```

## Time-Decay Bonus

Early buyers receive a time-decay bonus that decreases linearly over the sale duration:

```
bonus_bps = max_bonus_bps * (1 - elapsed_sec / sale_duration_sec)
bonus_tokens = base_tokens * bonus_bps / 10_000
```

## Accounts

### `TokenSale`

| Field | Type | Description |
|-------|------|-------------|
| `authority` | `Pubkey` | Sale administrator |
| `sale_id` | `u64` | Unique sale identifier |
| `status` | `u8` | SaleStatus enum |
| `base_price_lamports` | `u64` | Starting price per token (in lamports) |
| `slope_bps` | `u64` | Bonding curve slope (basis points) |
| `max_bonus_bps` | `u16` | Maximum early bonus (basis points, max 5000) |
| `sale_duration_sec` | `u64` | Sale duration in seconds |
| `start_time` | `i64` | Activation timestamp |
| `end_time` | `i64` | Expiration timestamp |
| `total_raised_lamports` | `u64` | Total SOL raised |
| `total_tokens_allocated` | `u64` | Total tokens allocated |
| `contribution_count` | `u64` | Number of contributions |
| `treasury` | `Pubkey` | Treasury wallet |
| `protocol_wallet` | `Pubkey` | Protocol fee wallet |
| `validator_pool` | `Pubkey` | Validator reward wallet |
| `split_treasury_bps` | `u16` | Treasury split (default 8000) |
| `split_protocol_bps` | `u16` | Protocol split (default 1000) |
| `split_validator_bps` | `u16` | Validator split (default 500) |
| `split_early_reward_bps` | `u16` | Early reward split (default 500) |
| `early_reward_cap_lamports` | `u64` | Max SOL in early reward pool |
| `early_reward_distributed_lamports` | `u64` | SOL already sent to early reward pool |
| `max_rebate_per_buyer_lamports` | `u64` | Max rebate per buyer |
| `rebate_rate_bps` | `u16` | Rebate rate (basis points, max 2000) |
| `hard_cap_lamports` | `u64` | Total sale hard cap |
| `min_contribution_lamports` | `u64` | Minimum contribution amount |
| `bump` | `u8` | TokenSale PDA bump |
| `early_reward_pool_bump` | `u8` | EarlyRewardPool PDA bump |

### `Contribution`

Recorded for each buyer contribution.

### `BuyerReceipt`

Aggregates all contributions per buyer and tracks rebate claim status.

## Events

- `SaleInitialized`
- `SaleActivated`
- `SalePaused`
- `SaleResumed`
- `ContributionRecorded`
- `RebateClaimed`
- `SaleFinalized`
- `LiquidityMigrated`
- `SaleCancelled`

## Errors

| Error | Trigger |
|-------|---------|
| `Unauthorized` | Non-authority calls admin instruction |
| `InvalidSaleStatus` | Wrong status for operation |
| `SaleNotActive` | Contribution when sale not active |
| `SaleExpired` | Contribution after end_time |
| `InvalidPrice` | base_price_lamports == 0 |
| `BonusTooHigh` | max_bonus_bps > 5000 |
| `RebateTooHigh` | rebate_rate_bps > 2000 |
| `InvalidDuration` | sale_duration_sec == 0 |
| `InvalidHardCap` | hard_cap < early_reward_cap |
| `InvalidMinContribution` | min_contribution == 0 |
| `InvalidSplits` | Splits don't sum to 10000 |
| `ZeroContribution` | amount == 0 |
| `ContributionTooSmall` | amount < min_contribution |
| `HardCapReached` | Contribution would exceed hard cap |
| `MathOverflow` | Checked math overflow |
| `InsufficientFunds` | Buyer can't cover transfer |
| `EarlyRewardCapReached` | Early reward pool full |
| `InvalidWallet` | Wallet address mismatch |
| `ClaimsNotEnabled` | Claim before finalization |
| `AlreadyClaimed` | Double claim attempt |
| `ClaimExpired` | Receipt marked expired |
| `ClaimWindowClosed` | >30 days after sale end |
| `NoRebateAvailable` | Pool empty or rebate == 0 |

## Deployment

```bash
# 1. Build
anchor build

# 2. Deploy (replace program ID)
anchor deploy --provider.cluster devnet

# 3. Update declare_id! in lib.rs with deployed program ID
# 4. Rebuild and redeploy
```

## Testing

```bash
anchor test
```

## Security Notes

- All lamport transfers use CPI through the System Program (no raw lamport manipulation).
- All math uses checked integer arithmetic.
- PDA seeds are validated via Anchor constraints.
- `has_one = authority` enforces admin ownership on `ManageSale`.

## Production Boundaries

**MEMBRA tokenomics does NOT guarantee:**

- Profit, yield, appreciation, or redemption.
- Infinite or passive rewards.
- Liquidity or exit opportunities.

**Cashback / rebate terms:**

- Rebate is **capped** per buyer (`max_rebate_per_buyer_lamports`).
- Rebate is **pool-limited** (`early_reward_cap_lamports`).
- Claims are **available only if funded** — the early-reward-pool must hold sufficient SOL.
- Claims are **available only after finalization**.
- Token ownership does **not** imply equity, voting rights, or revenue share.

**Contribution risk:**

- Contributions may be **irreversible**.
- Mainnet transactions cost **real SOL**.
- Buyer must **accept risk disclosure** before signing.
- UI must show **estimated quote** and require **final wallet confirmation**.

## License

MIT
