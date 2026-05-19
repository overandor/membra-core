use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Constants
pub const PRESERVATION_WALLET_ADDRESS_SIZE: usize = 32;
pub const PRESERVATION_THRESHOLD_LAMPORTS: u64 = 5_000; // Preserve at 5000 lamports
pub const CRITICAL_THRESHOLD_LAMPORTS: u64 = 1_000; // Critical threshold
pub const ARCHIVAL_COOLDOWN_SECONDS: u64 = 86_400; // 24 hours between archival operations

/// Preservation status types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreservationStatus {
    Active,           // Wallet is actively used
    Monitored,        // Balance is being monitored
    Preserved,        // Wallet has been preserved (archived)
    Recovered,        // Wallet has been recovered from archival
    GovernanceLocked,  // Locked by governance decision
    EmergencyFrozen,   // Emergency freeze
}

impl PreservationStatus {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(PreservationStatus::Active),
            1 => Some(PreservationStatus::Monitored),
            2 => Some(PreservationStatus::Preserved),
            3 => Some(PreservationStatus::Recovered),
            4 => Some(PreservationStatus::GovernanceLocked),
            5 => Some(PreservationStatus::EmergencyFrozen),
            _ => None,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        match self {
            PreservationStatus::Active => 0,
            PreservationStatus::Monitored => 1,
            PreservationStatus::Preserved => 2,
            PreservationStatus::Recovered => 3,
            PreservationStatus::GovernanceLocked => 4,
            PreservationStatus::EmergencyFrozen => 5,
        }
    }
}

/// Preservation action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreservationAction {
    None,             // No action needed
    Archive,          // Archive wallet state
    Restore,          // Restore from archive
    Freeze,           // Freeze wallet operations
    Unfreeze,         // Unfreeze wallet
    TransferFunds,    // Transfer minimal funds to maintain threshold
    NotifyUser,       // Notify user of low balance
    GovernanceReview,  // Request governance review
}

impl PreservationAction {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(PreservationAction::None),
            1 => Some(PreservationAction::Archive),
            2 => Some(PreservationAction::Restore),
            3 => Some(PreservationAction::Freeze),
            4 => Some(PreservationAction::Unfreeze),
            5 => Some(PreservationAction::TransferFunds),
            6 => Some(PreservationAction::NotifyUser),
            7 => Some(PreservationAction::GovernanceReview),
            _ => None,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        match self {
            PreservationAction::None => 0,
            PreservationAction::Archive => 1,
            PreservationAction::Restore => 2,
            PreservationAction::Freeze => 3,
            PreservationAction::Unfreeze => 4,
            PreservationAction::TransferFunds => 5,
            PreservationAction::NotifyUser => 6,
            PreservationAction::GovernanceReview => 7,
        }
    }
}

/// Wallet state snapshot for archival
#[derive(Debug, Clone)]
pub struct WalletSnapshot {
    pub wallet_address: String,
    pub balance: u64,
    pub transaction_count: u64,
    pub last_transaction_hash: String,
    pub last_activity_timestamp: i64,
    pub associated_account: String, // Associated user account if any
    pub metadata: HashMap<String, String>,
    pub state_hash: [u8; 32],
    pub snapshot_timestamp: i64,
}

impl Default for WalletSnapshot {
    fn default() -> Self {
        Self {
            wallet_address: String::new(),
            balance: 0,
            transaction_count: 0,
            last_transaction_hash: String::new(),
            last_activity_timestamp: 0,
            associated_account: String::new(),
            metadata: HashMap::new(),
            state_hash: [0; 32],
            snapshot_timestamp: 0,
        }
    }
}

/// Preservation record
#[derive(Debug, Clone)]
pub struct PreservationRecord {
    pub record_id: String,
    pub wallet_address: String,
    pub status: PreservationStatus,
    pub action_taken: PreservationAction,
    pub balance_at_preservation: u64,
    pub preservation_amount: u64, // Amount added to maintain threshold
    pub preservation_timestamp: i64,
    pub recovery_timestamp: i64,
    pub governance_signature: String,
    pub recovery_key: String, // Encrypted recovery key
    pub archive_location: String, // IPFS hash or storage reference
    pub metadata: HashMap<String, String>,
}

