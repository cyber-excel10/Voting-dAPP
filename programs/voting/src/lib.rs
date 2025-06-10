use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

pub mod state;
pub mod instructions;
pub mod error;
pub mod utils;

use instructions::*;
use state::*;
use error::*;

declare_id!();   // For now it is still empty it as not  yet be deployed on a testnet..

#[program]
pub mod globalvote{
    use std::task::Context;

    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    // Creation of a new poll
    pub fn create_poll(
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
    ) -> Result <()> {
        instructions::create_poll::handler(ctx, poll_name, options, description, eligibility_criteria, poll_begin_time, poll_end_time, max_voters, poll_type, geolocation_required, quadratic_parameters, allow_recast, recast_vote_window)
    }

    // This allows to cast a vote with Zk proof
    pub fn cast_vote(
        ctx: Context<CastVote>, 
        vote_choice: u8, 
        zk_proof: Vec<u8>, 
        nullifier_hash: [u8; 32], 
        public_signals: Vec<String>, 
        vote_weight: Option<u64>,) -> Result <()> {
        instructions::cast_vote::handler(ctx, vote_choice, zk_proof, nullifier_hash, public_signals, vote_weight)
    }
        pub fn recast_vote(ctx: Context<RecastVote>, new_vote_choice: u8, zk_proof: Vec<u8>, nullifier_hash: [u8; 32], public_signals: Vec<String>, new_vote_weight: Option<u64>,) -> Result <()> {
            instructions::recast_vote::handler(ctx, new_vote_choice, zk_proof, nullifier_hash, public_signals, new_vote_weight,)
        }
        pub fn close_poll(ctx: Context<ClosePoll>) -> Result<()> {
            instructions::close_poll::handler(ctx)
        }

        pub fn award_badge(ctx: Context<AwardBadge>, badge_type: BadgeType, recipient: Pubkey, data: String) -> Result<()> {
            instructions::award_badge::handler(ctx, badge_type, recipient, data)
        }

        pub fn delegate_vote(ctx: Context<DelegateVote>, zk_proof: Vec<u8>, delegate_pubkey: Pubkey, nullifier_hash: [u8; 32], public_signals: Vec<String>, expires_at: Option<i64>,) -> Result<()> {
            instructions::vote_delegation::handler(ctx, zk_proof, delegate_pubkey, nullifier_hash, public_signals, expires_at,)
        }

        pub fn initialize_user(ctx: Context<InitializeUser>, did_identifier: Option<String>, location_hash: Option<[u8; 32]>, preferred_language: Option<String>,) -> Result<()> {
            instructions::initialize_user::handler(ctx, did_identifier, location_hash, preferred_language,)
        }

        pub fn emergency_pause(ctx: Context<EmergencyPause>) -> Result<()> {
            instructions::emergency_pause::handler(ctx)
        }

        pub fn resume_operations(ctx: Context<ResumeOperations>) -> Result<()> {
            instructions::resume_operations::handler(ctx)
        }

        pub fn cast_delegated_vote(ctx: Context<CastDelegatedVote>, vote_choice: u8, vote_weight: Option<u64>, zk_proof: Vec<u8>, public_signals: Vec<String>,  delegator_nullifier: [u8; 32], delegate_nullifier: [u8; 32], ) -> Result <()> {
            instructions::cast_delegated_vote::handler(ctx, vote_choice, zk_proof, delegator_nullifier, delegate_nullifier, public_signals, vote_weight, )
        }

        pub fn update_user_preferences(ctx: Context<UpdateUserPreferences>, notification_settings: NotificationSettings, privacy_settings: PrivacySettings, accessibility_settings: AccessibilitySettings) -> Result <()> { instructions::update_user_preferences::handler(ctx, notification_settings, privacy_settings, accessibility_settings)
    }
}
