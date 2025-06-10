use anchor_lang::prelude::*;

#[error_code]
pub enum VotingError {
    #[msg("Voting is not currently allowed for this poll")]
    VotingNotAllowed,
    
    #[msg("Poll is not active")]
    PollNotActive,
    
    #[msg("Voting period has ended")]
    VotingEnded,
    
    #[msg("Poll has already been closed")]
    PollAlreadyClosed,
    
    #[msg("Cannot close poll at this time")]
    CannotClosePoll,
    
    #[msg("Unauthorized access - you are not the poll authority")]
    UnauthorizedAccess,
    
    #[msg("Invalid vote choice")]
    InvalidVoteChoice,
    
    #[msg("Voter has already cast a vote")]
    VoterAlreadyVoted,
    
    #[msg("ZK proof verification failed")]
    ZkProofVerificatiionFailed,
    
    #[msg("Invalid ZK proof provided")]
    InvalidZkProof,
    
    #[msg("Data provided is too long")]
    DataTooLong,
    
    #[msg("Poll name is too long")]
    PollNameTooLong,
    
    #[msg("Poll description is too long")]
    PollDescriptionTooLong,
    
    #[msg("Too many poll options provided")]
    TooManyOptions,
    
    #[msg("Poll option text is too long")]
    OptionTooLong,
    
    #[msg("Invalid poll duration")]
    InvalidPollDuration,
    
    #[msg("Poll start time must be in the future")]
    PollStartTimeInPast,
    
    #[msg("Poll end time must be after start time")]
    InvalidPollEndTime,
    
    #[msg("Cannot delegate vote to yourself")]
    CannotDelegateToSelf,
    
    #[msg("Delegation has expired")]
    DelegationExpired,
    
    #[msg("Invalid expiration time for delegation")]
    InvalidExpirationTime,
    
    #[msg("Expiration time cannot be after voting ends")]
    ExpirationAfterVotingEnded,
    
    #[msg("Delegation is not active")]
    DelegationNotActive,
    
    #[msg("Recast window has expired")]
    RecastWindowExpired,
    
    #[msg("Vote recasting is not allowed for this poll")]
    RecastNotAllowed,
    
    #[msg("Maximum voters limit reached")]
    MaxVotersReached,
    
    #[msg("Eligibility criteria not met")]
    EligibilityNotMet,
    
    #[msg("Invalid vote weight")]
    InvalidVoteWeight,
    
    #[msg("Insufficient credits for quadratic voting")]
    InsufficientCredits,
    
    #[msg("User not found")]
    UserNotFound,
    
    #[msg("Invalid nullifier hash")]
    InvalidNullifierHash,
    
    #[msg("Nullifier already used")]
    NullifierAlreadyUsed,
    
    #[msg("Invalid public signals")]
    InvalidPublicSignals,
    
    #[msg("Location verification failed")]
    LocationVerificationFailed,
    
    #[msg("DID verification failed")]
    DidVerificationFailed,
    
    #[msg("Invalid badge type")]
    InvalidBadgeType,
    
    #[msg("Badge already awarded")]
    BadgeAlreadyAwarded,
    
    #[msg("System is paused")]
    SystemPaused,
    
    #[msg("Operation not allowed")]
    OperationNotAllowed,
    
    #[msg("Invalid program state")]
    InvalidProgramState,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Arithmetic underflow")]
    ArithmeticUnderflow,
}
