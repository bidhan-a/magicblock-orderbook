use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use ephemeral_rollups_sdk::anchor::delegate;
use ephemeral_rollups_sdk::cpi::DelegateConfig;

use crate::state::Orderbook;

#[delegate]
#[derive(Accounts)]
pub struct DelegateOrderbook<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_base: InterfaceAccount<'info, Mint>,
    pub mint_quote: InterfaceAccount<'info, Mint>,

    #[account(
        mut, 
        del, 
        seeds=[b"orderbook", pda.seed.to_le_bytes().as_ref()], 
        bump=pda.bump,
        has_one=mint_base,
        has_one=mint_quote,
    )]
    pub pda: Account<'info, Orderbook>,
}

impl<'info> DelegateOrderbook<'info> {
    pub fn delegate_orderbook(&mut self) -> Result<()> {
        self.delegate_pda(
            &self.user,
            &[b"orderbook", self.pda.seed.to_le_bytes().as_ref()],
            DelegateConfig::default(),
        )?;
        Ok(())
    }
}
