use anchor_lang::prelude::*;

#[account]
pub struct Vote {
    pub poll: Pubkey,
    pub nullifier_hash: [u8; 32],
    pub vote_choice: u8,
    pub zk_proof_hash: [u8; 32],
    pub timestamp: i64,
    pub delegate_from: Option<Pubkey>,
    pub delegate_to: bool,
    pub vote_weight: u64,
    pub bump : u8,
}

// Vote Implementation
impl Vote {
    pub const MAX_SIZE: usize = 8 + 
        32 + // poll
        32 + // nullifier_hash
        1 + // vote_choice
        32 + // zk_proof_hash
        8 + // timestamp
        1+ 32 + // delegate_from
        1 + // delegate_to
        8 ; // vote_weight
}

#[account]
pub struct VoteDelegation {
    pub poll: Pubkey,
    pub o_delegator: Pubkey,
    pub o_delegatee: Pubkey,
    pub is_active: bool,
    pub created_at: i64,
    pub expires_at: Option<i64>,
    pub bump: u8,
}

#[account]
pub struct VoterNullifier {
    pub poll: Pubkey,
    pub created_at: i64,
    pub bump: u8,
    pub nullifier_hash: [u8; 32],
}

impl VoterNullifier {
    pub const MAX_SIZE: usize = 8 + 
        32 + // poll
        8 + // created_at
        1 + // bump
        32; // nullifier_hash
}

impl VoteDelegation {
    pub const MAX_SIZE: usize = 8 + 
        32 + // poll
        32 + // o_delegator
        32 + // o_delegatee
        1 + // is_active
        8 + // created_at
        1 + 8 + // expires_at
        1;
}