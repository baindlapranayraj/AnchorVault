use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    constants::{VAULT_ACCOUNT_SEED, VAULT_STATE_SEED},
    error::VaultError,
    VaultState,
};

#[derive(Accounts)]
pub struct WithDrawAmount<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_STATE_SEED,depositer.key.to_bytes().as_ref()],
        bump = vault_state.vault_state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [VAULT_ACCOUNT_SEED,vault_state.key().as_ref()],
        bump = vault_state.vault_account_bump
    )]
    pub vault_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> WithDrawAmount<'info> {
    pub fn with_draw_amount_instruction(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        require!(
            amount < self.vault_state.amount_deposited,
            VaultError::InvalidAmountWithDraw
        );

        // TODO:- How to Programatically close the account ?

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

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds); // PDA signing require seeds.

        transfer(cpi_context, amount)?;

        self.vault_state.amount_withdraw += amount;
        self.vault_state.amount_deposited -= amount;

        Ok(())
    }
}
