use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::state::{Orderbook, Trader};

#[derive(Accounts)]
pub struct MatchOrders<'info> {
    // TODO
    pub system_program: Program<'info, System>,
}

impl<'info> MatchOrders<'info> {
    pub fn match_orders(&mut self) -> Result<()> {
        // TODO
        Ok(())
    }
}