impl Default for PreservationRecord {
    fn default() -> Self {
        Self {
            record_id: String::new(),
            wallet_address: String::new(),
            status: PreservationStatus::Active,
            action_taken: PreservationAction::None,
            balance_at_preservation: 0,
            preservation_amount: 0,
            preservation_timestamp: 0,
            recovery_timestamp: 0,
            governance_signature: String::new(),
            recovery_key: String::new(),
            archive_location: String::new(),
            metadata: HashMap::new(),
        }
    }
}

/// Preservation policy configuration
#[derive(Debug, Clone)]
pub struct PreservationPolicy {
    pub policy_id: String,
    pub preservation_threshold: u64,
    pub critical_threshold: u64,
    pub minimum_maintenance_amount: u64,
    pub auto_preserve_enabled: bool,
    pub auto_restore_enabled: bool,
    pub governance_approval_required: bool,
    pub archival_cooldown_seconds: u64,
    pub exempt_wallets: Vec<String>, // Wallets exempt from preservation
    pub priority_wallets: Vec<String>, // High-priority wallets for preservation
}

impl Default for PreservationPolicy {
    fn default() -> Self {
        Self {
            policy_id: "default".to_string(),
            preservation_threshold: PRESERVATION_THRESHOLD_LAMPORTS,
            critical_threshold: CRITICAL_THRESHOLD_LAMPORTS,
            minimum_maintenance_amount: 1000,
            auto_preserve_enabled: true,
            auto_restore_enabled: false,
            governance_approval_required: false,
            archival_cooldown_seconds: ARCHIVAL_COOLDOWN_SECONDS,
            exempt_wallets: Vec::new(),
            priority_wallets: Vec::new(),
        }
    }
}

/// Wallet preservation monitor
#[derive(Clone)]
pub struct WalletPreservationMonitor {
    wallet_snapshots: HashMap<String, WalletSnapshot>,
    wallet_statuses: HashMap<String, PreservationStatus>,
    policy: PreservationPolicy,
}

impl WalletPreservationMonitor {
    pub fn new() -> Self {
        Self {
            wallet_snapshots: HashMap::new(),
            wallet_statuses: HashMap::new(),
            policy: PreservationPolicy::default(),
        }
    }
    
    pub fn with_policy(policy: PreservationPolicy) -> Self {
        Self {
            wallet_snapshots: HashMap::new(),
            wallet_statuses: HashMap::new(),
            policy,
        }
    }
    
    /// Monitor wallet balance
    pub fn check_wallet_status(&self, wallet_address: &str, current_balance: u64) -> PreservationAction {
        // Check if wallet is exempt
        if self.is_exempt(wallet_address) {
            return PreservationAction::None;
        }
        
        // Check current balance against thresholds
        if current_balance <= self.policy.critical_threshold {
            // Critical threshold - immediate preservation needed
            return PreservationAction::TransferFunds;
        } else if current_balance <= self.policy.preservation_threshold {
            // Below preservation threshold - monitor and prepare for archival
            return PreservationAction::NotifyUser;
        } else if current_balance < self.policy.preservation_threshold * 2 {
            // Approaching threshold - monitor
            return PreservationAction::None;
        }
        
        PreservationAction::None
    }
    
    /// Create wallet snapshot for archival
    pub fn create_snapshot(&mut self, wallet_address: &str, current_balance: u64) -> WalletSnapshot {
        let mut snapshot = WalletSnapshot::default();
        snapshot.wallet_address = wallet_address.to_string();
        snapshot.balance = current_balance;
        snapshot.transaction_count = 0; // Would be fetched from blockchain
        snapshot.last_activity_timestamp = Self::current_timestamp();
        snapshot.snapshot_timestamp = Self::current_timestamp();
        
        // Generate state hash
        snapshot.state_hash = self.generate_state_hash(&snapshot);
        
        // Store snapshot
        self.wallet_snapshots.insert(wallet_address.to_string(), snapshot.clone());
        
        snapshot
    }
    
