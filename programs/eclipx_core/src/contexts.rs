use crate::state::{PrivacyState, TransactionRecord};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializePrivacyState<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + PrivacyState::LEN,
        seeds = [b"privacy", authority.key().as_ref()],
        bump,
    )]
    pub privacy_state: Account<'info, PrivacyState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActivateStealthMode<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"privacy", authority.key().as_ref()],
        bump = privacy_state.bump,
        has_one = authority,
    )]
    pub privacy_state: Account<'info, PrivacyState>,
}

#[derive(Accounts)]
pub struct SubmitCompressedTx<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"privacy", authority.key().as_ref()],
        bump = privacy_state.bump,
        has_one = authority,
    )]
    pub privacy_state: Account<'info, PrivacyState>,

    #[account(
        init,
        payer = authority,
        space = 8 + TransactionRecord::LEN,
        seeds = [
            b"tx_record",
            authority.key().as_ref(),
            &privacy_state.total_transactions.to_le_bytes(),
        ],
        bump,
    )]
    pub tx_record: Account<'info, TransactionRecord>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeactivateStealthMode<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"privacy", authority.key().as_ref()],
        bump = privacy_state.bump,
        has_one = authority,
    )]
    pub privacy_state: Account<'info, PrivacyState>,
}

// touch: 794b1844

// touch: e57dec3c

// touch: 67fbe083

// touch: ad15faac

// touch: f1ec4a90
