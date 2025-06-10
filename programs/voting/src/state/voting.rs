use anchor_lang::prelude::*;

#[account]
pub struct Voting {
    pub authority: Pubkey,
    pub bump: u8,
    pub created_at: i64,
    pub is_paused: bool,
    pub total_polls: u64,
    pub total_votes: u64,
    pub total_users: u64
}

impl Voting {
    pub const MAX_SIZE: usize = 8 +
        32 + // authority
        1 + // bump
        8 + // created_at
        1 + // is_paused
        8 + // total_polls
        8 + // total_votes
        8; // total_users
}