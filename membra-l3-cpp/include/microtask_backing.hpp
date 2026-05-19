#ifndef MEMBRA_MICROTASK_BACKING_HPP
#define MEMBRA_MICROTASK_BACKING_HPP

#include <string>
#include <vector>
#include <map>
#include <unordered_map>
#include <cstdint>
#include <chrono>
#include <memory>
#include <functional>
#include <mutex>

namespace membra {
namespace microtask {

// Constants
constexpr uint64_t TASK_ID_SIZE = 32;
constexpr uint64_t MIN_TASK_VALUE = 1000; // Minimum task value in lamports
constexpr uint64_t MAX_TASK_VALUE = 10000000; // Maximum task value in lamports
constexpr uint64_t DEFAULT_REPUTATION_SCORE = 1000; // Starting reputation
constexpr uint64_t MAX_REPUTATION_SCORE = 10000; // Maximum reputation
constexpr uint64_t MIN_REPUTATION_SCORE = 0; // Minimum reputation
constexpr uint32_t TASK_EXPIRY_SECONDS = 86400; // 24 hours
constexpr uint32_t COMPLETION_TIMEOUT_SECONDS = 3600; // 1 hour

// Microtask types
enum class MicrotaskType {
    DATA_LABELING,
    CONTENT_MODERATION,
    TRANSCRIPTION,
    TRANSLATION,
    VALIDATION,
    ORACLE_SUBMISSION,
    PROOF_VERIFICATION,
    COMPUTE_TASK,
    QUALITY_ASSURANCE,
    SENTIMENT_ANALYSIS,
    IMAGE_CLASSIFICATION,
    TEXT_SUMMARIZATION,
    CODE_REVIEW,
    TESTING,
    DOCUMENTATION
};

// Convert microtask type to string
inline std::string microtaskTypeToString(MicrotaskType type) {
    switch (type) {
        case MicrotaskType::DATA_LABELING: return "DATA_LABELING";
        case MicrotaskType::CONTENT_MODERATION: return "CONTENT_MODERATION";
        case MicrotaskType::TRANSCRIPTION: return "TRANSCRIPTION";
        case MicrotaskType::TRANSLATION: return "TRANSLATION";
        case MicrotaskType::VALIDATION: return "VALIDATION";
        case MicrotaskType::ORACLE_SUBMISSION: return "ORACLE_SUBMISSION";
        case MicrotaskType::PROOF_VERIFICATION: return "PROOF_VERIFICATION";
        case MicrotaskType::COMPUTE_TASK: return "COMPUTE_TASK";
        case MicrotaskType::QUALITY_ASSURANCE: return "QUALITY_ASSURANCE";
        case MicrotaskType::SENTIMENT_ANALYSIS: return "SENTIMENT_ANALYSIS";
        case MicrotaskType::IMAGE_CLASSIFICATION: return "IMAGE_CLASSIFICATION";
        case MicrotaskType::TEXT_SUMMARIZATION: return "TEXT_SUMMARIZATION";
        case MicrotaskType::CODE_REVIEW: return "CODE_REVIEW";
        case MicrotaskType::TESTING: return "TESTING";
        case MicrotaskType::DOCUMENTATION: return "DOCUMENTATION";
        default: return "UNKNOWN";
    }
}

// Task status
enum class TaskStatus {
    PENDING,
    ASSIGNED,
    IN_PROGRESS,
    COMPLETED,
    VERIFIED,
    REJECTED,
    EXPIRED,
    CANCELLED
};

// Convert task status to string
inline std::string taskStatusToString(TaskStatus status) {
    switch (status) {
        case TaskStatus::PENDING: return "PENDING";
        case TaskStatus::ASSIGNED: return "ASSIGNED";
        case TaskStatus::IN_PROGRESS: return "IN_PROGRESS";
        case TaskStatus::COMPLETED: return "COMPLETED";
        case TaskStatus::VERIFIED: return "VERIFIED";
        case TaskStatus::REJECTED: return "REJECTED";
        case TaskStatus::EXPIRED: return "EXPIRED";
        case TaskStatus::CANCELLED: return "CANCELLED";
        default: return "UNKNOWN";
    }
}

// Task difficulty
enum class TaskDifficulty {
    TRIVIAL,
    EASY,
    MEDIUM,
    HARD,
    EXPERT
};

// Convert difficulty to value multiplier
inline double difficultyToMultiplier(TaskDifficulty difficulty) {
    switch (difficulty) {
        case TaskDifficulty::TRIVIAL: return 1.0;
        case TaskDifficulty::EASY: return 1.5;
        case TaskDifficulty::MEDIUM: return 2.0;
        case TaskDifficulty::HARD: return 3.0;
        case TaskDifficulty::EXPERT: return 5.0;
        default: return 1.0;
    }
}

// Utility function declarations
std::vector<MicrotaskType> getAllTaskTypes();
std::vector<TaskStatus> getAllTaskStatuses();

// Microtask definition
struct Microtask {
    std::string task_id;
    MicrotaskType type;
    TaskStatus status;
    TaskDifficulty difficulty;
    std::string creator_address;
    std::string worker_address;
    std::string title;
    std::string description;
    std::map<std::string, std::string> metadata;
    uint64_t base_value;
    uint64_t total_value;
    uint64_t created_at;
    uint64_t assigned_at;
    uint64_t completed_at;
    uint64_t verified_at;
    uint64_t expires_at;
    std::string backing_transaction_id;
    std::string proof_hash;
    std::string result_hash;
    std::string verification_signature;
    