    /// Update wallet state
    pub fn update_wallet_state(&mut self, wallet_address: &str, new_balance: u64) {
        if let Some(snapshot) = self.wallet_snapshots.get_mut(wallet_address) {
            snapshot.balance = new_balance;
            snapshot.last_activity_timestamp = Self::current_timestamp();
        } else {
            // Create new snapshot if wallet not monitored
            self.create_snapshot(wallet_address, new_balance);
        }
        
        // Update status based on new balance
        let action = self.check_wallet_status(wallet_address, new_balance);
        if action != PreservationAction::None {
            self.wallet_statuses.insert(wallet_address.to_string(), PreservationStatus::Monitored);
        } else {
            self.wallet_statuses.insert(wallet_address.to_string(), PreservationStatus::Active);
        }
    }
    
    /// Get wallet status
    pub fn get_wallet_status(&self, wallet_address: &str) -> PreservationStatus {
        *self.wallet_statuses.get(wallet_address).unwrap_or(&PreservationStatus::Active)
    }
    
    /// Check if wallet needs preservation
    pub fn needs_preservation(&self, wallet_address: &str, current_balance: u64) -> bool {
        current_balance <= self.policy.preservation_threshold && !self.is_exempt(wallet_address)
    }
    
    /// Check if wallet is exempt
    pub fn is_exempt(&self, wallet_address: &str) -> bool {
        self.policy.exempt_wallets.contains(&wallet_address.to_string())
    }
    
    /// Set preservation policy
    pub fn set_policy(&mut self, policy: PreservationPolicy) {
        self.policy = policy;
    }
    
    /// Get current policy
    pub fn get_policy(&self) -> PreservationPolicy {
        self.policy.clone()
    }
    
    fn generate_state_hash(&self, snapshot: &WalletSnapshot) -> [u8; 32] {
        let mut state_data: Vec<u8> = snapshot.wallet_address.bytes().collect();
        
        // Add balance to hash
        let balance_bytes = snapshot.balance.to_be_bytes();
        state_data.extend_from_slice(&balance_bytes);
        
        // Add timestamp
        let timestamp_bytes = snapshot.snapshot_timestamp.to_be_bytes();
        state_data.extend_from_slice(&timestamp_bytes);
        
        // Simple hash (in production would use actual crypto)
        let hash = Self::simple_hash(&state_data);
        hash
    }
    
    fn simple_hash(data: &[u8]) -> [u8; 32] {
        let mut hash: [u8; 32] = [0; 32];
        for (i, byte) in data.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(*byte);
        }
        hash
    }
    
    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}

impl Default for WalletPreservationMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Wallet preservation manager
pub struct WalletPreservationManager {
    preservation_records: HashMap<String, PreservationRecord>,
    monitor: WalletPreservationMonitor,
    stats: PreservationStats,
}

#[derive(Debug, Clone, Default)]
pub struct PreservationStats {
    pub total_preservations: u64,
    pub total_restorations: u64,
    pub total_funds_transferred: u64,
    pub total_frozen: u64,
    pub active_preservations: u64,
    pub average_preservation_time_hours: f64,
}

impl WalletPreservationManager {
    pub fn new() -> Self {
        Self {
            preservation_records: HashMap::new(),
            monitor: WalletPreservationMonitor::new(),
            stats: PreservationStats::default(),
        }
    }
    
    pub fn with_monitor(monitor: WalletPreservationMonitor) -> Self {
        Self {
            preservation_records: HashMap::new(),
            monitor,
            stats: PreservationStats::default(),
        }
    }
    
    /// Preserve wallet (archive)
    pub fn preserve_wallet(&mut self, wallet_address: &str, current_balance: u64) -> String {
        let record_id = self.generate_record_id();
        
        // Create wallet snapshot
        let snapshot = self.monitor.create_snapshot(wallet_address, current_balance);
        
        // Archive snapshot
        let archive_location = self.archive_snapshot(&snapshot);
        
        // Create preservation record
        let mut record = PreservationRecord::default();
        record.record_id = record_id.clone();
        record.wallet_address = wallet_address.to_string();
        record.status = PreservationStatus::Preserved;
        record.action_taken = PreservationAction::Archive;
        record.balance_at_preservation = current_balance;
        record.preservation_amount = self.monitor.get_policy().minimum_maintenance_amount;
        record.preservation_timestamp = Self::current_timestamp();
        record.archive_location = archive_location;
        
        self.preservation_records.insert(record_id.clone(), record);
        
        // Update wallet status
        self.monitor.update_wallet_state(wallet_address, current_balance);
        
        // Update statistics
        self.stats.total_preservations += 1;
        self.stats.active_preservations += 1;
        
        record_id
    }
    
