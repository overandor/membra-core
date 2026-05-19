#include "../include/microtask_backing.hpp"
#include <cassert>
#include <iostream>
#include <thread>
#include <chrono>

using namespace membra::microtask;

void test_microtask_creation() {
    std::cout << "Testing Microtask Creation..." << std::endl;
    
    MicrotaskManager manager;
    
    std::string task_id = manager.createTask(
        "creator_001",
        MicrotaskType::DATA_LABELING,
        TaskDifficulty::MEDIUM,
        "Test Task",
        "Test Description",
        {{"key", "value"}}
    );
    
    assert(!task_id.empty());
    
    Microtask task = manager.getTask(task_id);
    assert(task.task_id == task_id);
    assert(task.status == TaskStatus::PENDING);
    assert(task.type == MicrotaskType::DATA_LABELING);
    assert(task.difficulty == TaskDifficulty::MEDIUM);
    assert(task.creator_address == "creator_001");
    assert(task.base_value > 0);
    
    std::cout << "  ✓ Task creation successful" << std::endl;
}

void test_task_assignment() {
    std::cout << "Testing Task Assignment..." << std::endl;
    
    MicrotaskManager manager;
    
    std::string task_id = manager.createTask(
        "creator_001",
        MicrotaskType::VALIDATION,
        TaskDifficulty::EASY,
        "Assignment Test",
        "Test assignment",
        {}
    );
    
    bool assigned = manager.assignTask(task_id, "worker_001");
    assert(assigned);
    
    Microtask task = manager.getTask(task_id);
    assert(task.status == TaskStatus::ASSIGNED);
    assert(task.worker_address == "worker_001");
    assert(task.assigned_at > 0);
    
    // Test double assignment (should fail)
    bool double_assigned = manager.assignTask(task_id, "worker_002");
    assert(!double_assigned);
    
    std::cout << "  ✓ Task assignment successful" << std::endl;
}

void test_result_submission() {
    std::cout << "Testing Result Submission..." << std::endl;
    
    MicrotaskManager manager;
    
    std::string task_id = manager.createTask(
        "creator_001",
        MicrotaskType::TRANSCRIPTION,
        TaskDifficulty::HARD,
        "Submission Test",
        "Test submission",
        {}
    );
    
    manager.assignTask(task_id, "worker_001");
    
    std::string result_id = manager.submitResult(
        task_id,
        "worker_001",
        "Test result data",
        {1, 2, 3, 4}
    );
    
    assert(!result_id.empty());
    
    TaskResult result = manager.getResult(result_id);
    assert(result.result_id == result_id);
    assert(result.task_id == task_id);
    assert(result.worker_address == "worker_001");
    assert(result.result_data == "Test result data");
    assert(!result.result_hash.empty());
    
    Microtask task = manager.getTask(task_id);
    assert(task.status == TaskStatus::COMPLETED);
    assert(task.result_hash == result.result_hash);
    
    std::cout << "  ✓ Result submission successful" << std::endl;
}

void test_result_verification() {
    std::cout << "Testing Result Verification..." << std::endl;
    
    MicrotaskManager manager;
    
    std::string task_id = manager.createTask(
        "creator_001",
        MicrotaskType::CONTENT_MODERATION,
        TaskDifficulty::MEDIUM,
        "Verification Test",
        "Test verification",
        {}
    );
    
    manager.assignTask(task_id, "worker_001");
    std::string result_id = manager.submitResult(
        task_id,
        "worker_001",
        "High quality result data",
        {}
    );
    
    // Verify with high quality score
    bool verified = manager.verifyResult(result_id, 85, "signature_001");
    assert(verified);
    
    TaskResult result = manager.getResult(result_id);
    assert(result.verified);
    assert(result.quality_score == 85);
    
    Microtask task = manager.getTask(task_id);
    assert(task.status == TaskStatus::VERIFIED);
    
    std::cout << "  ✓ Result verification successful" << std::endl;
}

void test_result_rejection() {
    std::cout << "Testing Result Rejection..." << std::endl;
    
    MicrotaskManager manager;
    
    std::string task_id = manager.createTask(
        "creator_001",
        MicrotaskType::TRANSLATION,
        TaskDifficulty::EASY,
        "Rejection Test",
        "Test rejection",
        {}
    );
    
    manager.assignTask(task_id, "worker_001");
    std::string result_id = manager.submitResult(
        task_id,
        "worker_001",
        "Poor quality result",
        {}
    );
    
    // Verify with low quality score
    bool verified = manager.verifyResult(result_id, 30, "signature_002");
    assert(!verified);
    
    TaskResult result = manager.getResult(result_id);
    assert(!result.verified);
    assert(result.quality_score == 30);
    
    Microtask task = manager.getTask(task_id);
    assert(task.status == TaskStatus::REJECTED);
    
    std::cout << "  ✓ Result rejection successful" << std::endl;
}

