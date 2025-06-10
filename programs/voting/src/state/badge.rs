use anchor_lang::prelude::*;
use super::BadgeType;

#[account]
pub struct UserBadge {
    pub badge_type: BadgeType,
    pub data: String,
    pub recipient: Pubkey,
    pub awarded_at: i64,
    pub is_active: bool,
    pub bump: u8,
}

impl UserBadge {
    pub const MAX_SIZE: usize = 8 +
        32 + // badge_type
        4 + 200 + // data
        32 + // recipient
        8 + // awarded_at
        1 + // is_active
        1; // bump
}