    /// Restore wallet from archival
    pub fn restore_wallet(&mut self, record_id: &str, recovery_key: &str) -> bool {
        if let Some(record) = self.preservation_records.get_mut(record_id) {
            // Validate recovery key
            if record.recovery_key != recovery_key {
                return false;
            }
            
            // Update record
            record.status = PreservationStatus::Recovered;
            record.action_taken = PreservationAction::Restore;
            record.recovery_timestamp = Self::current_timestamp();
            
            // Update wallet status
            self.monitor.update_wallet_state(&record.wallet_address, record.balance_at_preservation);
            
            // Update statistics
            self.stats.total_restorations += 1;
            self.stats.active_preservations = self.stats.active_preservations.saturating_sub(1);
            
            return true;
        }
        
        false
    }
    
    /// Transfer minimal funds to maintain threshold
    pub fn transfer_maintenance_funds(&mut self, wallet_address: &str, amount: u64) -> bool {
        // In a real implementation, this would interact with the gas reimbursement system
        // or directly transfer funds from a preservation pool
        
        // Update wallet state to reflect the transfer
        let current_balance = self.monitor.create_snapshot(wallet_address, 0).balance;
        self.monitor.update_wallet_state(wallet_address, current_balance + amount);
        
        self.stats.total_funds_transferred += amount;
        
        true
    }
    
    /// Freeze wallet operations
    pub fn freeze_wallet(&mut self, wallet_address: &str, reason: &str) -> bool {
        let record_id = self.generate_record_id();
        
        // Create preservation record for frozen wallet
        let mut record = PreservationRecord::default();
        record.record_id = record_id.clone();
        record.wallet_address = wallet_address.to_string();
        record.status = PreservationStatus::EmergencyFrozen;
        record.action_taken = PreservationAction::Freeze;
        record.preservation_timestamp = Self::current_timestamp();
        record.metadata.insert("freeze_reason".to_string(), reason.to_string());
        
        self.preservation_records.insert(record_id, record);
        
        // Update wallet status
        self.monitor.wallet_statuses.insert(wallet_address.to_string(), PreservationStatus::EmergencyFrozen);
        
        self.stats.total_frozen += 1;
        
        true
    }
    
    /// Unfreeze wallet operations
    pub fn unfreeze_wallet(&mut self, wallet_address: &str) -> bool {
        if let Some(record) = self.preservation_records.get_mut(wallet_address) {
            record.status = PreservationStatus::Active;
            record.action_taken = PreservationAction::Unfreeze;
            
            // Update wallet status
            self.monitor.wallet_statuses.insert(wallet_address.to_string(), PreservationStatus::Active);
            
            return true;
        }
        
        false
    }
    
    /// Request governance review
    pub fn request_governance_review(&mut self, wallet_address: &str, reason: &str) -> String {
        let request_id = format!("gov_review_{}", Self::current_timestamp());
        
        // Create preservation record for governance review
        let mut record = PreservationRecord::default();
        record.record_id = request_id.clone();
        record.wallet_address = wallet_address.to_string();
        record.status = PreservationStatus::GovernanceLocked;
        record.action_taken = PreservationAction::GovernanceReview;
        record.preservation_timestamp = Self::current_timestamp();
        record.metadata.insert("review_reason".to_string(), reason.to_string());
        
        self.preservation_records.insert(request_id.clone(), record);
        
        request_id
    }
    
    /// Get preservation record
    pub fn get_preservation_record(&self, record_id: &str) -> PreservationRecord {
        self.preservation_records.get(record_id).cloned().unwrap_or_default()
    }
    
    /// Get wallet preservation history
    pub fn get_wallet_history(&self, wallet_address: &str) -> Vec<PreservationRecord> {
        self.preservation_records
            .values()
            .filter(|record| record.wallet_address == wallet_address)
            .cloned()
            .collect()
    }
    
