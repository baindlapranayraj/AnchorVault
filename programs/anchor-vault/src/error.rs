use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("You are trying to withdraw more then you have deposited")]
    InvalidAmountWithDraw,
}
