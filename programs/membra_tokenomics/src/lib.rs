use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, program::{invoke, invoke_signed}};
use anchor_spl::token::Mint;

pub mod merkle_provenance;
pub mod collateral_lock;
pub mod zk_compute;
pub mod gas_reimbursement;
pub mod zero_balance_preservation;
pub mod microtask_backing;
pub mod zk_seigniorage;
pub use merkle_provenance::*;
pub use collateral_lock::*;
pub use zk_compute::*;
pub use gas_reimbursement::*;
pub use zero_balance_preservation::*;
pub use microtask_backing::*;
pub use zk_seigniorage::*;

// =============================================================================
// MEMBRA Early-Risk Curve / QR Tokenomics Protocol — Solana Program v0.1
// =============================================================================
// Core Guardrail: No guaranteed profit. No infinite passive rewards.
// Cashback is capped, disclosed, pool-limited, and claimable only if funded.
//
// Flow:
// 1. Authority initializes a TokenSale (bonding curve, decay, splits, caps).
// 2. Buyers scan QR → see terms → connect wallet → send SOL/USDC.
// 3. Bonding curve calculates base tokens. Decay formula calculates early bonus.
// 4. Contribution is split: 80% treasury, 10% protocol, 5% validator, 5% early-reward-pool.
// 5. BuyerReceipt is recorded on-chain.
// 6. Earlier eligible buyers may claim a capped rebate from the early-reward-pool.
// 7. Authority finalizes the sale → liquidity migrated → claims enabled.
// =============================================================================

declare_id!("38tgbireEP2AFq5YQBroNN9wQTqAECUuDNYmRjkogKca"); // TODO: replace with deployed program ID

#[program]
pub mod membra_tokenomics {
    use super::*;

