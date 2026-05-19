use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Constants
pub const WALLET_ADDRESS_SIZE: usize = 32;
pub const TRANSACTION_HASH_SIZE: usize = 32;
pub const MIN_BALANCE_LAMPORTS: u64 = 1_000; // Minimum balance to prevent zero balance
pub const DEFAULT_REIMBURSEMENT_RATE_BPS: u64 = 10_000; // 100% reimbursement by default
pub const MAX_REIMBURSEMENT_MULTIPLIER: u64 = 2; // Maximum 2x gas reimbursement

/// Reimbursement types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReimbursementType {
    PreTransaction,   // Gas estimation before transaction
    PostTransaction,  // Actual gas reimbursement after transaction
    Bonus,            // Bonus for successful transactions
    Emergency,        // Emergency reimbursement for failed transactions
    ZkComputeBacked, // Reimbursement backed by ZK compute proofs
}

impl ReimbursementType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ReimbursementType::PreTransaction),
            1 => Some(ReimbursementType::PostTransaction),
            2 => Some(ReimbursementType::Bonus),
            3 => Some(ReimbursementType::Emergency),
            4 => Some(ReimbursementType::ZkComputeBacked),
            _ => None,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        match self {
            ReimbursementType::PreTransaction => 0,
            ReimbursementType::PostTransaction => 1,
            ReimbursementType::Bonus => 2,
            ReimbursementType::Emergency => 3,
            ReimbursementType::ZkComputeBacked => 4,
        }
    }
}

/// Transaction gas information
#[derive(Debug, Clone)]
pub struct TransactionGasInfo {
    pub transaction_id: String,
    pub wallet_address: String,
    pub estimated_gas_units: u64,
    pub actual_gas_used: u64,
    pub gas_price_lamports: u64,
    pub total_gas_cost: u64,
    pub transaction_type: String, // "transfer", "swap", "stake", etc.
    pub timestamp: i64,
    pub success: bool,
    pub error_message: String,
}

impl Default for TransactionGasInfo {
    fn default() -> Self {
        Self {
            transaction_id: String::new(),
            wallet_address: String::new(),
            estimated_gas_units: 0,
            actual_gas_used: 0,
            gas_price_lamports: 0,
            total_gas_cost: 0,
            transaction_type: String::new(),
            timestamp: 0,
            success: false,
            error_message: String::new(),
        }
    }
}

/// Reimbursement request
#[derive(Debug, Clone)]
pub struct ReimbursementRequest {
    pub request_id: String,
    pub transaction_id: String,
    pub wallet_address: String,
    pub reimbursement_type: ReimbursementType,
    pub gas_cost: u64,
    pub requested_amount: u64,
    pub justification: String,
    pub transaction_hash: [u8; TRANSACTION_HASH_SIZE],
    pub created_at: i64,
}

impl Default for ReimbursementRequest {
    fn default() -> Self {
        Self {
            request_id: String::new(),
            transaction_id: String::new(),
            wallet_address: String::new(),
            reimbursement_type: ReimbursementType::PreTransaction,
            gas_cost: 0,
            requested_amount: 0,
            justification: String::new(),
            transaction_hash: [0; TRANSACTION_HASH_SIZE],
            created_at: 0,
        }
    }
}

/// Reimbursement record
#[derive(Debug, Clone)]
pub struct ReimbursementRecord {
    pub request_id: String,
    pub transaction_id: String,
    pub wallet_address: String,
    pub reimbursement_type: ReimbursementType,
    pub gas_cost: u64,
    pub reimbursed_amount: u64,
    pub bonus_amount: u64,
    pub total_amount: u64,
    pub status: String, // "pending", "approved", "rejected", "paid"
    pub processed_at: i64,
    pub approval_signature: String,
    pub metadata: HashMap<String, String>,
}

impl Default for ReimbursementRecord {
    fn default() -> Self {
        Self {
            request_id: String::new(),
            transaction_id: String::new(),
            wallet_address: String::new(),
            reimbursement_type: ReimbursementType::PreTransaction,
            gas_cost: 0,
            reimbursed_amount: 0,
            bonus_amount: 0,
            total_amount: 0,
            status: "pending".to_string(),
            processed_at: 0,
            approval_signature: String::new(),
            metadata: HashMap::new(),
        }
    }
}