    /// Get all preserved wallets
    pub fn get_all_preserved(&self) -> Vec<PreservationRecord> {
        self.preservation_records
            .values()
            .filter(|record| record.status == PreservationStatus::Preserved)
            .cloned()
            .collect()
    }
    
    /// Get statistics
    pub fn get_stats(&self) -> PreservationStats {
        self.stats.clone()
    }
    
    fn generate_record_id(&self) -> String {
        format!("preserve_{}", Self::current_timestamp())
    }
    
    fn archive_snapshot(&self, snapshot: &WalletSnapshot) -> String {
        // In a real implementation, this would upload to IPFS or other storage
        // For now, we return a mock hash
        
        let mut snapshot_data: Vec<u8> = snapshot.wallet_address.bytes().collect();
        
        let balance_bytes = snapshot.balance.to_be_bytes();
        snapshot_data.extend_from_slice(&balance_bytes);
        
        let hash = Self::simple_hash(&snapshot_data);
        
        let mut hash_str = String::new();
        for byte in &hash {
            hash_str.push_str(&format!("{:02x}", byte));
        }
        
        format!("ipfs://{}", hash_str)
    }
    
    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
    
    fn simple_hash(data: &[u8]) -> [u8; 32] {
        let mut hash: [u8; 32] = [0; 32];
        for (i, byte) in data.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(*byte);
        }
        hash
    }
}

impl Default for WalletPreservationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Recovery service for preserved wallets
pub struct WalletRecoveryService {
    recovery_requests: HashMap<String, RecoveryStatus>,
}

#[derive(Debug, Clone)]
pub struct RecoveryStatus {
    pub recovery_id: String,
    pub record_id: String,
    pub status: String, // "pending", "verified", "completed", "failed"
    pub initiated_at: i64,
    pub completed_at: i64,
    pub verification_method: String,
}

impl Default for RecoveryStatus {
    fn default() -> Self {
        Self {
            recovery_id: String::new(),
            record_id: String::new(),
            status: "pending".to_string(),
            initiated_at: 0,
            completed_at: 0,
            verification_method: String::new(),
        }
    }
}

impl WalletRecoveryService {
    pub fn new() -> Self {
        Self {
            recovery_requests: HashMap::new(),
        }
    }
    
    /// Initiate recovery process
    pub fn initiate_recovery(&mut self, record_id: &str, _user_signature: &str) -> String {
        let recovery_id = format!("recovery_{}", Self::current_timestamp());
        
        let mut status = RecoveryStatus::default();
        status.recovery_id = recovery_id.clone();
        status.record_id = record_id.to_string();
        status.status = "pending".to_string();
        status.initiated_at = Self::current_timestamp();
        status.verification_method = "signature".to_string();
        
        self.recovery_requests.insert(recovery_id.clone(), status);
        
        recovery_id
    }
    
    /// Verify recovery request
    pub fn verify_recovery_request(&mut self, recovery_id: &str) -> bool {
        if let Some(status) = self.recovery_requests.get_mut(recovery_id) {
            // Mock verification - in production would verify signature
            status.status = "verified".to_string();
            status.completed_at = Self::current_timestamp();
            return true;
        }
        
        false
    }
    
    /// Complete recovery
    pub fn complete_recovery(&mut self, recovery_id: &str, recovery_key: &str) -> bool {
        // Validate recovery key first
        if !self.validate_recovery_key(recovery_key) {
            return false;
        }
        
        if let Some(status) = self.recovery_requests.get_mut(recovery_id) {
            if status.status != "verified" {
                return false;
            }
            
            status.status = "completed".to_string();
            status.completed_at = Self::current_timestamp();
            
            return true;
        }
        
        false
    }
    
    /// Generate recovery key
    pub fn generate_recovery_key(&self, wallet_address: &str) -> String {
        let mut key_data: Vec<u8> = wallet_address.bytes().collect();
        
        let timestamp = Self::current_timestamp();
        let timestamp_str = timestamp.to_string();
        key_data.extend_from_slice(timestamp_str.as_bytes());
        
        let hash = Self::simple_hash(&key_data);
        
        let mut key_str = String::new();
        for byte in &hash {
            key_str.push_str(&format!("{:02x}", byte));
        }
        
        key_str
    }
    
