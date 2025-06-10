use anchor_lang::prelude::*;
use crate::error::*;
use crate::state::*;
use crate::utils::*;

#[event]
pub struct VoteCast {
    pub poll: Pubkey,
    pub timestamp: i64,
    pub nullifier_hash: [u8; 32],
    pub vote_choice: u8,
    pub vote_weight: u64,
    pub total_votes: u64,
}

#[derive(Accounts)]
#[instruction(nullifier_hash: [u8; 32])]
pub struct CastVote<'info> {
    #[account(
        mut,
        constraint = poll.can_vote() @ VotingError::VotingNotAllowed
    )]
    pub poll: Account<'info, Poll>,
    
    #[account(
        mut,
        seeds = [b"poll_metadata", poll.key().as_ref()],
        bump = poll_metadata.bump
    )]
    pub poll_metadata: Account<'info, PollMetadata>,
    
    #[account(
        init,
        payer = voter,
        space = Vote::MAX_SIZE,
        seeds = [b"vote", poll.key().as_ref(), &nullifier_hash],
        bump
    )]
    pub vote: Account<'info, Vote>,
    
    #[account(
        init,
        payer = voter,
        space = VoterNullifier::MAX_SIZE,
        seeds = [b"nullifier", poll.key().as_ref(), &nullifier_hash],
        bump
    )]
    pub voter_nullifier: Account<'info, VoterNullifier>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CastVote>,
    vote_choice: u8,
    zk_proof: Vec<u8>,
    nullifier_hash: [u8; 32],
    public_signals: Vec<String>,
    vote_weight: Option<u64>,
) -> Result<()> {
    let poll = &mut ctx.accounts.poll;
    let vote = &mut ctx.accounts.vote;
    let voter_nullifier = &mut ctx.accounts.voter_nullifier;
    let poll_metadata = &mut ctx.accounts.poll_metadata;
    let moment = Clock::get()?.unix_timestamp;

    // Validate poll is active and voting is allowed
    require!(poll.is_active(), VotingError::PollNotActive);
    require!(moment <= poll.poll_end_time, VotingError::VotingEnded);
    require!(poll.can_vote(), VotingError::VotingNotAllowed);

    // Validate vote choice
    require!(
        (vote_choice as usize) < poll.options.len(),
        VotingError::InvalidVoteChoice
    );

    // Verify ZK proof
    require!(!zk_proof.is_empty(), VotingError::InvalidZkProof);
    require!(
        verify_zk_proof(&zk_proof, &public_signals, &poll.eligibility_criteria)?,
        VotingError::ZkProofVerificatiionFailed
    );

    // Calculate vote weight
    let final_vote_weight = match poll.poll_type {
        PollType::Quadratic => {
            let weight = vote_weight.unwrap_or(1);
            require!(weight > 0, VotingError::InvalidVoteWeight);
            
            if let Some(ref params) = poll.quadratic_parameters {
                require!(
                    weight <= params.max_credits,
                    VotingError::InsufficientCredits
                );
                // Calculate quadratic cost: weight^2
                weight * weight
            } else {
                weight
            }
        },
        _ => vote_weight.unwrap_or(1),
    };

    // Initialize vote record
    vote.poll = poll.key();
    vote.nullifier_hash = nullifier_hash;
    vote.vote_choice = vote_choice;
    vote.zk_proof_hash = hash_zk_proof(&zk_proof);
    vote.timestamp = moment;
    vote.delegate_from = None;
    vote.delegate_to = false;
    vote.vote_weight = final_vote_weight;
    vote.bump = ctx.bumps.vote;

    // Initialize voter nullifier
    voter_nullifier.poll = poll.key();
    voter_nullifier.nullifier_hash = nullifier_hash;
    voter_nullifier.created_at = moment;
    voter_nullifier.bump = ctx.bumps.voter_nullifier;

    // Update poll statistics
    poll.total_votes = poll.total_votes
        .checked_add(final_vote_weight)
        .ok_or(VotingError::ArithmeticOverflow)?;
    
    poll.unique_voters = poll.unique_voters
        .checked_add(1)
        .ok_or(VotingError::ArithmeticOverflow)?;
    
    poll.updated_at = moment;

    // Update vote counts array
    if poll.vote_counts.len() <= vote_choice as usize {
        poll.vote_counts.resize(poll.options.len(), 0);
    }
    
    poll.vote_counts[vote_choice as usize] = poll.vote_counts[vote_choice as usize]
        .checked_add(final_vote_weight)
        .ok_or(VotingError::ArithmeticOverflow)?;

    // Add nullifier to poll's nullifier list
    poll.nullifier_hashes.push(nullifier_hash);
    
    // Update poll metadata
    poll_metadata.nullifier_count = poll_metadata.nullifier_count
        .checked_add(1)
        .ok_or(VotingError::ArithmeticOverflow)?;

    // Emit vote cast event
    emit!(VoteCast {
        poll: poll.key(),
        timestamp: moment,
        nullifier_hash,
        vote_choice,
        vote_weight: final_vote_weight,
        total_votes: poll.total_votes,
    });

    msg!(
        "Vote cast successfully: Poll {}, Choice {}, Weight {}",
        poll.poll_name,
        vote_choice,
        final_vote_weight
    );

    Ok(())
}