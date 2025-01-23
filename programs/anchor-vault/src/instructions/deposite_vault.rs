use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    constants::{VAULT_ACCOUNT_SEED, VAULT_STATE_SEED},
    VaultState,
};

#[derive(Accounts)]
pub struct DepositeInstruction<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_STATE_SEED,depositer.key.to_bytes().as_ref()],
        bump = vault_state.vault_state_bump // no need to recalculate the bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED,vault_state.key().as_ref()],
        bump = vault_state.vault_account_bump // no need to recalculate the bump
    )]
    pub vault_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> DepositeInstruction<'info> {
    pub fn deposite_instruction(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info(); // program

        let cpi_accounts = Transfer {
            from: self.depositer.to_account_info(),
            to: self.vault_account.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_context, amount)?;

        self.vault_state.amount_deposited = self.vault_state.amount_deposited + amount;

        Ok(())
    }
}
