use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct PrivacyState {
    pub authority: Pubkey,
    pub relay_count: u8,
    pub obfuscation_level: u8,
    pub is_active: bool,
    pub total_transactions: u64,
    pub privacy_score: u16,
    pub bump: u8,
    pub created_at: i64,
    pub activated_at: i64,
    pub _padding: [u8; 6],
}

impl PrivacyState {
    pub const LEN: usize = 32 + 1 + 1 + 1 + 8 + 2 + 1 + 8 + 8 + 6;
}

#[account]
#[derive(Default)]
pub struct TransactionRecord {
    pub authority: Pubkey,
    pub zk_proof_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub relay_path_hash: [u8; 32],
    pub encrypted_amount: u64,
    pub timestamp: i64,
    pub status: u8,
    pub bump: u8,
    pub _padding: [u8; 6],
}

impl TransactionRecord {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 6;
}

// touch: 3c9a110c

// touch: 10e601d1

// touch: 2b7e3457

// touch: 27d12344

// touch: f15a115c

// touch: 75411988
