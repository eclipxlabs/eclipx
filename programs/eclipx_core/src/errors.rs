use anchor_lang::prelude::*;

#[error_code]
pub enum EclipxError {
    #[msg("Stealth mode is already active")]
    AlreadyActive,

    #[msg("Stealth mode is not active")]
    StealthModeInactive,

    #[msg("Invalid ZK proof hash")]
    InvalidProofHash,

    #[msg("Relay count exceeds maximum")]
    RelayCountExceeded,

    #[msg("Obfuscation level out of range")]
    InvalidObfuscationLevel,

    #[msg("Transaction record limit reached")]
    TransactionLimitReached,

    #[msg("Unauthorized access")]
    Unauthorized,
}
