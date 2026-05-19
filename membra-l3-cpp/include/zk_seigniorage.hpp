#ifndef ZK_SEIGNIORAGE_HPP
#define ZK_SEIGNIORAGE_HPP

#include <string>
#include <vector>
#include <array>
#include <cstdint>
#include <memory>
#include <map>
#include <chrono>
#include "crypto.hpp"

namespace membra {
namespace zkseigniorage {

// Constants
constexpr size_t HASH_SIZE = 32;
constexpr size_t MAX_VERIFIERS = 5;
constexpr uint8_t MIN_VERIFIERS_REQUIRED = 2;

constexpr uint64_t TARGET_CAPACITY_RATIO_BPS = 10000;
constexpr uint64_t BASE_MINT_REWARD = 1000000;
constexpr uint64_t MAX_MINT_PER_PROOF = 100000000000;
constexpr int64_t SECONDS_PER_EPOCH = 86400;

// Tier multipliers in basis points
constexpr uint64_t TIER_1_MULTIPLIER_BPS = 2500;
constexpr uint64_t TIER_2_MULTIPLIER_BPS = 5000;
constexpr uint64_t TIER_3_MULTIPLIER_BPS = 10000;
constexpr uint64_t TIER_4_MULTIPLIER_BPS = 15000;
constexpr uint64_t TIER_5_MULTIPLIER_BPS = 25000;

enum class ProofStatus : uint8_t {
    PENDING = 0,
    VERIFIED = 1,
    REJECTED = 2
};

/**
 * Verifier attestation for a capacity proof
 */
struct VerifierAttestation {
    std::string verifier;
    std::array<uint8_t, HASH_SIZE> attestation_hash;
    int64_t timestamp;
    bool signature_valid;

    VerifierAttestation()
        : timestamp(0), signature_valid(false) {
        attestation_hash.fill(0);
    }
};

/**
 * ZK proof of productive capacity
 */
struct CapacityProof {
    std::string prover;
    std::array<uint8_t, HASH_SIZE> proof_id;
    std::string collateral_lock;
    uint8_t collateral_tier;

    std::array<uint8_t, HASH_SIZE> input_commitment;
    std::array<uint8_t, HASH_SIZE> output_commitment;
    std::string circuit_identifier;

    uint64_t difficulty_score;
    uint64_t compute_units_consumed;
    uint64_t epoch;

    std::vector<VerifierAttestation> verifier_attestations;
    uint8_t attestation_count;
    ProofStatus verification_status;

    uint64_t mint_amount;
    int64_t minted_at;
    std::array<uint8_t, HASH_SIZE> nullifier_hash;

    CapacityProof()
        : collateral_tier(1),
          difficulty_score(0),
          compute_units_consumed(0),
          epoch(1),
          attestation_count(0),
          verification_status(ProofStatus::PENDING),
          mint_amount(0),
          minted_at(0) {
        proof_id.fill(0);
        input_commitment.fill(0);
        output_commitment.fill(0);
        nullifier_hash.fill(0);
    }
};

/**
 * Nullifier to prevent double-minting
 */
struct ProofNullifier {
    std::array<uint8_t, HASH_SIZE> proof_id;
    std::array<uint8_t, HASH_SIZE> nullifier_hash;
    bool used;
    int64_t used_at;

    ProofNullifier()
        : used(false), used_at(0) {
        proof_id.fill(0);
        nullifier_hash.fill(0);
    }
};

/**
 * Monetary policy configuration
 */
struct SeigniorageConfig {
    std::string authority;
    std::string token_mint;

    uint64_t base_mint_reward;
    uint64_t max_mint_per_proof;
    uint64_t target_capacity_ratio_bps;
    uint8_t min_verifiers_required;

    uint64_t tier_1_threshold;
    uint64_t tier_2_threshold;
    uint64_t tier_3_threshold;
    uint64_t tier_4_threshold;
    uint64_t tier_5_threshold;

    uint64_t total_proven_capacity;
    uint64_t total_tokens_minted;
    uint64_t current_epoch;
    int64_t last_epoch_timestamp;

