use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// Constants
pub const MICROTASK_TASK_ID_SIZE: usize = 32;
pub const MIN_TASK_VALUE: u64 = 1000; // Minimum task value in lamports
pub const MAX_TASK_VALUE: u64 = 10_000_000; // Maximum task value in lamports
pub const DEFAULT_REPUTATION_SCORE: u64 = 1000; // Starting reputation
pub const MAX_REPUTATION_SCORE: u64 = 10_000; // Maximum reputation
pub const MIN_REPUTATION_SCORE: u64 = 0; // Minimum reputation
pub const TASK_EXPIRY_SECONDS: u64 = 86_400; // 24 hours
pub const COMPLETION_TIMEOUT_SECONDS: u64 = 3_600; // 1 hour

/// Microtask types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MicrotaskType {
    DataLabeling,
    ContentModeration,
    Transcription,
    Translation,
    Validation,
    OracleSubmission,
    ProofVerification,
    ComputeTask,
    QualityAssurance,
    SentimentAnalysis,
    ImageClassification,
    TextSummarization,
    CodeReview,
    Testing,
    Documentation,
}

impl MicrotaskType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(MicrotaskType::DataLabeling),
            1 => Some(MicrotaskType::ContentModeration),
            2 => Some(MicrotaskType::Transcription),
            3 => Some(MicrotaskType::Translation),
            4 => Some(MicrotaskType::Validation),
            5 => Some(MicrotaskType::OracleSubmission),
            6 => Some(MicrotaskType::ProofVerification),
            7 => Some(MicrotaskType::ComputeTask),
            8 => Some(MicrotaskType::QualityAssurance),
            9 => Some(MicrotaskType::SentimentAnalysis),
            10 => Some(MicrotaskType::ImageClassification),
            11 => Some(MicrotaskType::TextSummarization),
            12 => Some(MicrotaskType::CodeReview),
            13 => Some(MicrotaskType::Testing),
            14 => Some(MicrotaskType::Documentation),
            _ => None,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        match self {
            MicrotaskType::DataLabeling => 0,
            MicrotaskType::ContentModeration => 1,
            MicrotaskType::Transcription => 2,
            MicrotaskType::Translation => 3,
            MicrotaskType::Validation => 4,
            MicrotaskType::OracleSubmission => 5,
            MicrotaskType::ProofVerification => 6,
            MicrotaskType::ComputeTask => 7,
            MicrotaskType::QualityAssurance => 8,
            MicrotaskType::SentimentAnalysis => 9,
            MicrotaskType::ImageClassification => 10,
            MicrotaskType::TextSummarization => 11,
            MicrotaskType::CodeReview => 12,
            MicrotaskType::Testing => 13,
            MicrotaskType::Documentation => 14,
        }
    }
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskStatus {
    Pending,
    Assigned,
    InProgress,
    Completed,
    Verified,
    Rejected,
    Expired,
    Cancelled,
}

impl TaskStatus {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TaskStatus::Pending),
            1 => Some(TaskStatus::Assigned),
            2 => Some(TaskStatus::InProgress),
            3 => Some(TaskStatus::Completed),
            4 => Some(TaskStatus::Verified),
            5 => Some(TaskStatus::Rejected),
            6 => Some(TaskStatus::Expired),
            7 => Some(TaskStatus::Cancelled),
            _ => None,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        match self {
            TaskStatus::Pending => 0,
            TaskStatus::Assigned => 1,
            TaskStatus::InProgress => 2,
            TaskStatus::Completed => 3,
            TaskStatus::Verified => 4,
            TaskStatus::Rejected => 5,
            TaskStatus::Expired => 6,
            TaskStatus::Cancelled => 7,
        }
    }
}

/// Task difficulty
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskDifficulty {
    Trivial,
    Easy,
    Medium,
    Hard,
    Expert,
}

impl TaskDifficulty {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TaskDifficulty::Trivial),
            1 => Some(TaskDifficulty::Easy),
            2 => Some(TaskDifficulty::Medium),
            3 => Some(TaskDifficulty::Hard),
            4 => Some(TaskDifficulty::Expert),
            _ => None,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        match self {
            TaskDifficulty::Trivial => 0,
            TaskDifficulty::Easy => 1,
            TaskDifficulty::Medium => 2,
            TaskDifficulty::Hard => 3,
            TaskDifficulty::Expert => 4,
        }
    }
    
    /// Convert difficulty to value multiplier
    pub fn to_multiplier(&self) -> f64 {
        match self {
            TaskDifficulty::Trivial => 1.0,
            TaskDifficulty::Easy => 1.5,
            TaskDifficulty::Medium => 2.0,
            TaskDifficulty::Hard => 3.0,
            TaskDifficulty::Expert => 5.0,
        }
    }
}

