//! Compute Resource Collateral Locking on Solana
//! 
//! This module provides on-chain tracking of compute resource collateral locks
//! for token minting and network participation.

use anchor_lang::prelude::*;

/// Collateral lock account tracking compute resources locked as collateral
#[account]
#[derive(Default)]
pub struct CollateralLock {
    /// Unique lock identifier
    pub lock_id: [u8; 16],
    /// Node/wallet that locked the resources
    pub locker: Pubkey,
    /// CPU cores locked
    pub cpu_cores_locked: u8,
    /// Memory locked in GB (scaled by 100 for 2 decimal precision)
    pub memory_gb_locked: u16, // x100 for precision
    /// Whether GPU is locked
    pub gpu_locked: bool,
    /// Collateral value in base token units
    pub collateral_value: u64,
    /// Lock start timestamp
    pub lock_start_ts: i64,
    /// Lock duration in seconds
    pub lock_duration_sec: u64,
    /// Purpose of the lock (token_mint, zk_compute, network_task)
    pub lock_purpose: u8, // 0=token_mint, 1=zk_compute, 2=network_task
    /// Associated transaction ID (if any)
    pub associated_tx: Option<[u8; 32]>,
    /// Lock status (0=locked, 1=unlocked, 2=expired)
    pub lock_status: u8,
    /// Bump seed
    pub bump: u8,
}

impl CollateralLock {
    pub const LEN: usize = 8 + // discriminator
        16 + // lock_id
        32 + // locker
        1 + // cpu_cores_locked
        2 + // memory_gb_locked
        1 + // gpu_locked
        8 + // collateral_value
        8 + // lock_start_ts
        8 + // lock_duration_sec
        1 + // lock_purpose
        33 + // associated_tx (Option<[u8; 32]>)
        1 + // lock_status
        1; // bump
    
    pub const PURPOSE_TOKEN_MINT: u8 = 0;
    pub const PURPOSE_ZK_COMPUTE: u8 = 1;
    pub const PURPOSE_NETWORK_TASK: u8 = 2;
    
    pub const STATUS_LOCKED: u8 = 0;
    pub const STATUS_UNLOCKED: u8 = 1;
    pub const STATUS_EXPIRED: u8 = 2;
}

/// Global collateral configuration
#[account]
#[derive(Default)]
pub struct CollateralConfig {
    /// Authority that can manage collateral configuration
    pub authority: Pubkey,
    /// Maximum CPU lock ratio (basis points, 10000 = 100%)
    pub max_cpu_lock_bps: u16,
    /// Maximum memory lock ratio (basis points)
    pub max_memory_lock_bps: u16,
    /// Minimum lock duration (seconds)
    pub min_lock_duration_sec: u64,
    /// Maximum lock duration (seconds)
    pub max_lock_duration_sec: u64,
    /// Collateral multiplier for token minting (basis points)
    pub collateral_multiplier_bps: u16,
    /// Bump seed
    pub bump: u8,
}

impl CollateralConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        2 + // max_cpu_lock_bps
        2 + // max_memory_lock_bps
        8 + // min_lock_duration_sec
        8 + // max_lock_duration_sec
        2 + // collateral_multiplier_bps
        1; // bump
}

/// Note: Error codes are defined in the main MembraTokenomicsError enum in lib.rs

/// Calculate collateral value from locked resources
pub fn calculate_collateral_value(
    cpu_cores: u8,
    memory_gb_scaled: u16, // x100
    gpu_locked: bool,
    duration_sec: u64,
    multiplier_bps: u16,
) -> Result<u64> {
    // Base values per unit per hour (in lamports)
    const CPU_VALUE_PER_CORE_HOUR: u64 = 5_000_000; // 0.005 SOL per core per hour
    const MEMORY_VALUE_PER_GB_HOUR: u64 = 2_000_000; // 0.002 SOL per GB per hour
    const GPU_VALUE_PER_HOUR: u64 = 200_000_000; // 0.2 SOL per hour
    
    let memory_gb = memory_gb_scaled as u64 / 100;
    
    // Calculate hourly value
    let cpu_hourly = (cpu_cores as u64).checked_mul(CPU_VALUE_PER_CORE_HOUR).unwrap();
    let memory_hourly = memory_gb.checked_mul(MEMORY_VALUE_PER_GB_HOUR).unwrap();
    let gpu_hourly = if gpu_locked { GPU_VALUE_PER_HOUR } else { 0 };
    
    let hourly_value = cpu_hourly
        .checked_add(memory_hourly)
        .unwrap()
        .checked_add(gpu_hourly)
        .unwrap();
    
    // Calculate total value: hourly_value * duration_sec / 3600
    let total_value = hourly_value
        .checked_mul(duration_sec)
        .unwrap()
        .checked_div(3600)
        .unwrap();
    
    // Apply collateral multiplier
    let collateral_value = total_value
        .checked_mul(multiplier_bps as u64)
        .unwrap()
        .checked_div(10_000)
        .unwrap();
    
    Ok(collateral_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_collateral_value_calculation() {
        // Test: 2 cores, 4GB RAM, no GPU, 1 hour, 100% multiplier
        let value = calculate_collateral_value(2, 400, false, 3600, 10000).unwrap();
        
        // Expected: (2 * 0.005 + 4 * 0.002) * 1 = 0.018 SOL = 18,000,000 lamports
        assert!(value > 17_000_000 && value < 19_000_000);
    }
    
    #[test]
    fn test_collateral_with_gpu() {
        // Test: 1 core, 2GB RAM, with GPU, 1 hour
        let value = calculate_collateral_value(1, 200, true, 3600, 10000).unwrap();
        
        // Should be significantly higher due to GPU
        let value_no_gpu = calculate_collateral_value(1, 200, false, 3600, 10000).unwrap();
        assert!(value > value_no_gpu + 100_000_000); // GPU adds at least 0.1 SOL
    }
    
    #[test]
    fn test_collateral_multiplier() {
        // Test multiplier effect
        let base_value = calculate_collateral_value(1, 100, false, 3600, 10000).unwrap();
        let doubled_value = calculate_collateral_value(1, 100, false, 3600, 20000).unwrap();
        
        assert!(doubled_value > base_value * 2 - 1000); // Allow small rounding error
        assert!(doubled_value < base_value * 2 + 1000);
    }
}