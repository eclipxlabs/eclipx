use anchor_lang::prelude::*;

pub mod compression;
pub mod contexts;
pub mod errors;
pub mod relay;
pub mod state;

pub use contexts::*;

declare_id!("Ec1pXmjiR5DVFhbYr4urnUdFKNhKwgjJXwbkFCfCa5sM");

#[program]
pub mod eclipx_core {
    use super::*;

    pub fn initialize_privacy_state(
        ctx: Context<InitializePrivacyState>,
        relay_count: u8,
        obfuscation_level: u8,
    ) -> Result<()> {
        let state = &mut ctx.accounts.privacy_state;
        state.authority = ctx.accounts.authority.key();
        state.relay_count = relay_count;
        state.obfuscation_level = obfuscation_level.min(5);
        state.is_active = false;
        state.total_transactions = 0;
        state.privacy_score = 0;
        state.bump = ctx.bumps.privacy_state;
        state.created_at = Clock::get()?.unix_timestamp;

        emit!(PrivacyStateCreated {
            authority: state.authority,
            relay_count,
            obfuscation_level: state.obfuscation_level,
        });

        Ok(())
    }

    pub fn activate_stealth_mode(ctx: Context<ActivateStealthMode>) -> Result<()> {
        let state = &mut ctx.accounts.privacy_state;
        require!(!state.is_active, errors::EclipxError::AlreadyActive);

        state.is_active = true;
        state.activated_at = Clock::get()?.unix_timestamp;

        emit!(StealthModeActivated { authority: state.authority, timestamp: state.activated_at });

        Ok(())
    }

    pub fn submit_compressed_tx(
        ctx: Context<SubmitCompressedTx>,
        zk_proof_hash: [u8; 32],
        merkle_root: [u8; 32],
        relay_path_hash: [u8; 32],
        encrypted_amount: u64,
    ) -> Result<()> {
        let state = &mut ctx.accounts.privacy_state;
        require!(state.is_active, errors::EclipxError::StealthModeInactive);

        let tx_record = &mut ctx.accounts.tx_record;
        tx_record.authority = ctx.accounts.authority.key();
        tx_record.zk_proof_hash = zk_proof_hash;
        tx_record.merkle_root = merkle_root;
        tx_record.relay_path_hash = relay_path_hash;
        tx_record.encrypted_amount = encrypted_amount;
        tx_record.timestamp = Clock::get()?.unix_timestamp;
        tx_record.status = compression::TxStatus::Confirmed as u8;
        tx_record.bump = ctx.bumps.tx_record;

        state.total_transactions = state.total_transactions.saturating_add(1);
        state.privacy_score = relay::calculate_privacy_score(
            state.relay_count,
            state.obfuscation_level,
            state.total_transactions,
        );

        emit!(CompressedTxSubmitted {
            authority: state.authority,
            zk_proof_hash,
            merkle_root,
            tx_index: state.total_transactions,
        });

        Ok(())
    }

    pub fn deactivate_stealth_mode(ctx: Context<DeactivateStealthMode>) -> Result<()> {
        let state = &mut ctx.accounts.privacy_state;
        require!(state.is_active, errors::EclipxError::StealthModeInactive);
        state.is_active = false;

        emit!(StealthModeDeactivated { authority: state.authority });

        Ok(())
    }
}

#[event]
pub struct PrivacyStateCreated {
    pub authority: Pubkey,
    pub relay_count: u8,
    pub obfuscation_level: u8,
}

#[event]
pub struct StealthModeActivated {
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct CompressedTxSubmitted {
    pub authority: Pubkey,
    pub zk_proof_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub tx_index: u64,
}

#[event]
pub struct StealthModeDeactivated {
    pub authority: Pubkey,
}

// touch: eb8c292e

// touch: 7bc3f965
