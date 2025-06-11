use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::Orderbook;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeOrderbook<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_base: InterfaceAccount<'info, Mint>,
    pub mint_quote: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer=user,
        associated_token::mint=mint_base,
        associated_token::authority=orderbook
    )]
    pub vault_mint_base: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer=user,
        associated_token::mint=mint_quote,
        associated_token::authority=orderbook
    )]
    pub vault_mint_quote: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer=user,
        space=Orderbook::INIT_SPACE + 8,
        seeds=[b"orderbook", seed.to_le_bytes().as_ref()],
        bump
    )]
    pub orderbook: Account<'info, Orderbook>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeOrderbook<'info> {
    pub fn initialize_orderbook(
        &mut self,
        seed: u64,
        bumps: &InitializeOrderbookBumps,
    ) -> Result<()> {
        self.orderbook.set_inner(Orderbook {
            seed,
            mint_base: self.mint_base.key(),
            mint_quote: self.mint_quote.key(),
            bump: bumps.orderbook,
        });
        Ok(())
    }
}