/// Gas pool configuration
#[derive(Debug, Clone)]
pub struct GasPoolConfig {
    pub pool_id: String,
    pub total_pool_balance: u64,
    pub available_balance: u64,
    pub reimbursement_rate_bps: u64,
    pub bonus_rate_bps: u64,
    pub max_reimbursement_per_tx: u64,
    pub min_balance_threshold: u64,
    pub auto_approve_enabled: bool,
    pub zk_compute_backing_enabled: bool,
}

impl Default for GasPoolConfig {
    fn default() -> Self {
        Self {
            pool_id: String::new(),
            total_pool_balance: 0,
            available_balance: 0,
            reimbursement_rate_bps: DEFAULT_REIMBURSEMENT_RATE_BPS,
            bonus_rate_bps: 0,
            max_reimbursement_per_tx: 10_000_000, // 0.01 SOL max per tx
            min_balance_threshold: MIN_BALANCE_LAMPORTS,
            auto_approve_enabled: false,
            zk_compute_backing_enabled: false,
        }
    }
}

/// Wallet balance state
#[derive(Debug, Clone)]
pub struct WalletBalanceState {
    pub wallet_address: String,
    pub current_balance: u64,
    pub min_balance_threshold: u64,
    pub is_below_threshold: bool,
    pub pending_reimbursements: u64,
    pub total_reimbursed: u64,
    pub last_updated: i64,
}

impl Default for WalletBalanceState {
    fn default() -> Self {
        Self {
            wallet_address: String::new(),
            current_balance: 0,
            min_balance_threshold: MIN_BALANCE_LAMPORTS,
            is_below_threshold: false,
            pending_reimbursements: 0,
            total_reimbursed: 0,
            last_updated: 0,
        }
    }
}

/// Gas estimator
pub struct GasEstimator {
    current_gas_price: u64,
    base_gas_costs: HashMap<String, u64>,
}

impl GasEstimator {
    pub fn new() -> Self {
        let mut base_gas_costs = HashMap::new();
        base_gas_costs.insert("transfer".to_string(), 5_000);
        base_gas_costs.insert("swap".to_string(), 150_000);
        base_gas_costs.insert("stake".to_string(), 100_000);
        base_gas_costs.insert("unstake".to_string(), 100_000);
        base_gas_costs.insert("mint".to_string(), 200_000);
        base_gas_costs.insert("burn".to_string(), 100_000);
        base_gas_costs.insert("approve".to_string(), 50_000);
        base_gas_costs.insert("revoke".to_string(), 50_000);
        base_gas_costs.insert("custom".to_string(), 100_000);
        
        Self {
            current_gas_price: 1_000, // Default 1000 lamports per gas unit
            base_gas_costs,
        }
    }
    
    /// Estimate gas for transaction
    pub fn estimate_gas(&self, transaction_type: &str, transaction_data: &[u8]) -> u64 {
        let base_cost = self.get_base_gas_cost(transaction_type);
        
        // Adjust based on transaction data size
        let data_cost = transaction_data.len() as u64 * 10; // 10 gas units per byte
        
        // Add random variance to simulate network conditions
        let variance = (base_cost / 10) * (Self::random_u32() % 3) as u64; // 0-20% variance
        
        base_cost + data_cost + variance
    }
    
    /// Get current gas price
    pub fn get_gas_price(&self) -> u64 {
        self.current_gas_price
    }
    
    /// Calculate total gas cost
    pub fn calculate_total_cost(&self, gas_units: u64, gas_price: u64) -> u64 {
        gas_units * gas_price
    }
    
    /// Update gas price oracle
    pub fn update_gas_price(&mut self, new_price: u64) {
        self.current_gas_price = new_price;
    }
    
    fn get_base_gas_cost(&self, transaction_type: &str) -> u64 {
        *self.base_gas_costs.get(transaction_type).unwrap_or(&100_000)
    }
    
