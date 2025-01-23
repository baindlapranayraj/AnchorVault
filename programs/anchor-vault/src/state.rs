use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub amount_deposited: u64,
    pub amount_withdraw: u64,
    pub vault_state_bump: u8,
    pub vault_account_bump: u8,
}

impl VaultState {
    pub fn len() -> usize {
        let lenghth = 8 + 8 + 1 + 1;
        return lenghth;
    }
}
