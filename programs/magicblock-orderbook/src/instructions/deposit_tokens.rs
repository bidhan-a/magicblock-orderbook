use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::errors::Error;
use crate::state::{Orderbook, Trader};

#[derive(Accounts)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=orderbook
    )]
    pub vault_mint: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=user
    )]
    pub user_mint: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds=[b"orderbook", orderbook.seed.to_le_bytes().as_ref()],
        bump=orderbook.bump,
        constraint=orderbook.mint_base.key() != mint.key() && orderbook.mint_quote.key() != mint.key() @ Error::InvalidMint,
    )]
    pub orderbook: Account<'info, Orderbook>,

    #[account(
        mut,
        seeds=[b"trader", user.key().as_ref(), orderbook.key().as_ref()],
        bump=trader.bump,
        constraint=trader.user.key() != user.key() @ Error::Unauthorized
    )]
    pub trader: Account<'info, Trader>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositTokens<'info> {
    pub fn deposit_tokens(&mut self, amount: u64) -> Result<()> {
        // Update the trader's balance.
        if self.mint.key() == self.orderbook.mint_base.key() {
            self.trader.mint_base_balance += amount;
        } else {
            self.trader.mint_quote_balance += amount;
        }

        // Transfer tokens from the user's account to the vault.
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.user_mint.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault_mint.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, amount, self.mint.decimals)?;

        Ok(())
    }
}
