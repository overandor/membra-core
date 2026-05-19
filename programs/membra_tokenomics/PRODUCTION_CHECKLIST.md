# MEMBRA Tokenomics — Production Deployment Checklist

## Pre-Deployment

- [ ] Replace placeholder `declare_id!` with deployed program ID
- [ ] Run `anchor build` and verify zero warnings
- [ ] Run `anchor test` and verify all tests pass
- [ ] Audit `INIT_SPACE` calculations match account struct sizes
- [ ] Verify all `checked_*` math paths have corresponding error branches
- [ ] Confirm `has_one = authority` constraint on `ManageSale`
- [ ] Confirm PDA seeds are deterministic and documented
- [ ] Verify event structs match instruction emissions exactly

## Deployment

- [ ] Deploy to devnet first: `anchor deploy --provider.cluster devnet`
- [ ] Update `declare_id!` in `lib.rs` with devnet program ID
- [ ] Rebuild and run devnet tests
- [ ] Deploy to mainnet: `anchor deploy --provider.cluster mainnet`
- [ ] Update `declare_id!` with mainnet program ID
- [ ] Update `Anchor.toml` program IDs for all clusters
- [ ] Pin program version and tag release

## Post-Deployment

- [ ] Initialize first TokenSale via SDK with real authority wallet
- [ ] Verify `SaleInitialized` event emitted on-chain
- [ ] Test contribution flow on devnet with small amount
- [ ] Verify split transfers reach treasury/protocol/validator/pool wallets
- [ ] Test claim rebate flow after finalization
- [ ] Verify `BuyerReceipt` state updates correctly
- [ ] Monitor for 24h before public sale

## Security

- [ ] Rotate authority keypair to multisig (e.g. Squads)
- [ ] Enable transaction simulation before signing
- [ ] Set up on-chain event monitoring
- [ ] Document incident response procedures
- [ ] Schedule quarterly access review

## Frontend Integration

- [ ] Update React/TypeScript app with deployed program ID
- [ ] Add terms + risk disclosure modal before contribution
- [ ] Display real-time bonding curve price
- [ ] Show decay bonus countdown timer
- [ ] Fetch and display BuyerReceipt status

## Monitoring

- [ ] Set up SolanaFM / Helius webhook for program events
- [ ] Alert on `SaleCancelled`, unauthorized access attempts
- [ ] Track total raised vs hard cap in real-time
- [ ] Monitor early reward pool balance vs cap
