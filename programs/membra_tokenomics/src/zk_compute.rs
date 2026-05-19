use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::hash::hashv;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Constants
pub const HASH_SIZE: usize = 32;
pub const TASK_ID_SIZE: usize = 32;
pub const DEFAULT_REWARD_AMOUNT: u64 = 1_000_000; // 0.001 SOL in lamports

/// Compute types for ZK proofs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComputeType {
    LlmInference,
    MerkleComputation,
    MicrotaskCompletion,
    CollateralLocking,
    GasReimbursement,
    IdoAppraisal,
    CircuitCompilation,
    ProofVerification,
}

impl ComputeType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ComputeType::LlmInference),
            1 => Some(ComputeType::MerkleComputation),
            2 => Some(ComputeType::MicrotaskCompletion),
            3 => Some(ComputeType::CollateralLocking),
            4 => Some(ComputeType::GasReimbursement),
            5 => Some(ComputeType::IdoAppraisal),
            6 => Some(ComputeType::CircuitCompilation),
            7 => Some(ComputeType::ProofVerification),
            _ => None,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        match self {
            ComputeType::LlmInference => 0,
            ComputeType::MerkleComputation => 1,
            ComputeType::MicrotaskCompletion => 2,
            ComputeType::CollateralLocking => 3,
            ComputeType::GasReimbursement => 4,
            ComputeType::IdoAppraisal => 5,
            ComputeType::CircuitCompilation => 6,
            ComputeType::ProofVerification => 7,
        }
    }
}

/// ZK compute proof structure
#[derive(Debug, Clone)]
pub struct ZKComputeProof {
    pub task_id: String,
    pub prover_endpoint: String,
    pub compute_type: ComputeType,
    pub input_hash: [u8; HASH_SIZE],
    pub output_hash: [u8; HASH_SIZE],
    pub resource_commitment: [u8; HASH_SIZE],
    pub proof_hash: [u8; HASH_SIZE],
    pub verification_status: String, // "pending", "verified", "failed"
    pub reward_amount: u64,
    pub created_at: i64,
    pub verified_at: i64,
    pub circuit_identifier: String,
    pub metadata: HashMap<String, String>,
}

impl Default for ZKComputeProof {
    fn default() -> Self {
        Self {
            task_id: String::new(),
            prover_endpoint: String::new(),
            compute_type: ComputeType::LlmInference,
            input_hash: [0; HASH_SIZE],
            output_hash: [0; HASH_SIZE],
            resource_commitment: [0; HASH_SIZE],
            proof_hash: [0; HASH_SIZE],
            verification_status: "pending".to_string(),
            reward_amount: DEFAULT_REWARD_AMOUNT,
            created_at: 0,
            verified_at: 0,
            circuit_identifier: String::new(),
            metadata: HashMap::new(),
        }
    }
}

/// Compute task specification
#[derive(Debug, Clone)]
pub struct ComputeTask {
    pub task_id: String,
    pub compute_type: ComputeType,
    pub input_data: Vec<u8>,
    pub circuit_identifier: String,
    pub required_compute_units: u64,
    pub timeout_seconds: u64,
    pub requester_wallet: Pubkey,
    pub created_at: i64,
}

impl Default for ComputeTask {
    fn default() -> Self {
        Self {
            task_id: String::new(),
            compute_type: ComputeType::LlmInference,
            input_data: Vec::new(),
            circuit_identifier: String::new(),
            required_compute_units: 1000,
            timeout_seconds: 300,
            requester_wallet: Pubkey::default(),
            created_at: 0,
        }
    }
}

/// Resource commitment for compute
#[derive(Debug, Clone)]
pub struct ResourceCommitment {
    pub commitment_id: String,
    pub node_id: String,
    pub cpu_cores_committed: u64,
    pub memory_mb_committed: u64,
    pub gpu_committed: bool,
    pub duration_seconds: u64,
    pub commitment_hash: [u8; HASH_SIZE],
    pub expires_at: i64,
}

impl Default for ResourceCommitment {
    fn default() -> Self {
        Self {
            commitment_id: String::new(),
            node_id: String::new(),
            cpu_cores_committed: 0,
            memory_mb_committed: 0,
            gpu_committed: false,
            duration_seconds: 0,
            commitment_hash: [0; HASH_SIZE],
            expires_at: 0,
        }
    }
}