void test_reputation_system() {
    std::cout << "Testing Reputation System..." << std::endl;
    
    MicrotaskManager manager;
    
    std::string worker = "worker_reputation";
    
    // Get initial reputation
    WorkerReputation initial_rep = manager.getReputation(worker);
    assert(initial_rep.reputation_score == DEFAULT_REPUTATION_SCORE);
    assert(initial_rep.tasks_completed == 0);
    
    // Create and complete a task successfully
    std::string task_id = manager.createTask(
        "creator_001",
        MicrotaskType::VALIDATION,
        TaskDifficulty::MEDIUM,
        "Reputation Test",
        "Test reputation",
        {}
    );
    
    manager.assignTask(task_id, worker);
    std::string result_id = manager.submitResult(task_id, worker, "Good result", {});
    manager.verifyResult(result_id, 80, "sig");
    
    WorkerReputation rep_after_success = manager.getReputation(worker);
    assert(rep_after_success.tasks_completed == 1);
    assert(rep_after_success.tasks_verified == 1);
    assert(rep_after_success.reputation_score > initial_rep.reputation_score);
    
    // Create and fail a task
    std::string task_id2 = manager.createTask(
        "creator_001",
        MicrotaskType::VALIDATION,
        TaskDifficulty::MEDIUM,
        "Reputation Test 2",
        "Test reputation 2",
        {}
    );
    
    manager.assignTask(task_id2, worker);
    std::string result_id2 = manager.submitResult(task_id2, worker, "Bad result", {});
    manager.verifyResult(result_id2, 30, "sig");
    
    WorkerReputation rep_after_failure = manager.getReputation(worker);
    assert(rep_after_failure.tasks_rejected == 1);
    assert(rep_after_failure.reputation_score < rep_after_success.reputation_score);
    
    std::cout << "  ✓ Reputation system successful" << std::endl;
}

void test_backing_pool() {
    std::cout << "Testing Backing Pool..." << std::endl;
    
    MicrotaskManager manager;
    
    std::string pool_id = manager.createBackingPool("tx_001", 1000000, 5);
    assert(!pool_id.empty());
    
    BackingPool pool = manager.getBackingPool(pool_id);
    assert(pool.pool_id == pool_id);
    assert(pool.transaction_id == "tx_001");
    assert(pool.transaction_value == 1000000);
    assert(pool.required_tasks == 5);
    assert(!pool.fully_backed);
    
    // Add tasks to pool
    for (int i = 0; i < 5; i++) {
        std::string task_id = manager.createTask(
            "creator_001",
            MicrotaskType::VALIDATION,
            TaskDifficulty::MEDIUM,
            "Pool Task " + std::to_string(i),
            "Task for pool",
            {}
        );
        manager.addTaskToPool(pool_id, task_id);
    }
    
    pool = manager.getBackingPool(pool_id);
    assert(pool.task_ids.size() == 5);
    
    std::cout << "  ✓ Backing pool successful" << std::endl;
}

void test_value_conversion() {
    std::cout << "Testing Value Conversion..." << std::endl;
    
    MicrotaskManager manager;
    
    // Test different difficulty levels
    std::string easy_task = manager.createTask(
        "creator_001",
        MicrotaskType::DATA_LABELING,
        TaskDifficulty::EASY,
        "Easy Task",
        "Easy task",
        {}
    );
    
    std::string hard_task = manager.createTask(
        "creator_001",
        MicrotaskType::DATA_LABELING,
        TaskDifficulty::HARD,
        "Hard Task",
        "Hard task",
        {}
    );
    
    Microtask easy = manager.getTask(easy_task);
    Microtask hard = manager.getTask(hard_task);
    
    assert(hard.total_value > easy.total_value);
    
    std::cout << "  ✓ Value conversion successful" << std::endl;
}

void test_task_types() {
    std::cout << "Testing Task Types..." << std::endl;
    
    std::vector<MicrotaskType> types = getAllTaskTypes();
    assert(!types.empty());
    
    for (auto type : types) {
        std::string type_str = microtaskTypeToString(type);
        assert(!type_str.empty());
        
        MicrotaskManager manager;
        std::string task_id = manager.createTask(
            "creator_001",
            type,
            TaskDifficulty::MEDIUM,
            "Type Test",
            "Test type",
            {}
        );
        
        Microtask task = manager.getTask(task_id);
        assert(task.type == type);
    }
    
    std::cout << "  ✓ All task types handled successfully" << std::endl;
}

void test_task_statuses() {
    std::cout << "Testing Task Statuses..." << std::endl;
    
    std::vector<TaskStatus> statuses = getAllTaskStatuses();
    assert(!statuses.empty());
    
    for (auto status : statuses) {
        std::string status_str = taskStatusToString(status);
        assert(!status_str.empty());
    }
    
    std::cout << "  ✓ All task statuses handled successfully" << std::endl;
}

void test_difficulty_multipliers() {
    std::cout << "Testing Difficulty Multipliers..." << std::endl;
    
    double trivial_mult = difficultyToMultiplier(TaskDifficulty::TRIVIAL);
    double easy_mult = difficultyToMultiplier(TaskDifficulty::EASY);
    double medium_mult = difficultyToMultiplier(TaskDifficulty::MEDIUM);
    double hard_mult = difficultyToMultiplier(TaskDifficulty::HARD);
    double expert_mult = difficultyToMultiplier(TaskDifficulty::EXPERT);
    
    assert(trivial_mult < easy_mult);
    assert(easy_mult < medium_mult);
    assert(medium_mult < hard_mult);
    assert(hard_mult < expert_mult);
    
    assert(expert_mult == 5.0);
    assert(trivial_mult == 1.0);
    
    std::cout << "  ✓ Difficulty multipliers correct" << std::endl;
}