    SeigniorageConfig()
        : base_mint_reward(BASE_MINT_REWARD),
          max_mint_per_proof(MAX_MINT_PER_PROOF),
          target_capacity_ratio_bps(TARGET_CAPACITY_RATIO_BPS),
          min_verifiers_required(MIN_VERIFIERS_REQUIRED),
          tier_1_threshold(1000000),
          tier_2_threshold(10000000),
          tier_3_threshold(100000000),
          tier_4_threshold(1000000000),
          tier_5_threshold(10000000000),
          total_proven_capacity(0),
          total_tokens_minted(0),
          current_epoch(1),
          last_epoch_timestamp(0) {}
};

/**
 * Per-epoch capacity ledger
 */
struct EpochCapacityLedger {
    uint64_t epoch;
    uint64_t total_proven_capacity;
    uint64_t total_proofs_verified;
    uint64_t total_tokens_minted;
    uint64_t average_difficulty;

    EpochCapacityLedger()
        : epoch(1),
          total_proven_capacity(0),
          total_proofs_verified(0),
          total_tokens_minted(0),
          average_difficulty(0) {}
};

/**
 * Monetary policy engine: calculates mint amount from verified capacity proofs
 */
class MonetaryPolicyEngine {
public:
    MonetaryPolicyEngine();
    explicit MonetaryPolicyEngine(std::shared_ptr<SeigniorageConfig> config);
    ~MonetaryPolicyEngine() = default;

    // Core formula: mint = base * tier_mult * difficulty/10000 * verifier_mult * network_adj
    uint64_t calculate_mint_amount(const CapacityProof& proof) const;

    // Counter-cyclical network adjustment
    uint64_t calculate_network_adjustment() const;

    // Update global state after a mint
    void record_mint(const CapacityProof& proof, uint64_t amount);

    // Epoch management
    void check_epoch_rollover(int64_t current_time);

    // Collateral tier derivation
    uint8_t derive_collateral_tier(uint64_t locked_value) const;

    // Config accessors
    SeigniorageConfig& config();
    const SeigniorageConfig& config() const;

private:
    std::shared_ptr<SeigniorageConfig> config_;
    std::unique_ptr<Crypto> crypto_;
};

/**
 * ZK-PoPC seigniorage manager: handles proof lifecycle and minting
 */
class ZKSeigniorageManager {
public:
    ZKSeigniorageManager();
    explicit ZKSeigniorageManager(std::shared_ptr<MonetaryPolicyEngine> engine);
    ~ZKSeigniorageManager() = default;

    // Submit a capacity proof (prover creates)
    bool submit_capacity_proof(const CapacityProof& proof);

    // Independent verifier attests to a proof
    bool attest_capacity_proof(const std::array<uint8_t, HASH_SIZE>& proof_id,
                               const std::string& verifier,
                               const std::array<uint8_t, HASH_SIZE>& attestation_hash);

    // Mint tokens from a verified proof
    uint64_t mint_from_capacity_proof(const std::array<uint8_t, HASH_SIZE>& proof_id);

    // Query proof status
    CapacityProof get_proof_status(const std::array<uint8_t, HASH_SIZE>& proof_id) const;

    // Check if nullifier is already used
    bool is_nullifier_used(const std::array<uint8_t, HASH_SIZE>& nullifier_hash) const;

    // Statistics
    struct SeigniorageStats {
        uint64_t total_proofs_submitted;
        uint64_t total_proofs_verified;
        uint64_t total_proofs_minted;
        uint64_t total_tokens_minted;
        uint64_t total_proven_capacity;
        uint64_t current_epoch;
    };
    SeigniorageStats get_stats() const;

    // Ledger access
    EpochCapacityLedger get_epoch_ledger(uint64_t epoch) const;

private:
    std::shared_ptr<MonetaryPolicyEngine> engine_;
    std::map<std::string, CapacityProof> proofs_;
    std::map<std::string, ProofNullifier> nullifiers_;
    std::map<uint64_t, EpochCapacityLedger> ledgers_;
    std::unique_ptr<Crypto> crypto_;

    SeigniorageStats stats_;

    std::string hash_to_string(const std::array<uint8_t, HASH_SIZE>& hash) const;
};

/**
 * Factory for creating a complete ZK-PoPC stack
 */
struct ZKSeigniorageStack {
    std::shared_ptr<MonetaryPolicyEngine> engine;
    std::shared_ptr<ZKSeigniorageManager> manager;
};

ZKSeigniorageStack create_zk_seigniorage_stack();

} // namespace zkseigniorage
} // namespace membra

#endif // ZK_SEIGNIORAGE_HPP