/// ZK proof verification result
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub success: bool,
    pub verification_status: String,
    pub error_message: String,
    pub gas_used: u64,
    pub verification_time_ms: u64,
    pub verified_hash: [u8; HASH_SIZE],
}

impl Default for VerificationResult {
    fn default() -> Self {
        Self {
            success: false,
            verification_status: "failed".to_string(),
            error_message: String::new(),
            gas_used: 0,
            verification_time_ms: 0,
            verified_hash: [0; HASH_SIZE],
        }
    }
}

/// ZK compute prover interface
#[derive(Clone)]
pub struct ZKProver {
    prover_endpoint: String,
}

impl ZKProver {
    pub fn new() -> Self {
        Self {
            prover_endpoint: "http://localhost:8080".to_string(),
        }
    }
    
    pub fn with_endpoint(endpoint: String) -> Self {
        Self {
            prover_endpoint: endpoint,
        }
    }
    
    /// Generate proof for compute task
    pub fn generate_proof(&self, task: &ComputeTask, commitment: &ResourceCommitment) -> ZKComputeProof {
        let mut proof = ZKComputeProof::default();
        
        // Generate task ID if not provided
        proof.task_id = if task.task_id.is_empty() {
            format!("task_{}", Self::current_timestamp())
        } else {
            task.task_id.clone()
        };
        
        proof.prover_endpoint = self.prover_endpoint.clone();
        proof.compute_type = task.compute_type;
        
        // Hash input data
        proof.input_hash = self.sha256(&task.input_data);
        
        // Hash resource commitment
        let commitment_data: Vec<u8> = commitment.commitment_id.bytes().collect();
        proof.resource_commitment = self.sha256(&commitment_data);
        
        // Generate mock proof
        proof.proof_hash = self.generate_mock_proof(task);
        
        // Set circuit identifier
        proof.circuit_identifier = task.circuit_identifier.clone();
        
        // Set timestamp
        proof.created_at = Self::current_timestamp();
        
        proof
    }
    
    /// Verify proof
    pub fn verify_proof(&self, proof: &ZKComputeProof) -> VerificationResult {
        let mut result = VerificationResult::default();
        
        // Mock verification
        result.success = true;
        result.verification_status = "verified".to_string();
        result.verification_time_ms = 100 + (Self::random_u32() % 200) as u64; // 100-300ms
        result.gas_used = 50_000 + (Self::random_u32() % 100_000) as u64; // 50k-150k gas
        
        // Hash the proof for verification
        let mut proof_data: Vec<u8> = proof.task_id.bytes().collect();
        proof_data.extend_from_slice(&proof.proof_hash);
        result.verified_hash = self.sha256(&proof_data);
        
        // Check proof hash consistency
        if proof.proof_hash[0] == 0 {
            result.success = false;
            result.verification_status = "invalid_proof".to_string();
            result.error_message = "Proof hash is empty".to_string();
        }
        
        result
    }
    
    /// Get prover endpoint info
    pub fn get_endpoint_info(&self) -> &str {
        &self.prover_endpoint
    }
    
    /// Set prover endpoint
    pub fn set_endpoint(&mut self, endpoint: String) {
        self.prover_endpoint = endpoint;
    }
    
    fn sha256(&self, data: &[u8]) -> [u8; HASH_SIZE] {
        let hash = hashv(&[data]);
        hash.to_bytes()
    }
    
    fn generate_mock_proof(&self, task: &ComputeTask) -> [u8; HASH_SIZE] {
        let mut proof_data: Vec<u8> = task.task_id.bytes().collect();
        proof_data.extend_from_slice(&task.input_data);
        proof_data.push(task.compute_type.to_u8());
        
        self.sha256(&proof_data)
    }
    
    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
    
    fn random_u32() -> u32 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u32;
        timestamp % 1000
    }
}

impl Default for ZKProver {
    fn default() -> Self {
        Self::new()
    }
}

/// ZK compute manager
pub struct ZKComputeManager {
    proofs: HashMap<String, ZKComputeProof>,
    prover: ZKProver,
    stats: ComputeStats,
}

#[derive(Debug, Clone, Default)]
pub struct ComputeStats {
    pub total_tasks_submitted: u64,
    pub total_tasks_verified: u64,
    pub total_rewards_paid: u64,
    pub total_compute_units: u64,
    pub average_verification_time_ms: f64,
}

