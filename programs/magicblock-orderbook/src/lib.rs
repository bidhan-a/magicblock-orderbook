#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("66HyxPB7WgnmbjLLvE7LHN8HCAcugrFC5t9wKQFgaECY");

mod errors;
mod instructions;
mod state;

use instructions::*;

#[program]
pub mod magicblock_orderbook {
    use super::*;

    pub fn initialize_orderbook(ctx: Context<InitializeOrderbook>, seed: u64) -> Result<()> {
        ctx.accounts.initialize_orderbook(seed, &ctx.bumps)
    }

    pub fn register_trader(ctx: Context<RegisterTrader>) -> Result<()> {
        ctx.accounts.register_trader(&ctx.bumps)
    }

    // TODO: deposit_tokens and create_order can be potentially combined into a single instruction.

    pub fn deposit_tokens(ctx: Context<DepositTokens>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_tokens(amount)
    }

    pub fn create_order(ctx: Context<CreateOrder>) -> Result<()> {
        ctx.accounts.create_order()
    }

    pub fn delegate_orderbook(ctx: Context<DelegateOrderbook>) -> Result<()> {
        ctx.accounts.delegate_orderbook()
    }

    pub fn undelegate_orderbook(ctx: Context<UndelegateOrderbook>) -> Result<()> {
        ctx.accounts.undelegate_orderbook()
    }

    pub fn match_orders(ctx: Context<MatchOrders>) -> Result<()> {
        ctx.accounts.match_orders()
    }

    pub fn withdraw_tokens(ctx: Context<WithdrawTokens>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw_tokens(amount)
    }
}