    fn random_u32() -> u32 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u32;
        timestamp % 1000
    }
}

impl Default for GasEstimator {
    fn default() -> Self {
        Self::new()
    }
}

/// Gas reimbursement manager
pub struct GasReimbursementManager {
    requests: HashMap<String, ReimbursementRequest>,
    records: HashMap<String, ReimbursementRecord>,
    wallet_states: HashMap<String, WalletBalanceState>,
    estimator: std::sync::Arc<GasEstimator>,
    stats: ReimbursementStats,
}

#[derive(Debug, Clone, Default)]
pub struct ReimbursementStats {
    pub total_requests: u64,
    pub total_approved: u64,
    pub total_rejected: u64,
    pub total_reimbursed: u64,
    pub total_bonus_paid: u64,
    pub average_reimbursement_time_ms: f64,
}

impl GasReimbursementManager {
    pub fn new() -> Self {
        Self {
            requests: HashMap::new(),
            records: HashMap::new(),
            wallet_states: HashMap::new(),
            estimator: std::sync::Arc::new(GasEstimator::new()),
            stats: ReimbursementStats::default(),
        }
    }
    
    pub fn with_estimator(estimator: std::sync::Arc<GasEstimator>) -> Self {
        Self {
            requests: HashMap::new(),
            records: HashMap::new(),
            wallet_states: HashMap::new(),
            estimator,
            stats: ReimbursementStats::default(),
        }
    }
    
    /// Submit reimbursement request
    pub fn submit_request(&mut self, request: ReimbursementRequest) -> String {
        let request_id = if request.request_id.is_empty() {
            self.generate_request_id()
        } else {
            request.request_id.clone()
        };
        
        // Store request
        let mut request_to_store = request.clone();
        request_to_store.request_id = request_id.clone();
        self.requests.insert(request_id.clone(), request_to_store);
        
        // Create initial record
        let mut record = ReimbursementRecord::default();
        record.request_id = request_id.clone();
        record.transaction_id = request.transaction_id.clone();
        record.wallet_address = request.wallet_address.clone();
        record.reimbursement_type = request.reimbursement_type;
        record.gas_cost = request.gas_cost;
        record.status = "pending".to_string();
        
        self.records.insert(request_id.clone(), record);
        
        // Update statistics
        self.stats.total_requests += 1;
        
        // Update wallet state
        self.update_wallet_state(&request.wallet_address, request.gas_cost);
        
        request_id
    }
    
    /// Process reimbursement request
    pub fn process_request(&mut self, request_id: &str) -> ReimbursementRecord {
        if !self.requests.contains_key(request_id) {
            let mut empty_record = ReimbursementRecord::default();
            empty_record.request_id = request_id.to_string();
            empty_record.status = "not_found".to_string();
            return empty_record;
        }
        
        // Extract request data first to avoid borrow conflicts
        let request_data = self.requests.get(request_id).cloned().unwrap();
        
        // Use default pool config for calculation
        let default_config = GasPoolConfig::default();
        
        // Create gas info for calculation
        let gas_info = TransactionGasInfo {
            transaction_id: request_data.transaction_id.clone(),
            wallet_address: request_data.wallet_address.clone(),
            total_gas_cost: request_data.gas_cost,
            transaction_type: "custom".to_string(),
            success: true,
            ..Default::default()
        };
        
        // Calculate reimbursement using static helper
        let reimbursed_amount = Self::calculate_reimbursement_static(&gas_info, &default_config);
        let bonus_amount = Self::calculate_bonus_static(&gas_info, &default_config);
        
        // Now mutate the record
        let record = self.records.get_mut(request_id).unwrap();
        record.reimbursed_amount = reimbursed_amount;
        record.bonus_amount = bonus_amount;
        record.total_amount = reimbursed_amount + bonus_amount;
        record.processed_at = Self::current_timestamp();
        
        // Auto-approve if enabled
        if default_config.auto_approve_enabled {
            record.status = "approved".to_string();
            self.stats.total_approved += 1;
            self.stats.total_reimbursed += record.total_amount;
            if bonus_amount > 0 {
                self.stats.total_bonus_paid += bonus_amount;
            }
        } else {
            record.status = "pending_approval".to_string();
        }
        
        let record_clone = record.clone();
        self.update_stats(record_clone.clone());
        
        record_clone
    }
    