impl ZKComputeManager {
    pub fn new() -> Self {
        Self {
            proofs: HashMap::new(),
            prover: ZKProver::new(),
            stats: ComputeStats::default(),
        }
    }
    
    pub fn with_prover(prover: ZKProver) -> Self {
        Self {
            proofs: HashMap::new(),
            prover,
            stats: ComputeStats::default(),
        }
    }
    
    /// Submit compute task
    pub fn submit_task(&mut self, task: &ComputeTask, commitment: &ResourceCommitment) -> String {
        let proof = self.prover.generate_proof(task, commitment);
        let task_id = proof.task_id.clone();
        
        // Store proof
        self.proofs.insert(task_id.clone(), proof);
        
        // Update statistics
        self.stats.total_tasks_submitted += 1;
        self.stats.total_compute_units += task.required_compute_units;
        
        task_id
    }
    
    /// Get task status
    pub fn get_task_status(&self, task_id: &str) -> ZKComputeProof {
        match self.proofs.get(task_id) {
            Some(proof) => proof.clone(),
            None => {
                let mut empty_proof = ZKComputeProof::default();
                empty_proof.task_id = task_id.to_string();
                empty_proof.verification_status = "not_found".to_string();
                empty_proof
            }
        }
    }
    
    /// Verify task completion
    pub fn verify_task_completion(&mut self, task_id: &str) -> VerificationResult {
        // First get the current compute type and verification
        let compute_type = match self.proofs.get(task_id) {
            Some(proof) => proof.compute_type,
            None => {
                let mut result = VerificationResult::default();
                result.success = false;
                result.verification_status = "task_not_found".to_string();
                return result;
            }
        };
        
        match self.proofs.get_mut(task_id) {
            Some(proof) => {
                let verification = self.prover.verify_proof(proof);
                
                if verification.success {
                    proof.verification_status = verification.verification_status.clone();
                    proof.verified_at = Self::current_timestamp();
                    proof.output_hash = verification.verified_hash;
                    
                    // Calculate reward
                    let verification_clone = verification.clone();
                    let mut mock_task = ComputeTask::default();
                    mock_task.compute_type = compute_type;
                    let reward_amount = {
                        // Create a temporary scope to avoid borrow issues
                        let task = &mock_task;
                        let verif = &verification_clone;
                        Self::calculate_reward_static(task, verif)
                    };
                    proof.reward_amount = reward_amount;
                    
                    // Update statistics
                    self.stats.total_tasks_verified += 1;
                    self.stats.total_rewards_paid += reward_amount;
                    
                    let total_time = self.stats.average_verification_time_ms * (self.stats.total_tasks_verified - 1) as f64 
                                   + verification.verification_time_ms as f64;
                    self.stats.average_verification_time_ms = total_time / self.stats.total_tasks_verified as f64;
                }
                
                verification
            }
            None => {
                let mut result = VerificationResult::default();
                result.success = false;
                result.verification_status = "task_not_found".to_string();
                result
            }
        }
    }
    
    /// Calculate reward based on compute type and complexity
    pub fn calculate_reward(&self, task: &ComputeTask, verification: &VerificationResult) -> u64 {
        Self::calculate_reward_static(task, verification)
    }
    
    /// Static helper for reward calculation to avoid borrow issues
    fn calculate_reward_static(task: &ComputeTask, verification: &VerificationResult) -> u64 {
        if !verification.success {
            return 0;
        }
        
        // Base reward based on compute type
        let base_reward = match task.compute_type {
            ComputeType::LlmInference => 2_000_000,      // 0.002 SOL
            ComputeType::MerkleComputation => 1_500_000,  // 0.0015 SOL
            ComputeType::MicrotaskCompletion => 500_000,  // 0.0005 SOL
            ComputeType::CollateralLocking => 1_000_000,  // 0.001 SOL
            ComputeType::GasReimbursement => 750_000,     // 0.00075 SOL
            ComputeType::IdoAppraisal => 3_000_000,       // 0.003 SOL
            ComputeType::CircuitCompilation => 2_500_000, // 0.0025 SOL
            ComputeType::ProofVerification => 500_000,    // 0.0005 SOL
        };
        
        // Adjust based on compute complexity
        let complexity_multiplier = task.required_compute_units / 1000;
        let mut reward = base_reward * complexity_multiplier.max(1);
        
        // Adjust based on verification speed (faster = higher reward)
        let speed_bonus = (1000.0 / verification.verification_time_ms as f64).max(1.0);
        reward = (reward as f64 * speed_bonus) as u64;
        
        reward
    }
    