/// Microtask definition
#[derive(Debug, Clone)]
pub struct Microtask {
    pub task_id: String,
    pub task_type: MicrotaskType,
    pub status: TaskStatus,
    pub difficulty: TaskDifficulty,
    pub creator_address: String,
    pub worker_address: String,
    pub title: String,
    pub description: String,
    pub metadata: HashMap<String, String>,
    pub base_value: u64,
    pub total_value: u64,
    pub created_at: i64,
    pub assigned_at: i64,
    pub completed_at: i64,
    pub verified_at: i64,
    pub expires_at: i64,
    pub backing_transaction_id: String,
    pub proof_hash: String,
    pub result_hash: String,
    pub verification_signature: String,
}

impl Default for Microtask {
    fn default() -> Self {
        Self {
            task_id: String::new(),
            task_type: MicrotaskType::DataLabeling,
            status: TaskStatus::Pending,
            difficulty: TaskDifficulty::Medium,
            creator_address: String::new(),
            worker_address: String::new(),
            title: String::new(),
            description: String::new(),
            metadata: HashMap::new(),
            base_value: 0,
            total_value: 0,
            created_at: 0,
            assigned_at: 0,
            completed_at: 0,
            verified_at: 0,
            expires_at: 0,
            backing_transaction_id: String::new(),
            proof_hash: String::new(),
            result_hash: String::new(),
            verification_signature: String::new(),
        }
    }
}

/// Task result
#[derive(Debug, Clone)]
pub struct TaskResult {
    pub result_id: String,
    pub task_id: String,
    pub worker_address: String,
    pub result_data: String,
    pub result_hash: String,
    pub proof: Vec<u8>,
    pub submitted_at: i64,
    pub quality_score: u32,
    pub verified: bool,
    pub verification_signature: String,
}

impl Default for TaskResult {
    fn default() -> Self {
        Self {
            result_id: String::new(),
            task_id: String::new(),
            worker_address: String::new(),
            result_data: String::new(),
            result_hash: String::new(),
            proof: Vec::new(),
            submitted_at: 0,
            quality_score: 0,
            verified: false,
            verification_signature: String::new(),
        }
    }
}

/// Worker reputation
#[derive(Debug, Clone)]
pub struct WorkerReputation {
    pub worker_address: String,
    pub reputation_score: u64,
    pub tasks_completed: u64,
    pub tasks_verified: u64,
    pub tasks_rejected: u64,
    pub total_earned: u64,
    pub average_quality: f64,
    pub last_activity: i64,
}

impl Default for WorkerReputation {
    fn default() -> Self {
        Self {
            worker_address: String::new(),
            reputation_score: DEFAULT_REPUTATION_SCORE,
            tasks_completed: 0,
            tasks_verified: 0,
            tasks_rejected: 0,
            total_earned: 0,
            average_quality: 0.0,
            last_activity: 0,
        }
    }
}

/// Backing pool for transaction backing
#[derive(Debug, Clone)]
pub struct BackingPool {
    pub pool_id: String,
    pub transaction_id: String,
    pub transaction_value: u64,
    pub backing_value: u64,
    pub required_tasks: u64,
    pub completed_tasks: u64,
    pub verified_tasks: u64,
    pub created_at: i64,
    pub expires_at: i64,
    pub fully_backed: bool,
    pub verified: bool,
    pub task_ids: Vec<String>,
}

impl Default for BackingPool {
    fn default() -> Self {
        Self {
            pool_id: String::new(),
            transaction_id: String::new(),
            transaction_value: 0,
            backing_value: 0,
            required_tasks: 0,
            completed_tasks: 0,
            verified_tasks: 0,
            created_at: 0,
            expires_at: 0,
            fully_backed: false,
            verified: false,
            task_ids: Vec::new(),
        }
    }
}