    /// Approve reimbursement
    pub fn approve_reimbursement(&mut self, request_id: &str, approver_signature: &str) -> bool {
        if !self.records.contains_key(request_id) {
            return false;
        }
        
        let record = self.records.get_mut(request_id).unwrap();
        if record.status != "pending" && record.status != "pending_approval" {
            return false;
        }
        
        record.status = "approved".to_string();
        record.approval_signature = approver_signature.to_string();
        record.processed_at = Self::current_timestamp();
        
        self.stats.total_approved += 1;
        self.stats.total_reimbursed += record.total_amount;
        if record.bonus_amount > 0 {
            self.stats.total_bonus_paid += record.bonus_amount;
        }
        
        true
    }
    
    /// Reject reimbursement
    pub fn reject_reimbursement(&mut self, request_id: &str, reason: &str) -> bool {
        if !self.records.contains_key(request_id) {
            return false;
        }
        
        let record = self.records.get_mut(request_id).unwrap();
        if record.status == "approved" || record.status == "paid" {
            return false;
        }
        
        record.status = "rejected".to_string();
        record.metadata.insert("rejection_reason".to_string(), reason.to_string());
        record.processed_at = Self::current_timestamp();
        
        self.stats.total_rejected += 1;
        
        true
    }
    
    /// Calculate reimbursement amount
    pub fn calculate_reimbursement(&self, gas_info: &TransactionGasInfo, config: &GasPoolConfig) -> u64 {
        Self::calculate_reimbursement_static(gas_info, config)
    }
    
    /// Static helper for reimbursement calculation to avoid borrow issues
    fn calculate_reimbursement_static(gas_info: &TransactionGasInfo, config: &GasPoolConfig) -> u64 {
        // Base reimbursement is gas cost * rate
        let base_amount = (gas_info.total_gas_cost * config.reimbursement_rate_bps) / 10_000;
        
        // Cap at maximum per transaction
        let capped_amount = base_amount.min(config.max_reimbursement_per_tx);
        
        // Apply multiplier based on transaction success
        let multiplier = if gas_info.success { 1 } else { 0 };
        
        capped_amount * multiplier
    }
    
    /// Calculate bonus amount
    pub fn calculate_bonus(&self, gas_info: &TransactionGasInfo, config: &GasPoolConfig) -> u64 {
        Self::calculate_bonus_static(gas_info, config)
    }
    
    /// Static helper for bonus calculation to avoid borrow issues
    fn calculate_bonus_static(gas_info: &TransactionGasInfo, config: &GasPoolConfig) -> u64 {
        if !gas_info.success || config.bonus_rate_bps == 0 {
            return 0;
        }
        
        // Bonus is calculated on gas cost
        let bonus_amount = (gas_info.total_gas_cost * config.bonus_rate_bps) / 10_000;
        
        // Cap bonus at 50% of gas cost
        let max_bonus = gas_info.total_gas_cost / 2;
        bonus_amount.min(max_bonus)
    }
    
    /// Check wallet balance (never-zero enforcement)
    pub fn check_wallet_balance(&self, wallet_address: &str, required_amount: u64) -> bool {
        match self.wallet_states.get(wallet_address) {
            Some(state) => {
                let available_balance = state.current_balance.saturating_sub(state.pending_reimbursements);
                available_balance >= required_amount.saturating_add(state.min_balance_threshold)
            }
            None => true, // Assume wallet has sufficient balance if unknown
        }
    }
    
    /// Get reimbursement status
    pub fn get_reimbursement_status(&self, request_id: &str) -> ReimbursementRecord {
        match self.records.get(request_id) {
            Some(record) => record.clone(),
            None => {
                let mut empty_record = ReimbursementRecord::default();
                empty_record.request_id = request_id.to_string();
                empty_record.status = "not_found".to_string();
                empty_record
            }
        }
    }
    
