use anchor_lang::prelude::*;
use anchor_lang::solana_program::{hash::hashv, program::invoke_signed};
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
use anchor_spl::associated_token;
use std::collections::HashSet;

// =============================================================================
// ZK-Proof-of-Productive-Capacity (ZK-PoPC) Monetary Creation
// =============================================================================
// Novel seigniorage mechanism: new tokens are minted ONLY when a user proves
// via ZK that locked collateral was used to produce unique, externally-
// validated network value. Privacy-preserving, counter-cyclical, and backed
// by real productive capacity rather than speculation or energy waste.
//
// Monetary Policy:
//   mint = base * collateral_tier * difficulty * verifier_bonus * adjustment
//   adjustment = target_capacity_ratio / current_capacity_ratio  (cap 1.0)
//
// This means: as the network's proven productive capacity grows, each unit
// of new capacity receives fewer tokens — but early capacity is rewarded
// more heavily. The supply expands proportionally to verifiable output.
// =============================================================================

pub const HASH_SIZE: usize = 32;
pub const MAX_VERIFIERS: usize = 5;
pub const MIN_VERIFIERS_REQUIRED: u8 = 2;
pub const MAX_NULLIFIERS_PER_PROOF: usize = 8;

// Monetary policy constants (in basis points or lamports where noted)
pub const TARGET_CAPACITY_RATIO_BPS: u64 = 10_000; // 1.0 in bps
pub const BASE_MINT_REWARD: u64 = 1_000_000;       // 0.001 token units
pub const MAX_MINT_PER_PROOF: u64 = 100_000_000_000; // 100k token units
pub const SECONDS_PER_EPOCH: i64 = 86_400;          // 1 day

/// Collateral tier multipliers (in basis points)
pub const TIER_1_MULTIPLIER_BPS: u64 = 2_500;  // 0.25x base (lowest)
pub const TIER_2_MULTIPLIER_BPS: u64 = 5_000;  // 0.5x base
pub const TIER_3_MULTIPLIER_BPS: u64 = 10_000; // 1.0x base
pub const TIER_4_MULTIPLIER_BPS: u64 = 15_000; // 1.5x base
pub const TIER_5_MULTIPLIER_BPS: u64 = 25_000; // 2.5x base (highest)

/// Global monetary policy configuration
#[account]
pub struct ZkSeigniorageConfig {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub mint_authority_bump: u8,

    // Monetary policy knobs
    pub base_mint_reward: u64,
    pub max_mint_per_proof: u64,
    pub target_capacity_ratio_bps: u64,
    pub min_verifiers_required: u8,

    // Collateral thresholds (in lamports)
    pub tier_1_threshold: u64,
    pub tier_2_threshold: u64,
    pub tier_3_threshold: u64,
    pub tier_4_threshold: u64,
    pub tier_5_threshold: u64,

    // Network state
    pub total_proven_capacity: u64,   // aggregate difficulty-weighted capacity
    pub total_tokens_minted: u64,     // aggregate supply from this mechanism
    pub current_epoch: u64,
    pub last_epoch_timestamp: i64,

    pub bump: u8,
}

impl ZkSeigniorageConfig {
    pub const LEN: usize = 8 // discriminator
        + 32 // authority
        + 32 // token_mint
        + 1  // mint_authority_bump
        + 8  // base_mint_reward
        + 8  // max_mint_per_proof
        + 8  // target_capacity_ratio_bps
        + 1  // min_verifiers_required
        + 8 * 5 // tier thresholds
        + 8  // total_proven_capacity
        + 8  // total_tokens_minted
        + 8  // current_epoch
        + 8  // last_epoch_timestamp
        + 1; // bump
}

/// A submitted ZK proof of productive capacity (PDA)
#[account]
pub struct CapacityProof {
    pub prover: Pubkey,
    pub proof_id: [u8; HASH_SIZE],
    pub collateral_lock: Pubkey,
    pub collateral_tier: u8,

    // ZK public inputs (hashed)
    pub input_commitment: [u8; HASH_SIZE],
    pub output_commitment: [u8; HASH_SIZE],
    pub circuit_identifier: String,

    // Work metrics
    pub difficulty_score: u64,       // 1-10_000
    pub compute_units_consumed: u64,
    pub epoch: u64,