/// Value conversion configuration
#[derive(Debug, Clone)]
pub struct ValueConversionConfig {
    pub base_task_value: u64,
    pub difficulty_multiplier: f64,
    pub reputation_multiplier: f64,
    pub urgency_multiplier: f64,
    pub complexity_multiplier: f64,
    pub max_task_value: u64,
    pub min_task_value: u64,
}

impl Default for ValueConversionConfig {
    fn default() -> Self {
        Self {
            base_task_value: 1000,
            difficulty_multiplier: 1.0,
            reputation_multiplier: 1.0,
            urgency_multiplier: 1.0,
            complexity_multiplier: 1.0,
            max_task_value: MAX_TASK_VALUE,
            min_task_value: MIN_TASK_VALUE,
        }
    }
}

/// Microtask manager
pub struct MicrotaskManager {
    tasks: Arc<Mutex<HashMap<String, Microtask>>>,
    results: Arc<Mutex<HashMap<String, TaskResult>>>,
    reputations: Arc<Mutex<HashMap<String, WorkerReputation>>>,
    backing_pools: Arc<Mutex<HashMap<String, BackingPool>>>,
    config: Arc<Mutex<ValueConversionConfig>>,
}

impl MicrotaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            results: Arc::new(Mutex::new(HashMap::new())),
            reputations: Arc::new(Mutex::new(HashMap::new())),
            backing_pools: Arc::new(Mutex::new(HashMap::new())),
            config: Arc::new(Mutex::new(ValueConversionConfig::default())),
        }
    }
    
    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }
    
    fn generate_task_id() -> String {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("task_{}", ts)
    }
    
    fn generate_result_id() -> String {
        format!("result_{}", Self::current_timestamp())
    }
    
    fn generate_pool_id() -> String {
        format!("pool_{}", Self::current_timestamp())
    }
    
    fn calculate_task_value(&self, _task_type: MicrotaskType, difficulty: TaskDifficulty, reputation_score: u64) -> u64 {
        let config = self.config.lock().unwrap();
        let base = config.base_task_value;
        let multiplier = difficulty.to_multiplier();
        
        // Apply reputation multiplier (higher reputation = slightly higher value)
        let rep_mult = 1.0 + ((reputation_score as f64 - DEFAULT_REPUTATION_SCORE as f64) / 10000.0);
        
        let value = (base as f64 * multiplier * rep_mult) as u64;
        
        // Clamp to min/max
        let value = value.max(config.min_task_value).min(config.max_task_value);
        
        value
    }
    
    /// Create a new microtask
    pub fn create_task(
        &self,
        creator_address: &str,
        task_type: MicrotaskType,
        difficulty: TaskDifficulty,
        title: &str,
        description: &str,
        metadata: HashMap<String, String>,
        backing_transaction_id: &str,
    ) -> String {
        let mut tasks = self.tasks.lock().unwrap();
        let reputations = self.reputations.lock().unwrap();
        
        let task_id = Self::generate_task_id();
        
        // Get reputation for value calculation
        let rep = reputations.get(creator_address).cloned().unwrap_or_default();
        
        let mut task = Microtask::default();
        task.task_id = task_id.clone();
        task.task_type = task_type;
        task.status = TaskStatus::Pending;
        task.difficulty = difficulty;
        task.creator_address = creator_address.to_string();
        task.title = title.to_string();
        task.description = description.to_string();
        task.metadata = metadata;
        task.backing_transaction_id = backing_transaction_id.to_string();
        task.created_at = Self::current_timestamp();
        task.expires_at = task.created_at + TASK_EXPIRY_SECONDS as i64;
        
        // Calculate task value
        task.base_value = self.calculate_task_value(task_type, difficulty, rep.reputation_score);
        task.total_value = task.base_value;
        
        tasks.insert(task_id.clone(), task);
        task_id
    }
    
    /// Assign task to worker
    pub fn assign_task(&self, task_id: &str, worker_address: &str) -> bool {
        let mut tasks = self.tasks.lock().unwrap();
        
        if let Some(task) = tasks.get_mut(task_id) {
            if task.status != TaskStatus::Pending {
                return false;
            }
            
            // Check if task is expired
            if Self::current_timestamp() > task.expires_at {
                task.status = TaskStatus::Expired;
                return false;
            }
            
            task.status = TaskStatus::Assigned;
            task.worker_address = worker_address.to_string();
            task.assigned_at = Self::current_timestamp();
            return true;
        }
        
        false
    }
    
    /// Submit task result
    pub fn submit_result(
        &self,
        task_id: &str,
        worker_address: &str,
        result_data: &str,
        proof: Vec<u8>,
    ) -> String {
        let mut tasks = self.tasks.lock().unwrap();
        let mut results = self.results.lock().unwrap();
        
        if let Some(task) = tasks.get_mut(task_id) {
            if task.status != TaskStatus::Assigned {
                return String::new();
            }
            
            if task.worker_address != worker_address {
                return String::new();
            }
            
            let result_id = Self::generate_result_id();
            let mut result = TaskResult::default();
            result.result_id = result_id.clone();
            result.task_id = task_id.to_string();
            result.worker_address = worker_address.to_string();
            result.result_data = result_data.to_string();
            result.proof = proof;
            result.submitted_at = Self::current_timestamp();
            let submitted_at = result.submitted_at;

            // Simple hash for result
            let hash_input = format!("{}{}", result_data, submitted_at);
            result.result_hash = self.simple_hash(&hash_input);

            let result_hash = result.result_hash.clone();
            results.insert(result_id.clone(), result);

            task.status = TaskStatus::Completed;
            task.completed_at = submitted_at;
            task.result_hash = result_hash;
            
            return result_id;
        }
        
        String::new()
    }
    
    /// Verify task result
    pub fn verify_result(&self, result_id: &str, quality_score: u32, verification_signature: &str) -> bool {
        let mut tasks = self.tasks.lock().unwrap();
        let mut results = self.results.lock().unwrap();
        let mut reputations = self.reputations.lock().unwrap();
        
        if let Some(result) = results.get_mut(result_id) {
            if let Some(task) = tasks.get_mut(&result.task_id) {
                if task.status != TaskStatus::Completed {
                    return false;
                }
                
                result.quality_score = quality_score;
                result.verified = quality_score >= 50; // Minimum quality threshold
                result.verification_signature = verification_signature.to_string();
                
                if result.verified {
                    task.status = TaskStatus::Verified;
                    task.verified_at = Self::current_timestamp();
                    task.verification_signature = verification_signature.to_string();
                    
                    // Update worker reputation
                    self.update_reputation(&result.worker_address, quality_score, true, &mut reputations);
                } else {
                    task.status = TaskStatus::Rejected;
                    self.update_reputation(&result.worker_address, quality_score, false, &mut reputations);
                }
                
                return result.verified;
            }
        }
        
        false
    }
    
    fn update_reputation(
        &self,
        worker_address: &str,
        quality_score: u32,
        successful: bool,
        reputations: &mut HashMap<String, WorkerReputation>,
    ) {
        let rep = reputations.entry(worker_address.to_string()).or_default();
        
        if successful {
            rep.tasks_completed += 1;
            rep.tasks_verified += 1;
            rep.total_earned += 100; // Base reward
            
            // Update reputation score based on quality
            let rep_change = ((quality_score as f64 - 50.0) * 2.0) as i64;
            rep.reputation_score = (rep.reputation_score as i64 + rep_change)
                .max(MIN_REPUTATION_SCORE as i64)
                .min(MAX_REPUTATION_SCORE as i64) as u64;
            
            // Update average quality
            let total_quality = rep.average_quality * (rep.tasks_completed - 1) as f64 + quality_score as f64;
            rep.average_quality = total_quality / rep.tasks_completed as f64;
        } else {
            rep.tasks_rejected += 1;
            rep.reputation_score = rep.reputation_score.saturating_sub(50);
        }
        
        rep.last_activity = Self::current_timestamp();
    }
    
    /// Create backing pool for transaction
    pub fn create_backing_pool(&self, transaction_id: &str, transaction_value: u64, required_tasks: u64) -> String {
        let mut backing_pools = self.backing_pools.lock().unwrap();
        
        let pool_id = Self::generate_pool_id();
        
        let mut pool = BackingPool::default();
        pool.pool_id = pool_id.clone();
        pool.transaction_id = transaction_id.to_string();
        pool.transaction_value = transaction_value;
        pool.required_tasks = required_tasks;
        pool.created_at = Self::current_timestamp();
        pool.expires_at = pool.created_at + TASK_EXPIRY_SECONDS as i64;
        
        // Calculate required backing value (tasks should cover transaction value)
        let task_value = transaction_value / required_tasks;
        pool.backing_value = task_value * required_tasks;
        
        backing_pools.insert(pool_id.clone(), pool);
        pool_id
    }
    
    /// Add task to backing pool
    pub fn add_task_to_pool(&self, pool_id: &str, task_id: &str) -> bool {
        let mut backing_pools = self.backing_pools.lock().unwrap();
        let mut tasks = self.tasks.lock().unwrap();
        
        if let Some(pool) = backing_pools.get_mut(pool_id) {
            if let Some(task) = tasks.get_mut(task_id) {
                pool.task_ids.push(task_id.to_string());
                task.backing_transaction_id = pool.transaction_id.clone();
                return true;
            }
        }
        
        false
    }
    
    /// Update pool status based on task completion
    pub fn update_pool_status(&self, pool_id: &str) {
        let mut backing_pools = self.backing_pools.lock().unwrap();
        let tasks = self.tasks.lock().unwrap();
        
        if let Some(pool) = backing_pools.get_mut(pool_id) {
            let mut completed = 0;
            let mut verified = 0;
            
            for task_id in &pool.task_ids {
                if let Some(task) = tasks.get(task_id) {
                    if task.status == TaskStatus::Completed || task.status == TaskStatus::Verified {
                        completed += 1;
                    }
                    if task.status == TaskStatus::Verified {
                        verified += 1;
                    }
                }
            }
            
            pool.completed_tasks = completed;
            pool.verified_tasks = verified;
            pool.fully_backed = completed >= pool.required_tasks;
            pool.verified = verified >= pool.required_tasks;
        }
    }
    
    /// Get task by ID
    pub fn get_task(&self, task_id: &str) -> Microtask {
        let tasks = self.tasks.lock().unwrap();
        tasks.get(task_id).cloned().unwrap_or_default()
    }
    
    /// Get result by ID
    pub fn get_result(&self, result_id: &str) -> TaskResult {
        let results = self.results.lock().unwrap();
        results.get(result_id).cloned().unwrap_or_default()
    }
    
    /// Get worker reputation
    pub fn get_reputation(&self, worker_address: &str) -> WorkerReputation {
        let reputations = self.reputations.lock().unwrap();
        reputations.get(worker_address).cloned().unwrap_or_default()
    }
    
    /// Get backing pool
    pub fn get_backing_pool(&self, pool_id: &str) -> BackingPool {
        let backing_pools = self.backing_pools.lock().unwrap();
        backing_pools.get(pool_id).cloned().unwrap_or_default()
    }
    
    /// Get available tasks
    pub fn get_available_tasks(&self, type_filter: MicrotaskType) -> Vec<Microtask> {
        let tasks = self.tasks.lock().unwrap();
        tasks
            .values()
            .filter(|task| task.status == TaskStatus::Pending)
            .filter(|task| task.task_type == type_filter)
            .cloned()
            .collect()
    }
    
    /// Get tasks for worker
    pub fn get_worker_tasks(&self, worker_address: &str) -> Vec<Microtask> {
        let tasks = self.tasks.lock().unwrap();
        tasks
            .values()
            .filter(|task| task.worker_address == worker_address)
            .cloned()
            .collect()
    }
    
    /// Set value conversion config
    pub fn set_config(&self, config: ValueConversionConfig) {
        let mut cfg = self.config.lock().unwrap();
        *cfg = config;
    }
    
    /// Get config
    pub fn get_config(&self) -> ValueConversionConfig {
        let cfg = self.config.lock().unwrap();
        cfg.clone()
    }
    
    fn simple_hash(&self, input: &str) -> String {
        let mut hash: u64 = 0;
        for byte in input.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        format!("{:x}", hash)
    }
}