    /// Validate recovery key
    pub fn validate_recovery_key(&self, recovery_key: &str) -> bool {
        // Basic validation - check length and format
        recovery_key.len() == 64
    }
    
    /// Get recovery status
    pub fn get_recovery_status(&self, recovery_id: &str) -> RecoveryStatus {
        self.recovery_requests.get(recovery_id).cloned().unwrap_or_default()
    }
    
    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
    
    fn simple_hash(data: &[u8]) -> [u8; 32] {
        let mut hash: [u8; 32] = [0; 32];
        for (i, byte) in data.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(*byte);
        }
        hash
    }
}

impl Default for WalletRecoveryService {
    fn default() -> Self {
        Self::new()
    }
}

/// Governance integration for preservation decisions
pub struct PreservationGovernance {
    approval_requests: HashMap<String, ApprovalStatus>,
}

#[derive(Debug, Clone)]
pub struct ApprovalStatus {
    pub request_id: String,
    pub wallet_address: String,
    pub action: PreservationAction,
    pub status: String, // "pending", "approved", "rejected"
    pub governance_signature: String,
    pub requested_at: i64,
    pub decided_at: i64,
}

impl Default for ApprovalStatus {
    fn default() -> Self {
        Self {
            request_id: String::new(),
            wallet_address: String::new(),
            action: PreservationAction::None,
            status: "pending".to_string(),
            governance_signature: String::new(),
            requested_at: 0,
            decided_at: 0,
        }
    }
}

impl PreservationGovernance {
    pub fn new() -> Self {
        Self {
            approval_requests: HashMap::new(),
        }
    }
    
    /// Request approval for preservation action
    pub fn request_approval(&mut self, wallet_address: &str, action: PreservationAction) -> String {
        let request_id = format!("gov_approve_{}", Self::current_timestamp());
        
        let mut status = ApprovalStatus::default();
        status.request_id = request_id.clone();
        status.wallet_address = wallet_address.to_string();
        status.action = action;
        status.status = "pending".to_string();
        status.requested_at = Self::current_timestamp();
        
        self.approval_requests.insert(request_id.clone(), status);
        
        request_id
    }
    
    /// Submit governance decision
    pub fn submit_decision(&mut self, request_id: &str, approved: bool, signature: &str) -> bool {
        if let Some(status) = self.approval_requests.get_mut(request_id) {
            status.status = if approved { "approved" } else { "rejected" }.to_string();
            status.governance_signature = signature.to_string();
            status.decided_at = Self::current_timestamp();
            return true;
        }
        
        false
    }
    
    /// Get approval status
    pub fn get_approval_status(&self, request_id: &str) -> ApprovalStatus {
        self.approval_requests.get(request_id).cloned().unwrap_or_default()
    }
    
    /// Check if governance approval is required
    pub fn requires_approval(&self, _wallet_address: &str, action: PreservationAction) -> bool {
        // Check if wallet is in priority list or requires governance
        // For now, we'll say governance approval is required for freezing actions
        matches!(action, PreservationAction::Freeze | PreservationAction::GovernanceReview)
    }
    
    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}

impl Default for PreservationGovernance {
    fn default() -> Self {
        Self::new()
    }
}

/// Zero balance preservation factory
pub struct PreservationStack {
    pub monitor: WalletPreservationMonitor,
    pub manager: WalletPreservationManager,
    pub recovery_service: WalletRecoveryService,
    pub governance: PreservationGovernance,
}

impl PreservationStack {
    pub fn new() -> Self {
        let monitor = WalletPreservationMonitor::new();
        Self {
            manager: WalletPreservationManager::with_monitor(monitor.clone()),
            monitor,
            recovery_service: WalletRecoveryService::new(),
            governance: PreservationGovernance::new(),
        }
    }
}

impl Default for PreservationStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wallet_preservation_monitor() {
        let mut monitor = WalletPreservationMonitor::new();
        
        // Test wallet status checking
        let wallet_address = "test_wallet_001";
        let normal_balance = 1_000_000; // 0.001 SOL
        
        let action = monitor.check_wallet_status(wallet_address, normal_balance);
        assert!(action == PreservationAction::None);
        