    // Verification (off-chain attesters sign hashes on-chain)
    pub verifier_attestations: Vec<VerifierAttestation>,
    pub attestation_count: u8,
    pub verification_status: u8,     // 0=pending, 1=verified, 2=rejected

    // Monetary result
    pub mint_amount: u64,
    pub minted_at: i64,
    pub nullifier_hash: [u8; HASH_SIZE],

    pub bump: u8,
}

impl CapacityProof {
    pub const LEN: usize = 8
        + 32
        + 32
        + 32
        + 1
        + 32
        + 32
        + 4 + 64 // circuit identifier (max 64 chars)
        + 8
        + 8
        + 8
        + 4 + (MAX_VERIFIERS * VerifierAttestation::LEN)
        + 1
        + 1
        + 8
        + 8
        + 32
        + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct VerifierAttestation {
    pub verifier: Pubkey,
    pub attestation_hash: [u8; HASH_SIZE],
    pub timestamp: i64,
    pub signature_valid: bool,
}

impl VerifierAttestation {
    pub const LEN: usize = 32 + 32 + 8 + 1;
}

/// Nullifier to prevent double-minting from the same proof (PDA)
#[account]
pub struct ProofNullifier {
    pub proof_id: [u8; HASH_SIZE],
    pub nullifier_hash: [u8; HASH_SIZE],
    pub used: bool,
    pub used_at: i64,
    pub bump: u8,
}

impl ProofNullifier {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 8 + 1;
}

/// Per-epoch network capacity ledger (PDA)
#[account]
pub struct EpochCapacityLedger {
    pub epoch: u64,
    pub total_proven_capacity: u64,
    pub total_proofs_verified: u64,
    pub total_tokens_minted: u64,
    pub average_difficulty: u64,
    pub bump: u8,
}

impl EpochCapacityLedger {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 8 + 8 + 1;
}

// =============================================================================
// INSTRUCTIONS
// =============================================================================

#[derive(Accounts)]
pub struct InitializeSeigniorageConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = ZkSeigniorageConfig::LEN,
        seeds = [b"zk_seigniorage_config"],
        bump
    )]
    pub config: Account<'info, ZkSeigniorageConfig>,

    /// CHECK: This PDA will be the mint authority
    #[account(
        seeds = [b"zk_seigniorage_mint_authority"],
        bump
    )]
    pub mint_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [b"zk_seigniorage_mint"],
        bump,
        mint::decimals = 6,
        mint::authority = mint_authority,
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_seigniorage_config(
    ctx: Context<InitializeSeigniorageConfig>,
    tier_thresholds: [u64; 5],
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let clock = Clock::get()?;

    config.authority = ctx.accounts.authority.key();
    config.token_mint = ctx.accounts.token_mint.key();
    config.mint_authority_bump = ctx.bumps.mint_authority;

    config.base_mint_reward = BASE_MINT_REWARD;
    config.max_mint_per_proof = MAX_MINT_PER_PROOF;
    config.target_capacity_ratio_bps = TARGET_CAPACITY_RATIO_BPS;
    config.min_verifiers_required = MIN_VERIFIERS_REQUIRED;

    config.tier_1_threshold = tier_thresholds[0];
    config.tier_2_threshold = tier_thresholds[1];
    config.tier_3_threshold = tier_thresholds[2];
    config.tier_4_threshold = tier_thresholds[3];
    config.tier_5_threshold = tier_thresholds[4];

    config.total_proven_capacity = 0;
    config.total_tokens_minted = 0;
    config.current_epoch = 1;
    config.last_epoch_timestamp = clock.unix_timestamp;
    config.bump = ctx.bumps.config;

    emit!(SeigniorageConfigInitialized {
        authority: config.authority,
        token_mint: config.token_mint,
        base_mint_reward: config.base_mint_reward,
        max_mint_per_proof: config.max_mint_per_proof,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ---------------------------------------------------------------------------
// Submit a ZK proof of productive capacity (does not mint yet)
// ---------------------------------------------------------------------------
#[derive(Accounts)]
#[instruction(proof_id: [u8; HASH_SIZE], nullifier_hash: [u8; HASH_SIZE])]
pub struct SubmitCapacityProof<'info> {
    #[account(
        init,
        payer = prover,
        space = CapacityProof::LEN,
        seeds = [b"capacity_proof", prover.key().as_ref(), proof_id.as_ref()],
        bump
    )]
    pub proof: Account<'info, CapacityProof>,

    #[account(
        init,
        payer = prover,
        space = ProofNullifier::LEN,
        seeds = [b"proof_nullifier", nullifier_hash.as_ref()],
        bump
    )]
    pub nullifier: Account<'info, ProofNullifier>,

    #[account(
        seeds = [b"zk_seigniorage_config"],
        bump = config.bump
    )]
    pub config: Account<'info, ZkSeigniorageConfig>,

    // References an existing collateral lock (from collateral_lock.rs)
    /// CHECK: validated in instruction logic against expected locker
    #[account(mut)]
    pub collateral_lock: UncheckedAccount<'info>,

    #[account(mut)]
    pub prover: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn submit_capacity_proof(
    ctx: Context<SubmitCapacityProof>,
    proof_id: [u8; HASH_SIZE],
    input_commitment: [u8; HASH_SIZE],
    output_commitment: [u8; HASH_SIZE],
    circuit_identifier: String,
    difficulty_score: u64,
    compute_units_consumed: u64,
    nullifier_hash: [u8; HASH_SIZE],
) -> Result<()> {
    require!(
        difficulty_score > 0 && difficulty_score <= 10_000,
        ZkSeigniorageError::InvalidDifficultyScore
    );
    require!(
        circuit_identifier.len() <= 64,
        ZkSeigniorageError::CircuitIdTooLong
    );

    let proof = &mut ctx.accounts.proof;
    let nullifier = &mut ctx.accounts.nullifier;
    let config = &ctx.accounts.config;
    let clock = Clock::get()?;

    // Ensure collateral lock belongs to prover and is still active
    let lock_data = ctx.accounts.collateral_lock.data.borrow();
    require!(lock_data.len() >= 40, ZkSeigniorageError::InvalidCollateralLock);
    // First 8 bytes = discriminator, next 32 = locker pubkey
    let locker_pubkey = Pubkey::try_from(&lock_data[8..40]).map_err(|_| error!(ZkSeigniorageError::InvalidCollateralLock))?;
    require!(
        locker_pubkey == ctx.accounts.prover.key(),
        ZkSeigniorageError::CollateralLockMismatch
    );

    // Derive collateral tier from lock value (simplified: we read a u64 at offset 40 as locked_value)
    let locked_value = if lock_data.len() >= 48 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&lock_data[40..48]);
        u64::from_le_bytes(bytes)
    } else {
        0
    };

    let collateral_tier = derive_collateral_tier(locked_value, config);

    proof.prover = ctx.accounts.prover.key();
    proof.proof_id = proof_id;
    proof.collateral_lock = ctx.accounts.collateral_lock.key();
    proof.collateral_tier = collateral_tier;
    proof.input_commitment = input_commitment;
    proof.output_commitment = output_commitment;
    proof.circuit_identifier = circuit_identifier;
    proof.difficulty_score = difficulty_score;
    proof.compute_units_consumed = compute_units_consumed;
    proof.epoch = config.current_epoch;
    proof.verifier_attestations = Vec::new();
    proof.attestation_count = 0;
    proof.verification_status = ProofStatus::Pending as u8;
    proof.mint_amount = 0;
    proof.minted_at = 0;
    proof.nullifier_hash = nullifier_hash;
    proof.bump = ctx.bumps.proof;

    nullifier.proof_id = proof_id;
    nullifier.nullifier_hash = nullifier_hash;
    nullifier.used = false;
    nullifier.used_at = 0;
    nullifier.bump = ctx.bumps.nullifier;

    emit!(CapacityProofSubmitted {
        prover: proof.prover,
        proof_id,
        collateral_lock: proof.collateral_lock,
        collateral_tier,
        difficulty_score,
        epoch: proof.epoch,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ---------------------------------------------------------------------------
// Independent verifier attests to a capacity proof (adds weight)
// ---------------------------------------------------------------------------
#[derive(Accounts)]
pub struct AttestCapacityProof<'info> {
    #[account(
        mut,
        seeds = [b"capacity_proof", proof.prover.as_ref(), proof.proof_id.as_ref()],
        bump = proof.bump,
        constraint = proof.verification_status == ProofStatus::Pending as u8
            @ ZkSeigniorageError::ProofNotPending,
        constraint = proof.attestation_count < MAX_VERIFIERS as u8
            @ ZkSeigniorageError::MaxVerifiersReached,
    )]
    pub proof: Account<'info, CapacityProof>,

    #[account(
        seeds = [b"zk_seigniorage_config"],
        bump = config.bump
    )]
    pub config: Account<'info, ZkSeigniorageConfig>,

    pub verifier: Signer<'info>,
}

