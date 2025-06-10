use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::*;

#[event]
pub struct PollCreated {
    pub poll: Pubkey,
    pub authority: Pubkey,
    pub poll_name: String,
    pub poll_type: PollType,
    pub start_time: i64,
    pub end_time: i64,
    pub max_voters: Option<u64>,
    pub created_at: i64,
}

#[derive(Accounts)]
#[instruction(poll_name: String)]
pub struct CreatePoll<'info> {
    #[account(
        init,
        payer = authority,
        space = Poll::MAX_SIZE,
        seeds = [b"poll", authority.key().as_ref(), poll_name.as_bytes()],
        bump
    )]
    pub poll: Account<'info, Poll>,
    
    #[account(
        init,
        payer = authority,
        space = PollMetadata::MAX_SIZE,
        seeds = [b"poll_metadata", poll.key().as_ref()],
        bump
    )]
    pub poll_metadata: Account<'info, PollMetadata>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreatePoll>,
    poll_name: String,
    options: Vec<String>,
    description: String,
    eligibility_criteria: EligibilityCriteria,
    poll_begin_time: i64,
    poll_end_time: i64,
    max_voters: Option<u64>,
    poll_type: PollType,
    geolocation_required: bool,
    quadratic_parameters: Option<QuadraticParameters>,
    allow_recast: bool,
    recast_vote_window: Option<i64>,
) -> Result<()> {
    let poll = &mut ctx.accounts.poll;
    let poll_metadata = &mut ctx.accounts.poll_metadata;
    let current_time = Clock::get()?.unix_timestamp;

    // Validate inputs
    require!(poll_name.len() <= 100, VotingError::PollNameTooLong);
    require!(description.len() <= 500, VotingError::PollDescriptionTooLong);
    require!(options.len() >= 2 && options.len() <= 10, VotingError::TooManyOptions);
    
    for option in &options {
        require!(option.len() <= 50, VotingError::OptionTooLong);
    }

    // Validate timing
    require!(poll_begin_time >= current_time, VotingError::PollStartTimeInPast);
    require!(poll_end_time > poll_begin_time, VotingError::InvalidPollEndTime);
    
    let poll_duration = poll_end_time - poll_begin_time;
    require!(poll_duration >= 3600, VotingError::InvalidPollDuration); // At least 1 hour
    require!(poll_duration <= 31536000, VotingError::InvalidPollDuration); // At most 1 year

    // Validate recast window if provided
    if let Some(recast_window) = recast_vote_window {
        require!(allow_recast, VotingError::RecastNotAllowed);
        require!(recast_window > 0 && recast_window <= poll_duration, VotingError::InvalidPollDuration);
    }

    // Validate quadratic parameters if provided
    if let Some(ref quad_params) = quadratic_parameters {
        require!(poll_type == PollType::Quadratic, VotingError::InvalidPollDuration);
        require!(quad_params.max_credits > 0, VotingError::InvalidVoteWeight);
        require!(!quad_params.credit_cost_curve.is_empty(), VotingError::InvalidVoteWeight);
    }

    // Initialize poll
    poll.authority = ctx.accounts.authority.key();
    poll.poll_name = poll_name.clone();
    poll.options = options.clone();
    poll.description = description;
    poll.eligibility_criteria = eligibility_criteria;
    poll.status = if poll_begin_time <= current_time {
        PollStatus::Active
    } else {
        PollStatus::Pending
    };
    poll.poll_type = poll_type.clone();
    poll.poll_begin_time = poll_begin_time;
    poll.poll_end_time = poll_end_time;
    poll.max_voters = max_voters;
    poll.total_votes = 0;
    poll.nullifier_hashes = Vec::new();
    poll.vote_counts = vec![0; options.len()];
    poll.recast_vote_window = recast_vote_window;
    poll.unique_voters = 0;
    poll.created_at = current_time;
    poll.updated_at = current_time;
    poll.allow_recast = allow_recast;
    poll.geolocation_required = geolocation_required;
    poll.quadratic_parameters = quadratic_parameters;
    poll.bump = ctx.bumps.poll;

    // Initialize poll metadata
    poll_metadata.poll = poll.key();
    poll_metadata.nullifier_count = 0;
    poll_metadata.bump = ctx.bumps.poll_metadata;

    // Emit poll created event
    emit!(PollCreated {
        poll: poll.key(),
        authority: ctx.accounts.authority.key(),
        poll_name: poll_name.clone(),
        poll_type: poll_type.clone(),
        start_time: poll_begin_time,
        end_time: poll_end_time,
        max_voters,
        created_at: current_time,
    });

    msg!(
        "Poll '{}' created successfully by {} with {} options",
        poll_name,
        ctx.accounts.authority.key(),
        options.len()
    );

    Ok(())
}