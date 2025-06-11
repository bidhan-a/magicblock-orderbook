use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;

use crate::state::Orderbook;

#[commit]
#[derive(Accounts)]
pub struct UndelegateOrderbook<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_base: InterfaceAccount<'info, Mint>,
    pub mint_quote: InterfaceAccount<'info, Mint>,

    #[account(
        mut, 
        seeds=[b"orderbook", pda.seed.to_le_bytes().as_ref()], 
        bump=pda.bump,
        has_one=mint_base,
        has_one=mint_quote,
    )]
    pub pda: Account<'info, Orderbook>,
}

impl<'info> UndelegateOrderbook<'info> {
    pub fn undelegate_orderbook(&mut self) -> Result<()> {
        commit_and_undelegate_accounts(&self.user, vec![&self.pda.to_account_info()], &self.magic_context, &self.magic_program)?;
        Ok(())
    }
}