        // Test critical threshold
        let critical_balance = 500; // Below critical threshold
        let critical_action = monitor.check_wallet_status(wallet_address, critical_balance);
        assert!(critical_action == PreservationAction::TransferFunds);
        
        // Test preservation threshold
        let preservation_balance = 3000; // Below preservation threshold but above critical
        let preservation_action = monitor.check_wallet_status(wallet_address, preservation_balance);
        assert!(preservation_action == PreservationAction::NotifyUser);
        
        // Test snapshot creation
        let snapshot = monitor.create_snapshot(wallet_address, normal_balance);
        assert_eq!(snapshot.wallet_address, wallet_address);
        assert_eq!(snapshot.balance, normal_balance);
        assert!(snapshot.state_hash[0] != 0); // Hash should be generated
        
        // Test wallet state update
        monitor.update_wallet_state(wallet_address, 50000);
        let status = monitor.get_wallet_status(wallet_address);
        assert!(status == PreservationStatus::Active);
        
        // Test needs preservation
        let needs = monitor.needs_preservation(wallet_address, 2000);
        assert!(needs == true);
        
        // Test exempt wallet
        let mut policy = monitor.get_policy();
        policy.exempt_wallets.push("exempt_wallet_001".to_string());
        monitor.set_policy(policy);
        
