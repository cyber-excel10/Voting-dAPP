use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = Voting::MAX_SIZE,
        seeds = [b"voting"],
        bump
    )]
    pub voting: Account<'info, Voting>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let voting = &mut ctx.accounts.voting;

    voting.authority = ctx.accounts.authority.key();
    voting.total_polls = 0;
    voting.total_votes = 0;
    voting.total_voters = 0;
    voting.total_users = 0;
    voting.created_at = Clock::get()?.unix_timestamp;
    voting.is_paused = false;
    voting.bump = *ctx.bumps.voting;

    msg!("Voting initialized successfully");
    Ok(())
}