    // =========================================================================
    // 1. Initialize Token Sale
    // =========================================================================
    #[allow(clippy::too_many_arguments)]
    pub fn initialize_sale(
        ctx: Context<InitializeSale>,
        sale_id: u64,
        _sale_id_bytes: [u8; 8],
        base_price_lamports: u64,
        slope_bps: u64,
        max_bonus_bps: u16,
        sale_duration_sec: u64,
        early_reward_cap_lamports: u64,
        max_rebate_per_buyer_lamports: u64,
        rebate_rate_bps: u16,
        hard_cap_lamports: u64,
        min_contribution_lamports: u64,
    ) -> Result<()> {
        require!(
            base_price_lamports > 0,
            MembraTokenomicsError::InvalidPrice
        );
        require!(
            max_bonus_bps <= 5000,
            MembraTokenomicsError::BonusTooHigh
        ); // max 50% bonus
        require!(
            rebate_rate_bps <= 2000,
            MembraTokenomicsError::RebateTooHigh
        ); // max 20% rebate
        require!(
            sale_duration_sec > 0,
            MembraTokenomicsError::InvalidDuration
        );
        require!(
            hard_cap_lamports > 0,
            MembraTokenomicsError::InvalidHardCap
        );
        require!(
            hard_cap_lamports >= early_reward_cap_lamports,
            MembraTokenomicsError::InvalidHardCap
        );
        require!(
            early_reward_cap_lamports > 0,
            MembraTokenomicsError::InvalidHardCap
        );
        require!(
            max_rebate_per_buyer_lamports > 0,
            MembraTokenomicsError::RebateTooHigh
        );
        require!(
            min_contribution_lamports > 0,
            MembraTokenomicsError::InvalidMinContribution
        );

        let sale = &mut ctx.accounts.token_sale;
        sale.authority = ctx.accounts.authority.key();
        sale.sale_id = sale_id;
        sale.status = SaleStatus::Draft as u8;
        sale.base_price_lamports = base_price_lamports;
        sale.slope_bps = slope_bps;
        sale.max_bonus_bps = max_bonus_bps;
        sale.sale_duration_sec = sale_duration_sec;
        sale.start_time = 0; // set when activated
        sale.end_time = 0;
        sale.total_raised_lamports = 0;
        sale.total_tokens_allocated = 0;
        sale.contribution_count = 0;

        // Splits (default 80/10/5/5)
        sale.split_treasury_bps = 8000;
        sale.split_protocol_bps = 1000;
        sale.split_validator_bps = 500;
        sale.split_early_reward_bps = 500;
        let total_split = sale.split_treasury_bps as u64
            + sale.split_protocol_bps as u64
            + sale.split_validator_bps as u64
            + sale.split_early_reward_bps as u64;
        require!(total_split == 10000, MembraTokenomicsError::InvalidSplits);

        // Wallets
        sale.treasury = ctx.accounts.treasury.key();
        sale.protocol_wallet = ctx.accounts.protocol_wallet.key();
        sale.validator_pool = ctx.accounts.validator_pool.key();

        // Rebate guardrails
        sale.early_reward_cap_lamports = early_reward_cap_lamports;
        sale.early_reward_distributed_lamports = 0;
        sale.max_rebate_per_buyer_lamports = max_rebate_per_buyer_lamports;
        sale.rebate_rate_bps = rebate_rate_bps;
        sale.hard_cap_lamports = hard_cap_lamports;
        sale.min_contribution_lamports = min_contribution_lamports;

        sale.bump = ctx.bumps.token_sale;
        sale.early_reward_pool_bump = ctx.bumps.early_reward_pool;

        let clock = Clock::get()?;
        emit!(SaleInitialized {
            sale: sale.key(),
            authority: sale.authority,
            sale_id,
            base_price_lamports,
            slope_bps,
            max_bonus_bps,
            sale_duration_sec,
            hard_cap_lamports,
            min_contribution_lamports,
            early_reward_cap_lamports,
            max_rebate_per_buyer_lamports,
            rebate_rate_bps,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    // =========================================================================
    // 2. Activate Sale (authority only)
    // =========================================================================
    pub fn activate_sale(ctx: Context<ManageSale>) -> Result<()> {
        let sale = &mut ctx.accounts.token_sale;
        require!(
            sale.status == SaleStatus::Draft as u8,
            MembraTokenomicsError::InvalidSaleStatus
        );

        let clock = Clock::get()?;
        sale.status = SaleStatus::Active as u8;
        sale.start_time = clock.unix_timestamp;
        sale.end_time = clock.unix_timestamp + sale.sale_duration_sec as i64;

        emit!(SaleActivated {
            sale: sale.key(),
            start_time: sale.start_time,
            end_time: sale.end_time,
        });

        Ok(())
    }

    // =========================================================================
    // 3. Contribute (buyer sends SOL, receives token allocation + potential bonus)
    // =========================================================================
    pub fn contribute(
        ctx: Context<Contribute>,
        amount_lamports: u64,
        contribution_index: u64,
        _contribution_index_bytes: [u8; 8],
    ) -> Result<()> {
        require!(amount_lamports > 0, MembraTokenomicsError::ZeroContribution);

        let sale = &mut ctx.accounts.token_sale;
        let clock = Clock::get()?;

        // Guardrails
        require!(
            sale.status == SaleStatus::Active as u8,
            MembraTokenomicsError::SaleNotActive
        );
        require!(
            clock.unix_timestamp <= sale.end_time,
            MembraTokenomicsError::SaleExpired
        );
        require!(
            amount_lamports >= sale.min_contribution_lamports,
            MembraTokenomicsError::ContributionTooSmall
        );
        let new_total = sale
            .total_raised_lamports
            .checked_add(amount_lamports)
            .unwrap();
        require!(
            new_total <= sale.hard_cap_lamports,
            MembraTokenomicsError::HardCapReached
        );

        // ─── Terms & Risk Disclosure Acknowledgment ───
        // Off-chain UI must display terms before calling this instruction.
        // On-chain we record that buyer accepted by signing the tx.

        // ─── Bonding Curve: price = base + slope * total_raised / 10_000 ───
        let current_price = calculate_price(
            sale.base_price_lamports,
            sale.slope_bps,
            sale.total_raised_lamports,
        )?;

        // Base tokens = amount * 1_000_000 / current_price (6 decimal precision)
        let base_tokens = (amount_lamports as u128)
            .checked_mul(1_000_000)
            .unwrap()
            .checked_div(current_price as u128)
            .unwrap() as u64;

        // ─── Decay Bonus: time-based early reward ───
        let elapsed = clock.unix_timestamp.saturating_sub(sale.start_time) as u64;
        let bonus_bps = calculate_time_decay_bonus(
            sale.max_bonus_bps,
            elapsed,
            sale.sale_duration_sec,
        )?;
        let bonus_tokens = (base_tokens as u128)
            .checked_mul(bonus_bps as u128)
            .unwrap()
            .checked_div(10_000)
            .unwrap() as u64;
        let total_tokens = base_tokens.saturating_add(bonus_tokens);

        // ─── Contribution Split ───
        let to_treasury = (amount_lamports as u128)
            .checked_mul(sale.split_treasury_bps as u128)
            .unwrap()
            .checked_div(10_000)
            .unwrap() as u64;
        let to_protocol = (amount_lamports as u128)
            .checked_mul(sale.split_protocol_bps as u128)
            .unwrap()
            .checked_div(10_000)
            .unwrap() as u64;
        let to_validator = (amount_lamports as u128)
            .checked_mul(sale.split_validator_bps as u128)
            .unwrap()
            .checked_div(10_000)
            .unwrap() as u64;
        let to_early_reward = amount_lamports
            .saturating_sub(to_treasury)
            .saturating_sub(to_protocol)
            .saturating_sub(to_validator);

        // Guardrail: early reward pool cannot exceed cap
        let new_early_total = sale
            .early_reward_distributed_lamports
            .checked_add(to_early_reward)
            .unwrap();
        require!(
            new_early_total <= sale.early_reward_cap_lamports,
            MembraTokenomicsError::EarlyRewardCapReached
        );

        // ─── Transfer Splits (via safe CPI) ───
        let buyer_info = ctx.accounts.buyer.to_account_info();
        let treasury_info = ctx.accounts.treasury.to_account_info();
        let protocol_info = ctx.accounts.protocol_wallet.to_account_info();
        let validator_info = ctx.accounts.validator_pool.to_account_info();
        let pool_info = ctx.accounts.early_reward_pool.to_account_info();
        let sys_info = ctx.accounts.system_program.to_account_info();

        invoke(
            &system_instruction::transfer(buyer_info.key, treasury_info.key, to_treasury),
            &[buyer_info.clone(), treasury_info.clone(), sys_info.clone()],
        )?;
        invoke(
            &system_instruction::transfer(buyer_info.key, protocol_info.key, to_protocol),
            &[buyer_info.clone(), protocol_info.clone(), sys_info.clone()],
        )?;
        invoke(
            &system_instruction::transfer(buyer_info.key, validator_info.key, to_validator),
            &[buyer_info.clone(), validator_info.clone(), sys_info.clone()],
        )?;
        invoke(
            &system_instruction::transfer(buyer_info.key, pool_info.key, to_early_reward),
            &[buyer_info.clone(), pool_info.clone(), sys_info.clone()],
        )?;

        // ─── Update Sale State ───
        sale.total_raised_lamports = sale
            .total_raised_lamports
            .checked_add(amount_lamports)
            .unwrap();
        sale.total_tokens_allocated = sale
            .total_tokens_allocated
            .checked_add(total_tokens)
            .unwrap();
        sale.contribution_count = sale.contribution_count.checked_add(1).unwrap();
        sale.early_reward_distributed_lamports = new_early_total;

        // ─── Record Contribution ───
        let contribution = &mut ctx.accounts.contribution;
        contribution.sale = sale.key();
        contribution.buyer = ctx.accounts.buyer.key();
        contribution.amount_lamports = amount_lamports;
        contribution.base_tokens = base_tokens;
        contribution.bonus_tokens = bonus_tokens;
        contribution.total_tokens = total_tokens;
        contribution.bonus_bps = bonus_bps;
        contribution.price_at_contribution = current_price;
        contribution.contribution_index = contribution_index;
        contribution.created_at = clock.unix_timestamp;
        contribution.bump = ctx.bumps.contribution;

        // ─── Record / Update Buyer Receipt ───
        let receipt = &mut ctx.accounts.buyer_receipt;
        receipt.sale = sale.key();
        receipt.buyer = ctx.accounts.buyer.key();
        receipt.total_contributed_lamports = receipt
            .total_contributed_lamports
            .checked_add(amount_lamports)
            .unwrap();
        receipt.total_tokens_allocated = receipt
            .total_tokens_allocated
            .checked_add(total_tokens)
            .unwrap();
        // rebate_claimed_lamports unchanged
        receipt.rebate_claim_status = if receipt.rebate_claim_status == RebateClaimStatus::Claimed as u8 {
            RebateClaimStatus::Claimed as u8
        } else {
            RebateClaimStatus::Eligible as u8
        };
        receipt.last_updated_at = clock.unix_timestamp;

        emit!(ContributionRecorded {
            sale: sale.key(),
            buyer: ctx.accounts.buyer.key(),
            contribution: contribution.key(),
            amount_lamports,
            base_tokens,
            bonus_tokens,
            total_tokens,
            bonus_bps,
            price_at_contribution: current_price,
            contribution_index: contribution.contribution_index,
            treasury_amount: to_treasury,
            protocol_amount: to_protocol,
            validator_amount: to_validator,
            early_reward_amount: to_early_reward,
            timestamp: clock.unix_timestamp,
        });
        emit!(BuyerReceiptRecorded {
            sale: sale.key(),
            buyer: ctx.accounts.buyer.key(),
            receipt: receipt.key(),
            total_contributed_lamports: receipt.total_contributed_lamports,
            total_tokens_allocated: receipt.total_tokens_allocated,
            rebate_eligible: receipt.rebate_claim_status == RebateClaimStatus::Eligible as u8,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    // =========================================================================
    // 4. Claim Rebate (earlier buyers claim capped rebate from early-reward-pool)
    // =========================================================================
    pub fn claim_rebate(ctx: Context<ClaimRebate>) -> Result<()> {
        let sale = &ctx.accounts.token_sale;
        require!(
            sale.status == SaleStatus::Finalized as u8
                || sale.status == SaleStatus::LiquidityMigrated as u8,
            MembraTokenomicsError::ClaimsNotEnabled
        );

        let receipt = &mut ctx.accounts.buyer_receipt;
        require!(
            receipt.rebate_claim_status != RebateClaimStatus::Claimed as u8,
            MembraTokenomicsError::AlreadyClaimed
        );
        require!(
            receipt.rebate_claim_status != RebateClaimStatus::Expired as u8,
            MembraTokenomicsError::ClaimExpired
        );

        let clock = Clock::get()?;
        // Optional: claim window expires 30 days after finalization
        let claim_deadline = sale.end_time.saturating_add(30 * 24 * 60 * 60);
        require!(
            clock.unix_timestamp <= claim_deadline,
            MembraTokenomicsError::ClaimWindowClosed
        );

        // ─── Calculate Capped Rebate ───
        // Rebate = contribution * rebate_rate_bps / 10_000, capped per buyer and pool
        let raw_rebate = (receipt.total_contributed_lamports as u128)
            .checked_mul(sale.rebate_rate_bps as u128)
            .unwrap()
            .checked_div(10_000)
            .unwrap() as u64;

        let _pool_remaining = sale
            .early_reward_cap_lamports
            .saturating_sub(sale.early_reward_distributed_lamports);
        // Note: distributed tracks what went INTO the pool. For claims we need pool balance.
        // The pool balance is tracked by the early_reward_pool account lamports minus rent.
        let pool_balance = ctx
            .accounts
            .early_reward_pool
            .lamports()
            .saturating_sub(Rent::get()?.minimum_balance(0));

        let rebate = raw_rebate
            .min(sale.max_rebate_per_buyer_lamports)
            .min(pool_balance);

        require!(rebate > 0, MembraTokenomicsError::NoRebateAvailable);

        // ─── Transfer Rebate ───
        let sale_key = sale.key();
        let seeds = &[
            b"early_reward_pool",
            sale_key.as_ref(),
            &[sale.early_reward_pool_bump],
        ];
        let signer = &[&seeds[..]];

        invoke_signed(
            &system_instruction::transfer(
                &ctx.accounts.early_reward_pool.key(),
                &ctx.accounts.buyer.key(),
                rebate,
            ),
            &[
                ctx.accounts.early_reward_pool.to_account_info(),
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            signer,
        )?;

        // ─── Update Receipt ───
        receipt.rebate_claimed_lamports = receipt
            .rebate_claimed_lamports
            .checked_add(rebate)
            .unwrap();
        receipt.rebate_claim_status = RebateClaimStatus::Claimed as u8;
        receipt.last_updated_at = clock.unix_timestamp;

        emit!(RebateClaimed {
            sale: sale.key(),
            buyer: receipt.buyer,
            amount_lamports: rebate,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    // =========================================================================
    // 5. Finalize Sale (authority only → claims enabled, liquidity migrated)
    // =========================================================================
    pub fn finalize_sale(ctx: Context<ManageSale>) -> Result<()> {
        let sale = &mut ctx.accounts.token_sale;
        require!(
            sale.status == SaleStatus::Active as u8,
            MembraTokenomicsError::InvalidSaleStatus
        );

        sale.status = SaleStatus::Finalized as u8;

        let clock = Clock::get()?;
        emit!(SaleFinalized {
            sale: sale.key(),
            total_raised_lamports: sale.total_raised_lamports,
            total_tokens_allocated: sale.total_tokens_allocated,
            contribution_count: sale.contribution_count,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    // =========================================================================
    // 6. Migrate Liquidity (authority only → status LiquidityMigrated)
    // =========================================================================
    pub fn migrate_liquidity(ctx: Context<ManageSale>) -> Result<()> {
        let sale = &mut ctx.accounts.token_sale;
        require!(
            sale.status == SaleStatus::Finalized as u8,
            MembraTokenomicsError::InvalidSaleStatus
        );

        sale.status = SaleStatus::LiquidityMigrated as u8;

        let clock = Clock::get()?;
        emit!(LiquidityMigrated {
            sale: sale.key(),
            treasury: sale.treasury,
            total_raised_lamports: sale.total_raised_lamports,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    // =========================================================================
    // 7. Cancel Sale (authority only → refunds must be handled off-chain in v0.1)
    // =========================================================================
    pub fn cancel_sale(ctx: Context<ManageSale>) -> Result<()> {
        let sale = &mut ctx.accounts.token_sale;
        require!(
            sale.status == SaleStatus::Draft as u8
                || sale.status == SaleStatus::Active as u8
                || sale.status == SaleStatus::Paused as u8,
            MembraTokenomicsError::InvalidSaleStatus
        );

        sale.status = SaleStatus::Cancelled as u8;

        let clock = Clock::get()?;
        emit!(SaleCancelled {
            sale: sale.key(),
            total_raised_lamports: sale.total_raised_lamports,
            timestamp: clock.unix_timestamp,
        });

        Ok(())
    }

    // =========================================================================
    // 8. Pause / Resume (authority only)
    // =========================================================================
    pub fn set_sale_pause(ctx: Context<ManageSale>, paused: bool) -> Result<()> {
        let sale = &mut ctx.accounts.token_sale;
        let clock = Clock::get()?;
        if paused {
            require!(
                sale.status == SaleStatus::Active as u8,
                MembraTokenomicsError::InvalidSaleStatus
            );
            sale.status = SaleStatus::Paused as u8;
            emit!(SalePaused {
                sale: sale.key(),
                timestamp: clock.unix_timestamp,
            });
        } else {
            require!(
                sale.status == SaleStatus::Paused as u8,
                MembraTokenomicsError::InvalidSaleStatus
            );
            sale.status = SaleStatus::Active as u8;
            emit!(SaleResumed {
                sale: sale.key(),
                timestamp: clock.unix_timestamp,
            });
        }
        Ok(())
    }

    // =========================================================================
    // 9. Provenance Attestation Instructions
    // =========================================================================
    
    /// Create a provenance attestation for token appraisal
    #[allow(clippy::too_many_arguments)]
    pub fn create_provenance_attestation(
        ctx: Context<CreateProvenanceAttestation>,
        attestation_id: u64,
        merkle_root: [u8; 32],
        novelty_score: u16,
        rarity_score: u16,
        execution_difficulty: u16,
        base_value: u64,
    ) -> Result<()> {
        let attestation = &mut ctx.accounts.attestation;
        
        require!(
            novelty_score <= 10000,
            MembraTokenomicsError::InvalidAppraisalScores
        );
        require!(
            rarity_score <= 10000,
            MembraTokenomicsError::InvalidAppraisalScores
        );
        require!(
            execution_difficulty <= 10000,
            MembraTokenomicsError::InvalidAppraisalScores
        );
        
        attestation.attestation_id = attestation_id;
        attestation.token_mint = ctx.accounts.token_mint.key();
        attestation.merkle_root = merkle_root;
        attestation.novelty_score = novelty_score;
        attestation.rarity_score = rarity_score;
        attestation.execution_difficulty = execution_difficulty;
        
        // Calculate market cap appraisal
        attestation.market_cap_appraisal = MarketCapCalculator::calculate_appraisal(
            base_value,
            novelty_score,
            rarity_score,
            execution_difficulty,
        );
        
        let clock = Clock::get()?;
        attestation.attestation_timestamp = clock.unix_timestamp;
        attestation.authority = ctx.accounts.authority.key();
        attestation.verified = false;
        attestation.bump = ctx.bumps.attestation;
        
        Ok(())
    }
    
    /// Verify a provenance attestation with merkle proof
    pub fn verify_provenance_attestation(
        ctx: Context<VerifyProvenanceAttestation>,
        proof: MerkleProof,
    ) -> Result<()> {
        let attestation = &mut ctx.accounts.attestation;
        
        require!(
            !attestation.verified,
            MembraTokenomicsError::AlreadyVerified
        );
        
        // Verify merkle proof
        require!(
            proof.verify(&attestation.merkle_root),
            MembraTokenomicsError::InvalidMerkleProof
        );
        
        attestation.verified = true;
        
        Ok(())
    }

    // =========================================================================
    // 10. Collateral Lock Instructions
    // =========================================================================
    
    /// Initialize collateral configuration
    pub fn initialize_collateral_config(
        ctx: Context<InitializeCollateralConfig>,
        max_cpu_lock_bps: u16,
        max_memory_lock_bps: u16,
        min_lock_duration_sec: u64,
        max_lock_duration_sec: u64,
        collateral_multiplier_bps: u16,
    ) -> Result<()> {
        let config = &mut ctx.accounts.collateral_config;
        
        require!(
            max_cpu_lock_bps <= 10000,
            MembraTokenomicsError::InvalidLockParameters
        );
        require!(
            max_memory_lock_bps <= 10000,
            MembraTokenomicsError::InvalidLockParameters
        );
        require!(
            min_lock_duration_sec < max_lock_duration_sec,
            MembraTokenomicsError::InvalidLockDuration
        );
        require!(
            collateral_multiplier_bps <= 20000, // max 2x multiplier
            MembraTokenomicsError::InvalidLockParameters
        );
        
        config.authority = ctx.accounts.authority.key();
        config.max_cpu_lock_bps = max_cpu_lock_bps;
        config.max_memory_lock_bps = max_memory_lock_bps;
        config.min_lock_duration_sec = min_lock_duration_sec;
        config.max_lock_duration_sec = max_lock_duration_sec;
        config.collateral_multiplier_bps = collateral_multiplier_bps;
        config.bump = ctx.bumps.collateral_config;
        
        Ok(())
    }
    
    /// Lock compute resources as collateral
    #[allow(clippy::too_many_arguments)]
    pub fn lock_collateral(
        ctx: Context<LockCollateral>,
        lock_id: [u8; 16],
        cpu_cores_locked: u8,
        memory_gb_locked: u16,
        gpu_locked: bool,
        lock_duration_sec: u64,
        lock_purpose: u8,
        associated_tx: Option<[u8; 32]>,
    ) -> Result<()> {
        let config = &ctx.accounts.collateral_config;
        let lock = &mut ctx.accounts.collateral_lock;
        let clock = Clock::get()?;
        
        // Validate lock duration
        require!(
            lock_duration_sec >= config.min_lock_duration_sec,
            MembraTokenomicsError::InvalidLockDuration
        );
        require!(
            lock_duration_sec <= config.max_lock_duration_sec,
            MembraTokenomicsError::InvalidLockDuration
        );
        
        // Validate lock purpose
        require!(
            lock_purpose <= CollateralLock::PURPOSE_NETWORK_TASK,
            MembraTokenomicsError::InvalidLockParameters
        );
        
        // Calculate collateral value
        let collateral_value = calculate_collateral_value(
            cpu_cores_locked,
            memory_gb_locked,
            gpu_locked,
            lock_duration_sec,
            config.collateral_multiplier_bps,
        )?;
        
        // Create lock record
        lock.lock_id = lock_id;
        lock.locker = ctx.accounts.locker.key();
        lock.cpu_cores_locked = cpu_cores_locked;
        lock.memory_gb_locked = memory_gb_locked;
        lock.gpu_locked = gpu_locked;
        lock.collateral_value = collateral_value;
        lock.lock_start_ts = clock.unix_timestamp;
        lock.lock_duration_sec = lock_duration_sec;
        lock.lock_purpose = lock_purpose;
        lock.associated_tx = associated_tx;
        lock.lock_status = CollateralLock::STATUS_LOCKED;
        lock.bump = ctx.bumps.collateral_lock;
        
        Ok(())
    }
    
    /// Unlock previously locked collateral
    pub fn unlock_collateral(
        ctx: Context<UnlockCollateral>,
        lock_id: [u8; 16],
    ) -> Result<()> {
        let lock = &mut ctx.accounts.collateral_lock;
        
        require!(
            lock.lock_id == lock_id,
            MembraTokenomicsError::LockNotFound
        );
        require!(
            lock.locker == ctx.accounts.locker.key(),
            MembraTokenomicsError::UnauthorizedCollateral
        );
        require!(
            lock.lock_status == CollateralLock::STATUS_LOCKED,
            MembraTokenomicsError::AlreadyUnlocked
        );
        
        lock.lock_status = CollateralLock::STATUS_UNLOCKED;
        
        Ok(())
    }
    
    /// Force unlock expired collateral (can be called by anyone)
    pub fn unlock_expired_collateral(
        ctx: Context<UnlockExpiredCollateral>,
        lock_id: [u8; 16],
    ) -> Result<()> {
        let lock = &mut ctx.accounts.collateral_lock;
        let clock = Clock::get()?;
        
        require!(
            lock.lock_id == lock_id,
            MembraTokenomicsError::LockNotFound
        );
        require!(
            lock.lock_status == CollateralLock::STATUS_LOCKED,
            MembraTokenomicsError::AlreadyUnlocked
        );
        
        // Check if lock has expired
        let expiry_time = lock.lock_start_ts + lock.lock_duration_sec as i64;
        require!(
            clock.unix_timestamp >= expiry_time,
            MembraTokenomicsError::LockNotExpired
        );
        
        lock.lock_status = CollateralLock::STATUS_EXPIRED;
        
        Ok(())
    }

    // =========================================================================
    // ZK-PoPC: Monetary Creation from Verified Productive Capacity
    // =========================================================================
    pub fn initialize_seigniorage_config(
        ctx: Context<InitializeSeigniorageConfig>,
        tier_thresholds: [u64; 5],
    ) -> Result<()> {
        zk_seigniorage::initialize_seigniorage_config(ctx, tier_thresholds)
    }

    pub fn submit_capacity_proof(
        ctx: Context<SubmitCapacityProof>,
        proof_id: [u8; 32],
        input_commitment: [u8; 32],
        output_commitment: [u8; 32],
        circuit_identifier: String,
        difficulty_score: u64,
        compute_units_consumed: u64,
        nullifier_hash: [u8; 32],
    ) -> Result<()> {
        zk_seigniorage::submit_capacity_proof(
            ctx,
            proof_id,
            input_commitment,
            output_commitment,
            circuit_identifier,
            difficulty_score,
            compute_units_consumed,
            nullifier_hash,
        )
    }

    pub fn attest_capacity_proof(
        ctx: Context<AttestCapacityProof>,
        attestation_hash: [u8; 32],
    ) -> Result<()> {
        zk_seigniorage::attest_capacity_proof(ctx, attestation_hash)
    }

    pub fn mint_from_capacity_proof(ctx: Context<MintFromCapacityProof>) -> Result<()> {
        zk_seigniorage::mint_from_capacity_proof(ctx)
    }
}

// =============================================================================
// MATH HELPERS (checked integer arithmetic, no floats)
// =============================================================================

/// Linear bonding curve: price = base + slope * total_raised / 10_000
fn calculate_price(
    base_price: u64,
    slope_bps: u64,
    total_raised: u64,
) -> Result<u64> {
    let slope_component = (total_raised as u128)
        .checked_mul(slope_bps as u128)
        .unwrap()
        .checked_div(10_000)
        .unwrap() as u64;
    base_price
        .checked_add(slope_component)
        .ok_or(MembraTokenomicsError::MathOverflow.into())
}

/// Time-decay bonus: max_bonus * (1 - elapsed / duration)
/// Returns basis points (0 .. max_bonus_bps)
fn calculate_time_decay_bonus(
    max_bonus_bps: u16,
    elapsed_sec: u64,
    duration_sec: u64,
) -> Result<u16> {
    if elapsed_sec >= duration_sec {
        return Ok(0);
    }
    // remaining_ratio = (duration - elapsed) * 10_000 / duration
    let remaining_ratio = ((duration_sec - elapsed_sec) as u128)
        .checked_mul(10_000)
        .unwrap()
        .checked_div(duration_sec as u128)
        .unwrap() as u64;

    let bonus = (max_bonus_bps as u128)
        .checked_mul(remaining_ratio as u128)
        .unwrap()
        .checked_div(10_000)
        .unwrap() as u16;

    Ok(bonus)
}

// =============================================================================
// ENUMS
// =============================================================================

#[derive(Clone, Copy, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum SaleStatus {
    Draft = 0,
    Active = 1,
    Paused = 2,
    Finalized = 3,
    Cancelled = 4,
    LiquidityMigrated = 5,
}

#[derive(Clone, Copy, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum RebateClaimStatus {
    Pending = 0,
    Eligible = 1,
    Claimed = 2,
    Expired = 3,
    Denied = 4,
}

// =============================================================================
// ACCOUNTS
// =============================================================================

#[account]
pub struct TokenSale {
    pub authority: Pubkey,
    pub sale_id: u64,
    pub status: u8,
    pub base_price_lamports: u64,
    pub slope_bps: u64,
    pub max_bonus_bps: u16,
    pub sale_duration_sec: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub total_raised_lamports: u64,
    pub total_tokens_allocated: u64,
    pub contribution_count: u64,
    pub treasury: Pubkey,
    pub protocol_wallet: Pubkey,
    pub validator_pool: Pubkey,
    pub split_treasury_bps: u16,
    pub split_protocol_bps: u16,
    pub split_validator_bps: u16,
    pub split_early_reward_bps: u16,
    pub early_reward_cap_lamports: u64,
    pub early_reward_distributed_lamports: u64,
    pub max_rebate_per_buyer_lamports: u64,
    pub rebate_rate_bps: u16,
    pub hard_cap_lamports: u64,
    pub min_contribution_lamports: u64,
    pub bump: u8,
    pub early_reward_pool_bump: u8,
}

#[account]
pub struct Contribution {
    pub sale: Pubkey,
    pub buyer: Pubkey,
    pub amount_lamports: u64,
    pub base_tokens: u64,
    pub bonus_tokens: u64,
    pub total_tokens: u64,
    pub bonus_bps: u16,
    pub price_at_contribution: u64,
    pub contribution_index: u64,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
pub struct BuyerReceipt {
    pub sale: Pubkey,
    pub buyer: Pubkey,
    pub total_contributed_lamports: u64,
    pub total_tokens_allocated: u64,
    pub rebate_claimed_lamports: u64,
    pub rebate_claim_status: u8,
    pub last_updated_at: i64,
    pub bump: u8,
}

// =============================================================================
// CONTEXTS
// =============================================================================

#[derive(Accounts)]
#[instruction(sale_id: u64, sale_id_bytes: [u8; 8])]
pub struct InitializeSale<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + TokenSale::INIT_SPACE,
        seeds = [b"token_sale", sale_id_bytes.as_ref()],
        bump
    )]
    pub token_sale: Account<'info, TokenSale>,
    /// CHECK: treasury wallet (external system account)
    pub treasury: AccountInfo<'info>,
    /// CHECK: protocol wallet (external system account)
    pub protocol_wallet: AccountInfo<'info>,
    /// CHECK: validator pool wallet (external system account)
    pub validator_pool: AccountInfo<'info>,
    #[account(
        init,
        payer = authority,
        space = 8, // minimal account, holds lamports
        seeds = [b"early_reward_pool", token_sale.key().as_ref()],
        bump
    )]
    /// CHECK: early reward pool PDA (holds SOL for rebates)
    pub early_reward_pool: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ManageSale<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        has_one = authority @ MembraTokenomicsError::Unauthorized,
    )]
    pub token_sale: Account<'info, TokenSale>,
}

#[derive(Accounts)]
#[instruction(amount_lamports: u64, contribution_index: u64, contribution_index_bytes: [u8; 8])]
pub struct Contribute<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub token_sale: Account<'info, TokenSale>,
    /// CHECK: treasury wallet
    #[account(mut, address = token_sale.treasury @ MembraTokenomicsError::InvalidWallet)]
    pub treasury: AccountInfo<'info>,
    /// CHECK: protocol wallet
    #[account(mut, address = token_sale.protocol_wallet @ MembraTokenomicsError::InvalidWallet)]
    pub protocol_wallet: AccountInfo<'info>,
    /// CHECK: validator pool wallet
    #[account(mut, address = token_sale.validator_pool @ MembraTokenomicsError::InvalidWallet)]
    pub validator_pool: AccountInfo<'info>,
    /// CHECK: early reward pool PDA
    #[account(
        mut,
        seeds = [b"early_reward_pool", token_sale.key().as_ref()],
        bump = token_sale.early_reward_pool_bump,
    )]
    pub early_reward_pool: AccountInfo<'info>,
    #[account(
        init,
        payer = buyer,
        space = 8 + Contribution::INIT_SPACE,
        seeds = [b"contribution", token_sale.key().as_ref(), buyer.key().as_ref(), &contribution_index_bytes],
        bump
    )]
    pub contribution: Account<'info, Contribution>,
    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + BuyerReceipt::INIT_SPACE,
        seeds = [b"buyer_receipt", token_sale.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub buyer_receipt: Account<'info, BuyerReceipt>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimRebate<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub token_sale: Account<'info, TokenSale>,
    #[account(
        mut,
        seeds = [b"early_reward_pool", token_sale.key().as_ref()],
        bump = token_sale.early_reward_pool_bump,
    )]
    /// CHECK: early reward pool PDA
    pub early_reward_pool: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"buyer_receipt", token_sale.key().as_ref(), buyer.key().as_ref()],
        bump = buyer_receipt.bump,
        constraint = buyer_receipt.buyer == buyer.key() @ MembraTokenomicsError::Unauthorized,
    )]
    pub buyer_receipt: Account<'info, BuyerReceipt>,
    pub system_program: Program<'info, System>,
}