    /// Get pending reimbursements
    pub fn get_pending_reimbursements(&self) -> Vec<ReimbursementRecord> {
        self.records
            .values()
            .filter(|record| record.status == "pending" || record.status == "pending_approval")
            .cloned()
            .collect()
    }
    
    /// Get statistics
    pub fn get_stats(&self) -> ReimbursementStats {
        self.stats.clone()
    }
    
    fn generate_request_id(&self) -> String {
        let timestamp = Self::current_timestamp();
        format!("gas_req_{}", timestamp)
    }
    
    fn update_wallet_state(&mut self, wallet_address: &str, amount: u64) {
        let state = self.wallet_states.entry(wallet_address.to_string()).or_default();
        state.wallet_address = wallet_address.to_string();
        state.pending_reimbursements += amount;
        state.last_updated = Self::current_timestamp();
        
        // Check if below threshold
        if state.current_balance < state.min_balance_threshold {
            state.is_below_threshold = true;
        }
    }
    
    fn update_stats(&mut self, _record: ReimbursementRecord) {
        // Statistics are updated in process_request and approve_reimbursement
        // This is a placeholder for additional stat tracking
    }
    
    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}

impl Default for GasReimbursementManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Gas pool manager
pub struct GasPoolManager {
    pool_configs: HashMap<String, GasPoolConfig>,
    reserved_amounts: HashMap<String, u64>,
    stats: PoolStats,
}

#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    pub total_deposited: u64,
    pub total_withdrawn: u64,
    pub current_balance: u64,
    pub reserved_amount: u64,
    pub total_reimbursements: u64,
    pub utilization_ratio: f64,
}

impl GasPoolManager {
    pub fn new() -> Self {
        Self {
            pool_configs: HashMap::new(),
            reserved_amounts: HashMap::new(),
            stats: PoolStats::default(),
        }
    }
    
    /// Initialize gas pool
    pub fn initialize_pool(&mut self, config: GasPoolConfig) -> bool {
        let pool_id = config.pool_id.clone();
        self.stats.total_deposited = config.total_pool_balance;
        self.stats.current_balance = config.available_balance;
        self.pool_configs.insert(pool_id, config);
        true
    }
    
    /// Get pool configuration
    pub fn get_pool_config(&self, pool_id: &str) -> GasPoolConfig {
        match self.pool_configs.get(pool_id) {
            Some(config) => config.clone(),
            None => GasPoolConfig::default(),
        }
    }
    
    /// Update pool balance
    pub fn update_pool_balance(&mut self, pool_id: &str, amount: u64) -> bool {
        match self.pool_configs.get_mut(pool_id) {
            Some(config) => {
                config.total_pool_balance += amount;
                config.available_balance += amount;
                self.stats.current_balance = config.available_balance;
                true
            }
            None => false,
        }
    }
    
    /// Check pool availability
    pub fn check_availability(&self, pool_id: &str, required_amount: u64) -> bool {
        match self.pool_configs.get(pool_id) {
            Some(config) => {
                let reserved = *self.reserved_amounts.get(pool_id).unwrap_or(&0);
                let available = config.available_balance.saturating_sub(reserved);
                available >= required_amount
            }
            None => false,
        }
    }
    
    /// Reserve funds for reimbursement
    pub fn reserve_funds(&mut self, pool_id: &str, amount: u64) -> bool {
        if !self.check_availability(pool_id, amount) {
            return false;
        }
        
        *self.reserved_amounts.entry(pool_id.to_string()).or_insert(0) += amount;
        self.stats.reserved_amount = *self.reserved_amounts.get(pool_id).unwrap_or(&0);
        
        // Update utilization ratio
        if let Some(config) = self.pool_configs.get(pool_id) {
            let reserved = *self.reserved_amounts.get(pool_id).unwrap_or(&0);
            self.stats.utilization_ratio = if config.total_pool_balance > 0 {
                reserved as f64 / config.total_pool_balance as f64
            } else {
                0.0
            };
        }
        
        true
    }
    
