use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Trader {
    pub user: Pubkey,
    pub mint_base_balance: u64,
    pub mint_quote_balance: u64,
    pub bump: u8,
}
