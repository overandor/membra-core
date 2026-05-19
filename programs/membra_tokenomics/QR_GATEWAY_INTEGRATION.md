# MEMBRA QR Gateway Integration

This document describes the full QR-to-contribution flow and the API contract between the QR Gateway frontend and the MEMBRA tokenomics program.

---

## Full QR Flow

```
QR created (authority)
  → Buyer scans QR code
  → Risk disclosure displayed (off-chain UI)
  → Wallet connected (Phantom / Solflare / Backpack)
  → Contribution quoted (bonding curve + decay bonus)
  → 80/10/5/5 split previewed
  → Buyer signs contribution transaction
  → On-chain receipt recorded (BuyerReceipt PDA)
  → Claim eligibility shown
  → Sale finalized by authority
  → Capped rebate claim enabled (only if pool funded)
```

---

## API Contract for QR Gateway Frontend

### `GET /api/qr-sales/:saleId`

**Response:**
```json
{
  "sale": "<Pubkey>",
  "status": "Active",
  "basePriceLamports": 100000,
  "slopeBps": 100,
  "maxBonusBps": 2000,
  "saleDurationSec": 3600,
  "startTime": 1710000000,
  "endTime": 1710003600,
  "totalRaisedLamports": 5000000000,
  "hardCapLamports": 100000000000,
  "minContributionLamports": 10000000,
  "earlyRewardCapLamports": 10000000000,
  "maxRebatePerBuyerLamports": 1000000000,
  "rebateRateBps": 500,
  "termsUrl": "https://membra.xyz/terms/qr-sale/:saleId",
  "riskDisclosure": "Contributions are irreversible. No guaranteed profit. Rebate is capped and pool-limited."
}
```

### `GET /api/qr-sales/:saleId/quote?amountLamports=<value>`

**Query:** `amountLamports` must be >= `minContributionLamports`

**Response:**
```json
{
  "amountLamports": 1000000000,
  "priceAtContribution": 110000,
  "baseTokens": 9090909,
  "bonusBps": 1500,
  "bonusTokens": 1363636,
  "totalTokens": 10454545,
  "split": {
    "treasury": 800000000,
    "protocol": 100000000,
    "validator": 50000000,
    "earlyReward": 50000000
  },
  "rebateEstimate": 50000000,
  "rebateCappedAt": 1000000000,
  "poolRemaining": 4500000000,
  "hardCapRemaining": 95000000000
}
```

### `POST /api/qr-sales/:saleId/build-contribution-tx`

**Body:**
```json
{
  "buyerWallet": "<Pubkey>",
  "amountLamports": 1000000000,
  "contributionIndex": 1
}
```

**Response:**
```json
{
  "transaction": "<base64 serialized transaction>",
  "accounts": {
    "buyer": "<Pubkey>",
    "tokenSale": "<PDA>",
    "treasury": "<Pubkey>",
    "protocolWallet": "<Pubkey>",
    "validatorPool": "<Pubkey>",
    "earlyRewardPool": "<PDA>",
    "contribution": "<PDA>",
    "buyerReceipt": "<PDA>"
  },
  "recentBlockhash": "<blockhash>",
  "instructions": ["membra_tokenomics::contribute"]
}
```

### `POST /api/qr-sales/:saleId/receipt`

**Body:**
```json
{
  "buyerWallet": "<Pubkey>"
}
```

**Response:**
```json
{
  "receipt": "<PDA>",
  "totalContributedLamports": 1000000000,
  "totalTokensAllocated": 10454545,
  "rebateClaimStatus": "Eligible",
  "rebateEstimate": 50000000,
  "transactions": [
    {
      "signature": "<txSig>",
      "explorerUrl": "https://explorer.solana.com/tx/<txSig>?cluster=devnet"
    }
  ]
}
```

### `GET /api/qr-sales/:saleId/claim-status/:wallet`

**Response:**
```json
{
  "sale": "<Pubkey>",
  "buyer": "<Pubkey>",
  "claimStatus": "Eligible",
  "claimableLamports": 50000000,
  "maxRebate": 1000000000,
  "poolBalance": 4500000000,
  "claimWindowOpen": true,
  "claimDeadline": 1712592000
}
```

---

## On-Chain Instruction Mapping

| Frontend Action | Anchor Instruction | Accounts |
|---|---|---|
| Initialize sale | `initialize_sale` | authority, token_sale, treasury, protocol_wallet, validator_pool, early_reward_pool, system_program |
| Activate sale | `activate_sale` | authority, token_sale |
| Contribute | `contribute` | buyer, token_sale, treasury, protocol_wallet, validator_pool, early_reward_pool, contribution, buyer_receipt, system_program |
| Claim rebate | `claim_rebate` | buyer, token_sale, early_reward_pool, buyer_receipt, system_program |
| Finalize sale | `finalize_sale` | authority, token_sale |
| Migrate liquidity | `migrate_liquidity` | authority, token_sale |
| Pause sale | `set_sale_pause(true)` | authority, token_sale |
| Resume sale | `set_sale_pause(false)` | authority, token_sale |

---

## Security Notes

- **Risk disclosure is off-chain.** The UI must display terms before calling `contribute`. Buyer signature implies acceptance.
- **Quote is off-chain.** The frontend should mirror the bonding curve and decay math exactly. Any mismatch is a UX bug, not a security flaw — the on-chain program is the source of truth.
- **Wallet substitution is blocked.** The `Contribute` context enforces `address = token_sale.treasury` etc. Malicious frontend cannot redirect funds.
- **Contribution index must be unique.** The frontend must track the next available index or fetch it from `sale.contribution_count`.
