use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::*;

#[event]
pub struct BadgeAwarded {
    pub recipient: Pubkey,
    pub badge_type: BadgeType,
    pub data: String,
    pub awarded_at: i64
}

#[derive(Accounts)]
#[instruction(badge_type: BadgeType, recipient: Pubkey)]
pub struct AwardBadge<'info> {
    #[account(init, payer = authority, space = UserBadge::MAX_SIZE, seeds = [b"badge", recipient.as_ref(), &badge_type as u8], bump)]
    pub user_badge: Account<'info, UserBadge>,
    #[account(mut, seeds = [b"voting", bump = voting.bump], constraint = voting.authority == authority.key() @ VotingError::UnauthorizedAccess)]
    pub voting: Account<'info, Voting>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub recipient: UncheckedAccount<'info>, // this perform checks if the recipient pubkey is validated in seeds.
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AwardBadge>, badge_type: BadgeType, data: String, recipient: Pubkey) -> Result<()> {
    let user_badge = &mut ctx.accounts.user_badge;
    let moment = Clock::get()?.unix_timestamp;

    require!(data.len() <= 200, VotingError::DataTooLong); // length validation of data

    // badge initiallisation
    user_badge.badge_type = badge_type.clone();
    user_badge.data = data;
    user_badge.recipient = recipient;
    user_badge.awarded_at = moment;
    user_badge.is_active = true;
    user_badge.bump = ctx.bumps.user_badge;

    emit!(BadgeAwarded {
        recipient: recipient,
        badge_type: badge_type.clone(),
        data: data.clone(),
        awarded_at: moment,
    });
    msg!("Badge {:?} awarded to {}", badge_type, recipient);
    Ok(())
}