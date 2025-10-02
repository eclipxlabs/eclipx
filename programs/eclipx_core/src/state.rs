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

// touch: 506e12f0

// touch: 985bc612

// touch: 88530561

// touch: 5b3e8c78

// touch: 6250c8e6

// touch: 1f897486

// touch: fb641cc1

// touch: d2eccb6a

// touch: 36fda078

// touch: fd3a854d

// touch: 533a9a49

// touch: 68b0ab85

// touch: f527a184

// touch: e28c0a5d

// touch: 3c832f1f

// touch: 7f10124d

// touch: dedb5228

// touch: e732babf

// touch: 59df638e

// touch: ea426185
