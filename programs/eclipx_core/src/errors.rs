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

// touch: 16d7abfa

// touch: 4cba9f2d

// touch: 53030d39

// touch: e0241897

// touch: 7da0a847

// touch: 6f144d45

// touch: dc4de2d5

// touch: b597864e

// touch: 4b3fe7cd

// touch: 34fffd45

// touch: ec8fd47d
