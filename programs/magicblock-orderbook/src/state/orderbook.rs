use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Orderbook {
    pub seed: u64,
    pub mint_base: Pubkey,
    pub mint_quote: Pubkey,
    pub bump: u8,
    // TODO: How to store orders? In a Vec<..>?
}
