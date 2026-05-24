#include "zk_seigniorage.hpp"
#include <iostream>
#include <cassert>
#include <cstring>

using namespace membra::zkseigniorage;

static std::array<uint8_t, HASH_SIZE> make_hash(uint8_t b) {
    std::array<uint8_t, HASH_SIZE> h{};
    h.fill(b);
    return h;
}

void test_basic_mint_calculation() {
    auto stack = create_zk_seigniorage_stack();
    CapacityProof proof;
    proof.proof_id = make_hash(1);
    proof.collateral_tier = 3;
    proof.difficulty_score = 5000;
    proof.verification_status = ProofStatus::VERIFIED;
    proof.attestation_count = 2;
    proof.nullifier_hash = make_hash(2);

    uint64_t amount = stack.engine->calculate_mint_amount(proof);
    // base(1M) * tier(10k=1.0) * diff(5k=0.5) * ver(10k=1.0) * net(10k=1.0) / 10T = 500k
    assert(amount == 500000);
    std::cout << "PASS: test_basic_mint_calculation (amount=" << amount << ")" << std::endl;
}

void test_tier_and_verifier_bonus() {
    auto stack = create_zk_seigniorage_stack();
    CapacityProof proof;
    proof.proof_id = make_hash(3);
    proof.collateral_tier = 5;
    proof.difficulty_score = 10000;
    proof.verification_status = ProofStatus::VERIFIED;
    proof.attestation_count = 4;
    proof.nullifier_hash = make_hash(4);

    uint64_t amount = stack.engine->calculate_mint_amount(proof);
    // base(1M) * tier(25k=2.5) * diff(10k=1.0) * ver(12k=1.2 capped) * net(10k=1.0) / 10T = 3M
    assert(amount == 3000000);
    std::cout << "PASS: test_tier_and_verifier_bonus (amount=" << amount << ")" << std::endl;
}

void test_counter_cyclical_adjustment() {
    auto stack = create_zk_seigniorage_stack();
    // Simulate saturated network
    stack.engine->config().total_tokens_minted = 100000000;
    stack.engine->config().total_proven_capacity = 10000;

    uint64_t adj = stack.engine->calculate_network_adjustment();
    assert(adj == 1); // heavily suppressed
    std::cout << "PASS: test_counter_cyclical_adjustment (adj=" << adj << ")" << std::endl;

    // Reset to low ratio
    stack.engine->config().total_tokens_minted = 5000;
    stack.engine->config().total_proven_capacity = 10000;
    uint64_t adj2 = stack.engine->calculate_network_adjustment();
    assert(adj2 == 10000); // full adjustment
    std::cout << "PASS: test_counter_cyclical_adjustment low ratio (adj=" << adj2 << ")" << std::endl;
}

void test_collateral_tier_derivation() {
    auto stack = create_zk_seigniorage_stack();
    assert(stack.engine->derive_collateral_tier(500000) == 1);
    assert(stack.engine->derive_collateral_tier(50000000) == 2);
    assert(stack.engine->derive_collateral_tier(500000000) == 3);
    assert(stack.engine->derive_collateral_tier(5000000000ULL) == 4);
    assert(stack.engine->derive_collateral_tier(50000000000ULL) == 5);
    std::cout << "PASS: test_collateral_tier_derivation" << std::endl;
}

void test_max_mint_cap() {
    auto stack = create_zk_seigniorage_stack();
    stack.engine->config().max_mint_per_proof = 100;
    CapacityProof proof;
    proof.proof_id = make_hash(5);
    proof.collateral_tier = 5;
    proof.difficulty_score = 10000;
    proof.verification_status = ProofStatus::VERIFIED;
    proof.attestation_count = 5;
    proof.nullifier_hash = make_hash(6);

    uint64_t amount = stack.engine->calculate_mint_amount(proof);
    assert(amount == 100); // capped
    std::cout << "PASS: test_max_mint_cap (amount=" << amount << ")" << std::endl;
}

