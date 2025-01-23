use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use crate::{instructions::*, state::*};

declare_id!("GALCB7KNTUruL8dZwZ9HbcdDP2EX7W66QDgpEARqCuDs");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<InitializeVaulte>) -> Result<()> {
        ctx.accounts.initialize_vault(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposite(ctx: Context<DepositeInstruction>, amount: u64) -> Result<()> {
        ctx.accounts.deposite_instruction(amount)?;
        Ok(())
    }

    pub fn with_draw_ammount(ctx: Context<WithDrawAmount>, amount: u64) -> Result<()> {
        ctx.accounts.with_draw_amount_instruction(amount)?;
        Ok(())
    }

    pub fn with_draw_all(ctx: Context<WithDrawAll>) -> Result<()> {
        ctx.accounts.withdraw_all()?;
        Ok(())
    }
}