// =============================================================================
// SPACE CALCULATIONS
// =============================================================================

impl TokenSale {
    pub const INIT_SPACE: usize =
        32 + 8 + 1 + 8 + 8 + 2 + 8 + 8 + 8 + 8 + 8 + 8 + 32 + 32 + 32 + 2 + 2 + 2 + 2 + 8 + 8 + 8 + 2 + 8 + 8 + 1 + 1;
}

impl Contribution {
    pub const INIT_SPACE: usize =
        32 + 32 + 8 + 8 + 8 + 8 + 2 + 8 + 8 + 8 + 1;
}

impl BuyerReceipt {
    pub const INIT_SPACE: usize =
        32 + 32 + 8 + 8 + 8 + 1 + 8 + 1;
}

// =============================================================================
// EVENTS
// =============================================================================

#[event]
pub struct SaleInitialized {
    pub sale: Pubkey,
    pub authority: Pubkey,
    pub sale_id: u64,
    pub base_price_lamports: u64,
    pub slope_bps: u64,
    pub max_bonus_bps: u16,
    pub sale_duration_sec: u64,
    pub hard_cap_lamports: u64,
    pub min_contribution_lamports: u64,
    pub early_reward_cap_lamports: u64,
    pub max_rebate_per_buyer_lamports: u64,
    pub rebate_rate_bps: u16,
    pub timestamp: i64,
}