void test_proof_lifecycle() {
    auto stack = create_zk_seigniorage_stack();
    CapacityProof proof;
    proof.proof_id = make_hash(10);
    proof.collateral_tier = 3;
    proof.difficulty_score = 8000;
    proof.circuit_identifier = "test_circuit_v1";
    proof.compute_units_consumed = 50000;
    proof.nullifier_hash = make_hash(11);
    proof.verification_status = ProofStatus::PENDING;

    // Submit
    bool submitted = stack.manager->submit_capacity_proof(proof);
    assert(submitted);

    // Attest from verifier A
    bool att1 = stack.manager->attest_capacity_proof(proof.proof_id, "verifier_a", make_hash(20));
    assert(att1);

    // Attest from verifier B
    bool att2 = stack.manager->attest_capacity_proof(proof.proof_id, "verifier_b", make_hash(21));
    assert(att2);

    // Should now be verified
    CapacityProof status = stack.manager->get_proof_status(proof.proof_id);
    assert(status.verification_status == ProofStatus::VERIFIED);
    assert(status.attestation_count == 2);

    // Mint
    uint64_t minted = stack.manager->mint_from_capacity_proof(proof.proof_id);
    assert(minted > 0);

    // Double mint should fail
    uint64_t minted2 = stack.manager->mint_from_capacity_proof(proof.proof_id);
    assert(minted2 == 0);

    // Nullifier should be used
    assert(stack.manager->is_nullifier_used(proof.nullifier_hash));

    // Stats should reflect mint
    auto stats = stack.manager->get_stats();
    assert(stats.total_proofs_submitted == 1);
    assert(stats.total_proofs_verified == 1);
    assert(stats.total_proofs_minted == 1);
    assert(stats.total_tokens_minted == minted);

    std::cout << "PASS: test_proof_lifecycle (minted=" << minted << ")" << std::endl;
}

void test_duplicate_attestation_rejected() {
    auto stack = create_zk_seigniorage_stack();
    CapacityProof proof;
    proof.proof_id = make_hash(30);
    proof.difficulty_score = 5000;
    proof.circuit_identifier = "test";
    proof.nullifier_hash = make_hash(31);
    proof.verification_status = ProofStatus::PENDING;

    stack.manager->submit_capacity_proof(proof);
    bool att1 = stack.manager->attest_capacity_proof(proof.proof_id, "verifier_x", make_hash(40));
    assert(att1);
    bool att2 = stack.manager->attest_capacity_proof(proof.proof_id, "verifier_x", make_hash(41));
    assert(!att2); // duplicate
    std::cout << "PASS: test_duplicate_attestation_rejected" << std::endl;
}

void test_unverified_proof_cannot_mint() {
    auto stack = create_zk_seigniorage_stack();
    CapacityProof proof;
    proof.proof_id = make_hash(50);
    proof.difficulty_score = 5000;
    proof.circuit_identifier = "test";
    proof.nullifier_hash = make_hash(51);
    proof.verification_status = ProofStatus::PENDING;

    stack.manager->submit_capacity_proof(proof);
    // Only 1 attestation (below threshold of 2)
    stack.manager->attest_capacity_proof(proof.proof_id, "verifier_y", make_hash(60));

    uint64_t minted = stack.manager->mint_from_capacity_proof(proof.proof_id);
    assert(minted == 0);
    std::cout << "PASS: test_unverified_proof_cannot_mint" << std::endl;
}

int main() {
    std::cout << "=== ZK-PoPC Seigniorage Tests ===" << std::endl;
    test_basic_mint_calculation();
    test_tier_and_verifier_bonus();
    test_counter_cyclical_adjustment();
    test_collateral_tier_derivation();
    test_max_mint_cap();
    test_proof_lifecycle();
    test_duplicate_attestation_rejected();
    test_unverified_proof_cannot_mint();
    std::cout << "=== All tests passed ===" << std::endl;
    return 0;
}
