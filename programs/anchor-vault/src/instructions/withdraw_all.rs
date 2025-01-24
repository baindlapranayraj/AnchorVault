use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    constants::{VAULT_ACCOUNT_SEED, VAULT_STATE_SEED},
    VaultState,
};

#[derive(Accounts)]
pub struct WithDrawAll<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    #[account(
        mut,
        seeds =[VAULT_STATE_SEED,depositer.key.as_ref()],
        bump = vault_state.vault_state_bump,
        close = depositer
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED,vault_state.key().as_ref()],
        bump = vault_state.vault_account_bump
    )]
    pub vault_account: SystemAccount<'info>,

    pub system_account: Program<'info, System>,
}

impl<'info> WithDrawAll<'info> {
    pub fn withdraw_all(&mut self) -> Result<()> {
        let cpi_program = self.system_account.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info(),
            to: self.depositer.to_account_info(),
        };

        let seeds = &[
            VAULT_ACCOUNT_SEED,
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_account_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_context, self.vault_account.lamports())?;

        // TODO:- Close the systemAccount hear
        // Now close the vault account and send any remaining lamports to destination

        Ok(())
    }
}
