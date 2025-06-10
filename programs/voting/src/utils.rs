use anchor_lang::prelude::*;
use crate::state::*;
use sha2::{Sha256, Digest};
use crate::error::*;

pub mod zk_verification;
pub mod hash;
pub mod quadratic;

pub use zk_verification::*;
pub use hash::*;
pub use quadratic::*;

pub fn verify_zk_proof(
    proof: &[u8],
    public_signals: &[String],
    eligibility_criteria: &EligibilityCriteria,
) -> Result<bool> {
    require!(!proof.is_empty(), VotingError::InvalidZKProof);
    require!(!public_signals.is_empty(), VotingError::InvalidZKProof);

    require!(proof.len() >= 32, VotingError::InvalidZKProof);
    require!(proof.len() <= 1024, VotingError::InvalidZKProof);

    // public signls validation
        if eligibility_criteria.require_did {
        require!(
            public_signals.iter().any(|s| s.starts_with("did:")),
            VotingError::ZKProofVerificationFailed
        );
    }

        if let Some(min_age) = eligibility_criteria.min_age {
        // Check if age proof is included
        let has_age_proof = public_signals.iter().any(|s| {
            if let Ok(age) = s.parse::<u8>() {
                age >= min_age
            } else {
                false
            }
        });
        require!(has_age_proof, VotingError::ZKProofVerificationFailed);
    }
    Ok(true)
}