#include "zk_seigniorage.hpp"
#include <algorithm>
#include <cmath>

namespace membra {
namespace zkseigniorage {

// =============================================================================
// MONETARY POLICY ENGINE
// =============================================================================

MonetaryPolicyEngine::MonetaryPolicyEngine()
    : config_(std::make_shared<SeigniorageConfig>()),
      crypto_(std::make_unique<Crypto>()) {}

MonetaryPolicyEngine::MonetaryPolicyEngine(std::shared_ptr<SeigniorageConfig> config)
    : config_(config),
      crypto_(std::make_unique<Crypto>()) {}

uint64_t MonetaryPolicyEngine::calculate_mint_amount(const CapacityProof& proof) const {
    if (proof.verification_status != ProofStatus::VERIFIED) {
        return 0;
    }

    uint64_t base = config_->base_mint_reward;

    // Collateral tier multiplier
    uint64_t tier_mult = TIER_1_MULTIPLIER_BPS;
    switch (proof.collateral_tier) {
        case 2: tier_mult = TIER_2_MULTIPLIER_BPS; break;
        case 3: tier_mult = TIER_3_MULTIPLIER_BPS; break;
        case 4: tier_mult = TIER_4_MULTIPLIER_BPS; break;
        case 5: tier_mult = TIER_5_MULTIPLIER_BPS; break;
        default: break;
    }

    // Difficulty factor: difficulty_score / 10000
    uint64_t difficulty = proof.difficulty_score;
    if (difficulty > 10000) difficulty = 10000;

    // Verifier bonus: 1.0 + 0.1 per extra verifier, cap 1.5x
    uint64_t extra = static_cast<uint64_t>(
        std::max(0, static_cast<int>(proof.attestation_count) - static_cast<int>(config_->min_verifiers_required))
    );
    uint64_t verifier_bonus = 10000 + extra * 1000;
    if (verifier_bonus > 15000) verifier_bonus = 15000;

    // Network adjustment
    uint64_t network_adj = calculate_network_adjustment();

    // Compute: base * tier * diff * verifier * adj / 10T
    __uint128_t mint = static_cast<__uint128_t>(base);
    mint *= tier_mult;
    mint *= difficulty;
    mint *= verifier_bonus;
    mint *= network_adj;
    mint /= 10000000000000000ULL; // 10^16

    uint64_t result = static_cast<uint64_t>(mint);
    if (result > config_->max_mint_per_proof) {
        result = config_->max_mint_per_proof;
    }
    return result;
}

uint64_t MonetaryPolicyEngine::calculate_network_adjustment() const {
    if (config_->total_proven_capacity == 0) {
        return 10000; // 1.0x when no capacity yet
    }

    __uint128_t current_ratio_bps = static_cast<__uint128_t>(config_->total_tokens_minted);
    current_ratio_bps *= 10000;
    current_ratio_bps /= config_->total_proven_capacity;

    if (current_ratio_bps == 0) {
        return 10000;
    }

    __uint128_t adjustment = static_cast<__uint128_t>(config_->target_capacity_ratio_bps);
    adjustment *= 10000;
    adjustment /= current_ratio_bps;

    uint64_t adj = static_cast<uint64_t>(adjustment);
    if (adj > 10000) adj = 10000; // cap at 1.0x
    return adj;
}

void MonetaryPolicyEngine::record_mint(const CapacityProof& proof, uint64_t amount) {
    config_->total_proven_capacity += proof.difficulty_score;
    config_->total_tokens_minted += amount;
}

void MonetaryPolicyEngine::check_epoch_rollover(int64_t current_time) {
    if (current_time >= config_->last_epoch_timestamp + SECONDS_PER_EPOCH) {
        config_->current_epoch += 1;
        config_->last_epoch_timestamp = current_time;
    }
}

uint8_t MonetaryPolicyEngine::derive_collateral_tier(uint64_t locked_value) const {
    if (locked_value >= config_->tier_5_threshold) return 5;
    if (locked_value >= config_->tier_4_threshold) return 4;
    if (locked_value >= config_->tier_3_threshold) return 3;
    if (locked_value >= config_->tier_2_threshold) return 2;
    return 1;
}

SeigniorageConfig& MonetaryPolicyEngine::config() { return *config_; }
const SeigniorageConfig& MonetaryPolicyEngine::config() const { return *config_; }

// =============================================================================
// ZK SEIGNIORAGE MANAGER
// =============================================================================

ZKSeigniorageManager::ZKSeigniorageManager()
    : engine_(std::make_shared<MonetaryPolicyEngine>()),
      crypto_(std::make_unique<Crypto>()),
      stats_{0, 0, 0, 0, 0, 1} {}

ZKSeigniorageManager::ZKSeigniorageManager(std::shared_ptr<MonetaryPolicyEngine> engine)
    : engine_(engine),
      crypto_(std::make_unique<Crypto>()),
      stats_{0, 0, 0, 0, 0, 1} {}

bool ZKSeigniorageManager::submit_capacity_proof(const CapacityProof& proof) {
    std::string id = hash_to_string(proof.proof_id);
    if (proofs_.find(id) != proofs_.end()) {
        return false; // already submitted
    }
    if (proof.difficulty_score == 0 || proof.difficulty_score > 10000) {
        return false;
    }
    if (proof.circuit_identifier.empty() || proof.circuit_identifier.size() > 64) {
        return false;
    }

    proofs_[id] = proof;
    stats_.total_proofs_submitted++;
    return true;
}

bool ZKSeigniorageManager::attest_capacity_proof(
    const std::array<uint8_t, HASH_SIZE>& proof_id,
    const std::string& verifier,
    const std::array<uint8_t, HASH_SIZE>& attestation_hash) {
    std::string id = hash_to_string(proof_id);
    auto it = proofs_.find(id);
    if (it == proofs_.end()) {
        return false;
    }

    CapacityProof& proof = it->second;
    if (proof.verification_status != ProofStatus::PENDING) {
        return false;
    }
    if (proof.attestation_count >= MAX_VERIFIERS) {
        return false;
    }

    // Check for duplicate verifier
    for (const auto& att : proof.verifier_attestations) {
        if (att.verifier == verifier) {
            return false;
        }
    }

    VerifierAttestation va;
    va.verifier = verifier;
    va.attestation_hash = attestation_hash;
    va.timestamp = std::chrono::duration_cast<std::chrono::seconds>(
        std::chrono::system_clock::now().time_since_epoch()).count();
    va.signature_valid = true;

    proof.verifier_attestations.push_back(va);
    proof.attestation_count = static_cast<uint8_t>(proof.verifier_attestations.size());

    // Auto-verify if threshold reached
    if (proof.attestation_count >= engine_->config().min_verifiers_required) {
        proof.verification_status = ProofStatus::VERIFIED;
        stats_.total_proofs_verified++;
    }

    return true;
}

uint64_t ZKSeigniorageManager::mint_from_capacity_proof(
    const std::array<uint8_t, HASH_SIZE>& proof_id) {
    std::string id = hash_to_string(proof_id);
    auto it = proofs_.find(id);
    if (it == proofs_.end()) {
        return 0;
    }

    CapacityProof& proof = it->second;
    if (proof.verification_status != ProofStatus::VERIFIED) {
        return 0;
    }
    if (proof.mint_amount > 0) {
        return 0; // already minted
    }

    // Check nullifier
    std::string nf_id = hash_to_string(proof.nullifier_hash);
    auto nf_it = nullifiers_.find(nf_id);
    if (nf_it != nullifiers_.end() && nf_it->second.used) {
        return 0;
    }

    // Calculate mint
    uint64_t amount = engine_->calculate_mint_amount(proof);
    if (amount == 0) {
        return 0;
    }

    // Mark nullifier used
    ProofNullifier nf;
    nf.proof_id = proof.proof_id;
    nf.nullifier_hash = proof.nullifier_hash;
    nf.used = true;
    nf.used_at = std::chrono::duration_cast<std::chrono::seconds>(
        std::chrono::system_clock::now().time_since_epoch()).count();
    nullifiers_[nf_id] = nf;

    // Record on proof
    proof.mint_amount = amount;
    proof.minted_at = nf.used_at;

    // Update engine state
    engine_->record_mint(proof, amount);

    // Update stats
    stats_.total_proofs_minted++;
    stats_.total_tokens_minted += amount;
    stats_.total_proven_capacity += proof.difficulty_score;
    stats_.current_epoch = engine_->config().current_epoch;

    return amount;
}

CapacityProof ZKSeigniorageManager::get_proof_status(
    const std::array<uint8_t, HASH_SIZE>& proof_id) const {
    std::string id = hash_to_string(proof_id);
    auto it = proofs_.find(id);
    if (it != proofs_.end()) {
        return it->second;
    }
    return CapacityProof();
}

bool ZKSeigniorageManager::is_nullifier_used(
    const std::array<uint8_t, HASH_SIZE>& nullifier_hash) const {
    std::string nf_id = hash_to_string(nullifier_hash);
    auto it = nullifiers_.find(nf_id);
    return (it != nullifiers_.end() && it->second.used);
}

ZKSeigniorageManager::SeigniorageStats ZKSeigniorageManager::get_stats() const {
    return stats_;
}

EpochCapacityLedger ZKSeigniorageManager::get_epoch_ledger(uint64_t epoch) const {
    auto it = ledgers_.find(epoch);
    if (it != ledgers_.end()) {
        return it->second;
    }
    return EpochCapacityLedger();
}

std::string ZKSeigniorageManager::hash_to_string(const std::array<uint8_t, HASH_SIZE>& hash) const {
    std::string hex;
    hex.reserve(64);
    for (auto b : hash) {
        char buf[3];
        snprintf(buf, 3, "%02x", b);
        hex += buf;
    }
    return hex;
}

// =============================================================================
// FACTORY
// =============================================================================

ZKSeigniorageStack create_zk_seigniorage_stack() {
    ZKSeigniorageStack stack;
    stack.engine = std::make_shared<MonetaryPolicyEngine>();
    stack.manager = std::make_shared<ZKSeigniorageManager>(stack.engine);
    return stack;
}

} // namespace zkseigniorage
} // namespace membra