        let exempt_needs = monitor.needs_preservation("exempt_wallet_001", 1000);
        assert!(exempt_needs == false);
    }
    
    #[test]
    fn test_wallet_preservation_manager() {
        let mut manager = WalletPreservationManager::new();
        
        // Test wallet preservation
        let wallet_address = "preserve_wallet_001";
        let low_balance = 1000;
        
        let record_id = manager.preserve_wallet(wallet_address, low_balance);
        assert!(!record_id.is_empty());
        
        // Test preservation record retrieval
        let record = manager.get_preservation_record(&record_id);
        assert_eq!(record.record_id, record_id);
        assert_eq!(record.wallet_address, wallet_address);
        assert_eq!(record.status, PreservationStatus::Preserved);
        assert_eq!(record.balance_at_preservation, low_balance);
        
        // Test wallet history
        let history = manager.get_wallet_history(wallet_address);
        assert!(!history.is_empty());
        
        // Test maintenance funds transfer
        let transferred = manager.transfer_maintenance_funds(wallet_address, 5000);
        assert!(transferred);
        
        // Test wallet freezing
        let frozen = manager.freeze_wallet(wallet_address, "Security freeze");
        assert!(frozen);
        
        // Test governance review request
        let gov_request_id = manager.request_governance_review("gov_wallet_001", "Suspicious activity");
        assert!(!gov_request_id.is_empty());
        
        // Test statistics
        let stats = manager.get_stats();
        assert!(stats.total_preservations >= 1);
        assert!(stats.total_frozen >= 1);
    }
    
    #[test]
    fn test_wallet_recovery_service() {
        let mut recovery_service = WalletRecoveryService::new();
        
        // Test recovery initiation
        let record_id = "recovery_record_001";
        let user_signature = "user_sig_123";
        
        let recovery_id = recovery_service.initiate_recovery(record_id, user_signature);
        assert!(!recovery_id.is_empty());
        
        // Test recovery verification
        let verified = recovery_service.verify_recovery_request(&recovery_id);
        assert!(verified);
        
        // Test recovery key generation
        let wallet_address = "recovery_wallet_001";
        let recovery_key = recovery_service.generate_recovery_key(wallet_address);
        assert!(!recovery_key.is_empty());
        assert_eq!(recovery_key.len(), 64);
        
        // Test recovery key validation
        let valid = recovery_service.validate_recovery_key(&recovery_key);
        assert!(valid);
        
        // Test recovery completion
        let completed = recovery_service.complete_recovery(&recovery_id, &recovery_key);
        assert!(completed);
        
        // Test recovery status retrieval
        let status = recovery_service.get_recovery_status(&recovery_id);
        assert_eq!(status.recovery_id, recovery_id);
        assert_eq!(status.status, "completed");
    }
    
    #[test]
    fn test_preservation_governance() {
        let mut governance = PreservationGovernance::new();
        
        // Test approval request
        let wallet_address = "gov_wallet_001";
        let action = PreservationAction::Freeze;
        
        let request_id = governance.request_approval(wallet_address, action);
        assert!(!request_id.is_empty());
        
        // Test approval status retrieval
        let status = governance.get_approval_status(&request_id);
        assert_eq!(status.request_id, request_id);
        assert_eq!(status.status, "pending");
        
        // Test governance decision submission
        let submitted = governance.submit_decision(&request_id, true, "gov_sig_123");
        assert!(submitted);
        
        let updated_status = governance.get_approval_status(&request_id);
        assert_eq!(updated_status.status, "approved");
        
        // Test approval requirement check
        let requires = governance.requires_approval(wallet_address, PreservationAction::Freeze);
        assert!(requires);
        
        let not_requires = governance.requires_approval(wallet_address, PreservationAction::None);
        assert!(!not_requires);
    }
    
    #[test]
    fn test_preservation_stack() {
        let mut stack = PreservationStack::new();
        
        // Test stack creation
        assert_eq!(stack.monitor.get_wallet_status("test"), PreservationStatus::Active);
        
        // Test end-to-end preservation workflow
        let wallet_address = "stack_wallet_001";
        let low_balance = 1000;
        
        // Monitor wallet
        let action = stack.monitor.check_wallet_status(wallet_address, low_balance);
        assert!(action == PreservationAction::TransferFunds);
        
        // Preserve wallet
        let record_id = stack.manager.preserve_wallet(wallet_address, low_balance);
        assert!(!record_id.is_empty());
        
        // Generate recovery key
        let recovery_key = stack.recovery_service.generate_recovery_key(wallet_address);
        assert!(!recovery_key.is_empty());
        
        // Request governance approval for restore
        let gov_request_id = stack.governance.request_approval(wallet_address, PreservationAction::Restore);
        assert!(!gov_request_id.is_empty());
        
        // Approve the request
        let approved = stack.governance.submit_decision(&gov_request_id, true, "stack_gov_sig");
        assert!(approved);
        
        // Initiate recovery
        let recovery_id = stack.recovery_service.initiate_recovery(&record_id, "user_sig");
        assert!(!recovery_id.is_empty());
    }
    
    #[test]
    fn test_preservation_types() {
        let mut monitor = WalletPreservationMonitor::new();
        
        let statuses = vec![
            PreservationStatus::Active,
            PreservationStatus::Monitored,
            PreservationStatus::Preserved,
            PreservationStatus::Recovered,
            PreservationStatus::GovernanceLocked,
            PreservationStatus::EmergencyFrozen,
        ];
        
        for status in statuses {
            let wallet_address = format!("type_test_wallet_{}", status as u8);
            monitor.wallet_statuses.insert(wallet_address.clone(), status);
            
            let retrieved = monitor.get_wallet_status(&wallet_address);
            assert_eq!(retrieved, status);
        }
    }
    
    #[test]
    fn test_threshold_enforcement() {
        let monitor = WalletPreservationMonitor::new();
        
        // Test preservation threshold (5000 lamports)
        let wallet1 = "threshold_wallet_001";
        let action1 = monitor.check_wallet_status(wallet1, 4000);
        assert!(action1 == PreservationAction::NotifyUser);
        
        // Test critical threshold (1000 lamports)
        let wallet2 = "threshold_wallet_002";
        let action2 = monitor.check_wallet_status(wallet2, 500);
        assert!(action2 == PreservationAction::TransferFunds);
        
        // Test above threshold
        let wallet3 = "threshold_wallet_003";
        let action3 = monitor.check_wallet_status(wallet3, 10000);
        assert!(action3 == PreservationAction::None);
    }
    
    #[test]
    fn test_concurrent_preservations() {
        let mut manager = WalletPreservationManager::new();
        let mut record_ids = Vec::new();
        
        // Preserve multiple wallets concurrently
        for i in 0..10 {
            let wallet_address = format!("concurrent_wallet_{}", i);
            let balance = 500 + (i * 100) as u64;
            
            let record_id = manager.preserve_wallet(&wallet_address, balance);
            record_ids.push(record_id);
        }
        
        assert_eq!(record_ids.len(), 10);
        
        // Check statistics
        let stats = manager.get_stats();
        assert_eq!(stats.total_preservations, 10);
    }
}