use anchor_lang::prelude::*;

pub mod badge;
pub mod poll;
pub mod user;
pub mod vote;
pub mod voting;

pub use badge::*;
pub use poll::*;
pub use user::*;
pub use vote::*;
pub use voting::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PollType{
    MultiChoice,
    RankedChoice,
    YesNo,
    AnonymousFeedback,
    Quadratic,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EligibilityCriteria {
    pub min_age: Option<u8>,
    pub required_location : Option<String>,
    pub required_citezenship : Option<String>,
    pub custom_criteria : Vec<String>,
    pub require_custom_criteria : bool
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BadgeType {
    FirstVote,
    Auditor,
    Delegate,
    FrequentVoter,
    CivicLeader,
    PollCreator,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PollStatus {
    Pending,
    Active,
    Cancelled,
    Closed,
}