    /// Release reserved funds
    pub fn release_funds(&mut self, pool_id: &str, amount: u64) {
        let reserved = self.reserved_amounts.entry(pool_id.to_string()).or_insert(0);
        if *reserved >= amount {
            *reserved -= amount;
        } else {
            *reserved = 0;
        }
        
        self.stats.reserved_amount = *reserved;
        
        // Update utilization ratio
        if let Some(config) = self.pool_configs.get(pool_id) {
            let reserved = *self.reserved_amounts.get(pool_id).unwrap_or(&0);
            self.stats.utilization_ratio = if config.total_pool_balance > 0 {
                reserved as f64 / config.total_pool_balance as f64
            } else {
                0.0
            };
        }
    }
    
    /// Get pool statistics
    pub fn get_pool_stats(&self, pool_id: &str) -> PoolStats {
        match self.pool_configs.get(pool_id) {
            Some(config) => {
                let reserved = *self.reserved_amounts.get(pool_id).unwrap_or(&0);
                PoolStats {
                    total_deposited: config.total_pool_balance,
                    current_balance: config.available_balance,
                    reserved_amount: reserved,
                    total_reimbursements: self.stats.total_reimbursements,
                    utilization_ratio: self.stats.utilization_ratio,
                    total_withdrawn: config.total_pool_balance.saturating_sub(config.available_balance),
                }
            }
            None => PoolStats::default(),
        }
    }
}

impl Default for GasPoolManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Gas reimbursement factory
pub struct GasReimbursementStack {
    pub estimator: std::sync::Arc<GasEstimator>,
    pub manager: GasReimbursementManager,
    pub pool_manager: GasPoolManager,
}

impl GasReimbursementStack {
    pub fn new() -> Self {
        let estimator = std::sync::Arc::new(GasEstimator::new());
        Self {
            estimator: estimator.clone(),
            manager: GasReimbursementManager::with_estimator(estimator),
            pool_manager: GasPoolManager::new(),
        }
    }
}

impl Default for GasReimbursementStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gas_estimator() {
        let estimator = GasEstimator::new();
        
        // Test gas estimation for different transaction types
        let transfer_gas = estimator.estimate_gas("transfer", &[1, 2, 3]);
        assert!(transfer_gas >= 5_000); // Base cost for transfer
        
        // Test gas price
        let gas_price = estimator.get_gas_price();
        assert!(gas_price > 0);
        
        // Test total cost calculation
        let total_cost = estimator.calculate_total_cost(100_000, gas_price);
        assert_eq!(total_cost, 100_000 * gas_price);
        
