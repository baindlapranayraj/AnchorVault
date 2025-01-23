use anchor_lang::prelude::*;

use crate::{
    constants::{VAULT_ACCOUNT_SEED, VAULT_STATE_SEED},
    VaultState,
};

#[derive(Accounts)]
pub struct InitializeVaulte<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>, // Signer Account

    #[account(
        init,
        payer = depositer,
        seeds = [VAULT_STATE_SEED,depositer.key.to_bytes().as_ref()],
        bump,
        space = 8 + VaultState::len(),
    )]
    pub vault_state: Account<'info, VaultState>, // vault_state controlled by this program, it is created during the transaction

    #[account(
        seeds = [VAULT_ACCOUNT_SEED,vault_state.key().as_ref()],
        bump
    )]
    pub vault_account: SystemAccount<'info>, // Vault Account Controlled by system program

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeVaulte<'info> {
    pub fn initialize_vault(&mut self, bump: &InitializeVaulteBumps) -> Result<()> {
        *self.vault_state = VaultState {
            amount_deposited: 0,
            amount_withdraw: 0,
            vault_account_bump: bump.vault_account,
            vault_state_bump: bump.vault_state,
        };

        Ok(())
    }
}