#[event]
pub struct SaleActivated {
    pub sale: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
}

#[event]
pub struct ContributionRecorded {
    pub sale: Pubkey,
    pub buyer: Pubkey,
    pub contribution: Pubkey,
    pub amount_lamports: u64,
    pub base_tokens: u64,
    pub bonus_tokens: u64,
    pub total_tokens: u64,
    pub bonus_bps: u16,
    pub price_at_contribution: u64,
    pub contribution_index: u64,
    pub treasury_amount: u64,
    pub protocol_amount: u64,
    pub validator_amount: u64,
    pub early_reward_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct BuyerReceiptRecorded {
    pub sale: Pubkey,
    pub buyer: Pubkey,
    pub receipt: Pubkey,
    pub total_contributed_lamports: u64,
    pub total_tokens_allocated: u64,
    pub rebate_eligible: bool,
    pub timestamp: i64,
}

#[event]
pub struct RebateClaimed {
    pub sale: Pubkey,
    pub buyer: Pubkey,
    pub amount_lamports: u64,
    pub timestamp: i64,
}

#[event]
pub struct SaleFinalized {
    pub sale: Pubkey,
    pub total_raised_lamports: u64,
    pub total_tokens_allocated: u64,
    pub contribution_count: u64,
    pub timestamp: i64,
}

#[event]
pub struct LiquidityMigrated {
    pub sale: Pubkey,
    pub treasury: Pubkey,
    pub total_raised_lamports: u64,
    pub timestamp: i64,
}

#[event]
pub struct SaleCancelled {
    pub sale: Pubkey,
    pub total_raised_lamports: u64,
    pub timestamp: i64,
}

#[event]
pub struct SalePaused {
    pub sale: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct SaleResumed {
    pub sale: Pubkey,
    pub timestamp: i64,
}

// =============================================================================
// PROVENANCE ATTESTATION INSTRUCTIONS
// =============================================================================

#[derive(Accounts)]
#[instruction(attestation_id: u64)]
pub struct CreateProvenanceAttestation<'info> {
    #[account(
        init,
        payer = authority,
        space = ProvenanceAttestation::LEN,
        seeds = [b"provenance", token_mint.key().as_ref(), attestation_id.to_le_bytes().as_ref()],
        bump
    )]
    pub attestation: Account<'info, ProvenanceAttestation>,
    
    pub token_mint: Account<'info, anchor_spl::token::Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyProvenanceAttestation<'info> {
    #[account(
        mut,
        seeds = [b"provenance", attestation.token_mint.as_ref(), attestation.attestation_id.to_le_bytes().as_ref()],
        bump = attestation.bump,
        has_one = authority
    )]
    pub attestation: Account<'info, ProvenanceAttestation>,
    
    pub authority: Signer<'info>,
}