    Microtask() : type(MicrotaskType::DATA_LABELING),
                  status(TaskStatus::PENDING),
                  difficulty(TaskDifficulty::MEDIUM),
                  base_value(0),
                  total_value(0),
                  created_at(0),
                  assigned_at(0),
                  completed_at(0),
                  verified_at(0),
                  expires_at(0) {}
};

// Task result
struct TaskResult {
    std::string result_id;
    std::string task_id;
    std::string worker_address;
    std::string result_data;
    std::string result_hash;
    std::vector<uint8_t> proof;
    uint64_t submitted_at;
    uint32_t quality_score;
    bool verified;
    std::string verification_signature;
    
    TaskResult() : submitted_at(0), quality_score(0), verified(false) {}
};

// Worker reputation
struct WorkerReputation {
    std::string worker_address;
    uint64_t reputation_score;
    uint64_t tasks_completed;
    uint64_t tasks_verified;
    uint64_t tasks_rejected;
    uint64_t total_earned;
    double average_quality;
    uint64_t last_activity;
    
    WorkerReputation() : reputation_score(DEFAULT_REPUTATION_SCORE),
                         tasks_completed(0),
                         tasks_verified(0),
                         tasks_rejected(0),
                         total_earned(0),
                         average_quality(0.0),
                         last_activity(0) {}
};

// Backing pool for transaction backing
struct BackingPool {
    std::string pool_id;
    std::string transaction_id;
    uint64_t transaction_value;
    uint64_t backing_value;
    uint64_t required_tasks;
    uint64_t completed_tasks;
    uint64_t verified_tasks;
    uint64_t created_at;
    uint64_t expires_at;
    bool fully_backed;
    bool verified;
    std::vector<std::string> task_ids;
    
    BackingPool() : transaction_value(0),
                    backing_value(0),
                    required_tasks(0),
                    completed_tasks(0),
                    verified_tasks(0),
                    created_at(0),
                    expires_at(0),
                    fully_backed(false),
                    verified(false) {}
};

// Value conversion configuration
struct ValueConversionConfig {
    uint64_t base_task_value;
    double difficulty_multiplier;
    double reputation_multiplier;
    double urgency_multiplier;
    double complexity_multiplier;
    uint64_t max_task_value;
    uint64_t min_task_value;
    
    ValueConversionConfig() : base_task_value(1000),
                              difficulty_multiplier(1.0),
                              reputation_multiplier(1.0),
                              urgency_multiplier(1.0),
                              complexity_multiplier(1.0),
                              max_task_value(MAX_TASK_VALUE),
                              min_task_value(MIN_TASK_VALUE) {}
};

// Microtask manager
class MicrotaskManager {
private:
    std::map<std::string, Microtask> tasks_;
    std::map<std::string, TaskResult> results_;
    std::map<std::string, WorkerReputation> reputations_;
    std::map<std::string, BackingPool> backing_pools_;
    ValueConversionConfig config_;
    std::mutex mutex_;
    
