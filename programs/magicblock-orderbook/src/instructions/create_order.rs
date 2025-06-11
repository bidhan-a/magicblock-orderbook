use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::state::{Orderbook, Trader};

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_base: InterfaceAccount<'info, Mint>,
    pub mint_quote: InterfaceAccount<'info, Mint>,

    #[account(
        seeds=[b"orderbook", orderbook.seed.to_le_bytes().as_ref()],
        bump=orderbook.bump,
        has_one=mint_base,
        has_one=mint_quote,
    )]
    pub orderbook: Account<'info, Orderbook>,

    #[account(
        init,
        payer=user,
        space=Trader::INIT_SPACE + 8,
        seeds=[b"trader", user.key().as_ref(), orderbook.key().as_ref()],
        bump
    )]
    pub trader: Account<'info, Trader>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateOrder<'info> {
    pub fn create_order(&mut self) -> Result<()> {
        // TODO: Add order to order book.
        Ok(())
    }
}