// =============================================================================
// COLLATERAL LOCK INSTRUCTIONS
// =============================================================================

#[derive(Accounts)]
pub struct InitializeCollateralConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = CollateralConfig::LEN,
        seeds = [b"collateral_config"],
        bump
    )]
    pub collateral_config: Account<'info, CollateralConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(lock_id: [u8; 16])]
pub struct LockCollateral<'info> {
    #[account(
        init,
        payer = locker,
        space = CollateralLock::LEN,
        seeds = [b"collateral_lock", locker.key().as_ref(), lock_id.as_ref()],
        bump
    )]
    pub collateral_lock: Account<'info, CollateralLock>,
    
    #[account(
        seeds = [b"collateral_config"],
        bump = collateral_config.bump
    )]
    pub collateral_config: Account<'info, CollateralConfig>,
    
    #[account(mut)]
    pub locker: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnlockCollateral<'info> {
    #[account(
        mut,
        seeds = [b"collateral_lock", locker.key().as_ref(), collateral_lock.lock_id.as_ref()],
        bump = collateral_lock.bump,
        has_one = locker
    )]
    pub collateral_lock: Account<'info, CollateralLock>,
    
    pub locker: Signer<'info>,
}

#[derive(Accounts)]
pub struct UnlockExpiredCollateral<'info> {
    #[account(
        mut,
        seeds = [b"collateral_lock", collateral_lock.locker.as_ref(), collateral_lock.lock_id.as_ref()],
        bump = collateral_lock.bump
    )]
    pub collateral_lock: Account<'info, CollateralLock>,
}

