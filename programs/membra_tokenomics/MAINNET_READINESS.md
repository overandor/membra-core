# MEMBRA Tokenomics Mainnet Readiness

Status: Production candidate, not mainnet-approved until all boxes are checked.

## Required before mainnet

- [ ] All leaked GitHub tokens revoked.
- [ ] No secrets in git history or tracked files.
- [ ] `cargo build` passes.
- [ ] `cargo clippy -- -D warnings` passes.
- [ ] `anchor build` passes.
- [ ] `anchor test` passes.
- [ ] Program ID replaced with deployed ID.
- [ ] IDL regenerated and committed.
- [ ] TypeScript SDK builds.
- [ ] QR Gateway integration tested on devnet.
- [ ] Devnet sale initialized.
- [ ] Devnet QR contribution completed.
- [ ] Devnet receipt indexed.
- [ ] Devnet claim tested after finalization.
- [ ] Treasury/protocol/validator wallets verified.
- [ ] Risk disclosure text approved.
- [ ] External audit scheduled or completed.
- [ ] Emergency pause authority tested.
- [ ] Mainnet launch parameters reviewed.

## Guardrails

MEMBRA tokenomics does not guarantee profit, yield, appreciation, redemption, or liquidity. Early rewards are capped, pool-limited, disclosed, and claimable only if funded. Contributions are subject to blockchain transaction risk and program risk.

## Current honest appraisal

Production candidate after compile, not production-ready yet. The missing gates are anchor build, anchor test, secret cleanup, program ID verification, devnet deployment, QR integration, and external review.
