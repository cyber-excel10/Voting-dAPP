use anchor_lang::prelude::*;
use super::{PollType, EligibilityCriteria, PollStatus};

#[account]
pub struct Poll {
    pub authority: Pubkey,
    pub poll_name: String,
    pub options: Vec<String>,
    pub description: String,
    pub eligibility_criteria: EligibilityCriteria,
    pub status: PollStatus,
    pub poll_type: PollType,
    pub poll_begin_time: i64,
    pub poll_end_time: i64,
    pub max_voters: Option<u64>,
    pub total_votes: u64,
    pub nullifier_hashes: Vec<[u8; 32]>,
    pub vote_counts: Vec<u64>,
    pub recast_vote_window: Option<i64>,
    pub unique_voters: u64,
    pub created_at: i64,
    pub updated_at: i64,
    pub allow_recast: bool,
    pub geolocation_required: bool,
    pub quadratic_parameters: Option<(QuadraticParameters)>,
    pub bump: u8,
}

#[account] 
pub struct PollMetadata {
    pub poll: Pubkey,
    pub bump: u8,
    pub nullifier_count: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct QuadraticParameters {
    pub credit_cost_curve: Vec<u64>,
    pub max_credits: u64,
}

// implementation of PollMetadata..
impl PollMetadata {
    pub const MAX_SIZE: usize = 8 + 32 + 8 + 1;
}
// Poll Implementation 
impl Poll {
    // Poll Logic
    pub const MAX_SIZE: usize = 8 + 
        32 + // author
        4 + 100 + // poll_name
        4 + (10 * (4 + 50)) + // options (maximum of 10 options, each with a maximum of 50 characters)
        4 + 500 + // description
        1 + 1 + 4 + 50 + 4 + 50 + 4 + (5 * (4 + 30)) + // eligibility_criteria
        1 + // status
        1 + // poll_type
        8 + // poll_begin_time
        8 + // poll_end_time
        1 + 8 + // max_voters
        8 + // total_votes
        4 + (1000 * 32) + // nullifier_hashes
        4 + (10 * 8) + // vote_counts
        1 + 8 + // recast_vote_window
        8 + // unique_voters
        8 + // created_at
        8 + // updated_at
        1 + // allow_recast
        1 + // geolocation_required
        1 + (8 + 4 + (10 * 8)) + // quadratic_parameters
        1; // bump


    pub fn is_active(&self) -> bool {
        let moment =  Clock::get().unwrap().unix_timestamp;
        self.status == PollStatus::Active && moment >= self.poll_begin_time && moment <= self.poll_end_time
    }

    pub fn can_vote(&self) -> bool {
        self.is_active() && (self.max_voters.is_none() || self.unique_voters < self.max_voters.unwrap())
    }
}