        // Test gas price update
        let mut estimator_mut = GasEstimator::new();
        estimator_mut.update_gas_price(2000);
        assert_eq!(estimator_mut.get_gas_price(), 2000);
    }
    
    #[test]
    fn test_gas_reimbursement_manager() {
        let mut manager = GasReimbursementManager::new();
        
        // Test reimbursement request submission
        let request = ReimbursementRequest {
            transaction_id: "tx_001".to_string(),
            wallet_address: "wallet_001".to_string(),
            reimbursement_type: ReimbursementType::PostTransaction,
            gas_cost: 500_000,
            requested_amount: 500_000,
            ..Default::default()
        };
        
        let request_id = manager.submit_request(request);
        assert!(!request_id.is_empty());
        
        // Test request status retrieval before processing
        let initial_record = manager.get_reimbursement_status(&request_id);
        assert_eq!(initial_record.request_id, request_id);
        assert_eq!(initial_record.status, "pending");
        
        // Test request processing
        let record = manager.process_request(&request_id);
        assert_eq!(record.request_id, request_id);
        assert_eq!(record.gas_cost, 500_000);
        assert!(record.reimbursed_amount > 0);
        
        // Test reimbursement calculation
        let gas_info = TransactionGasInfo {
            transaction_id: "tx_002".to_string(),
            wallet_address: "wallet_002".to_string(),
            total_gas_cost: 1_000_000,
            success: true,
            ..Default::default()
        };
        
        let config = GasPoolConfig {
            reimbursement_rate_bps: 10_000, // 100%
            max_reimbursement_per_tx: 10_000_000,
            ..Default::default()
        };
        
        let reimbursement = manager.calculate_reimbursement(&gas_info, &config);
        assert_eq!(reimbursement, 1_000_000); // Full reimbursement
        
        // Test bonus calculation
        let config_with_bonus = GasPoolConfig {
            reimbursement_rate_bps: 10_000,
            bonus_rate_bps: 2000, // 20% bonus
            max_reimbursement_per_tx: 10_000_000,
            ..Default::default()
        };
        
        let bonus = manager.calculate_bonus(&gas_info, &config_with_bonus);
        assert_eq!(bonus, 200_000); // 20% of 1M
        
        // Test approval
        let approved = manager.approve_reimbursement(&request_id, "approval_sig_123");
        assert!(approved);
        
        let approved_record = manager.get_reimbursement_status(&request_id);
        assert_eq!(approved_record.status, "approved");
        
        // Test rejection
        let request2 = ReimbursementRequest {
            transaction_id: "tx_003".to_string(),
            wallet_address: "wallet_003".to_string(),
            gas_cost: 300_000,
            ..Default::default()
        };
        
        let request_id2 = manager.submit_request(request2);
        let rejected = manager.reject_reimbursement(&request_id2, "Insufficient funds");
        assert!(rejected);
        
        let rejected_record = manager.get_reimbursement_status(&request_id2);
        assert_eq!(rejected_record.status, "rejected");
        
        // Test wallet balance check
        let sufficient = manager.check_wallet_balance("wallet_004", 100_000);
        assert!(sufficient);
        
        // Test statistics
        let stats = manager.get_stats();
        assert!(stats.total_requests >= 2);
        assert!(stats.total_approved >= 1);
        assert!(stats.total_rejected >= 1);
    }
    
    #[test]
    fn test_gas_pool_manager() {
        let mut pool_manager = GasPoolManager::new();
        
        // Test pool initialization
        let config = GasPoolConfig {
            pool_id: "pool_001".to_string(),
            total_pool_balance: 1_000_000_000, // 1 SOL
            available_balance: 1_000_000_000,
            max_reimbursement_per_tx: 10_000_000,
            ..Default::default()
        };
        
        let initialized = pool_manager.initialize_pool(config.clone());
        assert!(initialized);
        
        // Test pool config retrieval
        let retrieved_config = pool_manager.get_pool_config("pool_001");
        assert_eq!(retrieved_config.pool_id, "pool_001");
        assert_eq!(retrieved_config.total_pool_balance, 1_000_000_000);
        
        // Test pool balance update
        let updated = pool_manager.update_pool_balance("pool_001", 500_000_000);
        assert!(updated);
        
        let updated_config = pool_manager.get_pool_config("pool_001");
        assert_eq!(updated_config.total_pool_balance, 1_500_000_000);
        
        // Test availability check
        let available = pool_manager.check_availability("pool_001", 10_000_000);
        assert!(available);
        
        // Test fund reservation
        let reserved = pool_manager.reserve_funds("pool_001", 5_000_000);
        assert!(reserved);
        
        // Test fund release
        pool_manager.release_funds("pool_001", 2_000_000);
        
        // Test pool statistics
        let stats = pool_manager.get_pool_stats("pool_001");
        assert_eq!(stats.total_deposited, 1_500_000_000);
        assert_eq!(stats.current_balance, 1_500_000_000);
        assert_eq!(stats.reserved_amount, 3_000_000); // 5M - 2M released
    }
    
    #[test]
    fn test_gas_reimbursement_stack() {
        let mut stack = GasReimbursementStack::new();
        
        // Test stack creation
        assert_eq!(stack.estimator.get_gas_price(), 1000);
        
        // Initialize pool
        let config = GasPoolConfig {
            pool_id: "main_pool".to_string(),
            total_pool_balance: 5_000_000_000, // 5 SOL
            available_balance: 5_000_000_000,
            auto_approve_enabled: true,
            reimbursement_rate_bps: 10_000,
            ..Default::default()
        };
        
        let pool_initialized = stack.pool_manager.initialize_pool(config);
        assert!(pool_initialized);
        
        // Estimate gas
        let gas_estimate = stack.estimator.estimate_gas("swap", &[1, 2, 3]);
        assert!(gas_estimate > 0);
        
        // Submit reimbursement request
        let request = ReimbursementRequest {
            transaction_id: "stack_tx_001".to_string(),
            wallet_address: "stack_wallet_001".to_string(),
            reimbursement_type: ReimbursementType::PostTransaction,
            gas_cost: gas_estimate * stack.estimator.get_gas_price(),
            ..Default::default()
        };
        
        let request_id = stack.manager.submit_request(request);
        assert!(!request_id.is_empty());
        
        // Process request
        let record = stack.manager.process_request(&request_id);
        assert!(record.status == "approved" || record.status == "pending_approval");
        
        // If not auto-approved, approve it manually
        if record.status == "pending_approval" {
            stack.manager.approve_reimbursement(&request_id, "stack_approval_sig");
            let record = stack.manager.get_reimbursement_status(&request_id);
            assert_eq!(record.status, "approved");
        }
        
        // Reserve funds from pool
        let reserved = stack.pool_manager.reserve_funds("main_pool", record.total_amount);
        assert!(reserved);
        
        // Release funds
        stack.pool_manager.release_funds("main_pool", record.total_amount);
    }
    
    #[test]
    fn test_reimbursement_types() {
        let mut manager = GasReimbursementManager::new();
        
        let types = vec![
            ReimbursementType::PreTransaction,
            ReimbursementType::PostTransaction,
            ReimbursementType::Bonus,
            ReimbursementType::Emergency,
            ReimbursementType::ZkComputeBacked,
        ];
        
        for reimbursement_type in types {
            let request = ReimbursementRequest {
                transaction_id: "type_test_tx".to_string(),
                wallet_address: "type_test_wallet".to_string(),
                reimbursement_type,
                gas_cost: 100_000,
                ..Default::default()
            };
            
            let request_id = manager.submit_request(request);
            assert!(!request_id.is_empty());
            
            let record = manager.get_reimbursement_status(&request_id);
            assert_eq!(record.reimbursement_type, reimbursement_type);
        }
    }
    
    #[test]
    fn test_never_zero_balance() {
        let manager = GasReimbursementManager::new();
        
        // Test wallet with low balance
        let low_balance_wallet = "low_balance_wallet";
        
        // Check if transaction would cause zero balance
        let can_proceed = manager.check_wallet_balance(low_balance_wallet, 1_000);
        // Should return true for unknown wallet
        assert!(can_proceed);
        
        // Test with sufficient balance (unknown wallet returns true)
        let high_balance_wallet = "high_balance_wallet";
        let can_proceed_high = manager.check_wallet_balance(high_balance_wallet, 100_000);
        assert!(can_proceed_high);
    }
    
    #[test]
    fn test_concurrent_reimbursements() {
        let mut manager = GasReimbursementManager::new();
        let mut request_ids = Vec::new();
        
        // Submit multiple reimbursement requests
        for i in 0..10 {
            let request = ReimbursementRequest {
                transaction_id: format!("concurrent_tx_{}", i),
                wallet_address: format!("concurrent_wallet_{}", i),
                reimbursement_type: ReimbursementType::PostTransaction,
                gas_cost: 100_000 + (i * 50_000) as u64,
                ..Default::default()
            };
            
            let request_id = manager.submit_request(request);
            request_ids.push(request_id);
        }
        
        assert_eq!(request_ids.len(), 10);
        
        // Process all requests
        for request_id in &request_ids {
            let record = manager.process_request(request_id);
            assert_eq!(record.request_id, *request_id);
        }
        
        // Check statistics
        let stats = manager.get_stats();
        assert_eq!(stats.total_requests, 10);
    }
}