impl Default for MicrotaskManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Microtask backing stack (factory)
pub struct MicrotaskBackingStack {
    pub manager: Arc<MicrotaskManager>,
}

impl MicrotaskBackingStack {
    pub fn new() -> Self {
        Self {
            manager: Arc::new(MicrotaskManager::new()),
        }
    }
    
    /// Create complete backing workflow
    pub fn create_backed_transaction(
        &self,
        transaction_id: &str,
        transaction_value: u64,
        creator_address: &str,
        num_tasks: u32,
    ) -> String {
        // Create backing pool
        let pool_id = self
            .manager
            .create_backing_pool(transaction_id, transaction_value, num_tasks as u64);
        
        // Create tasks for the pool
        for i in 0..num_tasks {
            let task_id = self.manager.create_task(
                creator_address,
                MicrotaskType::Validation,
                TaskDifficulty::Medium,
                &format!("Validation Task {}", i),
                "Validate transaction data",
                {
                    let mut meta = HashMap::new();
                    meta.insert("pool_id".to_string(), pool_id.clone());
                    meta
                },
                "",
            );
            
            self.manager.add_task_to_pool(&pool_id, &task_id);
        }
        
        pool_id
    }
}

impl Default for MicrotaskBackingStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_microtask_creation() {
        let manager = MicrotaskManager::new();
        
