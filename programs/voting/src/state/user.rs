use anchor_lang::prelude::*;
use super::BadgeType;

#[account]
pub struct User {
    pub pubkey: Pubkey,
    pub did_identifier: Option<String>,
    pub total_votes_cast: u64,
    pub total_polls_created: u64,
    pub total_delegations_given: u64,
    pub total_delegations_received: u64,
    pub reputation_score: u64,
    pub is_verified: bool,
    pub verification_level: u8,
    pub created_at: i64,
    pub last_activity: i64,
    pub preferred_language: Option<String>,
    pub location_hash: Option<[u8; 32]>,
    pub badges_earned: Vec<BadgeType>,
    pub is_active: bool,
    pub bump: u8
}

impl User {
    pub const MAX_SIZE: usize = 8 + 
    32 + 
    1 + 4 + 100 + 
    8 + 8 + 8 + 8 + 8 + 
    1 + 1 + 
    8 + 
    8 + 
    1 + 32 + 
    4 + (10 * 1) + 
    1 +
    1;
        pub fn update_activity(&mut self) -> Result<()> {
        self.last_activity = Clock::get()?.unix_timestamp;
        Ok(())
    }
        pub fn add_badge(&mut self, badge_type: BadgeType) -> Result<()> {
        if !self.badges_earned.contains(&badge_type) {
            self.badges_earned.push(badge_type);
        }
        Ok(())
    }

    pub fn calculate_reputation(&self) -> u64 {
        let base_score = self.total_votes_cast * 10;
        let creator_bonus = self.total_polls_created * 50;
        let delegation_bonus = self.total_delegations_received * 5;
        let badge_bonus = self.badges_earned.len() as u64 * 25;
        
        base_score + creator_bonus + delegation_bonus + badge_bonus
    }
}

#[account]
pub struct UserStats  {
    pub user: Pubkey,
    pub polls_participated: Vec<Pubkey>,
    pub avg_participation_time: u64,
    pub streak_count: u32,
    pub last_streak_date: i64,
    pub civic_engagement_score: u64,
    pub bump: u8,
}

impl UserStats {
    pub const MAX_SIZE: usize = 8 +
    32 + // user
    4 + (100 * 32) + // polls_participated
    8 + // avg_participation_time
    4 + // streak_count
    8 + // last_streak_date
    8 + // civic_engagement_score
    1; // bump
}

#[account]
pub struct UserPreferences {
    pub user: Pubkey,
    pub notification_settings: NotificationSettings,
    pub privacy_settings: PrivacySettings,
    pub accessibility_settings: AccessibilitySettings,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NotificationSettings {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub poll_reminders: bool,
    pub result_notifications: bool,
    pub delegation_alerts: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PrivacySettings {
    pub hide_voting_history: bool,
    pub anonymous_participation: bool,
    pub location_sharing: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AccessibilitySettings {
    pub high_contrast_mode: bool,
    pub large_text: bool,
    pub screen_reader_mode: bool,
    pub keyboard_navigation: bool,
}

impl UserPreferences {
    pub const MAX_SIZE: usize = 8 +
    32 + // user
    5 + // notification_settings
    3 + // privacy_settings
    4 + // accessibility_settings
    1; // bump
}



