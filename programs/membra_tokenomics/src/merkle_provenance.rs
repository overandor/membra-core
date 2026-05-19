//! Merkle Tree Provenance Attestation System
//! 
//! This module provides mathematical verification of token market cap and appraisal
//! based on novelty, rarity, and ease of execution through Merkle tree commitments.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak;

/// Merkle tree node for provenance tracking
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub struct MerkleNode {
    pub hash: [u8; 32],
    pub left: Option<[u8; 32]>,
    pub right: Option<[u8; 32]>,
}

impl MerkleNode {
    pub fn leaf(data: &[u8]) -> Self {
        MerkleNode {
            hash: keccak::hashv(&[data]).to_bytes(),
            left: None,
            right: None,
        }
    }

    pub fn internal(left: [u8; 32], right: [u8; 32]) -> Self {
        let hash = keccak::hashv(&[&left, &right]).to_bytes();
        MerkleNode {
            hash,
            left: Some(left),
            right: Some(right),
        }
    }
}

/// Provenance attestation for token appraisal
#[account]
#[derive(Default)]
pub struct ProvenanceAttestation {
    /// Unique attestation ID
    pub attestation_id: u64,
    /// Token mint this attestation covers
    pub token_mint: Pubkey,
    /// Merkle root of provenance data
    pub merkle_root: [u8; 32],
    /// Market cap appraisal (in lamports)
    pub market_cap_appraisal: u64,
    /// Novelty score (0-10000 basis points)
    pub novelty_score: u16,
    /// Rarity score (0-10000 basis points)
    pub rarity_score: u16,
    /// Ease of execution score (0-10000 basis points, inverted)
    pub execution_difficulty: u16,
    /// Timestamp of attestation
    pub attestation_timestamp: i64,
    /// Authority that created the attestation
    pub authority: Pubkey,
    /// Whether attestation is verified
    pub verified: bool,
    /// Bump seed
    pub bump: u8,
}

impl ProvenanceAttestation {
    pub const LEN: usize = 8 + // discriminator
        8 + // attestation_id
        32 + // token_mint
        32 + // merkle_root
        8 + // market_cap_appraisal
        2 + // novelty_score
        2 + // rarity_score
        2 + // execution_difficulty
        8 + // attestation_timestamp
        32 + // authority
        1 + // verified
        1; // bump
}

/// Merkle proof for verification
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MerkleProof {
    pub leaf_hash: [u8; 32],
    pub siblings: Vec<[u8; 32]>,
    pub leaf_index: u64,
}

impl MerkleProof {
    pub fn verify(&self, root: &[u8; 32]) -> bool {
        let mut hash = self.leaf_hash;
        let mut index = self.leaf_index;

        for sibling in &self.siblings {
            if index % 2 == 0 {
                hash = keccak::hashv(&[&hash, sibling]).to_bytes();
            } else {
                hash = keccak::hashv(&[sibling, &hash]).to_bytes();
            }
            index /= 2;
        }

        hash == *root
    }
}

/// Token appraisal metrics
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TokenAppraisal {
    /// Novelty factors (code uniqueness, innovation score)
    pub novelty_factors: Vec<u8>,
    /// Rarity factors (supply scarcity, uniqueness)
    pub rarity_factors: Vec<u8>,
    /// Execution factors (deployment complexity, technical barriers)
    pub execution_factors: Vec<u8>,
    /// Historical performance data
    pub historical_data: Vec<u8>,
}

impl TokenAppraisal {
    pub fn compute_merkle_root(&self) -> [u8; 32] {
        let mut data = Vec::new();
        data.extend_from_slice(&self.novelty_factors);
        data.extend_from_slice(&self.rarity_factors);
        data.extend_from_slice(&self.execution_factors);
        data.extend_from_slice(&self.historical_data);
        keccak::hashv(&[&data]).to_bytes()
    }

    pub fn calculate_scores(&self) -> (u16, u16, u16) {
        // Calculate novelty score from factors
        let novelty = if self.novelty_factors.is_empty() {
            0
        } else {
            let sum: u32 = self.novelty_factors.iter().map(|&x| x as u32).sum();
            ((sum / self.novelty_factors.len() as u32) * 100) as u16
        };

        // Calculate rarity score from factors
        let rarity = if self.rarity_factors.is_empty() {
            0
        } else {
            let sum: u32 = self.rarity_factors.iter().map(|&x| x as u32).sum();
            ((sum / self.rarity_factors.len() as u32) * 100) as u16
        };

        // Calculate execution difficulty (inverted - higher = harder)
        let execution = if self.execution_factors.is_empty() {
            0
        } else {
            let sum: u32 = self.execution_factors.iter().map(|&x| x as u32).sum();
            ((sum / self.execution_factors.len() as u32) * 100) as u16
        };

        (novelty.min(10000), rarity.min(10000), execution.min(10000))
    }
}

/// Market cap calculator based on appraisal scores
pub struct MarketCapCalculator;

impl MarketCapCalculator {
    pub fn calculate_appraisal(
        base_value: u64,
        novelty_score: u16,
        rarity_score: u16,
        execution_difficulty: u16,
    ) -> u64 {
        // Formula: base * (1 + novelty/10000) * (1 + rarity/10000) * (1 + difficulty/10000)
        let novelty_multiplier = 10_000 + novelty_score as u64;
        let rarity_multiplier = 10_000 + rarity_score as u64;
        let difficulty_multiplier = 10_000 + execution_difficulty as u64;

        let total = base_value
            * novelty_multiplier
            * rarity_multiplier
            * difficulty_multiplier
            / 1_000_000_000; // Normalize by 10000^3

        total
    }
}

/// Note: Error codes are defined in the main MembraTokenomicsError enum in lib.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_proof_verification() {
        let leaf_data = b"test_leaf_data";
        let leaf = MerkleNode::leaf(leaf_data);
        
        let proof = MerkleProof {
            leaf_hash: leaf.hash,
            siblings: vec![],
            leaf_index: 0,
        };

        // Single node tree - leaf is root
        assert!(proof.verify(&leaf.hash));
    }

    #[test]
    fn test_appraisal_score_calculation() {
        let appraisal = TokenAppraisal {
            novelty_factors: vec![80, 90, 85],
            rarity_factors: vec![70, 75, 80],
            execution_factors: vec![60, 65, 70],
            historical_data: vec![],
        };

        let (novelty, rarity, execution) = appraisal.calculate_scores();
        assert!(novelty > 8000 && novelty <= 10000);
        assert!(rarity > 7000 && rarity <= 10000);
        assert!(execution > 6000 && execution <= 10000);
    }

    #[test]
    fn test_market_cap_calculation() {
        let base = 1_000_000; // 1 SOL
        let appraisal = MarketCapCalculator::calculate_appraisal(base, 5000, 3000, 2000);
        
        // Should be higher than base due to positive scores
        assert!(appraisal > base);
    }
}