        let task_id = manager.create_task(
            "creator_001",
            MicrotaskType::DataLabeling,
            TaskDifficulty::Medium,
            "Test Task",
            "Test Description",
            HashMap::new(),
            "",
        );
        
        assert!(!task_id.is_empty());
        
        let task = manager.get_task(&task_id);
        assert_eq!(task.task_id, task_id);
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.task_type, MicrotaskType::DataLabeling);
        assert_eq!(task.difficulty, TaskDifficulty::Medium);
        assert_eq!(task.creator_address, "creator_001");
        assert!(task.base_value > 0);
    }
    
    #[test]
    fn test_task_assignment() {
        let manager = MicrotaskManager::new();
        
        let task_id = manager.create_task(
            "creator_001",
            MicrotaskType::Validation,
            TaskDifficulty::Easy,
            "Assignment Test",
            "Test assignment",
            HashMap::new(),
            "",
        );
        
        let assigned = manager.assign_task(&task_id, "worker_001");
        assert!(assigned);
        
        let task = manager.get_task(&task_id);
        assert_eq!(task.status, TaskStatus::Assigned);
        assert_eq!(task.worker_address, "worker_001");
        assert!(task.assigned_at > 0);
        
        // Test double assignment (should fail)
        let double_assigned = manager.assign_task(&task_id, "worker_002");
        assert!(!double_assigned);
    }
    
    #[test]
    fn test_result_submission() {
        let manager = MicrotaskManager::new();
        
        let task_id = manager.create_task(
            "creator_001",
            MicrotaskType::Transcription,
            TaskDifficulty::Hard,
            "Submission Test",
            "Test submission",
            HashMap::new(),
            "",
        );
        
        manager.assign_task(&task_id, "worker_001");
        
        let result_id = manager.submit_result(&task_id, "worker_001", "Test result data", vec![1, 2, 3, 4]);
        
        assert!(!result_id.is_empty());
        
        let result = manager.get_result(&result_id);
        assert_eq!(result.result_id, result_id);
        assert_eq!(result.task_id, task_id);
        assert_eq!(result.worker_address, "worker_001");
        assert_eq!(result.result_data, "Test result data");
        assert!(!result.result_hash.is_empty());
        
        let task = manager.get_task(&task_id);
        assert_eq!(task.status, TaskStatus::Completed);
        assert_eq!(task.result_hash, result.result_hash);
    }
    
    #[test]
    fn test_result_verification() {
        let manager = MicrotaskManager::new();
        
        let task_id = manager.create_task(
            "creator_001",
            MicrotaskType::ContentModeration,
            TaskDifficulty::Medium,
            "Verification Test",
            "Test verification",
            HashMap::new(),
            "",
        );
        
        manager.assign_task(&task_id, "worker_001");
        let result_id = manager.submit_result(&task_id, "worker_001", "High quality result data", vec![]);
        
        // Verify with high quality score
        let verified = manager.verify_result(&result_id, 85, "signature_001");
        assert!(verified);
        
        let result = manager.get_result(&result_id);
        assert!(result.verified);
        assert_eq!(result.quality_score, 85);
        
        let task = manager.get_task(&task_id);
        assert_eq!(task.status, TaskStatus::Verified);
    }
    
    #[test]
    fn test_result_rejection() {
        let manager = MicrotaskManager::new();
        
        let task_id = manager.create_task(
            "creator_001",
            MicrotaskType::Translation,
            TaskDifficulty::Easy,
            "Rejection Test",
            "Test rejection",
            HashMap::new(),
            "",
        );
        
        manager.assign_task(&task_id, "worker_001");
        let result_id = manager.submit_result(&task_id, "worker_001", "Poor quality result", vec![]);
        
        // Verify with low quality score
        let verified = manager.verify_result(&result_id, 30, "signature_002");
        assert!(!verified);
        
        let result = manager.get_result(&result_id);
        assert!(!result.verified);
        assert_eq!(result.quality_score, 30);
        
        let task = manager.get_task(&task_id);
        assert_eq!(task.status, TaskStatus::Rejected);
    }
    
    #[test]
    fn test_reputation_system() {
        let manager = MicrotaskManager::new();
        
        let worker = "worker_reputation";
        
        // Get initial reputation
        let initial_rep = manager.get_reputation(worker);
        assert_eq!(initial_rep.reputation_score, DEFAULT_REPUTATION_SCORE);
        assert_eq!(initial_rep.tasks_completed, 0);
        
        // Create and complete a task successfully
        let task_id = manager.create_task(
            "creator_001",
            MicrotaskType::Validation,
            TaskDifficulty::Medium,
            "Reputation Test",
            "Test reputation",
            HashMap::new(),
            "",
        );
        
        manager.assign_task(&task_id, worker);
        let result_id = manager.submit_result(&task_id, worker, "Good result", vec![]);
        manager.verify_result(&result_id, 80, "sig");
        
        let rep_after_success = manager.get_reputation(worker);
        assert_eq!(rep_after_success.tasks_completed, 1);
        assert_eq!(rep_after_success.tasks_verified, 1);
        assert!(rep_after_success.reputation_score > initial_rep.reputation_score);
        
        // Create and fail a task
        let task_id2 = manager.create_task(
            "creator_001",
            MicrotaskType::Validation,
            TaskDifficulty::Medium,
            "Reputation Test 2",
            "Test reputation 2",
            HashMap::new(),
            "",
        );
        
        manager.assign_task(&task_id2, worker);
        let result_id2 = manager.submit_result(&task_id2, worker, "Bad result", vec![]);
        manager.verify_result(&result_id2, 30, "sig");
        
        let rep_after_failure = manager.get_reputation(worker);
        assert_eq!(rep_after_failure.tasks_rejected, 1);
        assert!(rep_after_failure.reputation_score < rep_after_success.reputation_score);
    }
    
    #[test]
    fn test_backing_pool() {
        let manager = MicrotaskManager::new();
        
        let pool_id = manager.create_backing_pool("tx_001", 1_000_000, 5);
        assert!(!pool_id.is_empty());
        
        let pool = manager.get_backing_pool(&pool_id);
        assert_eq!(pool.pool_id, pool_id);
        assert_eq!(pool.transaction_id, "tx_001");
        assert_eq!(pool.transaction_value, 1_000_000);
        assert_eq!(pool.required_tasks, 5);
        assert!(!pool.fully_backed);
        
        // Add tasks to pool
        for i in 0..5 {
            let task_id = manager.create_task(
                "creator_001",
                MicrotaskType::Validation,
                TaskDifficulty::Medium,
                &format!("Pool Task {}", i),
                "Task for pool",
                HashMap::new(),
                "",
            );
            manager.add_task_to_pool(&pool_id, &task_id);
        }
        
        let pool = manager.get_backing_pool(&pool_id);
        assert_eq!(pool.task_ids.len(), 5);
    }
    
    #[test]
    fn test_value_conversion() {
        let manager = MicrotaskManager::new();
        
        // Test different difficulty levels
        let easy_task = manager.create_task(
            "creator_001",
            MicrotaskType::DataLabeling,
            TaskDifficulty::Easy,
            "Easy Task",
            "Easy task",
            HashMap::new(),
            "",
        );
        
        let hard_task = manager.create_task(
            "creator_001",
            MicrotaskType::DataLabeling,
            TaskDifficulty::Hard,
            "Hard Task",
            "Hard task",
            HashMap::new(),
            "",
        );
        
        let easy = manager.get_task(&easy_task);
        let hard = manager.get_task(&hard_task);
        
        assert!(hard.total_value > easy.total_value);
    }
    
    #[test]
    fn test_task_types() {
        let manager = MicrotaskManager::new();
        
        let types = vec![
            MicrotaskType::DataLabeling,
            MicrotaskType::ContentModeration,
            MicrotaskType::Transcription,
            MicrotaskType::Translation,
            MicrotaskType::Validation,
            MicrotaskType::OracleSubmission,
            MicrotaskType::ProofVerification,
            MicrotaskType::ComputeTask,
            MicrotaskType::QualityAssurance,
            MicrotaskType::SentimentAnalysis,
            MicrotaskType::ImageClassification,
            MicrotaskType::TextSummarization,
            MicrotaskType::CodeReview,
            MicrotaskType::Testing,
            MicrotaskType::Documentation,
        ];
        
        for task_type in types {
            let task_id = manager.create_task(
                "creator_001",
                task_type,
                TaskDifficulty::Medium,
                "Type Test",
                "Test type",
                HashMap::new(),
                "",
            );
            
            let task = manager.get_task(&task_id);
            assert_eq!(task.task_type, task_type);
        }
    }
    
    #[test]
    fn test_task_statuses() {
        let statuses = vec![
            TaskStatus::Pending,
            TaskStatus::Assigned,
            TaskStatus::InProgress,
            TaskStatus::Completed,
            TaskStatus::Verified,
            TaskStatus::Rejected,
            TaskStatus::Expired,
            TaskStatus::Cancelled,
        ];
        
        for status in statuses {
            let _status_str = status.to_u8();
            let _converted = TaskStatus::from_u8(status.to_u8());
        }
    }
    
    #[test]
    fn test_difficulty_multipliers() {
        let trivial_mult = TaskDifficulty::Trivial.to_multiplier();
        let easy_mult = TaskDifficulty::Easy.to_multiplier();
        let medium_mult = TaskDifficulty::Medium.to_multiplier();
        let hard_mult = TaskDifficulty::Hard.to_multiplier();
        let expert_mult = TaskDifficulty::Expert.to_multiplier();
        
        assert!(trivial_mult < easy_mult);
        assert!(easy_mult < medium_mult);
        assert!(medium_mult < hard_mult);
        assert!(hard_mult < expert_mult);
        
        assert_eq!(expert_mult, 5.0);
        assert_eq!(trivial_mult, 1.0);
    }
    
    #[test]
    fn test_microtask_stack() {
        let stack = MicrotaskBackingStack::new();
        
        let pool_id = stack.create_backed_transaction("tx_stack_001", 5_000_000, "creator_001", 3);
        
        assert!(!pool_id.is_empty());
        
        let pool = stack.manager.get_backing_pool(&pool_id);
        assert_eq!(pool.transaction_id, "tx_stack_001");
        assert_eq!(pool.transaction_value, 5_000_000);
        assert_eq!(pool.required_tasks, 3);
        assert_eq!(pool.task_ids.len(), 3);
    }
    
    #[test]
    fn test_available_tasks_filtering() {
        let manager = MicrotaskManager::new();
        
        // Create tasks of different types
        let _task1 = manager.create_task(
            "creator_001",
            MicrotaskType::DataLabeling,
            TaskDifficulty::Medium,
            "Data Task",
            "Data task",
            HashMap::new(),
            "",
        );
        
        let _task2 = manager.create_task(
            "creator_001",
            MicrotaskType::Translation,
            TaskDifficulty::Medium,
            "Translation Task",
            "Translation task",
            HashMap::new(),
            "",
        );
        
        // Get available tasks for specific type
        let data_tasks = manager.get_available_tasks(MicrotaskType::DataLabeling);
        assert!(data_tasks.len() >= 1);
        let found_data = data_tasks.iter().any(|t| t.task_type == MicrotaskType::DataLabeling);
        assert!(found_data);
        
        let translation_tasks = manager.get_available_tasks(MicrotaskType::Translation);
        assert!(translation_tasks.len() >= 1);
        let found_translation = translation_tasks.iter().any(|t| t.task_type == MicrotaskType::Translation);
        assert!(found_translation);
    }
    
    #[test]
    fn test_worker_task_history() {
        let manager = MicrotaskManager::new();
        let worker = "worker_history";
        
        // Create and assign multiple tasks to worker
        for i in 0..3 {
            let task_id = manager.create_task(
                "creator_001",
                MicrotaskType::Validation,
                TaskDifficulty::Medium,
                &format!("History Task {}", i),
                "History test",
                HashMap::new(),
                "",
            );
            manager.assign_task(&task_id, worker);
        }
        
        let worker_tasks = manager.get_worker_tasks(worker);
        assert_eq!(worker_tasks.len(), 3);
        
        for task in &worker_tasks {
            assert_eq!(task.worker_address, worker);
        }
    }
}