    /// Get pending tasks
    pub fn get_pending_tasks(&self) -> Vec<ZKComputeProof> {
        self.proofs
            .values()
            .filter(|proof| proof.verification_status == "pending")
            .cloned()
            .collect()
    }
    
    /// Get verified proofs
    pub fn get_verified_proofs(&self) -> Vec<ZKComputeProof> {
        self.proofs
            .values()
            .filter(|proof| proof.verification_status == "verified")
            .cloned()
            .collect()
    }
    
    /// Get statistics
    pub fn get_stats(&self) -> ComputeStats {
        self.stats.clone()
    }
    
    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}

impl Default for ZKComputeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute resource allocator
pub struct ComputeResourceAllocator {
    commitments: HashMap<String, ResourceCommitment>,
    node_statuses: HashMap<String, NodeStatus>,
}

#[derive(Debug, Clone)]
pub struct NodeStatus {
    pub node_id: String,
    pub available_cpu_cores: u64,
    pub available_memory_mb: u64,
    pub gpu_available: bool,
    pub active_tasks: u64,
    pub utilization_ratio: f64,
}

impl ComputeResourceAllocator {
    pub fn new() -> Self {
        let mut allocator = Self {
            commitments: HashMap::new(),
            node_statuses: HashMap::new(),
        };
        
        // Initialize with a mock node status
        let mock_status = NodeStatus {
            node_id: "node_001".to_string(),
            available_cpu_cores: 8,
            available_memory_mb: 16384,
            gpu_available: true,
            active_tasks: 0,
            utilization_ratio: 0.0,
        };
        
        allocator.node_statuses.insert(mock_status.node_id.clone(), mock_status);
        
        allocator
    }
    
    /// Allocate resources for task
    pub fn allocate_resources(&mut self, task: &ComputeTask, node_id: &str) -> ResourceCommitment {
        // Check availability
        if !self.check_availability(task, node_id) {
            let mut commitment = ResourceCommitment::default();
            commitment.commitment_id = "unavailable".to_string();
            return commitment;
        }
        
        let node_status = self.node_statuses.get_mut(node_id).unwrap();
        
        // Generate commitment ID
        let timestamp = Self::current_timestamp();
        let combined = format!("{}{}", node_id, timestamp);
        let commitment_id = format!("{:016x}", Self::simple_hash(&combined));
        
        let cpu_committed = task.required_compute_units / 1000;
        let cpu_committed = cpu_committed.min(node_status.available_cpu_cores);
        
        let mem_committed = task.required_compute_units / 10;
        let mem_committed = mem_committed.min(node_status.available_memory_mb);
        
        let mut commitment = ResourceCommitment {
            commitment_id,
            node_id: node_id.to_string(),
            cpu_cores_committed: cpu_committed,
            memory_mb_committed: mem_committed,
            gpu_committed: node_status.gpu_available,
            duration_seconds: task.timeout_seconds,
            expires_at: Self::current_timestamp() + task.timeout_seconds as i64,
            ..Default::default()
        };
        
        // Hash commitment
        let mut commitment_data: Vec<u8> = commitment.commitment_id.bytes().collect();
        commitment_data.extend_from_slice(&commitment.cpu_cores_committed.to_be_bytes());
        commitment_data.extend_from_slice(&commitment.memory_mb_committed.to_be_bytes());
        commitment.commitment_hash = Self::simple_hash_bytes(&commitment_data);
        
        // Store commitment
        self.commitments.insert(commitment.commitment_id.clone(), commitment.clone());
        
        // Update node status
        node_status.available_cpu_cores -= cpu_committed;
        node_status.available_memory_mb -= mem_committed;
        node_status.active_tasks += 1;
        node_status.utilization_ratio = node_status.active_tasks as f64 / 8.0;
        
        commitment
    }
    
