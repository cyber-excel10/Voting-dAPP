use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::*;
use crate::utils::*;

#[event]
pub struct VoteRecast {
    pub poll: Pubkey,
    pub timestamp: i64,
    pub nullifier_hash: [u8; 32],
    pub old_vote_weight: u64,
    pub new_vote_choice: u8,
    pub old_vote_choice: u8,
    pub new_vote_weight: u64,
}

#[derive(Accounts)]
#[instruction(nullifier_hash: [u8; 32])]
pub struct RecastVote<'info> {
    #[account(mut, constraint = poll.allow_recast @ VotingError::RecastNotAllowed)]
    pub poll: Account<'info, Poll>,
    #[account(seeds =  [b"poll_metadata", poll.key().as_ref()],
    bump = poll_metadata.bump)]
    pub poll_metadata: Account<'info, PollMetadata>,

    #[account(seeds = [b"nullifier", poll.key().as_ref(), &nullifier_hash],
    bump = voter_nullifier.bump,
    constraint = voter_nullifier.nullifier_hash == nullifier_hash @ VotingError::InvalidNullifier)]
    pub voter_nullifier: Account<'info, VoterNullifier>,
    #[account(mut, seeds = [b"vote", poll.key().as_ref(), &nullifier_hash],
    bump = vote.bump,
    constraint = vote.nullifier_hash == nullifier_hash @ VotingError::InvalidNullifier)]

    pub vote: Account<'info, Vote>,
    #[account(mut)]
    pub voter: Signer<'info>
}

pub fn handler ( ctx: Context<RecastVote>, new_vote_choice: u8, zk_proof: Vec<u8>, nullifier_hash: [u8; 32], public_signals: Vec<String>, new_vote_weight: Option<u64>) -> Result <()> {
    let poll =  &mut ctx.accounts.poll;
    let vote = &mut ctx.accounts.vote;
    let moment = Clock::get()?.unix_timestamp;

    // Validation for if the election is active or not
    require!(poll.is_active(), VotingError::PollNotActive);
    require!(moment <= poll.poll_end_time, VotingError::VotingEnded);

    // checks if there is room for new vote
    require!(poll.allow_recast, VotingError::RecastNotAllowed);

    if let Some(recast) = poll.recast_vote_window {
        let recast_end_time = poll.poll_begin_time.checked_add(recast).ok_or(VotingError::TimeOverflowError)?;
        require!(moment <= recast_end_time, VotingError::RecastWindowEnded);
    }

    require!((new_vote_choice as usize) < poll.options.len(), VotingError::InvalidVoteChoice);

    // Verificatiion 
    require!(!zk_proof.is_empty(), VotingError::InvalidZkProof);
    require!(verify_zk_proof(&zk_proof, &public_signals, &poll.eligibility_criteria)?, VotingError::ZkProofVerificatiionFailed);

    let new_vote_weight_finally = match poll.poll_type {
        PollType::Quadratic => {
            let quadratic = poll.quadratic_parameters.as_ref().ok_or(VotingError::MissingQuadraticParameters)?;
            let weight =  new_vote_weight.ok_or(VotingError::MissingVoteWeight)?;

            require!(weight > 0, VotingError::InvalidVoteWeight);

            let cost = calculate_quadratic_cost(&quadratic.credit_cost_curve, weight)?;
            require!(cost <= quadratic.max_credits, VotingError::NotEnoughCredits);

            weight
        },
        _ => 
            1,
        };

        let old_vote_weight = vote.weight;
        let old_vote_choice = vote.vote_choice as usize;

        // operation 
        poll.vote_counts[old_vote_choice] = poll.vote_counts[old_vote_choice].checked_sub(old_vote_weight).ok_or(VotingError::VoteCountUnderflow)?;
        poll.total_votes = poll.total_votes.checked_sub(old_vote_weight).ok_or(VotingError::VoteCountUnderflow)?;

        poll.vote_counts[new_vote_choice as usize] = poll.vote_counts[new_vote_choice as usize].checked_add(new_vote_weight_finally).ok_or(VotingError::VoteCountOverflow)?;
        poll.total_votes = poll.total_votes.checked_add(new_vote_weight_finally).ok_or(VotingError::VoteCountOverflow)?;

        poll.updated_at = moment;

        vote.vote_choice = new_vote_choice;
        vote.vote_weight = new_vote_weight_finally;
        vote.zk_proof_hash = hash_zk_proof(&zk_proof);
        vote.timestamp = moment;

        emit!(VoteRecast {
            poll: poll.key(),
            nullifier_hash,
            timestamp: moment,
            new_vote_weight = new_vote_weight_finally,
            old_vote_choice: old_vote_choice as u8,
            new_vote_choice: new_vote_choice,
            old_vote_weight,
        });

        msg!("Vote recast successfully for poll {}", poll.poll_name());
        Ok(())
}

