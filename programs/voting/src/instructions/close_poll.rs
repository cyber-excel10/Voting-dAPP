use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::*;

#[event]
pub struct PollClosed {
    pub poll: Pubkey,
    pub authority: Pubkey,
    pub total_votes: u64,
    pub top_index: u8,
    pub winner_votes: u64,
    pub unique_voters: u64,
    pub ended_at: i64,
}

#[derive(Accounts)]
pub struct ClosePoll<'info> {
    #[account(mut, constraint = poll.authority == authority.key() @ VotingError::UnauthorizedAccess, constraint = poll.status == PollStatus::Active || poll.status == PollStatus::Pending @ VotingError::PollAlreadyClosed)]
    pub poll: Account<'info, Poll>,
    #[account(seeds = [b"poll_metadata", poll.key().as_ref()], bump = poll_metadata.bump)]
    pub poll_metadata: Account<'info, PollMetadata>,
    pub authority: Signer<'info>,
}

pub fn handler( ctx: Context<ClosePoll>) -> Result<()> {
    let poll = &mut ctx.accounts.poll;
    let moment = Clock::get()?.unix_timestamp;

    let closure = moment >= poll.poll_end_time || poll.authority == ctx.accounts.authority.key();
    require!(closure, VotingError::CannotClosePoll);

    poll.status = PollStatus::Closed;
    poll.updated_at = moment;

    // polll winnner determination for multi-choice poll
    let winner_position =  poll.vote_counts.iter().enumerate().max_by_key(|(_, &count) | count).map(| (position, _)| position).unwrap_or(0);

    emit!(PollClosed {
        poll: poll.key(),
        authority: poll.authority,
        total_votes: poll.total_votes,
        top_index: winner_position as u8,
        winner_votes: poll.vote_counts[winner_position],
        unique_voters: poll.unique_voters,
        ended_at: moment,
    });
    msg!("Poll closed: {} - Winner: {}", poll.name, poll.options[winner_position]);

    Ok(())
}