    uint64_t getCurrentTimestamp() {
        return std::chrono::duration_cast<std::chrono::seconds>(
            std::chrono::system_clock::now().time_since_epoch()
        ).count();
    }
    
    std::string generateTaskId() {
        return "task_" + std::to_string(getCurrentTimestamp()) + "_" + 
               std::to_string(rand() % 10000);
    }
    
    std::string generateResultId() {
        return "result_" + std::to_string(getCurrentTimestamp()) + "_" + 
               std::to_string(rand() % 10000);
    }
    
    std::string generatePoolId() {
        return "pool_" + std::to_string(getCurrentTimestamp()) + "_" + 
               std::to_string(rand() % 10000);
    }
    
    uint64_t calculateTaskValue(MicrotaskType, TaskDifficulty difficulty, 
                                double reputation_score) {
        uint64_t base = config_.base_task_value;
        double multiplier = difficultyToMultiplier(difficulty);
        
        // Apply reputation multiplier (higher reputation = slightly higher value)
        double rep_mult = 1.0 + (reputation_score - DEFAULT_REPUTATION_SCORE) / 10000.0;
        
        uint64_t value = static_cast<uint64_t>(base * multiplier * rep_mult);
        
        // Clamp to min/max
        if (value < config_.min_task_value) value = config_.min_task_value;
        if (value > config_.max_task_value) value = config_.max_task_value;
        
        return value;
    }
    
public:
    MicrotaskManager() = default;
    
    // Create a new microtask
    std::string createTask(const std::string& creator_address,
                          MicrotaskType type,
                          TaskDifficulty difficulty,
                          const std::string& title,
                          const std::string& description,
                          const std::map<std::string, std::string>& metadata,
                          const std::string& backing_transaction_id = "") {
        std::lock_guard<std::mutex> lock(mutex_);
        
        Microtask task;
        task.task_id = generateTaskId();
        task.type = type;
        task.status = TaskStatus::PENDING;
        task.difficulty = difficulty;
        task.creator_address = creator_address;
        task.title = title;
        task.description = description;
        task.metadata = metadata;
        task.backing_transaction_id = backing_transaction_id;
        task.created_at = getCurrentTimestamp();
        task.expires_at = task.created_at + TASK_EXPIRY_SECONDS;
        
        // Calculate task value
        WorkerReputation rep = reputations_[creator_address];
        task.base_value = calculateTaskValue(type, difficulty, rep.reputation_score);
        task.total_value = task.base_value;
        
        tasks_[task.task_id] = task;
        return task.task_id;
    }
    
    // Assign task to worker
    bool assignTask(const std::string& task_id, const std::string& worker_address) {
        std::lock_guard<std::mutex> lock(mutex_);
        
        auto it = tasks_.find(task_id);
        if (it == tasks_.end() || it->second.status != TaskStatus::PENDING) {
            return false;
        }
        
        // Check if task is expired
        if (getCurrentTimestamp() > it->second.expires_at) {
            it->second.status = TaskStatus::EXPIRED;
            return false;
        }
        
        it->second.status = TaskStatus::ASSIGNED;
        it->second.worker_address = worker_address;
        it->second.assigned_at = getCurrentTimestamp();
        
        return true;
    }
    
    // Submit task result
    std::string submitResult(const std::string& task_id,
                            const std::string& worker_address,
                            const std::string& result_data,
                            const std::vector<uint8_t>& proof) {
        std::lock_guard<std::mutex> lock(mutex_);
        
        auto it = tasks_.find(task_id);
        if (it == tasks_.end() || it->second.status != TaskStatus::ASSIGNED) {
            return "";
        }
        
        if (it->second.worker_address != worker_address) {
            return "";
        }
        
        TaskResult result;
        result.result_id = generateResultId();
        result.task_id = task_id;
        result.worker_address = worker_address;
        result.result_data = result_data;
        result.proof = proof;
        result.submitted_at = getCurrentTimestamp();
        
        // Simple hash for result
        std::string hash_input = result_data + std::to_string(result.submitted_at);
        result.result_hash = std::to_string(std::hash<std::string>{}(hash_input));
        
        results_[result.result_id] = result;
        
        it->second.status = TaskStatus::COMPLETED;
        it->second.completed_at = result.submitted_at;
        it->second.result_hash = result.result_hash;
        
        return result.result_id;
    }
    