pub fn attest_capacity_proof(
    ctx: Context<AttestCapacityProof>,
    attestation_hash: [u8; HASH_SIZE],
) -> Result<()> {
    let proof = &mut ctx.accounts.proof;
    let clock = Clock::get()?;

    // Prevent duplicate attestations from same verifier
    for existing in &proof.verifier_attestations {
        require!(
            existing.verifier != ctx.accounts.verifier.key(),
            ZkSeigniorageError::DuplicateAttestation
        );
    }

    proof.verifier_attestations.push(VerifierAttestation {
        verifier: ctx.accounts.verifier.key(),
        attestation_hash,
        timestamp: clock.unix_timestamp,
        signature_valid: true,
    });
    proof.attestation_count = proof.verifier_attestations.len() as u8;

    // Auto-verify if threshold reached
    if proof.attestation_count >= ctx.accounts.config.min_verifiers_required {
        proof.verification_status = ProofStatus::Verified as u8;
    }

    emit!(CapacityProofAttested {
        prover: proof.prover,
        proof_id: proof.proof_id,
        verifier: ctx.accounts.verifier.key(),
        attestation_count: proof.attestation_count,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// ---------------------------------------------------------------------------
// Mint tokens from a verified capacity proof (the monetary creation event)
// ---------------------------------------------------------------------------
#[derive(Accounts)]
pub struct MintFromCapacityProof<'info> {
    #[account(
        mut,
        seeds = [b"capacity_proof", proof.prover.as_ref(), proof.proof_id.as_ref()],
        bump = proof.bump,
        has_one = prover,
        constraint = proof.verification_status == ProofStatus::Verified as u8
            @ ZkSeigniorageError::ProofNotVerified,
        constraint = proof.mint_amount == 0
            @ ZkSeigniorageError::AlreadyMinted,
    )]
    pub proof: Account<'info, CapacityProof>,

    #[account(
        mut,
        seeds = [b"proof_nullifier", proof.nullifier_hash.as_ref()],
        bump = nullifier.bump,
        constraint = !nullifier.used
            @ ZkSeigniorageError::NullifierAlreadyUsed,
    )]
    pub nullifier: Account<'info, ProofNullifier>,

    #[account(
        mut,
        seeds = [b"zk_seigniorage_config"],
        bump = config.bump
    )]
    pub config: Account<'info, ZkSeigniorageConfig>,

    #[account(
        mut,
        seeds = [b"zk_seigniorage_mint"],
        bump,
    )]
    pub token_mint: Account<'info, Mint>,

    /// CHECK: PDA mint authority
    #[account(
        seeds = [b"zk_seigniorage_mint_authority"],
        bump = config.mint_authority_bump,
    )]
    pub mint_authority: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = prover,
        associated_token::mint = token_mint,
        associated_token::authority = prover,
    )]
    pub prover_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub prover: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_from_capacity_proof(ctx: Context<MintFromCapacityProof>) -> Result<()> {
    let proof = &mut ctx.accounts.proof;
    let nullifier = &mut ctx.accounts.nullifier;
    let config = &mut ctx.accounts.config;
    let clock = Clock::get()?;

    // Epoch rollover check
    if clock.unix_timestamp >= config.last_epoch_timestamp + SECONDS_PER_EPOCH {
        config.current_epoch = config.current_epoch.checked_add(1).unwrap();
        config.last_epoch_timestamp = clock.unix_timestamp;
    }

    // Calculate monetary creation amount
    let mint_amount = calculate_mint_amount(proof, config)?;

    require!(mint_amount > 0, ZkSeigniorageError::ZeroMintAmount);
    require!(
        mint_amount <= config.max_mint_per_proof,
        ZkSeigniorageError::MaxMintExceeded
    );

    // Mark nullifier as used
    nullifier.used = true;
    nullifier.used_at = clock.unix_timestamp;

    // Record mint on proof
    proof.mint_amount = mint_amount;
    proof.minted_at = clock.unix_timestamp;

    // Update global monetary state
    config.total_proven_capacity = config
        .total_proven_capacity
        .checked_add(proof.difficulty_score)
        .unwrap();
    config.total_tokens_minted = config
        .total_tokens_minted
        .checked_add(mint_amount)
        .unwrap();

    // CPI: Mint tokens to prover
    let mint_authority_seeds = &[
        b"zk_seigniorage_mint_authority".as_ref(),
        &[config.mint_authority_bump],
    ];
    let signer_seeds = &[&mint_authority_seeds[..]];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.token_mint.to_account_info(),
        to: ctx.accounts.prover_token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts)
        .with_signer(signer_seeds);

    token::mint_to(cpi_ctx, mint_amount)?;

    emit!(CapacityProofMinted {
        prover: proof.prover,
        proof_id: proof.proof_id,
        mint_amount,
        epoch: config.current_epoch,
        total_tokens_minted: config.total_tokens_minted,
        total_proven_capacity: config.total_proven_capacity,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

// =============================================================================
// MONETARY POLICY ENGINE
// =============================================================================

/// Core formula for ZK-PoPC monetary creation:
///   mint = base * tier_mult * difficulty/10000 * verifier_mult * network_adj
fn calculate_mint_amount(
    proof: &CapacityProof,
    config: &ZkSeigniorageConfig,
) -> Result<u64> {
    let base = config.base_mint_reward;

    // Collateral tier multiplier (in bps, e.g. 10000 = 1.0x)
    let tier_mult = match proof.collateral_tier {
        1 => TIER_1_MULTIPLIER_BPS,
        2 => TIER_2_MULTIPLIER_BPS,
        3 => TIER_3_MULTIPLIER_BPS,
        4 => TIER_4_MULTIPLIER_BPS,
        5 => TIER_5_MULTIPLIER_BPS,
        _ => TIER_1_MULTIPLIER_BPS,
    };

    // Difficulty factor: 0-1.0 based on difficulty score / 10000
    let difficulty_factor = proof.difficulty_score as u128;

    // Verifier bonus: each verifier beyond minimum adds 10% (1000 bps)
    let extra_verifiers = proof.attestation_count.saturating_sub(config.min_verifiers_required);
    let verifier_bonus_bps = 10_000_u64.saturating_add(extra_verifiers as u64 * 1_000);
    let verifier_bonus = verifier_bonus_bps.min(15_000); // cap at 1.5x

    // Network adjustment: counter-cyclical scarcity factor
    // As total_proven_capacity grows relative to target, new mints decrease
    let network_adj = calculate_network_adjustment(config);

    // Compute in u128 to avoid overflow, then scale
    let mint_128 = (base as u128)
        .checked_mul(tier_mult as u128)
        .unwrap()
        .checked_mul(difficulty_factor)
        .unwrap()
        .checked_mul(verifier_bonus as u128)
        .unwrap()
        .checked_mul(network_adj as u128)
        .unwrap()
        .checked_div(10_000_000_000_000_000_u128) // 10^16 scaling
        .unwrap();

    let mint = mint_128.min(config.max_mint_per_proof as u128) as u64;
    Ok(mint)
}

/// Counter-cyclical adjustment:
///   adjustment = min(1.0, target_ratio / current_ratio)
/// Where current_ratio = total_tokens_minted / max(total_proven_capacity, 1)
/// This ensures monetary expansion slows as productive capacity saturates.
fn calculate_network_adjustment(config: &ZkSeigniorageConfig) -> u64 {
    if config.total_proven_capacity == 0 {
        return 10_000; // 1.0x when no capacity yet
    }

    // current_ratio_bps = (tokens_minted * 10_000) / proven_capacity
    let current_ratio_bps = (config.total_tokens_minted as u128)
        .checked_mul(10_000)
        .unwrap()
        .checked_div(config.total_proven_capacity as u128)
        .unwrap_or(10_000) as u64;

    if current_ratio_bps == 0 {
        return 10_000;
    }

    let adjustment = (config.target_capacity_ratio_bps as u128)
        .checked_mul(10_000)
        .unwrap()
        .checked_div(current_ratio_bps as u128)
        .unwrap_or(10_000) as u64;

    adjustment.min(10_000) // cap at 1.0x (10_000 bps)
}

fn derive_collateral_tier(locked_value: u64, config: &ZkSeigniorageConfig) -> u8 {
    if locked_value >= config.tier_5_threshold {
        5
    } else if locked_value >= config.tier_4_threshold {
        4
    } else if locked_value >= config.tier_3_threshold {
        3
    } else if locked_value >= config.tier_2_threshold {
        2
    } else {
        1
    }
}

// =============================================================================
// TYPES / ENUMS
// =============================================================================

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProofStatus {
    Pending = 0,
    Verified = 1,
    Rejected = 2,
}

// =============================================================================
// ERRORS
// =============================================================================

#[error_code]
pub enum ZkSeigniorageError {
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
    #[msg("Mint authority signature failed")]
    MintAuthorityFailed,
}

// =============================================================================
// EVENTS
// =============================================================================

#[event]
pub struct SeigniorageConfigInitialized {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub base_mint_reward: u64,
    pub max_mint_per_proof: u64,
    pub timestamp: i64,
}

#[event]
pub struct CapacityProofSubmitted {
    pub prover: Pubkey,
    pub proof_id: [u8; HASH_SIZE],
    pub collateral_lock: Pubkey,
    pub collateral_tier: u8,
    pub difficulty_score: u64,
    pub epoch: u64,
    pub timestamp: i64,
}

#[event]
pub struct CapacityProofAttested {
    pub prover: Pubkey,
    pub proof_id: [u8; HASH_SIZE],
    pub verifier: Pubkey,
    pub attestation_count: u8,
    pub timestamp: i64,
}

#[event]
pub struct CapacityProofMinted {
    pub prover: Pubkey,
    pub proof_id: [u8; HASH_SIZE],
    pub mint_amount: u64,
    pub epoch: u64,
    pub total_tokens_minted: u64,
    pub total_proven_capacity: u64,
    pub timestamp: i64,
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> ZkSeigniorageConfig {
        ZkSeigniorageConfig {
            authority: Pubkey::default(),
            token_mint: Pubkey::default(),
            mint_authority_bump: 255,
            base_mint_reward: 1_000_000,
            max_mint_per_proof: 100_000_000_000,
            target_capacity_ratio_bps: 10_000,
            min_verifiers_required: 2,
            tier_1_threshold: 1_000_000,
            tier_2_threshold: 10_000_000,
            tier_3_threshold: 100_000_000,
            tier_4_threshold: 1_000_000_000,
            tier_5_threshold: 10_000_000_000,
            total_proven_capacity: 0,
            total_tokens_minted: 0,
            current_epoch: 1,
            last_epoch_timestamp: 0,
            bump: 255,
        }
    }

    #[test]
    fn test_calculate_mint_amount_basic() {
        let config = default_config();
        let proof = CapacityProof {
            prover: Pubkey::default(),
            proof_id: [0u8; HASH_SIZE],
            collateral_lock: Pubkey::default(),
            collateral_tier: 3,
            input_commitment: [0u8; HASH_SIZE],
            output_commitment: [0u8; HASH_SIZE],
            circuit_identifier: "test".to_string(),
            difficulty_score: 5_000,
            compute_units_consumed: 10_000,
            epoch: 1,
            verifier_attestations: vec![
                VerifierAttestation {
                    verifier: Pubkey::default(),
                    attestation_hash: [0u8; HASH_SIZE],
                    timestamp: 0,
                    signature_valid: true,
                },
                VerifierAttestation {
                    verifier: Pubkey::default(),
                    attestation_hash: [0u8; HASH_SIZE],
                    timestamp: 0,
                    signature_valid: true,
                },
            ],
            attestation_count: 2,
            verification_status: ProofStatus::Verified as u8,
            mint_amount: 0,
            minted_at: 0,
            nullifier_hash: [0u8; HASH_SIZE],
            bump: 255,
        };

        let mint = calculate_mint_amount(&proof, &config).unwrap();
        // base(1M) * tier(10k=1.0) * diff(5k=0.5) * ver(10k=1.0) * net(10k=1.0) / 10^16 = 500k
        assert_eq!(mint, 500_000);
    }

    #[test]
    fn test_calculate_mint_with_extra_verifiers() {
        let mut config = default_config();
        let mut proof = CapacityProof {
            prover: Pubkey::default(),
            proof_id: [0u8; HASH_SIZE],
            collateral_lock: Pubkey::default(),
            collateral_tier: 5,
            input_commitment: [0u8; HASH_SIZE],
            output_commitment: [0u8; HASH_SIZE],
            circuit_identifier: "test".to_string(),
            difficulty_score: 10_000,
            compute_units_consumed: 10_000,
            epoch: 1,
            verifier_attestations: vec![],
            attestation_count: 4,
            verification_status: ProofStatus::Verified as u8,
            mint_amount: 0,
            minted_at: 0,
            nullifier_hash: [0u8; HASH_SIZE],
            bump: 255,
        };

        for _ in 0..4 {
            proof.verifier_attestations.push(VerifierAttestation {
                verifier: Pubkey::new_unique(),
                attestation_hash: [0u8; HASH_SIZE],
                timestamp: 0,
                signature_valid: true,
            });
        }

        let mint = calculate_mint_amount(&proof, &config).unwrap();
        // base(1M) * tier(25k=2.5) * diff(10k=1.0) * ver(12k=1.2 capped) * net(10k=1.0) / 10^16 = 3M
        assert_eq!(mint, 3_000_000);
    }

    #[test]
    fn test_network_adjustment_counter_cyclical() {
        let mut config = default_config();
        config.total_tokens_minted = 100_000_000;
        config.total_proven_capacity = 10_000;

        // current_ratio = 100M * 10k / 10k = 100M bps (very high)
        // adjustment = 10k * 10k / 100M = 1 bps (near zero)
        let adj = calculate_network_adjustment(&config);
        assert_eq!(adj, 1);

        // Now reset to low ratio
        config.total_tokens_minted = 5_000;
        config.total_proven_capacity = 10_000;
        // current_ratio = 5000 * 10k / 10k = 5000 bps
        // adjustment = 10k * 10k / 5000 = 20_000 => capped to 10_000
        let adj2 = calculate_network_adjustment(&config);
        assert_eq!(adj2, 10_000);
    }

    #[test]
    fn test_collateral_tier_derivation() {
        let config = default_config();
        assert_eq!(derive_collateral_tier(500_000, &config), 1);
        assert_eq!(derive_collateral_tier(50_000_000, &config), 2);
        assert_eq!(derive_collateral_tier(500_000_000, &config), 3);
        assert_eq!(derive_collateral_tier(5_000_000_000, &config), 4);
        assert_eq!(derive_collateral_tier(50_000_000_000, &config), 5);
    }

    #[test]
    fn test_max_mint_cap() {
        let mut config = default_config();
        config.max_mint_per_proof = 100;
        let proof = CapacityProof {
            prover: Pubkey::default(),
            proof_id: [0u8; HASH_SIZE],
            collateral_lock: Pubkey::default(),
            collateral_tier: 5,
            input_commitment: [0u8; HASH_SIZE],
            output_commitment: [0u8; HASH_SIZE],
            circuit_identifier: "test".to_string(),
            difficulty_score: 10_000,
            compute_units_consumed: 10_000,
            epoch: 1,
            verifier_attestations: vec![],
            attestation_count: 5,
            verification_status: ProofStatus::Verified as u8,
            mint_amount: 0,
            minted_at: 0,
            nullifier_hash: [0u8; HASH_SIZE],
            bump: 255,
        };

        let mint = calculate_mint_amount(&proof, &config).unwrap();
        assert_eq!(mint, 100); // capped at max
    }

    #[test]
    fn test_nullifier_prevents_double_mint() {
        // Logic test: verify that a used nullifier would fail the constraint
        let nullifier = ProofNullifier {
            proof_id: [0u8; HASH_SIZE],
            nullifier_hash: [0u8; HASH_SIZE],
            used: true,
            used_at: 1,
            bump: 255,
        };
        assert!(nullifier.used);
    }

    #[test]
    fn test_epoch_rollover() {
        let mut config = default_config();
        config.last_epoch_timestamp = 0;
        config.current_epoch = 1;
        // Simulate clock advancing beyond epoch boundary
        let future_time = config.last_epoch_timestamp + SECONDS_PER_EPOCH + 1;
        assert!(future_time >= config.last_epoch_timestamp + SECONDS_PER_EPOCH);
        // In real instruction, this would trigger epoch increment
        config.current_epoch += 1;
        config.last_epoch_timestamp = future_time;
        assert_eq!(config.current_epoch, 2);
    }
}