void test_microtask_stack() {
    std::cout << "Testing Microtask Backing Stack..." << std::endl;
    
    MicrotaskBackingStack stack;
    
    std::string pool_id = stack.createBackedTransaction(
        "tx_stack_001",
        5000000,
        "creator_001",
        3
    );
    
    assert(!pool_id.empty());
    
    BackingPool pool = stack.manager->getBackingPool(pool_id);
    assert(pool.transaction_id == "tx_stack_001");
    assert(pool.transaction_value == 5000000);
    assert(pool.required_tasks == 3);
    assert(pool.task_ids.size() == 3);
    
    std::cout << "  ✓ Microtask backing stack successful" << std::endl;
}

void test_concurrent_operations() {
    std::cout << "Testing Concurrent Operations..." << std::endl;
    
    MicrotaskManager manager;
    std::vector<std::string> task_ids;
    
    // Create multiple tasks concurrently
    std::vector<std::thread> threads;
    for (int i = 0; i < 10; i++) {
        threads.emplace_back([&manager, &task_ids, i]() {
            std::string task_id = manager.createTask(
                "creator_001",
                MicrotaskType::VALIDATION,
                TaskDifficulty::MEDIUM,
                "Concurrent Task " + std::to_string(i),
                "Concurrent test",
                {}
            );
            task_ids.push_back(task_id);
        });
    }
    
    for (auto& thread : threads) {
        thread.join();
    }
    
    assert(task_ids.size() == 10);
    
    // Verify all tasks were created
    for (const auto& task_id : task_ids) {
        Microtask task = manager.getTask(task_id);
        assert(!task.task_id.empty());
    }
    
    std::cout << "  ✓ Concurrent operations successful" << std::endl;
}

void test_available_tasks_filtering() {
    std::cout << "Testing Available Tasks Filtering..." << std::endl;
    
    MicrotaskManager manager;
    
    // Create tasks of different types
    std::string task1 = manager.createTask(
        "creator_001",
        MicrotaskType::DATA_LABELING,
        TaskDifficulty::MEDIUM,
        "Data Task",
        "Data task",
        {}
    );
    
    std::string task2 = manager.createTask(
        "creator_001",
        MicrotaskType::TRANSLATION,
        TaskDifficulty::MEDIUM,
        "Translation Task",
        "Translation task",
        {}
    );
    
    // Get available tasks for specific type
    std::vector<Microtask> data_tasks = manager.getAvailableTasks(MicrotaskType::DATA_LABELING);
    assert(data_tasks.size() >= 1);
    bool found_data = false;
    for (const auto& task : data_tasks) {
        if (task.type == MicrotaskType::DATA_LABELING) {
            found_data = true;
            break;
        }
    }
    assert(found_data);
    
    std::vector<Microtask> translation_tasks = manager.getAvailableTasks(MicrotaskType::TRANSLATION);
    assert(translation_tasks.size() >= 1);
    bool found_translation = false;
    for (const auto& task : translation_tasks) {
        if (task.type == MicrotaskType::TRANSLATION) {
            found_translation = true;
            break;
        }
    }
    assert(found_translation);
    
    std::cout << "  ✓ Available tasks filtering successful" << std::endl;
}

void test_worker_task_history() {
    std::cout << "Testing Worker Task History..." << std::endl;
    
    MicrotaskManager manager;
    std::string worker = "worker_history";
    
    // Create and assign multiple tasks to worker
    for (int i = 0; i < 3; i++) {
        std::string task_id = manager.createTask(
            "creator_001",
            MicrotaskType::VALIDATION,
            TaskDifficulty::MEDIUM,
            "History Task " + std::to_string(i),
            "History test",
            {}
        );
        manager.assignTask(task_id, worker);
    }
    
    std::vector<Microtask> worker_tasks = manager.getWorkerTasks(worker);
    assert(worker_tasks.size() == 3);
    
    for (const auto& task : worker_tasks) {
        assert(task.worker_address == worker);
    }
    
    std::cout << "  ✓ Worker task history successful" << std::endl;
}

int main() {
    std::cout << "=== Microtask Backing System Test Suite ===" << std::endl;
    
    try {
        test_microtask_creation();
        test_task_assignment();
        test_result_submission();
        test_result_verification();
        test_result_rejection();
        test_reputation_system();
        test_backing_pool();
        test_value_conversion();
        test_task_types();
        test_task_statuses();
        test_difficulty_multipliers();
        test_microtask_stack();
        test_concurrent_operations();
        test_available_tasks_filtering();
        test_worker_task_history();
        
        std::cout << "\n✅ All Microtask Backing tests passed successfully!" << std::endl;
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "❌ Test failed: " << e.what() << std::endl;
        return 1;
    }
}