    // Verify task result
    bool verifyResult(const std::string& result_id,
                     uint32_t quality_score,
                     const std::string& verification_signature) {
        std::lock_guard<std::mutex> lock(mutex_);
        
        auto result_it = results_.find(result_id);
        if (result_it == results_.end()) {
            return false;
        }
        
        auto task_it = tasks_.find(result_it->second.task_id);
        if (task_it == tasks_.end() || task_it->second.status != TaskStatus::COMPLETED) {
            return false;
        }
        
        result_it->second.quality_score = quality_score;
        result_it->second.verified = quality_score >= 50; // Minimum quality threshold
        result_it->second.verification_signature = verification_signature;
        
        if (result_it->second.verified) {
            task_it->second.status = TaskStatus::VERIFIED;
            task_it->second.verified_at = getCurrentTimestamp();
            task_it->second.verification_signature = verification_signature;
            
            // Update worker reputation
            updateReputation(result_it->second.worker_address, quality_score, true);
        } else {
            task_it->second.status = TaskStatus::REJECTED;
            updateReputation(result_it->second.worker_address, quality_score, false);
        }
        
        return result_it->second.verified;
    }
    
    // Update worker reputation
    void updateReputation(const std::string& worker_address, 
                         uint32_t quality_score,
                         bool successful) {
        auto& rep = reputations_[worker_address];
        
        if (successful) {
            rep.tasks_completed++;
            rep.tasks_verified++;
            rep.total_earned += 100; // Base reward
            
            // Update reputation score based on quality
            double rep_change = (quality_score - 50) * 2.0;
            rep.reputation_score = std::min(MAX_REPUTATION_SCORE, 
                                           rep.reputation_score + static_cast<uint64_t>(rep_change));
            
            // Update average quality
            double total_quality = rep.average_quality * (rep.tasks_completed - 1) + quality_score;
            rep.average_quality = total_quality / rep.tasks_completed;
        } else {
            rep.tasks_rejected++;
            rep.reputation_score = std::max(MIN_REPUTATION_SCORE,
                                           rep.reputation_score - 50);
        }
        
        rep.last_activity = getCurrentTimestamp();
    }
    
    // Create backing pool for transaction
    std::string createBackingPool(const std::string& transaction_id,
                                 uint64_t transaction_value,
                                 uint64_t required_tasks) {
        std::lock_guard<std::mutex> lock(mutex_);
        
        BackingPool pool;
        pool.pool_id = generatePoolId();
        pool.transaction_id = transaction_id;
        pool.transaction_value = transaction_value;
        pool.required_tasks = required_tasks;
        pool.created_at = getCurrentTimestamp();
        pool.expires_at = pool.created_at + TASK_EXPIRY_SECONDS;
        
        // Calculate required backing value (tasks should cover transaction value)
        uint64_t task_value = transaction_value / required_tasks;
        pool.backing_value = task_value * required_tasks;
        
        backing_pools_[pool.pool_id] = pool;
        return pool.pool_id;
    }
    
    // Add task to backing pool
    bool addTaskToPool(const std::string& pool_id, const std::string& task_id) {
        std::lock_guard<std::mutex> lock(mutex_);
        
        auto pool_it = backing_pools_.find(pool_id);
        if (pool_it == backing_pools_.end()) {
            return false;
        }
        
        auto task_it = tasks_.find(task_id);
        if (task_it == tasks_.end()) {
            return false;
        }
        
        pool_it->second.task_ids.push_back(task_id);
        
        // Update task to reference backing pool
        task_it->second.backing_transaction_id = pool_it->second.transaction_id;
        
        return true;
    }
    