    /// Release resources
    pub fn release_resources(&mut self, commitment_id: &str) {
        if let Some(commitment) = self.commitments.remove(commitment_id) {
            if let Some(node_status) = self.node_statuses.get_mut(&commitment.node_id) {
                node_status.available_cpu_cores += commitment.cpu_cores_committed;
                node_status.available_memory_mb += commitment.memory_mb_committed;
                node_status.active_tasks -= 1;
                node_status.utilization_ratio = node_status.active_tasks as f64 / 8.0;
            }
        }
    }
    
    /// Check resource availability
    pub fn check_availability(&self, task: &ComputeTask, node_id: &str) -> bool {
        match self.node_statuses.get(node_id) {
            Some(status) => {
                let required_cpu = task.required_compute_units / 1000;
                let required_memory = task.required_compute_units / 10;
                
                status.available_cpu_cores >= required_cpu 
                    && status.available_memory_mb >= required_memory
                    && (task.circuit_identifier.is_empty() || status.gpu_available)
            }
            None => false,
        }
    }
    
    /// Get node status
    pub fn get_node_status(&self, node_id: &str) -> NodeStatus {
        match self.node_statuses.get(node_id) {
            Some(status) => status.clone(),
            None => NodeStatus {
                node_id: node_id.to_string(),
                ..Default::default()
            },
        }
    }
    
    fn simple_hash(s: &str) -> u64 {
        let mut hash: u64 = 5381;
        for c in s.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(c as u64);
        }
        hash
    }
    
    fn simple_hash_bytes(data: &[u8]) -> [u8; HASH_SIZE] {
        let hash = hashv(&[data]);
        hash.to_bytes()
    }
    
    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
}

impl Default for NodeStatus {
    fn default() -> Self {
        Self {
            node_id: String::new(),
            available_cpu_cores: 0,
            available_memory_mb: 0,
            gpu_available: false,
            active_tasks: 0,
            utilization_ratio: 0.0,
        }
    }
}

impl Default for ComputeResourceAllocator {
    fn default() -> Self {
        Self::new()
    }
}

/// Reward distributor
pub struct RewardDistributor {
    reward_balances: HashMap<String, u64>,
    total_rewards_distributed: u64,
}

impl RewardDistributor {
    pub fn new() -> Self {
        Self {
            reward_balances: HashMap::new(),
            total_rewards_distributed: 0,
        }
    }
    
    /// Distribute reward for verified proof
    pub fn distribute_reward(&mut self, proof: &ZKComputeProof, recipient_wallet: &str) -> bool {
        if proof.verification_status != "verified" {
            return false;
        }
        
        // Add reward to recipient's balance
        *self.reward_balances.entry(recipient_wallet.to_string()).or_insert(0) += proof.reward_amount;
        self.total_rewards_distributed += proof.reward_amount;
        
        true
    }
    
    /// Calculate reward amount
    pub fn calculate_reward_amount(&self, task: &ComputeTask, verification: &VerificationResult) -> u64 {
        if !verification.success {
            return 0;
        }
        
        // Base reward based on compute type
        let base_reward = match task.compute_type {
            ComputeType::LlmInference => 2_000_000,
            ComputeType::MerkleComputation => 1_500_000,
            ComputeType::MicrotaskCompletion => 500_000,
            ComputeType::CollateralLocking => 1_000_000,
            ComputeType::GasReimbursement => 750_000,
            ComputeType::IdoAppraisal => 3_000_000,
            ComputeType::CircuitCompilation => 2_500_000,
            ComputeType::ProofVerification => 500_000,
        };
        
        // Adjust for compute complexity
        let complexity_multiplier = task.required_compute_units / 1000;
        base_reward * complexity_multiplier.max(1)
    }
    
    /// Get reward history
    pub fn get_reward_history(&self, wallet: Option<&str>) -> HashMap<String, u64> {
        match wallet {
            Some(w) => {
                let mut history = HashMap::new();
                if let Some(&balance) = self.reward_balances.get(w) {
                    history.insert(w.to_string(), balance);
                }
                history
            }
            None => self.reward_balances.clone(),
        }
    }
    
    /// Get total rewards distributed
    pub fn get_total_rewards_distributed(&self) -> u64 {
        self.total_rewards_distributed
    }
}

impl Default for RewardDistributor {
    fn default() -> Self {
        Self::new()
    }
}

/// ZK compute integration factory
pub struct ZKComputeStack {
    pub prover: ZKProver,
    pub manager: ZKComputeManager,
    pub allocator: ComputeResourceAllocator,
    pub distributor: RewardDistributor,
}

