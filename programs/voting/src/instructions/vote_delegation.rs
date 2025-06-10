use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::*;
use crate::utils::*;

#[event]
pub struct VoteDelegationCreated {
    poll : Pubkey,
    delegator : Pubkey,
    delegatee : Pubkey,
    created_at : i64,
    expires_at : Option<i64>,
    nullifier_hash : [u8; 32],
}

#[derive(Accounts)]
#[instruction(nullifier_hash: [u8; 32], delegate_pubkey: Pubkey)]
pub struct DelegateVote<'info> {
    #[account(mut, constraint = poll.is_active() @ VotingError::PollNotActive)]
    pub poll: Account<'info, Poll>,
    #[account(init, payer = o_delegator, space = VoteDelegation::MAX_SIZE, seeds = [b"delegation", poll.key().as_ref(), &nullifier_hash], bump)]
    pub vote_delegation: Account<'info, VoteDelegation>,
    #[account(init, payer = o_delegator, space = VoterNullifier::MAX_SIZE, seeds = [b"nullifier", poll.key().as_ref(), &nullifier_hash], bump)]
    pub voter_nullifier: Account<'info, VoterNullifier>,
    #[account(mut)]
    pub o_delegator: Signer<'info>,
    pub o_delegatee: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler (
    ctx: Context<DelegateVote>,
    zk_proof: Vec<u8>,
    delegate_pubkey: Pubkey,
    nullifier_hash: [u8; 32],
    public_signals: Vec<String>,
    expires_at: Option<i64>,
) -> Result<()> {
    let poll = &mut ctx.accounts.poll;
    let moment = Clock::get()?.unix_timestamp;
    let vote_delegation = &mut ctx.accounts.vote_delegation;
    let voter_nullifier = &mut ctx.accounts.voter_nullifier;

    require!(poll.is_active(), VotingError::PollNotActive);
    require!(moment <= poll.poll_end_time, VotingError::VotingEnded);

    // delgate and delegeteee validation
    require!(delegate_pubkey != ctx.accounts.o_delegator.key(), VotingError::CannotDelegateToSelf);

    // verification
    require!(!zk_proof.is_empty(), VotingError::InvalidZkProof);
    require!(verify_zk_proof(&zk_proof, &public_signals, &poll.eligibility_criteria)?, VotingError::ZkProofVerificatiionFailed);


    if let Some(timeout) = expires_at {
        require!(timeout > moment, VotingError::InvalidExpirationTime);
        require!(timeout <= poll.poll_end_time, VotingError::ExpirationAfterVotingEnded);
    }

    // initialisation of the vote_delegation
    vote_delegation.poll = poll.key();
    vote_delegation.o_delegator = ctx.accounts.o_delegator.key();
    vote_delegation.is_active = true;
    vote_delegation.o_delegatee = delegate_pubkey;
    vote_delegation.created_at = moment;
    vote_delegation.expires_at = expires_at;
    vote_delegation.bump = ctx.bumps.vote_delegation;

    // initialisation of the voter_nullifier
    voter_nullifier.poll = poll.key();
    voter_nullifier.nullifier_hash = nullifier_hash;
    voter_nullifier.bump = ctx.bumps.voter_nullifier;
    voter_nullifier.created_at = moment;

    emit!(VoteDelegationCreated {
        poll: poll.key(),
        delegator: ctx.accounts.o_delegator.key(),
        delegatee: delegate_pubkey,
        created_at: moment,
        expires_at: expires_at,
        nullifier_hash: nullifier_hash,
    });
    msg!("Vote delegated successfully for poll: {}", poll.name);
    Ok(())
}
 