    // Update pool status based on task completion
    void updatePoolStatus(const std::string& pool_id) {
        std::lock_guard<std::mutex> lock(mutex_);
        
        auto pool_it = backing_pools_.find(pool_id);
        if (pool_it == backing_pools_.end()) {
            return;
        }
        
        uint64_t completed = 0;
        uint64_t verified = 0;
        
        for (const auto& task_id : pool_it->second.task_ids) {
            auto task_it = tasks_.find(task_id);
            if (task_it != tasks_.end()) {
                if (task_it->second.status == TaskStatus::COMPLETED || 
                    task_it->second.status == TaskStatus::VERIFIED) {
                    completed++;
                }
                if (task_it->second.status == TaskStatus::VERIFIED) {
                    verified++;
                }
            }
        }
        
        pool_it->second.completed_tasks = completed;
        pool_it->second.verified_tasks = verified;
        pool_it->second.fully_backed = completed >= pool_it->second.required_tasks;
        pool_it->second.verified = verified >= pool_it->second.required_tasks;
    }
    
    // Get task by ID
    Microtask getTask(const std::string& task_id) {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = tasks_.find(task_id);
        return it != tasks_.end() ? it->second : Microtask();
    }
    
    // Get result by ID
    TaskResult getResult(const std::string& result_id) {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = results_.find(result_id);
        return it != results_.end() ? it->second : TaskResult();
    }
    
    // Get worker reputation
    WorkerReputation getReputation(const std::string& worker_address) {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = reputations_.find(worker_address);
        return it != reputations_.end() ? it->second : WorkerReputation();
    }
    
    // Get backing pool
    BackingPool getBackingPool(const std::string& pool_id) {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = backing_pools_.find(pool_id);
        return it != backing_pools_.end() ? it->second : BackingPool();
    }
    
    // Get available tasks
    std::vector<Microtask> getAvailableTasks(MicrotaskType type_filter = MicrotaskType::DATA_LABELING) {
        std::lock_guard<std::mutex> lock(mutex_);
        std::vector<Microtask> available;
        
        for (const auto& pair : tasks_) {
            if (pair.second.status == TaskStatus::PENDING) {
                if (type_filter == MicrotaskType::DATA_LABELING || 
                    pair.second.type == type_filter) {
                    available.push_back(pair.second);
                }
            }
        }
        
        return available;
    }
    
    // Get tasks for worker
    std::vector<Microtask> getWorkerTasks(const std::string& worker_address) {
        std::lock_guard<std::mutex> lock(mutex_);
        std::vector<Microtask> worker_tasks;
        
        for (const auto& pair : tasks_) {
            if (pair.second.worker_address == worker_address) {
                worker_tasks.push_back(pair.second);
            }
        }
        
        return worker_tasks;
    }
    
    // Set value conversion config
    void setConfig(const ValueConversionConfig& config) {
        std::lock_guard<std::mutex> lock(mutex_);
        config_ = config;
    }
    
    // Get config
    ValueConversionConfig getConfig() {
        std::lock_guard<std::mutex> lock(mutex_);
        return config_;
    }
};

// Microtask backing stack (factory)
class MicrotaskBackingStack {
public:
    std::shared_ptr<MicrotaskManager> manager;
    
    MicrotaskBackingStack() : manager(std::make_shared<MicrotaskManager>()) {}
    
    // Create complete backing workflow
    std::string createBackedTransaction(const std::string& transaction_id,
                                       uint64_t transaction_value,
                                       const std::string& creator_address,
                                       uint32_t num_tasks) {
        // Create backing pool
        std::string pool_id = manager->createBackingPool(transaction_id, transaction_value, num_tasks);
        
        // Create tasks for the pool
        for (uint32_t i = 0; i < num_tasks; i++) {
            std::string task_id = manager->createTask(
                creator_address,
                MicrotaskType::VALIDATION,
                TaskDifficulty::MEDIUM,
                "Validation Task " + std::to_string(i),
                "Validate transaction data",
                {{"pool_id", pool_id}}
            );
            
            manager->addTaskToPool(pool_id, task_id);
        }
        
        return pool_id;
    }
};

} // namespace microtask
} // namespace membra

#endif // MEMBRA_MICROTASK_BACKING_HPP