impl ZKComputeStack {
    pub fn new() -> Self {
        Self {
            prover: ZKProver::new(),
            manager: ZKComputeManager::new(),
            allocator: ComputeResourceAllocator::new(),
            distributor: RewardDistributor::new(),
        }
    }
    
    pub fn with_prover(prover: ZKProver) -> Self {
        Self {
            prover: prover.clone(),
            manager: ZKComputeManager::with_prover(prover),
            allocator: ComputeResourceAllocator::new(),
            distributor: RewardDistributor::new(),
        }
    }
}

impl Default for ZKComputeStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zk_prover() {
        let prover = ZKProver::new();
        
        let task = ComputeTask {
            task_id: "test_task_001".to_string(),
            compute_type: ComputeType::LlmInference,
            input_data: vec![1, 2, 3, 4, 5],
            circuit_identifier: "llm_inference_circuit_v1".to_string(),
            required_compute_units: 5000,
            ..Default::default()
        };
        
        let commitment = ResourceCommitment {
            commitment_id: "commitment_001".to_string(),
            node_id: "node_001".to_string(),
            cpu_cores_committed: 4,
            memory_mb_committed: 8192,
            ..Default::default()
        };
        
        let proof = prover.generate_proof(&task, &commitment);
        
        assert_eq!(proof.task_id, "test_task_001");
        assert_eq!(proof.compute_type, ComputeType::LlmInference);
        assert_eq!(proof.verification_status, "pending");
        assert!(proof.created_at > 0);
        assert!(proof.proof_hash[0] != 0);
        