// =============================================================================
// ERRORS
// =============================================================================

#[error_code]
pub enum MembraTokenomicsError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid sale status for this operation")]
    InvalidSaleStatus,
    #[msg("Sale is not active")]
    SaleNotActive,
    #[msg("Sale has expired")]
    SaleExpired,
    #[msg("Invalid price parameters")]
    InvalidPrice,
    #[msg("Max bonus too high (max 50%)")]
    BonusTooHigh,
    #[msg("Rebate rate too high (max 20%)")]
    RebateTooHigh,
    #[msg("Invalid sale duration")]
    InvalidDuration,
    #[msg("Split percentages must sum to 10000 bps")]
    InvalidSplits,
    #[msg("Contribution amount must be greater than zero")]
    ZeroContribution,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Early reward cap reached")]
    EarlyRewardCapReached,
    #[msg("Invalid wallet address")]
    InvalidWallet,
    #[msg("Claims are not yet enabled")]
    ClaimsNotEnabled,
    #[msg("Rebate already claimed")]
    AlreadyClaimed,
    #[msg("Claim expired")]
    ClaimExpired,
    #[msg("Claim window closed")]
    ClaimWindowClosed,
    #[msg("No rebate available")]
    NoRebateAvailable,
    #[msg("Invalid hard cap")]
    InvalidHardCap,
    #[msg("Invalid minimum contribution")]
    InvalidMinContribution,
    #[msg("Hard cap reached")]
    HardCapReached,
    #[msg("Contribution too small")]
    ContributionTooSmall,
    #[msg("Invalid merkle proof")]
    InvalidMerkleProof,
    #[msg("Attestation already verified")]
    AlreadyVerified,
    #[msg("Invalid appraisal scores")]
    InvalidAppraisalScores,
    #[msg("Unauthorized attestation")]
    UnauthorizedAttestation,
    #[msg("Expired attestation")]
    ExpiredAttestation,
    #[msg("Invalid lock parameters")]
    InvalidLockParameters,
    #[msg("Lock duration outside allowed range")]
    InvalidLockDuration,
    #[msg("Lock would exceed maximum CPU ratio")]
    ExceedsMaxCpuRatio,
    #[msg("Lock would exceed maximum memory ratio")]
    ExceedsMaxMemoryRatio,
    #[msg("Lock not found")]
    LockNotFound,
    #[msg("Lock already unlocked")]
    AlreadyUnlocked,
    #[msg("Lock not expired")]
    LockNotExpired,
    #[msg("Unauthorized collateral operation")]
    UnauthorizedCollateral,
    #[msg("Invalid difficulty score (must be 1-10000)")]
    InvalidDifficultyScore,
    #[msg("Circuit identifier too long (max 64 chars)")]
    CircuitIdTooLong,
    #[msg("Invalid collateral lock account")]
    InvalidCollateralLock,
    #[msg("Collateral lock does not belong to prover")]
    CollateralLockMismatch,
    #[msg("Proof is not in pending status")]
    ProofNotPending,
    #[msg("Proof has not been verified")]
    ProofNotVerified,
    #[msg("Maximum verifiers reached")]
    MaxVerifiersReached,
    #[msg("Duplicate attestation from same verifier")]
    DuplicateAttestation,
    #[msg("Nullifier has already been used")]
    NullifierAlreadyUsed,
    #[msg("This proof has already been minted")]
    AlreadyMinted,
    #[msg("Calculated mint amount is zero")]
    ZeroMintAmount,
    #[msg("Mint amount exceeds maximum per proof")]
    MaxMintExceeded,
}