        let verification = prover.verify_proof(&proof);
        assert!(verification.success);
        assert_eq!(verification.verification_status, "verified");
        assert!(verification.verification_time_ms > 0);
        assert!(verification.gas_used > 0);
    }
    
    #[test]
    fn test_zk_compute_manager() {
        let mut manager = ZKComputeManager::new();
        
        let task = ComputeTask {
            task_id: "manager_task_001".to_string(),
            compute_type: ComputeType::MerkleComputation,
            input_data: vec![10, 20, 30, 40, 50],
            circuit_identifier: "merkle_circuit_v1".to_string(),
            required_compute_units: 2000,
            ..Default::default()
        };
        
        let commitment = ResourceCommitment {
            commitment_id: "manager_commitment_001".to_string(),
            node_id: "node_001".to_string(),
            cpu_cores_committed: 2,
            memory_mb_committed: 4096,
            ..Default::default()
        };
        
        let task_id = manager.submit_task(&task, &commitment);
        assert!(!task_id.is_empty());
        
        let proof = manager.get_task_status(&task_id);
        assert_eq!(proof.task_id, task_id);
        assert_eq!(proof.verification_status, "pending");
        
        let verification = manager.verify_task_completion(&task_id);
        assert!(verification.success);
        assert_eq!(verification.verification_status, "verified");
        
        let verified_proof = manager.get_task_status(&task_id);
        assert_eq!(verified_proof.verification_status, "verified");
        assert!(verified_proof.reward_amount > 0);
        
        // Add another pending task to test pending retrieval
        let task2 = ComputeTask {
            task_id: "pending_task_001".to_string(),
            compute_type: ComputeType::MicrotaskCompletion,
            input_data: vec![100, 200],
            required_compute_units: 1000,
            ..Default::default()
        };
        
        let commitment2 = ResourceCommitment {
            commitment_id: "pending_commitment_001".to_string(),
            node_id: "node_001".to_string(),
            ..Default::default()
        };
        
        let _task_id2 = manager.submit_task(&task2, &commitment2);
        
        let pending = manager.get_pending_tasks();
        assert!(!pending.is_empty());
        
        let verified = manager.get_verified_proofs();
        assert!(!verified.is_empty());
        
        let stats = manager.get_stats();
        assert!(stats.total_tasks_submitted >= 1);
        assert!(stats.total_tasks_verified >= 1);
        assert!(stats.total_rewards_paid > 0);
    }
    
    #[test]
    fn test_compute_resource_allocator() {
        let mut allocator = ComputeResourceAllocator::new();
        
        let task = ComputeTask {
            task_id: "allocation_task_001".to_string(),
            compute_type: ComputeType::LlmInference,
            required_compute_units: 4000,
            timeout_seconds: 120,
            ..Default::default()
        };
        
        let commitment = allocator.allocate_resources(&task, "node_001");
        assert!(!commitment.commitment_id.is_empty());
        assert_eq!(commitment.node_id, "node_001");
        assert!(commitment.cpu_cores_committed > 0);
        assert!(commitment.memory_mb_committed > 0);
        assert!(commitment.expires_at > 0);
        
        let available = allocator.check_availability(&task, "node_001");
        assert!(available);
        
        allocator.release_resources(&commitment.commitment_id);
        
        let status = allocator.get_node_status("node_001");
        assert_eq!(status.node_id, "node_001");
    }
    
    #[test]
    fn test_reward_distributor() {
        let mut distributor = RewardDistributor::new();
        
        let proof = ZKComputeProof {
            task_id: "reward_task_001".to_string(),
            compute_type: ComputeType::LlmInference,
            verification_status: "verified".to_string(),
            reward_amount: 2_000_000,
            ..Default::default()
        };
        
        let distributed = distributor.distribute_reward(&proof, "recipient_wallet_001");
        assert!(distributed);
        
        let history = distributor.get_reward_history(Some("recipient_wallet_001"));
        assert!(!history.is_empty());
        assert_eq!(history["recipient_wallet_001"], 2_000_000);
        
        let total = distributor.get_total_rewards_distributed();
        assert_eq!(total, 2_000_000);
    }
    
    #[test]
    fn test_zk_compute_stack() {
        let mut stack = ZKComputeStack::new();
        
        let task = ComputeTask {
            task_id: "workflow_task_001".to_string(),
            compute_type: ComputeType::GasReimbursement,
            input_data: vec![50, 60, 70],
            circuit_identifier: "gas_reimbursement_circuit_v1".to_string(),
            required_compute_units: 1500,
            ..Default::default()
        };
        
        let commitment = stack.allocator.allocate_resources(&task, "node_001");
        assert!(!commitment.commitment_id.is_empty());
        
        let task_id = stack.manager.submit_task(&task, &commitment);
        assert!(!task_id.is_empty());
        
        let verification = stack.manager.verify_task_completion(&task_id);
        assert!(verification.success);
        
        let proof = stack.manager.get_task_status(&task_id);
        let rewarded = stack.distributor.distribute_reward(&proof, "wallet_001");
        assert!(rewarded);
        
        stack.allocator.release_resources(&commitment.commitment_id);
    }
    
    #[test]
    fn test_compute_types() {
        let prover = ZKProver::new();
        
        let types = vec![
            ComputeType::LlmInference,
            ComputeType::MerkleComputation,
            ComputeType::MicrotaskCompletion,
            ComputeType::CollateralLocking,
            ComputeType::GasReimbursement,
            ComputeType::IdoAppraisal,
            ComputeType::CircuitCompilation,
            ComputeType::ProofVerification,
        ];
        
        for compute_type in types {
            let task = ComputeTask {
                compute_type,
                required_compute_units: 1000,
                ..Default::default()
            };
            
            let commitment = ResourceCommitment::default();
            let proof = prover.generate_proof(&task, &commitment);
            
            assert_eq!(proof.compute_type, compute_type);
        }
    }
    
    #[test]
    fn test_concurrent_operations() {
        let mut manager = ZKComputeManager::new();
        let mut task_ids = Vec::new();
        
        for i in 0..10 {
            let task = ComputeTask {
                task_id: format!("concurrent_task_{}", i),
                compute_type: ComputeType::MicrotaskCompletion,
                input_data: vec![i as u8],
                required_compute_units: 500 + (i * 100) as u64,
                ..Default::default()
            };
            
            let commitment = ResourceCommitment {
                commitment_id: format!("concurrent_commitment_{}", i),
                node_id: "node_001".to_string(),
                ..Default::default()
            };
            
            let task_id = manager.submit_task(&task, &commitment);
            task_ids.push(task_id);
        }
        
        assert_eq!(task_ids.len(), 10);
        
        for task_id in &task_ids {
            let verification = manager.verify_task_completion(task_id);
            assert!(verification.success);
        }
        
        let stats = manager.get_stats();
        assert_eq!(stats.total_tasks_submitted, 10);
        assert_eq!(stats.total_tasks_verified, 10);
    }
}